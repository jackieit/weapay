use serde::{Deserialize, Serialize};
///商品明细
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqGoodsDetail {
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
    //app 返回体使用
    //alipay_goods_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay_goods_id: Option<String>,
    //商品描述信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    //商家侧小程序商品ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_item_id: Option<String>,
    //商家侧小程序商品ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_sku_id: Option<String>,
}
///业务可选参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqExtendParams {
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
pub struct ReqBusinessParams {
    //商户传入业务信息
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mc_create_trade_ip: Option<String>,
}
///优惠明细参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqPromoParam {
    //存在延迟扣款这一类的场景，用这个时间表明用户发生交易的时间
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actual_order_time: Option<String>,
}
/// 外部指定买家
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqExtUserInfo {
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
pub struct ReqSubMerchant {
    //二级商户的支付宝id
    pub merchant_id: String,
    //商户id类型
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchant_type: Option<String>,
}
//开票关键信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqInvoiceKeyInfo {
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
pub struct ReqInvoiceDetail {
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
pub struct ReqInvoiceInfo {
    //开票关键信息
    pub key_info: ReqInvoiceKeyInfo,
    //开票内容
    pub details: Vec<ReqInvoiceDetail>,
}
//签约access_parms
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqAccessParams {
    pub channel: String,
}
//周期管控规则参数
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqPeriodRuleParams {
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
pub struct ReqSignParams {
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
pub struct ReqOrderBody {
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
    //小程序支付中，小程序的APPID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_app_id: Option<String>,
    //买家支付宝用户ID。
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    //买家支付宝用户唯一标识
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_open_id: Option<String>,
    //op_buyer_open_id
    #[serde(skip_serializing_if = "Option::is_none")]
    pub op_buyer_open_id: Option<String>,
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
//交易支付使用的资金渠道。
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResTradeFundBill {
    //交易使用的资金渠道
    pub fund_channel: String,
    //该支付工具类型所使用的金额
    pub amount: String,
    //渠道实际付款金额
    pub real_amount: Option<String>,
}
//本交易支付时使用的所有优惠券信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResVoucherDetail {
    //券id
    pub id: String,
    //券名称
    pub name: String,
    //券类型
    pub r#type: String,
    //优惠金额
    pub amount: String,
    //商家出资
    pub merchant_contribute: Option<String>,
    //其他出资方出资金额
    pub other_contribute: Option<String>,
    //优惠说明
    pub memo: Option<String>,
    //券模板id
    pub template_id: Option<String>,
    //如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时用户实际付款的金额
    pub purchase_buyer_contribute: Option<String>,
    //如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时商户优惠的金额
    pub purchase_merchant_contribute: Option<String>,
    //如果使用的这张券是用户购买的，则该字段代表用户在购买这张券时平台优惠的金额
    pub purchase_ant_contribute: Option<String>,
    //other_contribute_detail
    pub other_contribute_detail: Option<Vec<ResContributeDetail>>,
}
//ResContributeDetail
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResContributeDetail {
    //其它出资方金额
    pub contribute_amount: String,
    //其它出资方名称
    pub contribute_type: String,
}
// 下单返回
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResOrderBody {
    //支付宝交易号
    pub trade_no: Option<String>,
    //商家订单号
    pub out_trade_no: Option<String>,
    //当前预下单请求生成的二维码码串 //二维码支付
    pub qr_code: Option<String>,
    //签名字符串 app支付 & 周期扣款
    #[serde(rename(deserialize = "orderStr"))]
    pub order_str: Option<String>,
    //跳转页面数据 wap支付
    #[serde(rename(deserialize = "pageRedirectionData"))]
    pub page_redirection_data: Option<String>,
    //买家支付宝用户号
    pub buyer_logon_id: Option<String>,
    //交易状态
    pub trade_status: Option<String>,
    //总金额
    pub total_amount: Option<String>,
    //实收金额
    pub receipt_amount: Option<String>,
    //门店ID
    pub store_id: Option<String>,
    //商户机具终端编号
    pub terminal_id: Option<String>,
    //交易支付时间
    pub gmt_payment: Option<String>,
    //交易支付使用的资金渠道
    pub fund_bill_list: Option<Vec<ResTradeFundBill>>,
    //买家在支付宝的用户id
    pub buyer_user_id: Option<String>,
    //本次交易打款给卖家的时间
    pub send_pay_date: Option<String>,
    //买家支付宝用户唯一标识
    pub buyer_open_id: Option<String>,
    //买家用户类型
    pub buyer_user_type: Option<String>,
    //商家优惠金额
    pub mdiscount_amount: Option<String>,
    //平台优惠金额
    pub discount_amount: Option<String>,
    //ext_infos
    pub ext_infos: Option<String>,
    //买家实付金额
    pub buyer_pay_amount: Option<String>,
    //集分宝金额
    pub point_amount: Option<String>,
    //交易中可给用户开具发票的金额
    pub invoice_amount: Option<String>,
    //发生支付交易的商户门店名称
    pub store_name: Option<String>,
    //本次交易支付所使用的单品券优惠的商品优惠信息
    pub discount_goods_detail: Option<String>,
    //本交易支付时使用的所有优惠券信息
    pub voucher_detail_list: Option<ResVoucherDetail>,
    // app查询相关
    //req_goods_detail
    pub req_goods_detail: Option<Vec<ReqGoodsDetail>>,
    //该字段用于描述当前账期交易的场景。
    pub period_scene: Option<String>,
    //标价币种，该参数的值为支付时传入的trans_currency
    pub trans_currency: Option<String>,
    //订单结算币种
    pub settle_currency: Option<String>,
    //结算币种订单金额
    pub settle_amount: Option<String>,
    //订单支付币种
    pub pay_currency: Option<String>,
    //支付币种订单金额
    pub pay_amount: Option<String>,
    //结算币种兑换标价币种汇率
    pub settle_trans_rate: Option<String>,
    //标价币种兑换支付币种汇率
    pub trans_pay_rate: Option<String>,
    //行业特殊信息-统筹相关
    pub industry_sepc_detail: Option<String>,
    //行业特殊信息-个账相关
    pub industry_sepc_detail_acc: Option<String>,
    //该笔交易针对收款方的收费金额
    pub charge_amount: Option<String>,
    //费率活动标识。
    pub charge_flags: Option<String>,
    //支付清算编号，
    pub settlement_id: Option<String>,
    //返回的交易结算信息
    pub trade_settle_info: Option<ResTradeSettleInfo>,
    //预授权支付模式
    pub auth_trade_pay_mode: Option<String>,
    //间连商户在支付宝端的商户编号
    pub alipay_sub_merchant_id: Option<String>,
    //若用户使用花呗分期支付，且商家开通返回此通知参数，则会返回花呗分期信息
    pub hb_fq_pay_info: Option<ResHbFqPayInfo>,
    //履约详情列表
    pub fulfillment_detail_list: Option<ResFulfillmentDetail>,
    //交易附加状态
    pub additional_status: Option<String>,
    //公用回传参数
    pub passback_params: Option<String>,
    //信用支付模式
    pub credit_pay_mode: Option<String>,
    //信用业务单号
    pub credit_biz_order_id: Option<String>,
    //惠营宝回票金额。单位：元。
    pub hyb_amount: Option<String>,
    //间联交易下，返回给机构的信 息
    pub bkagent_resp_info: Option<ResBkAgentRespInfo>,
    //计费信息列表
    pub charge_info_list: Option<Vec<ResChargeInfo>>,
    //账期结算标识
    pub biz_settle_mode: Option<String>,
}
//计费信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResChargeInfo {
    //实收费用
    pub charge_fee: Option<String>,
    //原始费用
    pub original_charge_fee: Option<String>,
    //签约费率
    pub switch_fee_rate: Option<String>,
    //是否收款账号出资
    pub is_rating_on_trade_receiver: Option<String>,
    //是否合约指定收费账号出资
    pub is_rating_on_switch: Option<String>,
    //手续费类型
    pub charge_type: Option<String>,
    //组合支付收费明细
    pub sub_fee_detail_list: Option<Vec<ResSubFee>>,
}
//组合支付收费明细
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResSubFee {
    //实收费用
    pub charge_fee: Option<String>,
    //原始费用
    pub original_charge_fee: Option<String>,
    //签约费率
    pub switch_fee_rate: Option<String>,
}
//间联交易机构信息
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResBkAgentRespInfo {
    //原快捷交易流水号
    pub bindtrx_id: String,
    //枚举值，01 银联；02 网联；03 连通等
    pub bindclrissr_id: String,
    //付款机构在清算组织登记或分配的机构代码
    pub bindpyeracctbk_id: String,
    //用户在银行付款账号的标记化处理编号
    pub bkpyeruser_code: String,
    //设备推测位置
    pub estter_location: String,
}
//履约详情列表
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResFulfillmentDetail {
    //履约金额
    pub fulfillment_amount: String,
    //商户发起履约请求时，传入的out_request_no
    pub out_request_no: String,
    //履约支付时间
    pub gmt_payment: String,
}
//HbFqPayInfo
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResHbFqPayInfo {
    //花呗分期数
    pub user_install_num: Option<String>,
}
//TradeSettleInfo
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResTradeSettleInfo {
    //交易结算信息
    pub trade_unsettled_amount: Option<String>,
    //trade_settle_detail_list
    pub trade_settle_detail_list: Option<Vec<ResTradeSettleDetail>>,
}
//ResTradeSettleDetail
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResTradeSettleDetail {
    //结算操作类型
    pub operation_type: String,
    //操作日期
    pub operation_dt: String,
    //实际操作金额
    pub amount: String,
    //户操作序列号
    pub operation_serial_no: Option<String>,
    //转出账号
    pub trans_out: Option<String>,
    //转入账号
    pub trans_in: Option<String>,
    //商户请求的转出账号
    pub ori_trans_out: Option<String>,
    //商户请求的转入账号
    pub ori_trans_in: Option<String>,
}
/// 查询订单请示体
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqQueryOrderBody {
    //商户订单号
    pub out_trade_no: Option<String>,
    //支付宝交易号
    pub trade_no: Option<String>,
    //查询选项
    pub query_options: Option<Vec<String>>,
    //银行间联模式下有用
    //pub org_pid: Option<String>,
}
///关闭订单请示体
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqCloseOrderBody {
    //商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
    //支付宝交易号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    //卖家端自定义的的操作员 ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operator_id: Option<String>,
    //notify_url
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notify_url: Option<String>,
}
///关闭订单返回体
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResCloseOrderBody {
    //支付宝交易号
    pub trade_no: Option<String>,
    //商户订单号
    pub out_trade_no: Option<String>,
}
///撤销订单请示体
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ReqCancelOrderBody {
    //支付宝交易号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trade_no: Option<String>,
    //商户订单号
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_trade_no: Option<String>,
}
///撤销订单返回体
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ResCancelOrderBody {
    //支付宝交易号
    pub out_trade_no: String,
    //是否需要重试
    pub retry_flag: String,
    //支付宝交易号; 当发生交易关闭或交易退款时返回；
    pub trade_no: Option<String>,
    //本次撤销触发的交易动作
    pub action: Option<String>,
}
