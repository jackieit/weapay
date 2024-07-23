use serde::{Deserialize, Serialize};
// 退款通知退款金额
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ResourceRefundAmount {
    //币种
    pub total: i32,
    //退款金额
    pub refund: i32,
    //用户支付金额
    pub payer_total: i32,
    //用户支付币种
    pub payer_refund: i32,
}
// 退款通知返回解决后内容
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ResourceRefundBody {
    //商户号
    pub mchid: String,
    //交易ID
    pub transaction_id: String,
    //商户订单号
    pub out_trade_no: String,
    //微信退款单号
    pub refund_id: String,
    //商户退款单号
    pub out_refund_no: String,

    //退款状态
    pub refund_status: String,
    //退款成功时间
    pub success_time: Option<String>,
    //当前退款单的退款入账方
    pub user_received_account: String,
    //退款金额
    pub amount: ResourceRefundAmount,
}
// 退款金额From
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundAmountFrom {
    //退款金额
    pub account: String,
    //退款金额
    pub amount: i32,
}
//退款金额信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundAmountInfo {
    //退款金额
    pub refund: i32,
    //退款金额来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from: Option<Vec<ReqRefundAmountFrom>>,
    //币种
    pub currency: String,
    //原订单金额
    pub total: i32,

    //用户支付金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_total: Option<i32>,
    //用户退款金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_refund: Option<i32>,
    //应结退款金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_refund: Option<i32>,
    //应结订单金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settlement_total: Option<i32>,
    //优惠退款金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount_refund: Option<i32>,

    //手续费退款金额
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_fee: Option<i32>,
}
// 退款商品信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundGoodsDetail {
    //商户侧商品编码
    pub merchant_goods_id: String,
    //微信支付商品编码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechatpay_goods_id: Option<String>,
    //商品名称
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_name: Option<String>,
    //商品单价
    pub unit_price: i32,
    //商品退款金额
    pub refund_amount: i32,
    //商品退货数量
    pub refund_quantity: i32,
}
// 退款订单信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundOrder {
    //子商户号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_mchid: Option<String>,
    //微信支付订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_id: Option<String>,
    //商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    //商户退款单号
    pub out_refund_no: String,
    //退款原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    //退款资金来源
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funds_account: Option<String>,
    //退款金额
    pub amount: ReqRefundAmountInfo,
    //退款结果通知url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
    //退款商品列表
    pub goods_detail: Option<Vec<ReqRefundGoodsDetail>>,
}
// 退款优惠信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundPromotionDetail {
    //优惠券ID
    pub promotion_id: String,
    //优惠范围
    pub scope: String,
    //优惠类型
    pub r#type: String,
    //优惠券面额
    pub amount: i32,
    //优惠退款金额
    pub refund_amount: i32,

    pub goods_detail: Vec<ReqRefundGoodsDetail>,
}
// 退款订单返回
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct RefundResponse {
    //微信支付退款单号
    pub refund_id: String,
    //商户退款单号
    pub out_refund_no: String,
    //微信支付订单号
    pub transaction_id: String,
    //商户订单号
    pub out_trade_no: String,
    //退款渠道
    pub channel: String,
    //退款入账账户
    pub user_received_account: String,
    //退款成功时间
    pub success_time: Option<String>,
    //退款创建时间
    pub create_time: String,
    //退款状态
    pub status: String,
    //资金账户
    pub funds_account: String,
    //金额信息
    pub amount: ReqRefundAmountInfo,
    //优惠退款信息
    pub promotion_detail: Vec<ReqRefundPromotionDetail>,
}
