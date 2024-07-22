use aes::cipher::{generic_array::GenericArray, BlockDecrypt, BlockEncrypt, KeyInit};
use aes::Aes128;
use openssl::{
    base64::{decode_block, encode_block},
    hash::MessageDigest,
    sign::Verifier,
    x509::X509,
};
use reqwest::Url;
use std::fs;
use std::future::Future;
//use anyhow::{anyhow, Result};
use crate::alipay::prelude::*;
use crate::error::WeaError;
use crate::*;
use serde::de::DeserializeOwned;
use serde_json;

pub trait BaseTrait {
    /// create order
    /// method format like alipay.trade.app.pay
    fn create_order(
        &self,
        method: &str,
        data: ReqOrderBody,
        with_aes: bool,
    ) -> impl Future<Output = Result<ResOrderBody, WeaError>>;
    /// 构建请求client 同时设置好请求头
    #[allow(dead_code)]
    fn build_request_builder(
        &self,
        url: &str,
        method: &str,
        body: &str,
        with_aes: bool,
    ) -> Result<reqwest::RequestBuilder, WeaError>;
    /// 发起请求同时会根据传入的类型返回对应的结果
    #[allow(dead_code)]
    fn do_request<U: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: &str,
        with_aes: bool,
    ) -> impl Future<Output = Result<U, WeaError>>;
    /// method format like alipay.trade.app.pay
    #[allow(dead_code)]
    fn get_uri(&self, method: &str) -> String;
    /// 验证签名
    /// data 为验证签名的数据  vec!['1395712654', 'nonce_str', 'body']
    /// HTTP 头 alipay-signature：支付宝生成的签名内容，用于校验请求签名。
    /// HTTP 头 alipay-sn：支付宝使用的证书号，使用证书模式时会返回。商家需要确保使用的支付宝证书和该证书号一致，不一致则需要更新支付宝证书公钥。
    /// HTTP 头 alipay-timestamp：支付宝应答时间戳。
    /// HTTP 头 alipay-nonce：支付宝应答随机串。
    /// HTTP 报文主体 httpResponseBody：HTTP 报文内容。
    /// 待验签串拼接规则如下：
    /// ${alipay-timestamp}\n
    /// ${alipay-nonce}\n
    /// ${httpResponseBody}\n
    fn verify_signature(&self, data: Vec<&str>, signature: &str) -> Result<bool, WeaError>;
    /// 加密
    fn encrypt(&self, data: &str) -> Result<String, WeaError>;
    /// 解密
    fn decrypt(&self, data: &str) -> Result<String, WeaError>;
}

impl BaseTrait for Payment<AlipayConfig> {
    //create order
    fn create_order(
        &self,
        method: &str,
        data: ReqOrderBody,
        with_aes: bool,
    ) -> impl Future<Output = Result<ResOrderBody, WeaError>> {
        async move {
            let url = self.get_uri(method);
            let order_body = serde_json::to_string(&data)?;
            self.do_request::<ResOrderBody>(&url, &"POST", &order_body, with_aes)
                .await
        }
    }

    //get uri
    fn get_uri(&self, method: &str) -> String {
        let url = method.replace(".", "/");
        format!("/v3/{}", url)
    }
    // build_request_builder
    fn build_request_builder(
        &self,
        url: &str,
        method: &str,
        body: &str,
        with_aes: bool,
    ) -> Result<reqwest::RequestBuilder, WeaError> {
        let base_url = match self.config.is_sandbox {
            true => "https://openapi.alipay.com",
            false => "https://openapi-sandbox.dl.alipaydev.com",
        };

        let base_url = Url::parse(base_url).map_err(|_e| e("parse url error"))?;
        let full_url = base_url.join(url).map_err(|_e| e("join url error"))?;
        let full_url = full_url.as_str();
        let timestamp = get_timestamp_millis()?.to_string();
        let nonce_str = generate_random_string(32);
        let request_id = format!("{}{}", generate_random_string(10), timestamp);
        let app_serial_no = get_cert_serial(&self.config.app_public_cert.clone())?;
        let alipay_root_serial_no = get_cert_serial(&self.config.alipay_root_cert.clone())?;
        let auth_string = format!(
            "app_id={},app_cert_sn={},nonce={},timestamp={}",
            &self.config.app_id, &app_serial_no, nonce_str, timestamp
        );
        let body = if with_aes {
            let body = self.encrypt(body)?;
            body
        } else {
            body.to_string()
        };
        let sign_data: Vec<&str> = vec![&auth_string, method, url, &body];
        let signature = generate_signature(sign_data, &self.config.app_private_key)?;
        let authorization = format!("ALIPAY-SHA256withRSA {},sign={}", auth_string, signature);
        let client = reqwest::Client::new();
        let req_builder = match method {
            "GET" => client.get(full_url),
            "POST" => client.post(full_url),
            "PUT" => client.put(full_url),
            "DELETE" => client.delete(full_url),
            _ => client.get(full_url),
        };
        let req_builder = if !body.is_empty() {
            req_builder.body(body.to_string())
        } else {
            req_builder
        };
        let req_builder = if with_aes {
            req_builder.header("alipay-encrypt-type", "AES")
        } else {
            req_builder
        };
        let req_builder = req_builder
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "Weapay rust sdk/0.1.0")
            .header("alipay-request-id", request_id)
            .header("alipay-root-cert-sn", alipay_root_serial_no)
            .header("Authorization", authorization);
        Ok(req_builder)
    }
    // do request
    fn do_request<U: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: &str,
        with_aes: bool,
    ) -> impl Future<Output = Result<U, WeaError>> {
        async move {
            let req_builder = self.build_request_builder(url, method, body, with_aes)?;
            let res = req_builder.send().await?;
            let status_code = res.status();
            if status_code == 200 || status_code == 204 {
                let res = res.text().await?;
                let res: U = serde_json::from_str(&res.clone())?;
                return Ok(res);
            } else {
                let res = res.text().await?;
                if res.is_empty() {
                    return Err(e(&status_code.to_string()));
                }
                return Err(e(&res));
            }
        }
    }
    // verify_signature
    fn verify_signature(&self, data: Vec<&str>, signature: &str) -> Result<bool, WeaError> {
        let data = data.join("\n") + "\n";
        //let serial_no = self
        let alipay_public_cert = self.config.alipay_public_cert.clone();
        //let alipay_cert_no = get_cert_serial(&alipay_public_cert)?;
        //if alipay_cert_no != serial {
        //    return Err(e("serial_no error"));
        //}
        // 加载公钥,公钥为文件内容
        let alipay_public_cert_content = fs::read_to_string(alipay_public_cert)?;
        let app_cert = X509::from_pem(alipay_public_cert_content.as_bytes())?;
        let pkey = app_cert.public_key()?;
        //let data = data + "\n";
        // 创建验证器并设置哈希算法为 SHA256
        let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;
        // 添加待验证的数据
        verifier.update(data.as_bytes())?;
        // 对签名进行 Base64 解码
        let signature_decoded = decode_block(signature)?;
        // 验证签名
        let result = verifier.verify(&signature_decoded)?;
        Ok(result)
    }
    //encrypt
    fn encrypt(&self, data: &str) -> Result<String, WeaError> {
        let mch_key = self.config.mch_key.clone();
        if mch_key.is_none() {
            return Err(e("mch_key is none"));
        }

        let mch_key = decode_block(&mch_key.unwrap())?;
        let cipher = Aes128::new_from_slice(&mch_key).map_err(|_e| e("Aes128 loadkey error"))?;
        //let key = GenericArray::from(mch_key);
        //let cipher = Aes128::new(&mch_key);
        let mut block = GenericArray::clone_from_slice(data.as_bytes());
        cipher.encrypt_block(&mut block);
        Ok(encode_block(&block))
    }
    //decrypt
    fn decrypt(&self, data: &str) -> Result<String, WeaError> {
        let mch_key = self.config.mch_key.clone();
        if mch_key.is_none() {
            return Err(e("mch_key is none"));
        }

        let mch_key = decode_block(&mch_key.unwrap())?;
        let cipher = Aes128::new_from_slice(&mch_key).map_err(|_e| e("Aes128 loadkey error"))?;

        let data = decode_block(data)?;
        let mut block = GenericArray::clone_from_slice(&data);
        cipher.decrypt_block(&mut block);
        Ok(block.iter().map(|&x| x as char).collect())
    }
}
