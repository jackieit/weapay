use serde::{Deserialize, Serialize};

//FundBill
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyFundBill {
    //交易使用的资金渠道。
    pub fund_channel: Option<String>,
    //该支付工具类型所使用的金额。
    pub amount: Option<String>,
}
//其它出资方明细
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyOtherContributeDetail {
    //其它出资方金额
    pub contribute_amount: Option<String>,
    //其它出资方名称
    pub contribute_type: Option<String>,
}
//优惠券信息说明
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotifyVoucherDetail {
    // /券 ID
    pub voucher_id: String,
    //券模板 ID
    pub template_id: Option<String>,
    //券名称
    pub name: String,
    //券类型
    pub r#type: String,
    //优惠券面额
    pub amount: String,
    //商家出资金额。
    pub merchant_contribute: Option<String>,
    //其他出资方出资金额。
    pub other_contribute: Option<String>,
    //优惠券的其他出资方明细
    pub other_contribute_detail: Option<NotifyOtherContributeDetail>,
    //优惠券备注信息。
    pub memo: Option<String>,
}
/// 异步通知返回Body
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct NotifyOrderBody {
    //通知时间
    pub notify_time: String,
    //通知类型
    pub notify_type: String,
    //通知校验 ID
    pub notify_id: String,
    //charset
    pub charset: Option<String>,
    //version
    pub version: Option<String>,
    //sign_type
    pub sign_type: Option<String>,
    //签名。
    pub sign: Option<String>,
    //支付宝交易号。支付宝交易凭证号
    pub trade_no: String,
    //开发者的 app_id。
    pub app_id: String,
    //开发者的 app_id，在服务商调用的场景下为授权方的 app_id。
    pub auth_app_id: Option<String>,
    //商户订单号。
    pub out_trade_no: String,
    //商家业务号。商家业务 ID
    pub out_biz_no: Option<String>,
    //买家支付宝用户号
    pub buyer_id: String,
    pub buyer_open_id: Option<String>,
    //买家支付宝账号
    pub buyer_logon_id: String,
    //卖家支付宝用户号。
    pub seller_id: Option<String>,
    //卖家支付宝账号
    pub seller_email: Option<String>,
    //交易状态。咨询目前所处的状态。
    pub trade_status: String,
    //订单金额。
    pub total_amount: String,
    //实收金额。
    pub receipt_amount: String,
    //开票金额。
    pub invoice_amount: Option<String>,
    //付款金额
    pub buyer_pay_amount: Option<String>,
    //集分宝金额。
    pub point_amount: Option<String>,
    //总退款金额。
    pub refund_fee: Option<String>,
    //实际退款金额。
    pub send_back_fee: Option<String>,
    //订单标题。
    pub subject: Option<String>,
    //商品描述。
    pub body: Option<String>,
    //交易创建时间。
    pub gmt_create: Option<String>,
    //交易 付款时间。
    pub gmt_payment: Option<String>,
    //交易退款时间
    pub gmt_refund: Option<String>,
    //交易结束时间。
    pub gmt_close: Option<String>,
    //支付金额信息。
    pub fund_bill_list: Option<String>,
    //公共回传参数，如果请求时传递了该参数，则返回给商家时会在异步通知时将该参数原样返回。
    pub passback_params: Option<String>,
    //账期结算标识
    pub biz_settle_mode: Option<String>,
}
