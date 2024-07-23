use crate::alipay::prelude::*;
use crate::error::WeaError;
use crate::*;
use std::future::Future;
pub trait RefundTrait {
    /// 申请退款
    fn refund(
        &self,
        data: ReqRefundOrder,
    ) -> impl Future<Output = Result<ResRefundResponse, WeaError>>;

    /// 查询退款
    fn query_refund(
        &self,
        refund_query: ReqRefundQuery,
    ) -> impl Future<Output = Result<ResRefundResponse, WeaError>>;
}
impl RefundTrait for Payment<AlipayConfig> {
    fn refund(
        &self,
        data: ReqRefundOrder,
    ) -> impl Future<Output = Result<ResRefundResponse, WeaError>> {
        async move {
            let refund_body = serde_json::to_string(&data)?;
            let url = self.get_uri("alipay.trade.refund");
            self.do_request::<ResRefundResponse>(&url, "POST", &refund_body)
                .await
        }
    }
    // query_refund
    fn query_refund(
        &self,
        refund_query: ReqRefundQuery,
    ) -> impl Future<Output = Result<ResRefundResponse, WeaError>> {
        async move {
            let refund_query = serde_json::to_string(&refund_query)?;
            let url = self.get_uri("alipay.trade.fastpay.refund.query");
            self.do_request::<ResRefundResponse>(&url, "POST", &refund_query)
                .await
        }
    }
}
