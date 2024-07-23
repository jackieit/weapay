use serde::{Deserialize, Serialize};

//证书下载返回数据
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RespCert {
    //证书列表
    pub data: Vec<RespCertItem>,
}

//证书下载返回数据数组项目
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RespCertItem {
    //证书序列号
    pub serial_no: String,
    //证书启用时间
    pub effective_time: Option<String>,
    //证书弃用时间
    pub expire_time: Option<String>,
    //证书信息
    pub encrypt_certificate: CertData,
}
//证书信息
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct CertData {
    //加密算法
    pub algorithm: String,
    //加密内容
    pub nonce: String,
    //证书内容
    pub associated_data: String,
    //证书内容
    pub ciphertext: String,
}
