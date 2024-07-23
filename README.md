# weapay
wechat pay v3 and alipay v3 rust pay
支持微信支付和支付宝支付rust sdk，两种支付方式均基于 v3
包名称：weapay 意为 wechat pay & alipay
目前仅在通过单元测试了普通商户的功能，
代码目前处于开发阶段，且勿用于生产环境。
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
## 下单
```rust
use weapay::{WechatConfig,Payment};
use weapay::wechat::prelude::{ReqOrderBody,ReqAmountInfo,TradeType,BaseTrait};
// 无论是微信还是支付宝都需要传证书的文件绝对路径
let apiclient_key = "C:\\Users\\Windows\\Desktop\\doc\\cert\\apiclient_key.pem";
let apiclient_cert = "C:\\Users\\Windows\\Desktop\\doc\\cert\\apiclient_cert.pem";
//let key_content = std::fs::read_to_string(&apiclient_key).unwrap();
let config = WechatConfig {
    app_id: "wx123456".to_string(),
    mch_key: "123456".to_string(),
    apiclient_key, 
    apiclient_cert,
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
let _result = payment.create_order(TradeType::JSAPI, data).await
```
## 退款

退款需要引入RefundTrait
```rust
use weapay::wechat::prelude::{ReqRefundOrder,ReqRefundAmountInfo,ReqRefundGoodsDetail,BaseTrait,RefundTrait};

let payment = Payment::new(config);
let data = ReqRefundOrder{
    out_trade_no: Some("T20240407003".to_string()),
    out_refund_no: "RT20240407003".to_string(),
    reason: Some("商品已售完".to_string()),
    amount: ReqRefundAmountInfo{
        refund: 1,
        total: 1,
        currency: "CNY".to_string(),
        ..Default::default()
    },
    goods_detail: Some(
        vec![ReqRefundGoodsDetail{
            merchant_goods_id: "11".to_string(),
            goods_name: Some("旅行卡门票服务".to_string()),
            unit_price: 1,
            refund_amount: 1,
            refund_quantity: 1,
            ..Default::default()
        }]
    ),
    ..Default::default()
};
let result = payment.refund(data).await;
```

## 查询对帐单
需要引入 BillTrait
```rust
use weapay::wechat::prelude::BillTrait;
let payment = Payment::new(config);
let result = payment.trade_bill("2024-07-01".to_string(),Some("ALL".to_string()),None,false).await;
```

更多使用方法查看源码中测试方法