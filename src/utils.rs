use crate::error::{PayError, WeaError};
use openssl::{
    base64::{decode_block, encode_block},
    hash::MessageDigest,
    pkey::PKey,
    rsa::Rsa,
    sign::Signer,
};
use rand::{distributions::Alphanumeric, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

/// 生成签名 data: vec!['GET', 'https://xxx', '1395712654', 'nonce_str', 'body']
/// private_key: 商户私钥,支付宝提供的私钥可能没有 begin-- end 手动加上。注意两端不要有空格
pub fn generate_signature(data: Vec<&str>, private_key: &str) -> Result<String, WeaError> {
    let data = data.join("\n");
    let data = data + "\n";
    let private_key_content = std::fs::read_to_string(private_key)?;
    //private_key_content = prepair_cert(private_key_content, true);
    //print!("RSA PRIVATE, {}", private_key_content);
    let pkey = if private_key_content.contains("-----BEGIN") {
        let rsa = Rsa::private_key_from_pem(&private_key_content.as_bytes())?;
        PKey::from_rsa(rsa)?
    } else {
        let private_u8 = decode_block(&private_key_content)?;
        let rsa = Rsa::private_key_from_der(private_u8.as_slice())?;
        PKey::from_rsa(rsa)?
    };

    let mut signer = Signer::new(MessageDigest::sha256(), &pkey)?;
    //signer.set_rsa_padding(Padding::PKCS1).unwrap();
    signer.update(data.as_bytes())?;
    let sign = signer.sign_to_vec()?;

    Ok(encode_block(&sign))
}
// generate a random string
pub fn generate_random_string(len: usize) -> String {
    let rng = rand::thread_rng();
    rng.sample_iter(&Alphanumeric)
        .take(len)
        .map(char::from)
        .collect()
}
// get current unix timestamp
pub fn get_timestamp() -> Result<u64, WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp = since_the_epoch.as_secs();
    Ok(timestamp)
}
// 获取当前 Unix 时间戳的毫秒数
pub fn get_timestamp_millis() -> Result<u128, WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp_millis = since_the_epoch.as_millis();
    Ok(timestamp_millis)
}
// short for payerror
pub fn e(message: &str) -> WeaError {
    WeaError::PayError(PayError::new(message))
}
// get cert serial number
pub fn get_cert_serial(cert: &str) -> Result<String, WeaError> {
    let cert = std::fs::read(cert)?;
    let cert = openssl::x509::X509::stack_from_pem(&cert)?;
    let cert = cert[0].serial_number().to_bn()?.to_hex_str()?.to_string();
    // convert OpensslString to String
    //let cert = cert.to_string();
    //println!("get_cert_serial ==={}",cert);
    Ok(cert)
}
