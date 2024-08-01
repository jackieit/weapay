//! 支持微信支付和支付宝支付rust sdk，微信支付基于api v3
//! 包名称：weapay 意为 wechat pay & alipay
//! # 微信签名验签
//! 1. 关于签名：微信支付签名规则参考[微信支付签名生成算法](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)
//! 2. 关于验签：微信支付验签规则参考[微信支付验签](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_2.shtml)
//! 注意微信支付验签时需要平台证书，平台证书下载地址[微信支付平台证书下载](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_3.shtml)
//! 目前只能通过 api接口的方式下载平台证书，不支持手动下载，请保持 CARGO_MANIFEST_DIR/payment/certs/download 目录可写系统会每隔12小时自动下载平台证书
//! 3. 如果无法接收到异步通知请检查是否配置了正确的异步通知地址及设置了APIv3 密钥
//! # 支付宝签名验签
//! 1. 关于签名：[支付宝支付签名生成算法](https://opendocs.alipay.com/open-v3/05419m?pathHash=a5cb620e)
//! 2. 关于验签：[v3同步验签](https://opendocs.alipay.com/open-v3/054d0z?pathHash=dcad8d5c)，异步难答验签请参考[异步通知验签](https://opendocs.alipay.com/open-v3/05pf4k?pathHash=01c6e762)
//! 3. 支付宝最多有四种证书 分别 应用公钥证书，应用私钥证书，支付宝公钥证书，支付宝根证书，接入模式分为普通密钥模式和证书模式，前者需要 应用公钥证书，应用私钥证书，支付宝公钥证书，后者需要四种证书。
//! 应用私钥：用来给应用消息进行签名，请务必要妥善保管，避免遗失或泄露。
//! 应用公钥：需要提供给支付宝开放平台，平台会对应用发送的消息进行签名验证。
//! 支付宝公钥：应用收到支付宝发送的同步、异步消息时，使用支付宝公钥验证签名信息。
//! CSR 文件：CSR 即证书签名请求（Certificate Signing Request），CSR 文件（.csr）是申请证书时所需要的一个数据文件。
//! 应用公钥证书：在开放平台上传 CSR 文件后可以获取 CA 机构颁发的应用证书文件（.crt），其中包含了组织/公司名称、应用公钥、证书有效期等内容，一般有效期为 5 年。
//! 支付宝公钥证书：用来验证支付宝消息，包含了支付宝公钥、支付宝公司名称、证书有效期等内容，一般有效期为 5 年。
//! 支付宝根证书：用来验证支付宝消息，包含了根 CA 名称、根 CA 的公钥、证书有效期等内容。

use crate::error::WeaError;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

pub mod alipay;
pub mod error;
pub mod utils;
pub mod wechat;
// reqwest 请求 user-agent
const SDK_UA: &str = "Weapay rust sdk/0.1.0";

//pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = WeaResult<T>> + Send + 'a>>;
pub type WeaResult<T> = Result<T, WeaError>;
pub type BoxFuture<'a, T> = Pin<Box<dyn Future<Output = WeaResult<T>> + Send + 'a>>;

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

#[derive(Clone, Debug, Default)]
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
/// 支付宝支付配置
/// 支付宝配置分为普通密钥模式和证书模式
/// 普通密钥模式下,app_private_key,alipay_public_cert必填
/// 证书模式下,app_private_key,app_public_cert,alipay_public_cert,alipay_root_cert 必填
/// 查看 [接入前准备](https://opendocs.alipay.com/open-v3/054kaq)
/// ```rust
/// use weapay::{AlipayConfig,Payment};
/// use weapay::alipay::prelude::*;
/// let app_id = "2021003168621630".to_string();
/// let app_private_key = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\ali_private_key.txt".to_string();
/// let alipay_public_cert = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\ali_public_key.txt".to_string();
/// let config = AlipayConfig {
///    app_id,
///    app_private_key,
///    alipay_public_cert,
///    notify_url: "https://example.com".to_string(),
///    ..Default::default()
/// };
/// let payment = Payment::new(config.clone());
/// assert_eq!(payment.config.app_id, config.app_id);
/// ```
#[derive(Clone, Debug, Default)]
pub struct AlipayConfig {
    // 支付宝分配给开发者的应用ID
    pub app_id: String,
    // 应用私钥文件路径
    pub app_private_key: String,
    // 应用公钥文件路径
    pub app_public_cert: Option<String>,
    // 支付宝公钥文件路径
    pub alipay_public_cert: String,
    // 支付宝根证书文件路径
    pub alipay_root_cert: Option<String>,
    // 内容加密密钥
    pub mch_key: Option<String>,
    // 异步通知地址
    pub notify_url: Option<String>,
    // 沙盒模式
    pub is_sandbox: Option<bool>,
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

#[cfg(test)]
pub mod tests {
    use dotenv::dotenv;
    use std::env;
    use std::{
        collections::HashMap,
        time::{SystemTime, UNIX_EPOCH},
    };

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
    pub fn get_config() -> (super::WechatConfig, super::AlipayConfig) {
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
        let wechat_cfg = super::WechatConfig {
            app_id,
            mch_key,
            apiclient_key,
            apiclient_cert,
            notify_url,
            mchid,
            ..Default::default()
        };
        let ali_app_id = env_map.get("ali_app_id").unwrap().to_string();
        let app_private_key = env_map.get("app_private_key").unwrap().to_string();
        let app_public_cert = env_map.get("app_public_cert").unwrap().to_string();
        let app_public_cert = if app_public_cert.is_empty() {
            None
        } else {
            Some(app_public_cert)
        };

        let alipay_public_cert = env_map.get("alipay_public_cert").unwrap().to_string();
        let alipay_root_cert = env_map.get("alipay_root_cert").unwrap().to_string();
        let alipay_root_cert = if alipay_root_cert.is_empty() {
            None
        } else {
            Some(alipay_root_cert)
        };
        let notify_url = env_map.get("ali_notify_url").unwrap().to_string();
        let notify_url = if notify_url.is_empty() {
            None
        } else {
            Some(notify_url)
        };
        let mch_key = env_map.get("ali_mch_key").unwrap().to_string();
        let mch_key = if mch_key.is_empty() {
            None
        } else {
            Some(mch_key)
        };
        let is_sandbox = env_map.get("is_sandbox").unwrap().to_string();
        let is_sandbox = if is_sandbox == "true" {
            Some(true)
        } else {
            Some(false)
        };
        let alipay_cfg = super::AlipayConfig {
            app_id: ali_app_id,
            app_private_key,
            app_public_cert,
            alipay_public_cert,
            alipay_root_cert,
            mch_key,
            notify_url,
            is_sandbox,
        };
        (wechat_cfg, alipay_cfg)
    }
}
