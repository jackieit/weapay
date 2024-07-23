use crate::alipay::prelude::*;
use crate::error::WeaError;
use crate::*;
use std::future::Future;

pub trait BillTrait {
    /// 申请交易账单
    /// 帐单下载地址30秒后失效
    fn trade_bill(&self, query: ReqBillQuery) -> impl Future<Output = Result<ResBill, WeaError>>;
}
impl BillTrait for Payment<AlipayConfig> {
    fn trade_bill(&self, query: ReqBillQuery) -> impl Future<Output = Result<ResBill, WeaError>> {
        async move {
            let query = serde_json::to_string(&query)?;
            let url = self.get_uri("alipay.data.dataservice.bill.downloadurl.query");
            self.do_request::<ResBill>(&url, "POST", &query).await
        }
    }
}
