use std::{time::Duration,fs,path};
use std::future::Future;
use openssl::{
    base64::{decode_block, encode_block}, hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::{Signer, Verifier}, x509::X509
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
    /// create order
    //fn create_order(&self, method: &str, data: ReqOrderBody) -> impl Future<Output = Result<ResOrderBody, WeaError>>;
    // 验证签名 
    // data 为验证签名的数据  vec!['1395712654', 'nonce_str', 'body']
    fn verify_signature(&self,data: Vec<&str>,signature:&str,serial:&str) -> Result<bool, WeaError>;
}

impl BaseTrait for Payment<AlipayConfig> {
    
  // verify_signature
  fn verify_signature(&self,data: Vec<&str>, signature:&str, serial:&str) -> Result<bool, WeaError>{
    let data = data.join("\n");
    let data = data + "\n";
    
    Ok(true)
  }

}