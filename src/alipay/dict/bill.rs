use serde::{Deserialize, Serialize};
//bill请求参数
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ReqBillQuery {
    //账单日期
    pub bill_date: String,
    //账单类型
    pub bill_type: String,
    //二级商户smid，这个参数只在bill_type是trade_zft_merchant时才能使用
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smid: Option<String>,
}
// bill响应body
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct ResBill {
    //账单下载地址链接
    pub bill_download_url: String,
    //账单文件结果说明
    pub bill_file_code: String,
}
