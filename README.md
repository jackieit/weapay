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
支持微信支付和支付宝支付rust sdk，微信支付基于api v3
包名称：weapay 意为 wechat pay & alipay
# 微信签名验签
1. 关于签名：微信支付签名规则参考[微信支付签名生成算法](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_1.shtml)
2. 关于验签：微信支付验签规则参考[微信支付验签](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_2.shtml)
注意微信支付验签时需要平台证书，平台证书下载地址[微信支付平台证书下载](https://pay.weixin.qq.com/wiki/doc/apiv3/wechatpay/wechatpay4_3.shtml)
目前只能通过 api接口的方式下载平台证书，不支持手动下载，请保持 CARGO_MANIFEST_DIR/payment/certs/download 目录可写系统会每隔12小时自动下载平台证书
3. 如果无法接收到异步通知请检查是否配置了正确的异步通知地址及设置了APIv3 密钥
# 支付宝签名验签
1. 关于签名：[支付宝支付签名生成算法](https://opendocs.alipay.com/open-v3/05419m?pathHash=a5cb620e)
2. 关于验签：[v3同步验签](https://opendocs.alipay.com/open-v3/054d0z?pathHash=dcad8d5c)，异步难答验签请参考[异步通知验签](https://opendocs.alipay.com/open-v3/05pf4k?pathHash=01c6e762)
3. 支付宝最多有四种证书 分别 应用公钥证书，应用私钥证书，支付宝公钥证书，支付宝根证书，接入模式分为普通密钥模式和证书模式，前者需要 应用公钥证书，应用私钥证书，支付宝公钥证书，后者需要四种证书。  
以下为证书相关的说明：  
应用私钥：用来给应用消息进行签名，请务必要妥善保管，避免遗失或泄露。  
应用公钥：需要提供给支付宝开放平台，平台会对应用发送的消息进行签名验证。  
支付宝公钥：应用收到支付宝发送的同步、异步消息时，使用支付宝公钥验证签名信息。  
CSR 文件：CSR 即证书签名请求（Certificate Signing Request），CSR 文件（.csr）是申请证书时所需要的一个数据文件。  
应用公钥证书：在开放平台上传 CSR 文件后可以获取 CA 机构颁发的应用证书文件（.crt），其中包含了组织/公司名称、应用公钥、证书有效期等内容，一般有效期为 5 年。  
支付宝公钥证书：用来验证支付宝消息，包含了支付宝公钥、支付宝公司名称、证书有效期等内容，一般有效期为 5 年。  
支付宝根证书：用来验证支付宝消息，包含了根 CA 名称、根 CA 的公钥、证书有效期等内容。  
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

# 支付宝支付使用方法
支付宝配置分为普通密钥模式和证书模式
普通密钥模式下,app_private_key,alipay_public_cert必填
证书模式下,app_private_key,app_public_cert,alipay_public_cert,alipay_root_cert 必填
证书字段请提供文件路径，普通密钥模式下把原文存为文件，不要做任何修改。
如果传入mch_key 全部请求会以加密方式传递数据  

```rust
use weapay::{AlipayConfig,Payment};
use weapay::alipay::prelude::*;
let app_id = "2021003168621630".to_string();
let app_private_key = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\ali_private_key.txt".to_string();
let alipay_public_cert = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\ali_public_key.txt".to_string();
let config = AlipayConfig {
   app_id,
   app_private_key,
   alipay_public_cert,
   notify_url: "https://example.com".to_string(),
   ..Default::default()
};
let payment = Payment::new(config.clone());
assert_eq!(payment.config.app_id, config.app_id);
```

## 下单
```rust
let data = ReqOrderBody {
    out_trade_no: "T20240407007".to_string(),
    total_amount: "0.99".to_string(),
    subject: "旅行卡年卡服务".to_string(),
    //product_code: Some("FACE_TO_FACE_PAYMENT".to_string()),
    //buyer_id: Some("2088722032795825".to_string()),
    ..Default::default()
};
// 通过指定method 来进行各种支付类型的下单，如当面付、app支付 
let result = payment.create_order("alipay.trade.precreate", data).await;
if result.is_err() {
    let error = result.err().unwrap();
    println!("{}", error);
} else {
    let result = result.unwrap();
    assert_eq!(result.qr_code.is_some(), true);
    println!("qr_code==>{:?}", result.qr_code);
}
```

## 查询订单
```rust
//根据商家订单号查询
let result = payment.query_order("2406220006").await;
if result.is_err() {
    let error = result.err().unwrap();
    println!("{}", error);
} else {
    let result = result.unwrap();
    assert_eq!(result.out_trade_no, Some("2406220006".to_string()));
    //println!("{:?}", result);
}
//根据支付宝交易号查询

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
```
## 异步通知处理
notify 方法对数据进行验签，同时返回 NotifyOrderBody 数据结构数据。  

```rust
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
```
## 退款
退款相关需要引入 weapay::alipay::prelude::RefundTrait
```rust
let payment = Payment::new(config);
let data = ReqRefundOrder {
    out_trade_no: Some("T20240407004".to_string()),
    refund_amount: "10".to_string(),
    refund_reason: Some("测试退款".to_string()),
    ..Default::default()
};
let result = payment.refund(data).await;
if result.is_err() {
    let error = result.err().unwrap();
    println!("{}", error);
} else {
    let result = result.unwrap();
    assert_eq!(result.refund_fee, "10.00".to_string());
    println!("result==>{:?}", result);
}
```
## 查询退款
```rust
let payment = Payment::new(config);
let data = ReqRefundQuery {
    out_request_no: Some("T20240407004".to_string()),
    out_trade_no: Some("T20240407004".to_string()),
    ..Default::default()
};
let result = payment.query_refund(data).await;
if result.is_err() {
    let error = result.err().unwrap();
    println!("{}", error);
} else {
    let result = result.unwrap();
    assert_eq!(result.total_amount, Some("10.00".to_string()));
    println!("result==>{:?}", result);
}
```
## 帐单下载
```rust
use weapay::alipay::prelude::BillTrait;

let payment = Payment::new(config);
let data = ReqBillQuery {
    bill_type: "trade".to_string(),
    bill_date: "2024-07-24".to_string(),
    ..Default::default()
};
let result = payment.trade_bill(data).await;
if result.is_err() {
    println!("{:?}", result);
}
assert!(result.is_ok());
```
更多使用方法查看源码中测试方法