//! 支持微信支付和支付宝支付rust sdk，微信支付基于api v3
//! 包名称：weapay 意为 wechat pay & alipay
//! # 微信签名验签
//! 1. 关于签名：微信支付签名规则参考[微信支付签名生成算法](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)
//! 2. 关于验签：微信支付验签规则参考[微信支付验签](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_2.shtml)
//! 注意微信支付验签时需要平台证书，平台证书下载地址[微信支付平台证书下载](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_3.shtml)
//! 目前只能通过 api接口的方式下载平台证书，不支持手动下载，请保持 CARGO_MANIFEST_DIR/payment/certs/download 目录可写系统会每隔12小时自动下载平台证书
//! 3. 如果无法接收到异步通知请检查是否配置了正确的异步通知地址及设置了APIv3 密钥
use std::{fmt::Debug, time::{SystemTime, UNIX_EPOCH}};
use rand::{distributions::Alphanumeric, Rng};
use openssl::{
    base64::encode_block, hash::MessageDigest, pkey::PKey, rsa::Rsa, sign::Signer
};
use error::WeaError;
pub mod wechat;
pub mod alipay;
pub mod error;
// reqwest 请求 user-agent
const SDK_UA: &str = "Weapay rust sdk/0.1.0";
/// 微信支付配置
/// 查看 [接入前准备](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter5_5_2.shtml#doc-main)
/// 服务商模式下,app_id = sub_appid, mchid = sub_mchid
/// # Examples
/// ```rust
/// use weapay::{WechatConfig,Payment};
/// use weapay::wechat::prelude::{ReqOrderBody,ReqAmountInfo,TradeType,BaseTrait};
/// // 读取证书内容,注意apiclient_cert.pem 暂时没发现有什么用
/// let apiclient_key = "C:\\Users\\Windows\\Desktop\\doc\\cert\\apiclient_key.pem";
/// let apiclient_cert = "C:\\Users\\Windows\\Desktop\\doc\\cert\\apiclient_cert.pem";
/// //let key_content = std::fs::read_to_string(&apiclient_key).unwrap();
/// let config = WechatConfig {
///     app_id: "wx123456".to_string(),
///     mch_key: "123456".to_string(),
///     apiclient_key, 
///     apiclient_cert,
///     ..Default::default()
/// };
/// let payment = Payment::new(config);
/// let data = ReqOrderBody{
///     amount: ReqAmountInfo{
///         total: 1,
///         currency: None,
///     },
///     //notify_url: "https://example.com".to_string(),
///     ..Default::default()
/// };
/// //payment.create_order(TradeType::JSAPI, data).await
/// ```

#[derive(Clone,Debug,Default)]
pub struct WechatConfig {
    // 服务商公众号或小程序appid
    pub sp_appid: Option<String>,
    // 服务商商户号
    pub sp_mchid: Option<String>,
    // 公众号或小程序或绑定到三方平台应用的appid,
    // 如果是服务商模式，此处填写服务商的appid
    pub app_id: String,
    // 商户号，如果是服务商模式，此处填写服务商的商户号
    pub mchid: String,
    // 商户支付密钥
    pub mch_key: String,
    // 商户证书内容文件路径
    pub apiclient_key: String,
    // 商户证书内容文件路径
    pub apiclient_cert: String,
    // 异步通知地址
    pub notify_url: String,
}
// 支付宝支付配置
#[derive(Clone,Debug,Default)]
pub struct AlipayConfig {
    // 支付宝分配给开发者的应用ID
    pub app_id: String,
    // 应用私钥文件路径
    pub app_private_key: String,
    // 应用公钥文件路径
    pub app_public_cert: String,
    // 支付宝公钥文件路径
    pub alipay_public_cert: String,
    // 支付宝根证书文件路径
    pub alipay_root_cert: String,
    // 内容加密密钥
    pub mch_key: Option<String>,
    // 异步通知地址
    pub notify_url: String,
    // 沙盒模式
    pub is_sandbox: bool,
}

// 支付配置
pub struct Payment<T> {
    pub config: T,
}
impl<T> Payment<T>
where
    T: Debug + Clone + Default,
{
    pub fn new(config: T) -> Self {
        Payment { config }
    }
}
/// 生成签名 data: vec!['GET', 'https://xxx', '1395712654', 'nonce_str', 'body'] 
/// private_key: 商户私钥,支付宝提供的私钥可能没有 begin-- end 手动加上。注意两端不要有空格
fn generate_signature(data: Vec<&str>,private_key:&str) -> Result<String,WeaError> {
    let data = data.join("\n");
    let data = data + "\n";
    let mut private_key_content = std::fs::read_to_string(private_key)?;
    if !private_key_content.starts_with("-----BEGIN RSA PRIVATE KEY-----"){
        let mut tmp_key_content = "-----BEGIN RSA PRIVATE KEY-----\n".to_string();
        let mut line_length = 0;
        for ch in private_key_content.chars() {
            line_length = line_length + 1;
            tmp_key_content.push(ch);
            if line_length == 64 {
                tmp_key_content = tmp_key_content + "\n";
                line_length = 0;
            }
        }
        tmp_key_content.push_str("\n-----END RSA PRIVATE KEY-----");
        private_key_content = tmp_key_content;
    }
    let private_u8 = private_key_content.as_bytes();
    let rsa = Rsa::private_key_from_pem(private_u8)?;
    let pkey = PKey::from_rsa(rsa)?;
    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    //signer.set_rsa_padding(Padding::PKCS1).unwrap();
    signer.update(data.as_bytes())?;
    let sign = signer.sign_to_vec()?;

    Ok(encode_block(&sign))
}
// generate a random string
pub fn generate_random_string(len: usize) -> String {

    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
// get current unix timestamp
pub fn get_timestamp() -> Result<u64,WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp = since_the_epoch.as_secs();
    Ok(timestamp)
}
// 获取当前 Unix 时间戳的毫秒数
pub fn get_timestamp_millis() -> Result<u128, WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp_millis = since_the_epoch.as_millis();
    Ok(timestamp_millis)
}
// short for payerror
pub fn e(message:&str) -> WeaError {
    WeaError::PayError(error::PayError::new(message))
}
// get cert serial number
pub fn get_cert_serial(cert:&str) -> Result<String,WeaError> {

    let cert = std::fs::read(cert)?;
    let cert = openssl::x509::X509::stack_from_pem(&cert)?;
    let cert = cert[0].serial_number().to_bn()?.to_hex_str()?.to_string();
    // convert OpensslString to String
    //let cert = cert.to_string();
    //println!("get_cert_serial ==={}",cert);
    Ok(cert)
}
#[cfg(test)]
pub mod tests {
    use std::{ collections::HashMap, time::{SystemTime,UNIX_EPOCH}};
    use dotenv::dotenv;
    use std::env;
    #[test]
    fn test_generate_random_string() {
       
        let mii = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .subsec_nanos();
        println!("{}", mii);
    }
    /**
     * 获取微信支付配置
     */
    pub fn get_config() -> super::WechatConfig {
        dotenv().ok();
        let env_map = env::vars()
        .into_iter()
        .map(|i| (i.0, i.1))
        .collect::<HashMap<String, String>>();
        let app_id = env_map.get("app_id").unwrap().to_string();
        let mch_key = env_map.get("mch_key").unwrap().to_string();
        let apiclient_key = env_map.get("apiclient_key").unwrap().to_string();
        let apiclient_cert = env_map.get("apiclient_cert").unwrap().to_string();
        //let key_content = std::fs::read_to_string(&apiclient_key).unwrap();
        //let serial_no = env_map.get("serial_no").unwrap().to_string();
        let notify_url = env_map.get("notify_url").unwrap().to_string();
        let mchid = env_map.get("mch_id").unwrap().to_string();
        let config = super::WechatConfig {
            app_id,
            mch_key,
            apiclient_key,
            apiclient_cert,
            notify_url,
            mchid,
            ..Default::default()
        };
        config
    }


}
