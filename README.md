# weapay
wechat pay v3 and alipay rust pay
支持微信支付和支付宝支付rust sdk，微信支付基于api v3
包名称：weapay 意为 wechat pay & alipay
# 微信签名验签
1. 关于签名：微信支付签名规则参考[微信支付签名生成算法](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)
2. 关于验签：微信支付验签规则参考[微信支付验签](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_2.shtml)
注意微信支付验签时需要平台证书，平台证书下载地址[微信支付平台证书下载](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_3.shtml)
目前只能通过 api接口的方式下载平台证书，不支持手动下载，请保持 CARGO_MANIFEST_DIR/payment/certs/download 目录可写系统会每隔12小时自动下载平台证书
3. 如果无法接收到异步通知请检查是否配置了正确的异步通知地址及设置了APIv3 密钥

# 微信支付使用方法
查看 [接入前准备](https://pay.weixin.qq.com/wiki/doc/apiv3/open/pay/chapter5_5_2.shtml#doc-main)
服务商模式下,app_id = sub_appid, mchid = sub_mchid
# Examples
```rust
use weapay::{WechatConfig,Payment};
use weapay::wechat::prelude::{ReqOrderBody,ReqAmountInfo,TradeType,BaseTrait};
// 读取证书内容,注意apiclient_cert.pem 暂时没发现有什么用
let apiclient_key = "C:\\Users\\Windows\\Desktop\\doc\\cert\\apiclient_key.pem";
let key_content = std::fs::read_to_string(&apiclient_key).unwrap();
let config = WechatConfig {
    app_id: "wx123456".to_string(),
    mch_key: "123456".to_string(),
    apiclient_key: key_content, 
    ..Default::default()
};
let payment = Payment::new(config);
let data = ReqOrderBody{
    amount: ReqAmountInfo{
        total: 1,
        currency: None,
    },
    //notify_url: "https://example.com".to_string(),
    ..Default::default()
};
//payment.create_order(TradeType::JSAPI, data).await
```