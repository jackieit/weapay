use serde::{Deserialize, Serialize};
/// 下单支付类型
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum TradeType {
    #[default]
    JSAPI,
    NATIVE,
    App,
    MWEB,
    MICROPAY,
    FACEPAY,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub enum TradeState {
    #[default]
    SUCCESS,
    REFUND,
    NOTPAY,
    CLOSED,
    REVOKED,
    USERPAYING,
    PAYERROR,
}
//订单金额信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqAmountInfo {
    //币种
    pub currency: Option<String>,
    //总金额
    pub total: i32,
}
//支付者信息。
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct PayerInfo {
    //用户标识
    pub openid: String,
}
//单品列表信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqGoodsDetail {
    //商品编码
    pub merchant_goods_id: String,
    //微信支付商品编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_goods_id: Option<String>,
    //商品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    //商品数量
    pub quantity: i32,
    //商品单价
    pub unit_price: i32,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqOrderDetail {
    //订单原价
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cost_price: Option<i32>,
    //商品小票ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_id: Option<String>,
    //商品单价
    pub goods_detail: Vec<ReqGoodsDetail>,
}
// 商户门店信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqStoreInfo {
    //商户侧门店编号
    pub id: String,
    //商户侧门店名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    //门店行政区划码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub area_code: Option<String>,
    //门店详细地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
}
// H5场景信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqSceneH5Info {
    //场景类型
    pub r#type: String,
    //应用名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_name: Option<String>,
    //网站URL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_url: Option<String>,
    //iOS平台BundleID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_id: Option<String>,
    //Android平台PackageName
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_name: Option<String>,
}
//【场景信息】 支付场景描述
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqSceneInfo {
    pub payer_client_ip: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_info: Option<ReqStoreInfo>,
    pub h5_info: Option<ReqSceneH5Info>,
}
// 结算信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqSettleInfo {
    //是否指定分账
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profit_sharing: Option<bool>,
}
/// 统一下单请求体
/// appid 与 mchid 为可选字段
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqOrderBody {
    //服务端app_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_appid: Option<String>,
    //服务商商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sp_mchid: Option<String>,
    //服务商子商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
    //服务商子商户公众号ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_appid: Option<String>,

    //公众号ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub appid: Option<String>,
    //商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mchid: Option<String>,
    //商品描述
    pub description: String,
    //商户订单号
    pub out_trade_no: String,
    //交易结束时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    //附加数据
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach: Option<String>,
    //通知地址
    pub notify_url: Option<String>,
    //订单优惠标记
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_tag: Option<String>,
    //电子发票入口开放标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub support_fapiao: Option<bool>,
    //交易金额
    pub amount: ReqAmountInfo,
    //支付者信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer: Option<PayerInfo>,
    //订单详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    //场景信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene_info: Option<ReqSceneInfo>,
    //结算信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_info: Option<ReqSettleInfo>,
}
///统一下单返回体
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct CreateOrderResponse {
    //JSAPI,APP 支付,小程序 预支付交易会话标识
    pub prepay_id: Option<String>,
    //NATIVE 支付链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_url: Option<String>,
    //h5下单支付跳转链接
    #[serde(skip_serializing_if = "Option::is_none")]
    pub h5_url: Option<String>,
}
/// 用于JSAPI 支付的签名数据
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct JsapiSignPackage {
    //公众号ID
    pub app_id: String,
    //时间戳
    pub time_stamp: String,
    //随机字符串
    pub nonce_str: String,
    //统一下单返回的预支付交易会话标识
    pub package: String,
    //签名方式
    pub sign_type: String,
    //签名
    pub pay_sign: String,
}
/// 用于app 支付的签名数据
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct AppSignPackage {
    //应用ID
    pub app_id: String,
    //商户号
    pub partner_id: String,
    //统一下单返回的预支付交易会话标识
    pub prepay_id: String,
    // package value
    pub package_value: String,
    //随机字符串
    pub nonce_str: String,
    //时间戳
    pub time_stamp: String,
    //签名方式
    pub sign: String,
}
/// 统一下单返回格式
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CreateOrderResult {
    Default(CreateOrderResponse),
    JSAPI(JsapiSignPackage),
    APP(AppSignPackage),
}
// 错误返回详情 通常Status 4xx 5xx时返回
///错误返回方式以Json字符串方式返回
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ErrorDetail {
    //错误码
    pub field: String,
    //错误信息
    pub value: String,
    //issue
    pub issue: String,
    //错误详情
    pub location: String,
}
// 错误返回体
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ErrorResponse {
    //错误码
    pub code: String,
    //错误信息
    pub message: String,
    //错误详情
    pub detail: ErrorDetail,
}
// 支付通知数据体resource
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespResource {
    //原始回调类型
    pub original_type: String,
    //对开启结果数据进行加密的加密算法，目前只支持AEAD_AES_256_GCM。
    pub algorithm: String,
    //Base64编码后的开启/停用结果数据密文。
    pub ciphertext: String,
    //附加数据。
    pub associated_data: Option<String>,
    //加密使用的随机串。
    pub nonce: String,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespSceneInfo {
    //商户端设备号
    pub device_id: Option<String>,
}
// 支付通过数据体
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespBody {
    //通知ID
    pub id: String,
    //通知创建时间
    pub create_time: String,
    //通知数据类型
    pub resource_type: String,
    //通知类型
    pub event_type: String,
    //回调摘要
    pub summary: String,
    //通知数据
    pub resource: RespResource,
}
// 支付通知resource amount
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespAmount {
    //币种
    pub currency: String,
    //总金额
    pub total: i32,
    //用户支付金额
    pub payer_total: i32,
    //用户支付币种
    pub payer_currency: String,
}
// 支付通知商品信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespGoodsDetail {
    //商品ID
    pub goods_id: String,
    //商品数量
    pub quantity: i32,
    //商品单价
    pub unit_price: i32,
    //商品优惠金额
    pub discount_amount: i32,
    //商品备注
    pub goods_remark: Option<String>,
}
// 支付通知优惠信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RespPromotionDetail {
    //优惠券ID
    pub coupon_id: String,
    //优惠名称
    pub name: Option<String>,
    //优惠范围
    pub scope: Option<String>,
    //优惠类型
    pub r#type: Option<String>,
    //优惠券面额
    pub amount: i32,
    //优惠数量
    pub stock_id: Option<String>,
    //微信出资单位为分
    pub wechatpay_contribute: Option<i32>,
    //商户出资单位为分
    pub merchant_contribute: Option<i32>,
    //其他出资单位为分
    pub other_contribute: Option<i32>,
    //优惠币种
    pub currency: Option<String>,
    //单品列表信息
    pub goods_detail: Vec<RespGoodsDetail>,
}
// 支付通知返回解密后内容
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ResourceOrderBody {
    //服务端app_id
    pub sp_appid: Option<String>,
    //服务商商户号
    pub sp_mchid: Option<String>,
    //服务商子商户号
    pub sub_mchid: Option<String>,
    //服务商子商户公众号ID
    pub sub_appid: Option<String>,
    //应用ID
    pub appid: Option<String>,
    //商户号
    pub mchid: Option<String>,
    //商户订单号
    pub out_trade_no: String,
    //交易ID
    pub transaction_id: String,
    //交易类型
    pub trade_type: TradeType,
    //交易状态
    pub trade_state: TradeState,
    //交易状态描述
    pub trade_state_desc: String,
    //付款银行 银行类型，采用字符串类型的银行标识。银行标识请参考《银行类型对照表》。
    //https://pay.weixin.qq.com/docs/merchant/development/chart/bank-type.html
    pub bank_type: String,
    //附加数据
    pub attach: Option<String>,
    //支付完成时间
    pub success_time: String,
    //支付者信息
    pub payer: PayerInfo,
    //总金额
    pub amount: RespAmount,
    //支付场景描述
    pub scene_info: Option<RespSceneInfo>,
    //优惠标记
    pub promotion_detail: Option<Vec<RespPromotionDetail>>,
}
