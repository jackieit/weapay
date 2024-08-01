use crate::alipay::prelude::*;
use crate::*;

pub trait RefundTrait {
    /// 申请退款
    fn refund(&self, data: ReqRefundOrder) -> BoxFuture<ResRefundResponse>;

    /// 查询退款
    fn query_refund(&self, refund_query: ReqRefundQuery) -> BoxFuture<ResRefundQuery>;
}
impl RefundTrait for Payment<AlipayConfig> {
    fn refund(&self, data: ReqRefundOrder) -> BoxFuture<ResRefundResponse> {
        Box::pin(async move {
            let refund_body = serde_json::to_string(&data)?;
            let url = self.get_uri("alipay.trade.refund");
            self.do_request::<ResRefundResponse>(&url, "POST", &refund_body)
                .await
        })
    }
    // query_refund
    fn query_refund(&self, refund_query: ReqRefundQuery) -> BoxFuture<ResRefundQuery> {
        Box::pin(async move {
            let refund_query = serde_json::to_string(&refund_query)?;
            let url = self.get_uri("alipay.trade.fastpay.refund.query");
            self.do_request::<ResRefundQuery>(&url, "POST", &refund_query)
                .await
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::alipay::prelude::*;
    use crate::*;
    #[tokio::test]
    async fn test_refund() {
        let config = crate::tests::get_config().1;
        //println!("{:?}", config);
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
    }
    // test query refund
    #[tokio::test]
    async fn test_query_refund() {
        let config = crate::tests::get_config().1;
        //println!("{:?}", config);
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
    }
}
