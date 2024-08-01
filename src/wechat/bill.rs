use crate::utils::*;
use crate::wechat::prelude::*;
use crate::*;
pub trait BillTrait {
    /// 申请交易账单
    #[allow(dead_code)]
    fn trade_bill(
        &self,
        bill_date: String,
        bill_type: Option<String>,
        tar_type: Option<String>,
        with_mchid: bool,
    ) -> BoxFuture<BillResponse>;
    /// 申请资金账单
    #[allow(dead_code)]
    fn fund_bill(
        &self,
        bill_date: String,
        account_type: Option<String>,
        tar_type: Option<String>,
    ) -> BoxFuture<BillResponse>;
    /// 下载帐单
    #[allow(dead_code)]
    //fn download(&self,download_url: &str) -> WeaResult<Bytes>;
    fn download(&self, download_url: &str) -> BoxFuture<reqwest::Response>;
}
impl BillTrait for Payment<WechatConfig> {
    fn trade_bill(
        &self,
        bill_date: String,
        bill_type: Option<String>,
        tar_type: Option<String>,
        with_mchid: bool,
    ) -> BoxFuture<BillResponse> {
        let mut url = format!(
            "/v3/bill/tradebill?bill_date={}&bill_type={}&tar_type={}",
            bill_date,
            bill_type.unwrap_or("ALL".to_string()),
            tar_type.unwrap_or("GZIP".to_string())
        );
        if self.is_sp() && with_mchid {
            url = format!("{}&sub_mchid={}", url, self.config.mchid.clone());
        }
        Box::pin(async move { self.do_request::<BillResponse>(&url, "GET", "").await })
    }
    fn fund_bill(
        &self,
        bill_date: String,
        account_type: Option<String>,
        tar_type: Option<String>,
    ) -> BoxFuture<BillResponse> {
        let url = format!(
            "/v3/bill/fundflowbill?bill_date={}&account_type={}&tar_type={}",
            bill_date,
            account_type.unwrap_or("BASIC".to_string()),
            tar_type.unwrap_or("GZIP".to_string())
        );
        Box::pin(async move { self.do_request::<BillResponse>(&url, "GET", "").await })
    }
    fn download(&self, download_url: &str) -> BoxFuture<reqwest::Response> {
        let download_url = download_url.replace("https://api.mch.weixin.qq.com", "");
        Box::pin(async move {
            let req_builder = self.build_request_builder(&download_url, "GET", "")?;
            let resp = req_builder.send().await?;
            let status_code = resp.status();
            if status_code.is_success() {
                return Ok(resp);
            } else {
                let res = resp.text().await?;
                return Err(e(&res));
            }
        })
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    #[tokio::test]
    async fn test_trade_bill() {
        let config = crate::tests::get_config().0;
        let payment = Payment::new(config);
        let result = payment
            .trade_bill(
                "2024-07-01".to_string(),
                Some("ALL".to_string()),
                None,
                false,
            )
            .await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();

            println!("{:?}", result);
        }
    }
    #[tokio::test]
    async fn test_fund_bill() {
        let config = crate::tests::get_config().0;
        let payment = Payment::new(config);
        let result = payment
            .fund_bill("2024-07-01".to_string(), Some("BASIC".to_string()), None)
            .await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            let result = result.unwrap();

            println!("{:?}", result);
        }
    }
    #[tokio::test]
    async fn test_download() {
        let config = crate::tests::get_config().0;
        let payment = Payment::new(config);
        let result = payment.download("https://api.mch.weixin.qq.com/v3/billdownload/file?token=8c-06HzWlqxxIXry2c090B2jXl9CHcdZkH2iL29MTtH3C80axxwwa0AgYHXlbdPV&tartype=gzip").await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            assert!(result.is_ok());
            let result = result.unwrap();
            let result = result.bytes().await.unwrap();
            let tmp_file = env!("CARGO_MANIFEST_DIR").to_string() + "/bill.gz";
            std::fs::write(tmp_file, result).unwrap();
        }
    }
}
