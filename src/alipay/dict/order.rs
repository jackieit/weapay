use serde::{Deserialize, Serialize};
///商品明细
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqGoodsDetail{
    //商品的编号
    pub goods_id: String,
    //商品名称
    pub goods_name: String,
    //商品数量
    pub quantity: i32,
    //商品单价，单位为元
    pub price: String,
    //商品类目
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_category: Option<String>,
    //商品类目树
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories_tree: Option<String>,
    //商品的展示地址
    pub show_url: Option<String>,
}
///业务可选参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqExtendParams{
    //系统商编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sys_service_provider_id: Option<String>,
    //特殊场景下，允许商户指定交易展示的卖家名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub specified_seller_name: Option<String>,
    //卡类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_type: Option<String>,
}
///商户传入业务信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqBusinessParams{
    //商户传入业务信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_create_trade_ip: Option<String>,
}
///优惠明细参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqPromoParam{
    //存在延迟扣款这一类的场景，用这个时间表明用户发生交易的时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_order_time: Option<String>,
}
/// 外部指定买家
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqExtUserInfo{
    //买家证件号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_no: Option<String>,
    //允许的最小买家年龄。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min_age: Option<String>,
    //买家姓名  need_check_info=T时该参数才有效
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    //指定买家手机号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mobile: Option<String>,
    //指定买家证件类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cert_type: Option<String>,
    //是否强制校验买家信息；
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_check_info: Option<String>,
    //买家加密身份信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub identity_hash: Option<String>,
}
//二级商户信息。
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqSubMerchant{
    //二级商户的支付宝id
    pub merchant_id: String,
    //商户id类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_type: Option<String>,
}
//开票关键信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqInvoiceKeyInfo{
    //交易是否支持开票
    pub is_support_invoice: String,
    //开票商户名称
    pub invoice_merchant_name: String,
    //开票商户税号
    pub tax_no: String,
}
//开票内容
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReqInvoiceDetail{
    //代码
    pub code: String,
    //名称
    pub name: String,
    //数量
    pub num: String,
    //金额
    pub sum_price: String,
    //税率
    pub tax_rate: String,
}
//开票信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqInvoiceInfo{
    //开票关键信息
    pub key_info: ReqInvoiceKeyInfo,
    //开票内容
    pub details: Vec<ReqInvoiceDetail>,
}
//签约access_parms
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqAccessParams{
    pub channel: String,
}
//周期管控规则参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqPeriodRuleParams{
    //周期类型
    pub period_type: String,
    //周期值
    pub period: String,
    //首次执行时间execute_time是周期扣款产品必填
    pub execute_time: String,
    //单次扣款最大金额single_amount
    pub single_amount: String,
    //总金额限制，单位为元。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_amount: Option<String>,
    //总扣款次数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_payments: Option<String>,
}
///签约参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqSignParams{
 //签约产品码
  pub product_code: String,
  //个人签约产品码
  pub personal_product_code: String,
  //协议签约场景
  pub sign_scene: String,
  //access_params
  pub access_params: ReqAccessParams,
  //周期管控规则参数
  pub period_rule_params: ReqPeriodRuleParams,
  //设置签约请求的有效时间
  #[serde(skip_serializing_if = "Option::is_none")]
  pub effect_time: Option<String>,
  //商户签约号
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_agreement_no: Option<String>,
  //用户在商户网站的登录账号
  #[serde(skip_serializing_if = "Option::is_none")]
  pub external_logon_id: Option<String>,
  //签约成功后商户用于接收异步通知的地址
  #[serde(skip_serializing_if = "Option::is_none")]
  pub sign_notify_url: Option<String>,
}
///请求订单参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqOrderBody{
    //商户订单号
    pub out_trade_no: String,
    //订单总金额。
    pub total_amount: String,
    //订单标题
    pub subject: String,
    //支付授权码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_code: Option<String>,
    //资金预授权单号。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_no: Option<String>,
    //预授权确认模式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_confirm_mode: Option<String>,
    //场景值
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scene: Option<String>,
    //通知地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    //针对用户授权接口，获取用户相关数据时，用于标识用户授权关系
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
    //用户付款中途退出返回商户网站的地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quit_url: Option<String>,
    //PC扫码支付的方式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qr_pay_mode: Option<String>,
    //商户自定义二维码宽度。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qrcode_width: Option<String>,
    //商家和支付宝签约的产品码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_code: Option<String>,
    //卖家支付宝用户ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub seller_id: Option<String>,
    //订单附加信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    //goods_detail
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<ReqGoodsDetail>>,
    /// app 绝对超时时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_expire: Option<String>,
    //二级商户信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_merchant: Option<ReqSubMerchant>,
    //业务扩展参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extend_params: Option<ReqExtendParams>,
    ///公用回传参数，如果请求时传递了该参数，则返回给商户时会回传该参数。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passback_params: Option<String>,
    //商户传入业务信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_params: Option<ReqBusinessParams>,
    //可打折金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable_amount: Option<String>,
    //优惠明细参数
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promo_params: Option<ReqPromoParam>,
    //请求后页面的集成方式。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub integration_type: Option<String>,
    //请求来源地址。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_from_url: Option<String>,
    //商户门店编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub store_id: Option<String>,
    //商户操作员编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    //商户机具终端编号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terminal_id: Option<String>,
    //商户原始订单号，最大长度限制 32 位
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_order_no: Option<String>,
    ///外部指定买家
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ext_user_info: Option<ReqExtUserInfo>,
    //返回参数选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_options: Option<Vec<String>>,
    //签约信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agreement_sign_params: Option<ReqSignParams>,
}
// 支付统一下单End

// 下单返回
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResOrderBody{
    //支付宝交易号
    pub trade_no: String,
    //商家订单号
    pub out_trade_no: String,
    //买家支付宝用户号
    pub buyer_user_id: String,
    //交易状态
    pub trade_status: String,
    //总金额
    pub total_amount: String,
    //实收金额
    pub receipt_amount: String,
    //买家支付金额
    pub buyer_pay_amount: String,
    //集分宝金额
    pub point_amount: String,
    //开票金额
    pub invoice_amount: String,
    //付款金额
    pub pay_amount: String,
    //集分宝金额
    pub jfb_buyer_amount: String,
    //集分宝退回金额
    pub jfb_discount_amount: String,
}