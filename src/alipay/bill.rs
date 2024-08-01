use crate::alipay::prelude::*;
use crate::*;

pub trait BillTrait {
    /// 申请交易账单
    /// 帐单下载地址30秒后失效
    fn trade_bill(&self, query: ReqBillQuery) -> BoxFuture<ResBill>;
}
impl BillTrait for Payment<AlipayConfig> {
    fn trade_bill(&self, query: ReqBillQuery) -> BoxFuture<ResBill> {
        Box::pin(async move {
            let query = serde_json::to_string(&query)?;
            let url = self.get_uri("alipay.data.dataservice.bill.downloadurl.query");
            self.do_request::<ResBill>(&url, "POST", &query).await
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::alipay::prelude::*;
    use crate::*;
    #[tokio::test]
    async fn test_trade_bill() {
        let config = crate::tests::get_config().1;
        //println!("{:?}", config);
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
    }
}
