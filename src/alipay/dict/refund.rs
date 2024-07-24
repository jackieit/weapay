use serde::{Deserialize, Serialize};
/// 退款请示参数
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundOrder {
    //退款金额
    pub refund_amount: String,
    //商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    //支付宝交易号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    //退款原因
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_reason: Option<String>,
    //退款请求号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_request_no: Option<String>,
    //退款包含的商品列表信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub goods_detail: Option<Vec<ReqRefundGoodsDetail>>,
    //退分账明细信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_royalty_parameters: Option<Vec<ReqOpenApiRoyaltyDetailInfoPojo>>,
    //查询选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_options: Option<String>,
    //针对账期交易
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_settle_confirm_no: Option<String>,
}
//退分账明细信息
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqOpenApiRoyaltyDetailInfoPojo {
    //收入方账户
    pub trans_in: String,
    //分账类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_type: Option<String>,
    //支出方账户。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_out: Option<String>,
    //支出方账户类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_out_type: Option<String>,
    //收入方账户类型。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_in_type: Option<String>,
    //分账的金额，单位为元
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    //分账描述
    #[serde(skip_serializing_if = "Option::is_none")]
    pub desc: Option<String>,
    //可选值：达人佣金、平台服务费、技术服务费、其他
    #[serde(skip_serializing_if = "Option::is_none")]
    pub royalty_scene: Option<String>,
    //分账收款方姓名，
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trans_in_name: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundGoodsDetail {
    //商品的编号
    pub goods_id: String,
    //该商品的退款总金额，单位为元
    pub refund_amount: String,
    //商家侧小程序商品ID，对应支付时传入的out_item_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_item_id: Option<String>,
    //商家侧小程序商品sku ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_sku_id: Option<String>,
}

/// 退款返回body
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResRefundResponse {
    //支付宝交易号
    pub trade_no: String,
    //商户订单号
    pub out_trade_no: String,
    //用户的登录id
    pub buyer_logon_id: String,
    //退款总金额。
    pub refund_fee: String,
    //退款使用的资金渠道
    pub refund_detail_item_list: Vec<TradeFundBill>,
    //交易在支付时候的门店名称
    pub store_name: Option<String>,
    //买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    //家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    //本次商户实际退回金额
    pub send_back_fee: Option<String>,
    //本次退款是否发生了资金变化
    pub fund_change: Option<String>,
    //本次请求退惠营宝金额。
    pub refund_hyb_amount: Option<String>,
    //退费信息
    pub refund_charge_info_list: Option<RefundChargeInfo>,
    //本交易支付时使用的所有优惠券信息
    pub voucher_detail_list: Option<Vec<super::order::ResVoucherDetail>>,
}
///RefundChargeInfo
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefundChargeInfo {
    //实退费用
    pub refund_charge_fee: Option<String>,
    //签约费率
    pub switch_fee_rate: Option<String>,
    //手续费类型
    pub charge_type: Option<String>,
    //组合支付退费明细
    pub refund_sub_fee_detail_list: Option<Vec<RefundSubFee>>,
}
//RefundSubFee
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RefundSubFee {
    //实退费用
    pub refund_charge_fee: Option<String>,
    //签约费率
    pub switch_fee_rate: Option<String>,
}
///TradeFundBill
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TradeFundBill {
    //交易使用的资金渠道
    pub fund_channel: String,
    //该支付工具类型所使用的金额
    pub amount: String,
    //渠道实际付款金额
    pub real_amount: Option<String>,
    //渠道所使用的资金类型,目前只在资金渠道
    pub fund_type: Option<String>,
}

///退款查询
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqRefundQuery {
    //商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    //支付宝交易号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    //退款请求号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_request_no: Option<String>,
    //查询选项
    #[serde(skip_serializing_if = "Option::is_none")]
    pub query_options: Option<String>,
}

///退款查询返回
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResRefundQuery {
    //支付宝交易号
    pub trade_no: Option<String>,
    //商户订单号
    pub out_trade_no: Option<String>,
    //本笔退款对应的退款请求号
    pub out_request_no: Option<String>,
    //该笔退款所对应的交易的订单金额
    pub total_amount: Option<String>,
    //本次退款请求，对应的退款金额
    pub refund_amount: Option<String>,
    //退款状态。
    pub refund_status: Option<String>,
    //退分账明细信息，
    pub refund_royaltys: Option<Vec<ResRefundRoyalty>>,
    //退款时间
    pub gmt_refund_pay: Option<String>,
    //本次退款使用的资金渠道
    pub refund_detail_item_list: Option<Vec<TradeFundBill>>,
    //本次商户实际退回金额
    pub send_back_fee: Option<String>,
    //银行卡冲退信息
    pub deposit_back_info: Option<DepositBackInfo>,
    //本交易支付时使用的所有优惠券信息
    pub voucher_detail_list: Option<Vec<super::order::ResVoucherDetail>>,
    //本次退款金额中退惠营宝的金额
    pub refund_hyb_amount: Option<String>,
    //退费信息
    pub refund_charge_info_list: Option<Vec<RefundChargeInfo>>,
    //银行卡冲退信息列表。
    pub deposit_back_info_list: Option<Vec<DepositBackInfo>>,
}
//DepositBackInfo
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DepositBackInfo {
    // 是否存在银行卡冲退信息
    pub has_deposit_back: Option<String>,
    //银行卡冲退状态
    pub dback_status: Option<String>,
    //银行卡冲退金额
    pub dback_amount: Option<String>,
    //银行响应时间
    pub bank_ack_time: Option<String>,
    //预估银行到账时间
    pub est_bank_receipt_time: Option<String>,
}
///ResRefundRoyalty
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ResRefundRoyalty {
    //退分账金额
    pub refund_amount: String,
    //退分账结果码
    pub result_code: String,
    //分账类型
    pub royalty_type: Option<String>,
    //分账金额
    pub royalty_amount: Option<String>,
    //转出人支付宝账号对应用户ID
    pub trans_out: Option<String>,
    //转出人支付宝账号
    pub trans_out_email: Option<String>,
    // /转入人支付宝账号对应用户ID
    pub trans_in: Option<String>,
    //转入人支付宝账号
    pub trans_in_email: Option<String>,
    // /商户请求的转出账号
    pub ori_trans_out: Option<String>,
    //商户请求的转入账号
    pub ori_trans_in: Option<String>,
}
