use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};

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
    ) -> impl Future<Output = Result<ResOrderBody, WeaError>>;
    /// 构建请求client 同时设置好请求头
    /// 如果设置了mch_key 则会对body进行加密
    fn build_request_builder(
        &self,
        url: &str,
        method: &str,
        body: &str,
    ) -> Result<reqwest::RequestBuilder, WeaError>;
    /// 发起请求同时会根据传入的类型返回对应的结果
    fn do_request<U: DeserializeOwned>(
        &self,
        url: &str,
        method: &str,
        body: &str,
    ) -> impl Future<Output = Result<U, WeaError>>;
    /// method format like alipay.trade.app.pay
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
    ) -> impl Future<Output = Result<ResOrderBody, WeaError>> {
        async move {
            let url = self.get_uri(method);
            let order_body = serde_json::to_string(&data)?;
            self.do_request::<ResOrderBody>(&url, &"POST", &order_body)
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
    ) -> Result<reqwest::RequestBuilder, WeaError> {
        let is_sandbox = self.config.is_sandbox.unwrap_or(false);
        let base_url = match is_sandbox {
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
        let with_aes = self.config.mch_key.is_some();
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
            .header("User-Agent", SDK_UA)
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
    ) -> impl Future<Output = Result<U, WeaError>> {
        async move {
            let req_builder = self.build_request_builder(url, method, body)?;
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
        let mch_key = mch_key.as_slice();
        type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
        let iv = [0u8; 16];
        let mut buf = [0u8; 48];
        let data = data.as_bytes();
        let pt_len = data.len();
        buf[..pt_len].copy_from_slice(data);
        let cipher =
            Aes128CbcEnc::new_from_slices(mch_key, &iv).map_err(|_e| e("Aes128 loadkey error"))?;
        let ct = cipher
            .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
            .map_err(|_e| e("padding error"))?;
        Ok(encode_block(ct))
    }
    //decrypt
    fn decrypt(&self, data: &str) -> Result<String, WeaError> {
        let mch_key = self.config.mch_key.clone();
        if mch_key.is_none() {
            return Err(e("mch_key is none"));
        }

        let mch_key = decode_block(&mch_key.unwrap())?;
        let mch_key = mch_key.as_slice();
        type Aes128CbcDec = cbc::Decryptor<aes::Aes128>;
        let iv = [0u8; 16];

        let data = decode_block(data)?;
        let data = data.as_slice();
        let pt = Aes128CbcDec::new_from_slices(mch_key, &iv).unwrap();
        let mut buf = [0u8; 48];
        let pt = pt
            .decrypt_padded_b2b_mut::<Pkcs7>(data, &mut buf)
            .map_err(|_e| e("unPading error"))?;
        let pt = std::str::from_utf8(&pt).map_err(|_e| e("utf8 convert error"))?;
        Ok(pt.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::BaseTrait;
    use crate::*;
    //test aes encrypt and decrypt
    #[test]
    fn test_aes_encrypt_decrypt() {
        let key = openssl::base64::encode_block(b"1234567890123456");
        println!("key=={}", key);
        let config = AlipayConfig {
            mch_key: Some(key.to_string()),
            ..Default::default()
        };
        let payment = Payment::<AlipayConfig>::new(config);
        let data = "hello world";
        let encrypt_data = payment.encrypt(data).unwrap();
        let decrypt_data = payment.decrypt(&encrypt_data).unwrap();
        //println!("encrypt_data=={}",encrypt_data);
        //println!("decrypt_data=={}",decrypt_data);
        assert_eq!(data, decrypt_data);
    }
}
