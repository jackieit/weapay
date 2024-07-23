use serde::{Deserialize, Serialize};
//bill订单相关
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct BillResponse {
    //哈希类型
    pub hash_type: String,
    //账单类型
    pub hash_value: String,
    //下载地址
    pub download_url: String,
}
