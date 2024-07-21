use std::{time::Duration,fs,path};
use std::future::Future;
use openssl::{
    base64::decode_block, hash::MessageDigest, rsa::Rsa, sign::Verifier
};
use aes_gcm::{
    Aes256Gcm,KeyInit,Nonce,
    aead::{Aead,Payload}};
use reqwest::Url;
//use anyhow::{anyhow, Result};
use serde::de::DeserializeOwned;
use serde_json;
use crate::*;
use crate::error::WeaError;
use crate::alipay::prelude::*;

pub trait BaseTrait {
    /// 构建请求client 同时设置好请求头
    #[allow(dead_code)]
    fn build_request_builder(&self,url: &str,method: &str,body: &str) -> reqwest::RequestBuilder;
    /// create order
    //fn create_order(&self, method: &str, data: ReqOrderBody) -> impl Future<Output = Result<ResOrderBody, WeaError>>;
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
    fn verify_signature(&self,data: Vec<&str>,signature:&str,serial:&str) -> Result<bool, WeaError>;
}

impl BaseTrait for Payment<AlipayConfig> {
  // build_request_builder
  fn build_request_builder(&self,url: &str,method: &str,body: &str) -> reqwest::RequestBuilder {
    let base_url = match self.config.is_sandbox {
      true => "https://openapi.alipay.com",
      false => "https://openapi-sandbox.dl.alipaydev.com",
        
    };
    let base_url = Url::parse(base_url).unwrap();
    let full_url = base_url.join(url).unwrap();
    let full_url = full_url.as_str();
    let timestamp = get_timestamp_millis().unwrap().to_string();
    let nonce_str = generate_random_string(32);
    let request_id = format!("{}{}",generate_random_string(10),timestamp);
    let auth_string = format!("app_id={},app_cert_sn={},nonce={},timestamp={}",
                              &self.config.app_id,
                              &self.config.app_cert_sn,
                              nonce_str,
                              timestamp);
    let sign_data: Vec<&str> = vec![&auth_string, method ,url, body];
    let signature = generate_signature(sign_data, &self.config.app_private_key).unwrap();
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
    req_builder
  }
  // verify_signature
  fn verify_signature(&self,data: Vec<&str>, signature:&str, serial:&str) -> Result<bool, WeaError>{
    let data = data.join("\n") + "\n";
    //let serial_no = self
    if self.config.alipay_cert_sn.clone() != serial {
      return Err(e("serial_no error"));
    }
    // 加载公钥,公钥为文件内容
    let alipay_public_key = self.config.alipay_public_key.clone();
    let rsa = Rsa::public_key_from_pem(&alipay_public_key.as_bytes())?;
    let pkey = PKey::from_rsa(rsa)?;
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

}