use crate::wechat::prelude::*;
use crate::BoxFuture;
use crate::*;
pub trait RefundTrait {
    /// 申请退款
    fn refund(&self, data: ReqRefundOrder) -> BoxFuture<RefundResponse>;
    /// 查询退款
    fn query_refund(&self, out_refund_no: &str) -> BoxFuture<RefundResponse>;
}
impl RefundTrait for Payment<WechatConfig> {
    fn refund(&self, data: ReqRefundOrder) -> BoxFuture<RefundResponse> {
        let mut new_data: ReqRefundOrder;
        if self.is_sp() {
            new_data = ReqRefundOrder {
                sub_mchid: Some(self.config.mchid.clone()),
                ..data
            };
        } else {
            new_data = data;
        }
        if new_data.notify_url.is_none() {
            new_data.notify_url = Some(self.config.notify_url.clone());
        }
        Box::pin(async move {
            let refund_body = serde_json::to_string(&new_data)?;
            let url = self.get_uri("/v3/refund/domestic/refunds", false, false);
            self.do_request::<RefundResponse>(&url, "POST", &refund_body)
                .await
        })
    }
    fn query_refund(&self, out_refund_no: &str) -> BoxFuture<RefundResponse> {
        let url = format!("/v3/refund/domestic/refunds/{}", out_refund_no);
        let url = self.get_uri(&url, true, false);
        Box::pin(async move { self.do_request::<RefundResponse>(&url, "GET", "").await })
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    //use crate::wechat::prelude::*;
    #[tokio::test]
    async fn test_refund() {
        let config = crate::tests::get_config().0;
        let payment = Payment::new(config);
        let data = ReqRefundOrder {
            out_trade_no: Some("T20240407003".to_string()),
            out_refund_no: "RT20240407003".to_string(),
            reason: Some("商品已售完".to_string()),
            amount: ReqRefundAmountInfo {
                refund: 1,
                total: 1,
                currency: "CNY".to_string(),
                ..Default::default()
            },
            goods_detail: Some(vec![ReqRefundGoodsDetail {
                merchant_goods_id: "11".to_string(),
                goods_name: Some("旅行卡门票服务".to_string()),
                unit_price: 1,
                refund_amount: 1,
                refund_quantity: 1,
                ..Default::default()
            }]),
            ..Default::default()
        };
        let result = payment.refund(data).await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("{}", error);
        } else {
            assert!(result.is_ok());
            let result = result.unwrap();
            //assert_eq!(result.out_refund_no, out_refund_no);
            println!("{:?}", result.out_refund_no);
        }
    }
    #[tokio::test]
    async fn test_query_refund() {
        let config = crate::tests::get_config().0;
        let payment = Payment::new(config);
        let out_refund_no = "RT20240407003";
        let result = payment.query_refund(out_refund_no).await;
        if result.is_err() {
            let error = result.err().unwrap();
            println!("1112{:?}", error);
        } else {
            let result = result.unwrap();
            //assert_eq!(result.out_refund_no, out_refund_no);
            println!("111{:?}", result);
        }
    }
}
