use crate::alipay::prelude::*;
use crate::utils::*;
use crate::*;
use aes::cipher::{block_padding::Pkcs7, BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use openssl::{
    base64::{decode_block, encode_block},
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    sign::Verifier,
    x509::X509,
};
use reqwest::Url;
use serde::de::DeserializeOwned;
use serde_json;
use std::{collections::HashMap, fs};

pub trait BaseTrait {
    /// create order
    /// method format like alipay.trade.app.pay
    fn create_order<'a>(&'a self, method: &'a str, data: ReqOrderBody) -> BoxFuture<ResOrderBody>;
    /// 查询订单
    fn query_order<'a>(&'a self, out_trade_no: &'a str) -> BoxFuture<ResOrderBody>;
    /// 查询订单支付宝订单号
    fn query_order_by_trade_no<'a>(&'a self, trade_no: &'a str) -> BoxFuture<ResOrderBody>;
    /// 关闭订单
    fn close_order(&self, body: ReqCloseOrderBody) -> BoxFuture<ResCloseOrderBody>;
    /// 撤销订单
    fn cancel_order(&self, body: ReqCancelOrderBody) -> BoxFuture<ResCancelOrderBody>;
    /// 预处理异步通知此方法仅针对异步URL通知的数据进行验签
    /// 如当面付的预下单通知，APP支付的异步通知等
    fn notify(&self, query_str: &str) -> WeaResult<NotifyOrderBody>;
    /// 构建请求client 同时设置好请求头
    /// 如果设置了mch_key 则会对body进行加密
    fn build_request_builder(
        &self,
        url: &str,
        method: &str,
        body: &str,
    ) -> WeaResult<reqwest::RequestBuilder>;
    /// 发起请求同时会根据传入的类型返回对应的结果
    fn do_request<'a, U: DeserializeOwned>(
        &'a self,
        url: &'a str,
        method: &'a str,
        body: &'a str,
    ) -> BoxFuture<U>;
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
    fn verify_signature(&self, data: Vec<&str>, signature: &str) -> WeaResult<bool>;
    /// 加密
    fn encrypt(&self, data: &str) -> WeaResult<String>;
    /// 解密
    fn decrypt(&self, data: &str) -> WeaResult<String>;
}

impl BaseTrait for Payment<AlipayConfig> {
    //create order
    fn create_order<'a>(&'a self, method: &'a str, data: ReqOrderBody) -> BoxFuture<ResOrderBody> {
        let fut = async move {
            let url = self.get_uri(method);
            let data = match data.notify_url {
                Some(_) => data,
                None => {
                    let notify_url = self.config.notify_url.clone();
                    ReqOrderBody { notify_url, ..data }
                }
            };
            let order_body = serde_json::to_string(&data)?;
            self.do_request::<ResOrderBody>(&url, &"POST", &order_body)
                .await
        };
        Box::pin(fut)
    }
    //query order
    fn query_order<'a>(&'a self, out_trade_no: &'a str) -> BoxFuture<ResOrderBody> {
        let fut = async move {
            let url = self.get_uri("alipay.trade.query");
            let order_body = serde_json::to_string(&ReqQueryOrderBody {
                out_trade_no: Some(out_trade_no.to_string()),
                ..Default::default()
            })?;
            self.do_request::<ResOrderBody>(&url, &"POST", &order_body)
                .await
        };
        Box::pin(fut)
    }
    // query order by trade_no
    fn query_order_by_trade_no<'a>(&'a self, trade_no: &'a str) -> BoxFuture<ResOrderBody> {
        let fut = async move {
            let url = self.get_uri("alipay.trade.query");
            let order_body = serde_json::to_string(&ReqQueryOrderBody {
                trade_no: Some(trade_no.to_string()),
                ..Default::default()
            })?;
            self.do_request::<ResOrderBody>(&url, &"POST", &order_body)
                .await
        };
        Box::pin(fut)
    }
    //close order
    fn close_order(&self, body: ReqCloseOrderBody) -> BoxFuture<ResCloseOrderBody> {
        let fut = async move {
            let url = self.get_uri("alipay.trade.close");
            let order_body = serde_json::to_string(&body)?;
            self.do_request::<ResCloseOrderBody>(&url, &"POST", &order_body)
                .await
        };
        Box::pin(fut)
    }
    //cancel order
    fn cancel_order(&self, body: ReqCancelOrderBody) -> BoxFuture<ResCancelOrderBody> {
        let fut = async move {
            let url = self.get_uri("alipay.trade.cancel");
            let order_body = serde_json::to_string(&body)?;
            self.do_request::<ResCancelOrderBody>(&url, &"POST", &order_body)
                .await
        };
        Box::pin(fut)
    }
    //pre_notify
    fn notify(&self, query_str: &str) -> WeaResult<NotifyOrderBody> {
        let tmp = "https://xx.com/?".to_string() + &query_str;
        let url = Url::parse(&tmp).map_err(|_e| e("parse url error"))?;
        let url = url.query_pairs();
        let mut sign: String = "".to_string();
        let mut hm: HashMap<String, String> = HashMap::new();
        for (_, (key, value)) in url.enumerate() {
            if key == "sign_type" {
                continue;
            }
            if key == "sign" {
                sign = value.as_ref().to_string();
                //hm.insert(key.as_ref().to_string(), value.as_ref().to_string());
            } else {
                hm.insert(key.as_ref().to_string(), value.as_ref().to_string());
            }
        }
        //println!("sign=\n{}", sign);
        let mut sorted_keys: Vec<_> = hm.keys().collect();
        sorted_keys.sort();
        // println!("sorted_keys=={:?}", sorted_keys);
        let mut new_str = "".to_string();
        for key in sorted_keys.iter() {
            new_str = new_str + key + "=" + hm.get(*key).unwrap() + "&";
        }
        let new_str = new_str.trim_end_matches('&');
        //println!("new_str=={}", new_str);
        let signed = self.verify_signature(vec![&new_str], &sign)?;
        if !signed {
            return Err(e("verify signature error"));
        }

        let hm_value = serde_json::to_value(&hm)?;
        let notify: NotifyOrderBody = serde_json::from_value(hm_value)?;
        Ok(notify)
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
    ) -> WeaResult<reqwest::RequestBuilder> {
        let is_sandbox = self.config.is_sandbox.unwrap_or(false);
        let base_url = match is_sandbox {
            false => "https://openapi.alipay.com",
            true => "https://openapi-sandbox.dl.alipaydev.com",
        };

        let base_url = Url::parse(base_url).map_err(|_e| e("parse url error"))?;
        let full_url = base_url.join(url).map_err(|_e| e("join url error"))?;
        let full_url = full_url.as_str();
        let timestamp = get_timestamp_millis()?.to_string();
        let nonce_str = generate_random_string(32);
        let request_id = generate_random_string(32);
        let is_cert_model = self.config.alipay_root_cert.is_some();

        //let alipay_root_serial_no = get_cert_serial(&self.config.alipay_root_cert.clone())?;
        let auth_string = if is_cert_model {
            let app_public_cert_sn = get_cert_sn(&self.config.app_public_cert.clone().unwrap())?;
            format!(
                "app_id={},app_cert_sn={},nonce={},timestamp={}",
                &self.config.app_id, &app_public_cert_sn, nonce_str, timestamp
            )
        } else {
            format!(
                "app_id={},nonce={},timestamp={}",
                &self.config.app_id, nonce_str, timestamp
            )
        };
        //println!("auth_string=={}\n", auth_string);
        let with_aes = self.config.mch_key.is_some();
        let body = if with_aes {
            let body = self.encrypt(body)?;
            body
        } else {
            body.to_string()
        };
        let sign_data: Vec<&str> = vec![&auth_string, method, url, &body];
        //println!("sign_data=={:?}\n", sign_data);
        let signature = generate_signature(sign_data, &self.config.app_private_key)?;
        //println!("signature=={}\n", signature);
        let authorization = format!("ALIPAY-SHA256withRSA {},sign={}", auth_string, signature);
        //println!("authorization=={}\n", authorization);
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
            req_builder
                .header("alipay-encrypt-type", "AES")
                .header("Content-Type", "text/plain")
        } else {
            req_builder.header("Content-Type", "application/json")
        };
        let req_builder = if is_cert_model {
            let alipay_root_serial_no =
                get_root_cert_sn(&self.config.alipay_root_cert.clone().unwrap())?;
            req_builder.header("alipay-root-cert-sn", alipay_root_serial_no)
        } else {
            req_builder
        };
        let req_builder = req_builder
            .header("Accept", "application/json")
            .header("User-Agent", SDK_UA)
            .header("alipay-request-id", request_id)
            //.header("alipay-root-cert-sn", alipay_root_serial_no)
            .header("Authorization", authorization);
        Ok(req_builder)
    }
    // do request
    fn do_request<'a, U: DeserializeOwned>(
        &'a self,
        url: &'a str,
        method: &'a str,
        body: &'a str,
    ) -> BoxFuture<U> {
        let fut = async move {
            let req_builder = self.build_request_builder(url, method, body)?;
            let res = req_builder.send().await?;
            let status_code = res.status();
            let headers = res.headers().clone();
            let res = res.text().await?;
            let is_cert_model = self.config.alipay_root_cert.is_some();
            let sn = headers.get("alipay-sn");
            if is_cert_model && sn.is_some() {
                let mut verify_data: Vec<&str> = vec![];
                let sn = sn.unwrap().to_str()?;
                let alipay_public_cert_sn = get_cert_sn(&self.config.alipay_public_cert.clone())?;
                if sn != alipay_public_cert_sn {
                    return Err(e("alipay-sn is not match"));
                }
                let timestamp = headers.get("alipay-timestamp").unwrap().to_str()?;
                verify_data.push(timestamp);
                let nonce = headers.get("alipay-nonce").unwrap().to_str()?;
                verify_data.push(nonce);
                let signature = headers.get("alipay-signature").unwrap().to_str()?;
                verify_data.push(&res);
                let signed = self.verify_signature(verify_data, signature)?;
                if !signed {
                    return Err(e("response verify signature error"));
                }
            }
            if status_code == 200 || status_code == 204 {
                let with_aes = self.config.mch_key.is_some();
                let res = if with_aes {
                    let res = self.decrypt(&res)?;
                    res
                } else {
                    res
                };
                let res: U = serde_json::from_str(&res.clone())?;
                return Ok(res);
            } else {
                //let res = res.text().await?;
                if res.is_empty() {
                    return Err(e(&status_code.to_string()));
                }
                return Err(e(&res));
            }
        };
        Box::pin(fut)
    }
    // verify_signature
    fn verify_signature(&self, data: Vec<&str>, signature: &str) -> WeaResult<bool> {
        let data = if data.len() == 1 {
            data[0].to_string()
        } else {
            data.join("\n") + "\n"
        };
        //println!("data=={}", data);
        let alipay_public_cert = self.config.alipay_public_cert.clone();
        // 加载公钥,公钥为文件内容
        let alipay_public_cert_content = fs::read_to_string(alipay_public_cert)?;
        let pkey = if alipay_public_cert_content.contains("-----BEGIN CERTIFICATE-----") {
            let app_cert = X509::from_pem(alipay_public_cert_content.as_bytes())?;
            app_cert.public_key()?
        } else {
            let alipay_public_cert_content = decode_block(&alipay_public_cert_content)?;
            let rsa = Rsa::public_key_from_der(alipay_public_cert_content.as_slice())?;
            PKey::from_rsa(rsa)?
        };
        // 创建验证器并设置哈希算法为 SHA256
        let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey)?;
        // 添加待验证的数据
        //verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1)?;
        //verifier.set_rsa_padding(openssl::rsa::Padding::PKCS1)?;
        verifier.update(data.as_bytes())?;
        // 对签名进行 Base64 解码
        let signature_decoded = decode_block(signature)?;
        // 验证签名
        let result = verifier.verify(signature_decoded.as_slice())?;
        //println!("result=={}", result);
        Ok(result)
    }
    //encrypt
    fn encrypt(&self, data: &str) -> WeaResult<String> {
        let mch_key = self.config.mch_key.clone();
        if mch_key.is_none() {
            return Err(e("mch_key is none"));
        }
        let mch_key = decode_block(&mch_key.unwrap())?;
        let mch_key = mch_key.as_slice();
        type Aes128CbcEnc = cbc::Encryptor<aes::Aes128>;
        let pt_len = data.len();
        let iv = [0u8; 16];
        let buf_len = pt_len + (16 - pt_len % 16);
        let mut buf = vec![0u8; buf_len];
        //let mut buf = buf.as_mut_slice();
        let data = data.as_bytes();
        //let pt_len = data.len();
        buf[..pt_len].copy_from_slice(data);
        let cipher =
            Aes128CbcEnc::new_from_slices(mch_key, &iv).map_err(|_e| e("Aes128 loadkey error"))?;
        let ct = cipher
            .encrypt_padded_mut::<Pkcs7>(&mut buf, pt_len)
            .map_err(|_e| e("padding error"))?;
        Ok(encode_block(ct))
    }
    //decrypt
    fn decrypt(&self, data: &str) -> WeaResult<String> {
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
        let buf_len = data.len() + (16 - data.len() % 16);
        let mut buf = vec![0u8; buf_len];
        //let mut buf = buf.as_mut_slice();
        // let mut buf = [0u8; 1024];
        let pt = pt
            .decrypt_padded_b2b_mut::<Pkcs7>(data, &mut buf)
            .map_err(|_e| e("unPading error"))?;
        let pt = std::str::from_utf8(&pt).map_err(|_e| e("utf8 convert error"))?;
        Ok(pt.to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::alipay::prelude::*;
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
    // test create order
    #[tokio::test]
    async fn test_create_order() {
        let config = crate::tests::get_config().1;
        println!("{:?}", config);
        let payment = Payment::new(config.clone());
        let data = ReqOrderBody {
            out_trade_no: "T20240407003".to_string(),
            total_amount: "10.01".to_string(),
            subject: "旅行卡年卡服务".to_string(),
            product_code: Some("JSAPI_PAY".to_string()),
            op_app_id: Some(config.app_id),
            buyer_id: Some("2088722032795825".to_string()),
            ..Default::default()
        };
        let result = payment.create_order("alipay.trade.create", data).await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(result.out_trade_no, Some("T20240407003".to_string()));
            println!("trade_no==>{:?}", result.trade_no);
        }
    }
    // test query order
    #[tokio::test]
    async fn test_query_order() {
        let config = crate::tests::get_config().1;
        let payment = Payment::new(config);
        let result = payment.query_order("2406220006").await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(result.out_trade_no, Some("2406220006".to_string()));
            //println!("{:?}", result);
        }
        let result = payment
            .query_order_by_trade_no("2024062222001401371424183634")
            .await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(
                result.trade_no,
                Some("2024062222001401371424183634".to_string())
            );
            //println!("{:?}", result);
        }
    }
    // test h5 pay create order
    #[tokio::test]
    async fn test_h5_pay() {
        let config = crate::tests::get_config().1;
        let payment = Payment::new(config);
        let data = ReqOrderBody {
            out_trade_no: "T20240407003".to_string(),
            total_amount: "10.01".to_string(),
            subject: "旅行卡年卡服务".to_string(),
            product_code: Some("QUICK_WAP_WAY".to_string()),
            //buyer_id: Some("2088722032795825".to_string()),
            ..Default::default()
        };
        let result = payment.create_order("alipay.trade.wap.pay", data).await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(result.page_redirection_data.is_some(), true);
            println!("trade_no==>{:?}", result.page_redirection_data);
        }
    }
    // tests face to face pay create order
    #[tokio::test]
    async fn test_face_to_face_pay() {
        let config = crate::tests::get_config().1;
        let payment = Payment::new(config);
        let data = ReqOrderBody {
            out_trade_no: "T20240407007".to_string(),
            total_amount: "0.99".to_string(),
            subject: "旅行卡年卡服务".to_string(),
            //product_code: Some("FACE_TO_FACE_PAYMENT".to_string()),
            //buyer_id: Some("2088722032795825".to_string()),
            ..Default::default()
        };
        let result = payment.create_order("alipay.trade.precreate", data).await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(result.qr_code.is_some(), true);
            println!("qr_code==>{:?}", result.qr_code);
        }
    }
    // test pre_notify
    #[test]
    fn test_pre_notify() {
        let config = crate::tests::get_config().1;
        let payment = Payment::new(config);
        let query_str = "gmt_create=2024-07-24+10%3A43%3A59&charset=UTF-8&seller_email=lwojga1716%40sandbox.com&subject=%E6%97%85%E8%A1%8C%E5%8D%A1%E5%B9%B4%E5%8D%A1%E6%9C%8D%E5%8A%A1&sign=kT7bTHhFPgBeOqEqmNe09%2BxmsZWJrxihcAL6fuf3VSvsU3eg6b0o3yDU8xAZZbXkEBGyACRppAgiabnHzh9SyFrSbJTAY8GUemvgiVgh9r3Sbsb%2Fij1Ef94AgXJYxBclcGDNfcM%2FVtySaLuBjZLmqSX4M6cWq3b3vBG%2BYIxew83ZchOBEMSSSnzpIUkRoFPYQ9Y1YDUCaEnDlslJ%2BLSKlQS2ZsgLmbOmZ%2BeNAJ0wxIw8SCR4Kd6AkuSkinjiPhVVGqbtxJK6iu9q1T9MqwdrG8MqJl0ztni3emWMuuihCC%2B5biYVM0u49HUnHEW%2BS%2FyerbllJWu%2BykG%2FvHFAnrz2Bw%3D%3D&buyer_id=2088722032795825&invoice_amount=0.99&notify_id=2024072401222104407195820503475973&fund_bill_list=%5B%7B%22amount%22%3A%220.99%22%2C%22fundChannel%22%3A%22ALIPAYACCOUNT%22%7D%5D&notify_type=trade_status_sync&trade_status=TRADE_SUCCESS&receipt_amount=0.99&buyer_pay_amount=0.99&app_id=9021000135675809&sign_type=RSA2&seller_id=2088721032816228&gmt_payment=2024-07-24+10%3A44%3A06&notify_time=2024-07-24+10%3A44%3A07&version=1.0&out_trade_no=T20240407007&total_amount=0.99&trade_no=2024072422001495820503421248&auth_app_id=9021000135675809&buyer_logon_id=uyskdk2812%40sandbox.com&point_amount=0.00";
        let result = payment.notify(query_str);
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();
            assert_eq!(result.out_trade_no, "T20240407007".to_string());
            println!("{:?}", result);
        }
    }
}
