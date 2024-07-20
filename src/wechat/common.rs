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
use crate::error::{WeaError, PayError};
use crate::wechat::prelude::*;
//微信支付trait
pub trait BaseTrait {
    /// 商户系统先调用该接口在微信支付服务后台生成预支付交易单，返回正确的预支付交易会话标识后再按Native、JSAPI、APP等不同场景生成交易串调起支付。
    #[allow(dead_code)]
    fn create_order(&self,trade_type: TradeType,data: ReqOrderBody) -> impl Future<Output = Result<CreateOrderResponse,WeaError>>;
    /// 支付通知数据验证签名数据解密,验证签名的nonce_str,timestamp,signture 来自于请求头
    /// 解密nonce 来自于resource,根据返回结果中的event_type来判断
    /// 支付通知 U为ResourceOrderBody，退款通知 U为ResourceRefundBody
    /// #Example
    /// ```rust
    /// use weapay::wechat::common::BaseTrait;
    /// let payment = super::Payment::new(config);
    /// payment.notify::<ResourceOrderBody>(nonce_str, timestamp, body, signature,serial).await?;
    /// let notify = self.notify::<ResourceRefundBody>(nonce_str, timestamp, body, signature,serial).await?;
    /// ```
    #[allow(dead_code)]
    fn notify<U:DeserializeOwned>(&self,nonce_str: &str,timestamp: &str,body: &str,signature:&str,serial:&str) -> impl Future<Output = Result<U, WeaError>>;
    /// 根据商家订单号查询订单
    #[allow(dead_code)]
    fn query_order(&self,out_trade_no: &str) -> impl Future<Output = Result<ResourceOrderBody,WeaError>>;
    /// 根据微信支付订单号查询订单
    #[allow(dead_code)]
    fn query_order_by_transaction_id(&self,transaction_id: &str) -> impl Future<Output = Result<ResourceOrderBody, WeaError>>;
    /// 关闭订单
    #[allow(dead_code)]
    fn close_order(&self,out_trade_no: &str) -> impl Future<Output = Result<(),WeaError>>;
    /// 下载证书
    #[allow(dead_code)]
    fn download_cert(&self) -> impl Future<Output = Result<Vec<String>,WeaError>>;
    /// 构建请求client 同时设置好请求头
    #[allow(dead_code)]
    fn build_request_builder(&self,url: &str,method: &str,body: &str) -> reqwest::RequestBuilder;
    /// 发起请求同时会根据传入的类型返回对应的结果
    #[allow(dead_code)]
    fn do_request<U:DeserializeOwned>(&self,url: &str,method: &str,body: &str) -> impl Future<Output= Result<U, WeaError>>;
    #[allow(dead_code)]
    /// 判断是否是服务商模式
    fn is_sp(&self) -> bool;
    /// 获取请求uri服务商模式下uri前缀为/v3/pay/partner
    /// widh_mchid 是否带mchid,服务商模式下为 sub_mchid
    /// widh_sp 是否带sp_mchid,服务商模式下为 sp_mchid,默认为false
    #[allow(dead_code)]
    fn get_uri(&self,uri: &str,with_mchid:bool,with_sp:bool) -> String;
    /// 生成签名 data: vec!['GET', 'https://xxx', '1395712654', 'nonce_str', 'body'] 
    fn generate_signature(&self,data: Vec<&str>) -> Result<String,WeaError>;
    /// 验证签名 
    /// data 为验证签名的数据  vec!['1395712654', 'nonce_str', 'body']
    fn validate_signature(&self,data: Vec<&str>,signature:&str,serial:&str) -> impl Future<Output =  Result<bool, WeaError>>;
    /// 解密内容
    fn decrypt_content(&self,nonce: &str,ciphertext: &str,associated_data: &str) -> Result<String, WeaError>;
}

impl BaseTrait for Payment<WechatConfig> {
    fn create_order(&self,trade_type: TradeType, data: ReqOrderBody) -> impl Future<Output = Result<CreateOrderResponse, WeaError>> {
        async move {
            let url = match trade_type {
                TradeType::JSAPI => "/v3/pay/transactions/jsapi",
                TradeType::NATIVE => "/v3/pay/transactions/native",
                TradeType::App => "/v3/pay/transactions/app",
                TradeType::MWEB => "/v3/pay/transactions/h5",
                _ => ""
            };
            if url.is_empty() {
                return Err(WeaError::PayError(PayError::new("trade type error")));
            }
            let url = self.get_uri(url,false,false);
            let mut order_body:ReqOrderBody;
            if self.is_sp() {
                order_body = ReqOrderBody {
                    sp_appid: Some(self.config.sp_appid.clone().unwrap()),
                    sp_mchid: Some(self.config.sp_mchid.clone().unwrap()),
                    sub_appid: Some(self.config.app_id.clone()),
                    sub_mchid: Some(self.config.mchid.clone()),
                    ..data
                };
            } else {
                order_body = ReqOrderBody {
                    appid: Some(self.config.app_id.clone()),
                    mchid: Some(self.config.mchid.clone()),
                    ..data
                };
            }
            if order_body.notify_url.is_none() {
                order_body.notify_url = Some(self.config.notify_url.clone());
            }
            let order_body = serde_json::to_string(&order_body)?;
        //print!("{}",order_body);
        
            self.do_request::<CreateOrderResponse>(&url, "POST", &order_body).await
        }
 
    }
    fn notify<U:DeserializeOwned>(&self,nonce_str: &str,timestamp: &str,body: &str,signature:&str,serial:&str) -> impl Future<Output = Result<U,WeaError>> {
        async move {
            let is_valid = self.validate_signature(vec![timestamp, nonce_str,  body], signature,serial).await?;
            if !is_valid {
                return Err(WeaError::PayError(PayError::new("signature verify error")));
            }
            let notify_content = serde_json::from_str::<RespBody>(body)?;
            let nonce = notify_content.resource.nonce;
            let ciphertext = notify_content.resource.ciphertext;
            let associated_data = notify_content.resource.associated_data.unwrap_or("".to_string());
            let content = self.decrypt_content(&nonce,&ciphertext,&associated_data)?;
            let notify_resource = serde_json::from_str::<U>(&content)?;
            Ok(notify_resource)            
        }
    }
    fn query_order(&self,out_trade_no: &str) -> impl Future<Output = Result<ResourceOrderBody, WeaError>> {
        let url = format!("/v3/pay/transactions/out-trade-no/{}", out_trade_no);
        let url = self.get_uri(&url,true,true);
        async move{
            self.do_request::<ResourceOrderBody>(&url, "GET", &"").await
        }
    }
    fn query_order_by_transaction_id(&self,transaction_id: &str) -> impl Future<Output = Result<ResourceOrderBody, WeaError>> {
        let url = format!("/v3/pay/transactions/id/{}", transaction_id);
        let url = self.get_uri(&url,true,true);
        async move{
            self.do_request::<ResourceOrderBody>(&url, "GET", &"").await
        }
    }
    fn close_order(&self,out_trade_no: &str) -> impl Future<Output = Result<(), WeaError>> {
        let mchid = self.config.mchid.clone();
        let url = format!("/v3/pay/transactions/out-trade-no/{}/close", out_trade_no);
        let url = self.get_uri(&url,false,false);
        async move{
        let body: String;
            if self.is_sp() {
                body = format!("{{\"sp_mchid\":\"{}\",\"sub_mchid\":\"{}\"}}",self.config.sp_mchid.clone().unwrap(),mchid);
            } else {
                body = format!("{{\"mchid\":\"{}\"}}",mchid);
            }
            self.do_request::<()>(&url, "POST", &body).await            
        }

    }
    fn download_cert(&self) -> impl Future<Output = Result<Vec<String>,WeaError>> {
        let url = "/v3/certificates";
        let url = self.get_uri(url,false,false);
        async move {
            let _res:RespCert = self.do_request::<RespCert>(&url, "GET", "").await?;
            let data = _res.data;
            if data.len() == 0 {
                return Err(WeaError::PayError(PayError::new("certificates is empty")));
            }
            let mut cert_files:Vec<String> = vec![];
            for item in data {
                //println!("{:?}", item);
                let serial_no = item.serial_no;
                let dir = env!("CARGO_MANIFEST_DIR");
                let save_path = format!("{}/certs/download/",dir);
                let cert_path = format!("{}{}.pem",save_path,serial_no);
                let cert_file  = path::Path::new(&cert_path);
                if  cert_file.is_file()  {
                    let metadata = fs::metadata(cert_file)?;
                    let modify_time = metadata.modified()?;
                    let mtime = modify_time.elapsed()?;
                    if mtime < Duration::from_secs(12*3600) {
                        cert_files.push(cert_path);
                        continue;
                    }
                    fs::create_dir_all(&save_path)?;
                    
                }
                if !path::Path::new(&save_path).exists() {
                    fs::create_dir_all(&save_path)?;
                } 
                
                let encrypt_certificate = item.encrypt_certificate;
                let ciphertext = encrypt_certificate.ciphertext;
                let nonce = encrypt_certificate.nonce;
                let associated_data =  encrypt_certificate.associated_data;

                let decrypt_data =  self.decrypt_content(&nonce,&ciphertext,&associated_data)?;
                
                //println!("平台证书内容==>\n{}",decrypt_data);
                // let cert_content = decode_block(&cert_content)?;
                // get cargo root path
                fs::write(&cert_path, decrypt_data.as_bytes())?;
                cert_files.push(cert_path);
            }
            Ok(cert_files)
        }
    }
    /// build request client
    fn build_request_builder(&self,url: &str,method: &str,body: &str) -> reqwest::RequestBuilder {
        let base_url = Url::parse("https://api.mch.weixin.qq.com/").unwrap();
        let full_url = base_url.join(url).unwrap();
        let full_url = full_url.as_str();
        let timestamp = get_timestamp().unwrap().to_string();
        let nonce_str = generate_random_string(32);
        let sign_data = vec![method, url, &timestamp, &nonce_str, body];
        let signature = self.generate_signature(sign_data).unwrap();
        let mchid = if self.is_sp() {
            self.config.sp_mchid.clone().unwrap()
        } else {
            self.config.mchid.clone()
        };
        let authorization = format!(
            "WECHATPAY2-SHA256-RSA2048 mchid=\"{}\",nonce_str=\"{}\",timestamp=\"{}\",serial_no=\"{}\",signature=\"{}\"", 
            mchid, nonce_str,timestamp, 
            self.config.serial_no.clone(), signature );
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
            .header("Content-Type", "application/json")
            .header("Accept", "application/json")
            .header("User-Agent", "Weapay rust sdk/0.1.0")
            .header("Authorization", authorization)
    }
    // do request
    fn do_request<U:DeserializeOwned>(&self,url: &str,method: &str,body: &str) ->  impl Future<Output= Result<U, WeaError>> {
        let req_builder = self.build_request_builder(url,method,body);
        async move {
            let res = req_builder.send()
            .await?;
            let status_code = res.status();
            if status_code == 200 || status_code == 204{
                let res = res.text().await?;
                let res: U = serde_json::from_str(&res.clone())?;
                return Ok(res);
            } else {
                let res = res.text().await?;
                if res.is_empty() {
                    return Err(WeaError::PayError(PayError::new(&status_code.to_string())));
                }
                return Err(WeaError::PayError(PayError::new(&res)));
            }
        }
    }
    fn is_sp(&self) -> bool {
        self.config.sp_appid.is_some() && self.config.sp_mchid.is_some()
    }
    fn get_uri(&self, uri: &str,with_mchid:bool,with_sp:bool) -> String {
        let comm = if uri.contains("?"){
            "&"
        } else {
            "?"
        };
        //let mut url = uri;
        if self.is_sp() {
            let mut uri = uri.replace("/v3/pay/", "/v3/pay/partner/");
            if with_sp {
                uri = format!("{}{}sp_mchid={}", uri, {comm},self.config.sp_mchid.clone().unwrap());
            }
            if with_mchid {
                uri = format!("{}{}sub_mchid={}", uri, {comm},self.config.mchid.clone());
            }
            uri
        }else{
            let mut uri = uri.to_string();
            if with_mchid {
                uri = format!("{}{}mchid={}", uri, {comm},self.config.mchid.clone());
            }
            uri
        }
    }
    // generate wechat v3 signature
    fn generate_signature(&self, data: Vec<&str>) -> Result<String, WeaError> {
        let data = data.join("\n");
        let data = data + "\n";
        //println!("签名数据====>\n{}",data);
        let apiclient_key = self.config.apiclient_key.clone();
        //println!("apiclient_key====>\n{}",apiclient_key);
        let private_u8 = apiclient_key.as_bytes();
        let rsa = Rsa::private_key_from_pem(private_u8)?;
        let pkey = PKey::from_rsa(rsa)?;
        let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
        //signer.set_rsa_padding(Padding::PKCS1).unwrap();
        signer.update(data.as_bytes())?;
        let sign = signer.sign_to_vec()?;
    
        Ok(encode_block(&sign))
    }
    // validate signature
    fn validate_signature(&self, data: Vec<&str>, signature: &str, serial:&str) -> impl Future<Output = Result<bool, WeaError>> {
        let data = data.join("\n");
        let data = data + "\n";
        async move {
            //print!("验证签名数据====>\n{}",data);
            let cert_files = self.download_cert().await?;
            let cert_file = cert_files.iter().find(|&x| x.contains(serial));
            if cert_file.is_none() {
                return Err(WeaError::PayError(PayError::new("cert file not found")));
            }
            let cert_file = cert_file.unwrap();
            let apiclient_cert = fs::read_to_string(cert_file)?;
            let app_cert = X509::from_pem(apiclient_cert.as_bytes())?;

            let pkey = app_cert.public_key()?;
            let sign_u8 = decode_block(signature)?;
            let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;
            verifier.update(data.as_bytes())?;
            let result = verifier.verify(&sign_u8)?;
            Ok(result)
        }
    }
    // decrypt content
    fn decrypt_content(&self,nonce: &str,ciphertext: &str,associated_data: &str) -> Result<String, WeaError> {
        let cipher = Aes256Gcm::new_from_slice(self.config.mch_key.as_bytes());
        let cipher = match cipher {
            Ok(cipher) => cipher,
            Err(error) => return Err(WeaError::PayError(PayError::new(&format!("cipher error:{}",error)))),
        };
        let nonce = Nonce::from_slice(nonce.as_bytes());
        let ciphertext = decode_block(&ciphertext)?;
        let ciphertext = &ciphertext[..];
        let payload = Payload {
            msg: ciphertext,
            aad: associated_data.as_bytes(),
        };
        let decrypt_data = cipher.decrypt(nonce, payload)?;
        let decrypt_data = String::from_utf8(decrypt_data)?;
        Ok(decrypt_data)
    }
}
#[cfg(test)]
mod tests {
    use tokio;
    use crate::wechat::prelude::*;
    use crate::{generate_random_string,get_timestamp};
    fn gen_jsapi_params(prepay_id: String) -> (){
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config.clone());
        let timestamp = get_timestamp().unwrap();
        let nonce_str = generate_random_string(32);
        let package = format!("prepay_id={}", &prepay_id);
        let pay_sign = payment.generate_signature(vec![&config.app_id,timestamp.to_string().as_str(), nonce_str.as_str(), package.as_str()]).unwrap();
        println!("
        {{
            \"timeStamp\":\"{}\",
            \"nonceStr\": \"{}\",
            \"package\": \"{}\",
            \"signType\": \"RSA\",
            \"paySign\": \"{}\"
        }}",timestamp,nonce_str,package,pay_sign);
    }
    //测试下单
    async fn test_create_order(trade_type:TradeType,body: ReqOrderBody) -> () {
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config);
        let result = payment.create_order(trade_type, body).await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            let result = result.unwrap();
            if result.prepay_id.is_some() {
                gen_jsapi_params(result.prepay_id.clone().unwrap());
            }
            assert_eq!(result.prepay_id.is_some(), true);
            println!("{:?}", result);
        }
        //result
    }
    //测试下单 JSAPI
    #[tokio::test]
    async fn test_create_order_jsapi() {
        let data = ReqOrderBody{
            description: "旅行卡门票服务".to_string(),
            out_trade_no: "T20240407004".to_string(),
            time_expire: Some("2024-08-01T00:00:00+08:00".to_string()),
            goods_tag: Some("WXG".to_string()),
            support_fapiao: Some(true),
            amount: ReqAmountInfo{
                total: 1,
                currency: Some("CNY".to_string()),
            },
            payer:Some(PayerInfo{
                openid: "oPvUL7e0W_zjfgqCuZqE3rpf4zzs".to_string(),
            }),
            //notify_url: "https://example.com".to_string(),
            ..Default::default()
        };
        test_create_order(TradeType::JSAPI, data).await;
    }
    //测试下单 Native
    #[tokio::test]
    async fn test_create_order_native() {
       // let rs = dotenv::dotenv().ok();
        
        let data = ReqOrderBody{
            description: "旅行卡门票服务".to_string(),
            out_trade_no: "20210301000002".to_string(),
            time_expire: Some("2024-08-01T00:00:00+08:00".to_string()),
            goods_tag: Some("WXG".to_string()),
            support_fapiao: Some(true),

            amount: ReqAmountInfo{
                total: 1,
                currency: Some("CNY".to_string()),
            },
            ..Default::default()
        };
        test_create_order(TradeType::NATIVE, data).await;
    }
    //测试下单 APP
    #[tokio::test]
    async fn test_create_order_app() {
       // let rs = dotenv::dotenv().ok();
        
        let data = ReqOrderBody{
            description: "旅行卡门票服务".to_string(),
            out_trade_no: "20210301000002".to_string(),
            time_expire: Some("2024-08-01T00:00:00+08:00".to_string()),
            goods_tag: Some("WXG".to_string()),
            support_fapiao: Some(true),

            amount: ReqAmountInfo{
                total: 10,
                currency: Some("CNY".to_string()),
            },

            //notify_url: "https://example.com".to_string(),
            ..Default::default()
        };
        test_create_order(TradeType::App, data).await;
    }
    //测试下单H5
    #[tokio::test]
    async fn test_create_order_mweb() {
        let data = ReqOrderBody{
            description: "旅行卡门票服务".to_string(),
            out_trade_no: "20210301000002".to_string(),
            time_expire: Some("2024-08-01T00:00:00+08:00".to_string()),
            goods_tag: Some("WXG".to_string()),
            support_fapiao: Some(true),
            amount: ReqAmountInfo{
                total: 10,
                currency: Some("CNY".to_string()),
            },
            scene_info: Some(ReqSceneInfo{
                payer_client_ip: "124.134.157.78".to_string(),
                h5_info: Some(ReqSceneH5Info{
                    r#type: "ios".to_string(),
                    ..Default::default()
                }),
                ..Default::default()
            }),
            ..Default::default()
        };
        let _body = serde_json::to_string(&data).unwrap();
        println!("{}",_body);
        test_create_order(TradeType::MWEB, data).await;
    }
    // 测试订单查询
    #[tokio::test]
    async fn test_query_order() {
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config);
        let result = payment.query_order("T20240407001").await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            let result = result.unwrap();
            println!("{:?}", result);
            assert_eq!(result.out_trade_no, "T20240407001");
            
        }
        let result = payment.query_order_by_transaction_id("4200002321202407011113597346").await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            let result = result.unwrap();
            println!("{:?}", result);
            assert_eq!(result.out_trade_no, "2407010002");
            
        }
    }
    // 测试关闭订单
    #[tokio::test]
    async fn test_close_order() {
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config);
        let result = payment.close_order("2407020015").await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            assert_eq!(result.is_ok(), true);
        }
    }
    // 测试通知
    #[tokio::test]
    async fn test_notify() {
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config);
        let nonce_str = "wXGCUJV30xenmtIaT9sGPjcty2jova4n";
        let timestamp = "1721352091";
        let signature = "NcWOaHY4o+b5lc8FItwpc2YFAb3A5r9QGdotn/helSyLClxp8g/4mA77AVA5inLViyU7vU+golNZuIeYj43sl7mla/pPwVyyRo4zIbjN3n/oC8jK88v/EhppYOYBp4wg1raqsFf8XXX8ui7OErUJZQG+4SjFx+IRTAHIjmlQP8UKuw6RZ0y2eT03AviMXaJfGbdhux99XdYQ7iLVa+9VuvCufLYwa8JVjjzuIBH5+xe74el68E2zXU+3K1njkvvt2mRhi/nCqa0if+b5eqh7AmpERkrwsCd/JbtbsWNzLX30CcViVL/CSDUlD2ktkp5Q+b0Y1mJEki4mVu6CLTKDYw==";
        let body = r#"{"id":"be3b9a56-ef5c-55f1-93c2-0cfbf8a08e07","create_time":"2024-07-19T09:21:31+08:00","resource_type":"encrypt-resource","event_type":"TRANSACTION.SUCCESS","summary":"支付成功","resource":{"original_type":"transaction","algorithm":"AEAD_AES_256_GCM","ciphertext":"1NHld5DSwT0YjoYZcaokdbzBtuTpDlJ/LxAMwEHmOAD+0zwNpz2U7T3zIjcvjiDR5SfyBll/vYvEMFlw4i6yn1mT7AMel+U5Q4L+hQ370XGix2G7LXjtam3KFDUAjBjjaWcw/oIKN5qwUVv0tNRQUXO3k71x/p72RYTjG3pgT0m+gIF4IT0kKiZ4qzzVbVl8BJXZuKUwX9m87+pmtF9Hyy9a0S+jzTJJhZpTnmHN8NwlzBm7Ax8xhWFqn13eb+vA97OcO8NWX81ogiOJkIu07A9dj6z4uH85B8jqoxOMRixQQEjMmp2bEseUT3fgcrg8HtRLron9O51WHxg6/F4JewC0VA+0qkpdBBULZzaDL6+1/4uKxB3W/yEPCM6Ym2jkvRgMaxPiwgC/OygHaImWitekz/yoHPLlpM4/cI/lKq8DeYP5Ogo1S6hbQD1jRVbUMCxgyNvl9c8EKFdpC5W7jjDZ2vPGUpX/xf56wNHjg1PpUzDp9UT1j2r6kLze66FSRDFv5kZU5QfvpDT/aI2ira55VvpjB/uvl6iTWnOTewE11EouE5p+YKWUEA==","associated_data":"transaction","nonce":"SbmLlX0uxbQG"}}"#;
        let serial = "5AD141C1086A7945A1394A8AEAA9EB0619751859";
        //println!("{}",body);
        let body_struct = serde_json::from_str::<RespBody>(body).unwrap();
        //println!("{:?}",body_struct);
        let _body = serde_json::to_string(&body_struct).unwrap();
        assert_eq!(body, _body);
        //println!("{}",_body);
        let result = payment.notify::<ResourceOrderBody>(nonce_str, timestamp, &body, signature,serial).await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            let result = result.unwrap();
            println!("{:?}", result);
            //assert_eq!(result.out_trade_no, "T20240407003");
        }
    }
    // 测试下载证书
    #[tokio::test]
    async fn test_download_cert() {
        let config = crate::tests::get_config();
        let payment = super::Payment::new(config);
        let result = payment.download_cert().await;
        if result.is_err() {
            let error  = result.err().unwrap();
            println!("{}", error);
        }else{
            
            assert_eq!(result.is_ok(), true);
            let file = result.unwrap().into_iter().filter(|x| x.contains("5AD141C1086A7945A1394A8AEAA9EB0619751859")).next();
            println!("{:?}", file);
            //println!("{:?}", result.unwrap());
        }
    }
}