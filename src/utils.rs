use crate::error::WeaError;
use openssl::{
    base64::{decode_block, encode_block},
    hash::{hash, MessageDigest},
    //OpenSSLString,
    nid::Nid,
    pkey::PKey,
    rand::rand_bytes,
    rsa::Rsa,
    sign::Signer,
    x509::X509,
};
use std::time::{SystemTime, UNIX_EPOCH};

/// 生成签名 data: vec!['GET', 'https://xxx', '1395712654', 'nonce_str', 'body']
/// private_key: 商户私钥,支付宝提供的私钥可能没有 begin-- end 手动加上。注意两端不要有空格
pub(crate) fn generate_signature(data: Vec<&str>, private_key: &str) -> Result<String, WeaError> {
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
pub(crate) fn generate_random_string(len: usize) -> String {
    let num_bytes = (len + 1) / 2;
    let mut bytes = vec![0u8; num_bytes];
    rand_bytes(&mut bytes).unwrap();
    let random_string = bytes
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect::<String>();
    let random_string = if len % 2 == 0 {
        random_string
    } else {
        random_string[..len].to_string()
    };
    random_string
}
// get current unix timestamp
pub(crate) fn get_timestamp() -> Result<u64, WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp = since_the_epoch.as_secs();
    Ok(timestamp)
}
// 获取当前 Unix 时间戳的毫秒数
pub(crate) fn get_timestamp_millis() -> Result<u128, WeaError> {
    let start = SystemTime::now();
    let since_the_epoch = start.duration_since(UNIX_EPOCH)?;
    let timestamp_millis = since_the_epoch.as_millis();
    Ok(timestamp_millis)
}
// short for payerror
pub(crate) fn e(message: &str) -> WeaError {
    WeaError::new("".to_string(), message.to_string())
}
// get cert serial number usedby wechat pay
pub(crate) fn get_cert_serial(cert: &str) -> Result<String, WeaError> {
    let cert = std::fs::read(cert)?;
    let cert = X509::stack_from_pem(&cert)?;
    let cert = cert[0].serial_number().to_bn()?.to_hex_str()?.to_string();
    Ok(cert)
}
// get cert sn by cert file by alipay
pub(crate) fn get_cert_sn(cert: &str) -> Result<String, WeaError> {
    let cert = std::fs::read_to_string(cert)?;

    get_cert_sn_by_content(cert.as_ref())
}
// get cert cert sn content used by alipay
pub(crate) fn get_cert_sn_by_content(cert_content: &[u8]) -> Result<String, WeaError> {
    //let cert_content = std::fs::read(cert_content)?;
    let cert = X509::from_pem(cert_content).unwrap();
    /* */
    let mut sumary = cert
        //.clone()
        .issuer_name()
        .entries()
        .map(|item| {
            item.object().nid().short_name().unwrap().to_string()
                + "="
                + &item.data().as_utf8().unwrap().to_string()
        })
        .collect::<Vec<String>>();
    sumary.reverse();
    let sumary = sumary.join(",");
    //println!("sumary==={}\n", sumary);
    let serial_number = cert.serial_number().to_bn()?.to_dec_str()?;
    let sumary = sumary + &serial_number;

    let md5_digest = hash(MessageDigest::md5(), sumary.as_bytes())?;

    // Convert the hash to a hexadecimal string
    let cert_sn: &String = &md5_digest
        .iter()
        .map(|byte| format!("{:02x}", byte))
        .collect();
    //.to_string();
    let mut cert_sn = cert_sn.to_string();

    while cert_sn.len() < 32 {
        cert_sn.insert(0, '0');
    }

    Ok(cert_sn)
}

/// get alipay root cert sn
pub(crate) fn get_root_cert_sn(cert_content: &str) -> Result<String, WeaError> {
    let cert_content = std::fs::read_to_string(cert_content)?;
    let root_cert_sn = cert_content
        .split_inclusive("-----END CERTIFICATE-----")
        .filter(|cert| {
            let ssl = X509::from_pem(cert.as_ref()).unwrap();
            let algorithm = ssl.signature_algorithm().object().nid();
            algorithm == Nid::SHA256WITHRSAENCRYPTION || algorithm == Nid::SHA1WITHRSAENCRYPTION
        })
        .filter_map(|cert| get_cert_sn_by_content(cert.as_ref()).ok())
        .collect::<Vec<String>>()
        .join("_");
    Ok(root_cert_sn)
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_serial() {
        let cert_file = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\appPublicCert.crt";
        let sn = get_cert_sn(cert_file).unwrap();
        assert_eq!(sn, "8da0f25e83e0ce9829d8c864a4af361e");

        let root_cert = "E:\\work\\code\\lifebank\\backend\\weapay\\certs\\alipayRootCert.crt";
        let root_sn = get_root_cert_sn(root_cert).unwrap();
        assert_eq!(
            root_sn,
            "687b59193f3f462dd5336e5abf83c5d8_02941eef3187dddf3d3b83462e1dfcf6".to_string()
        );
    }
    //test generate random string
    #[test]
    fn test_generate_random_string() {
        let random_string = generate_random_string(32);
        println!("random_string==={}", random_string);
        assert_eq!(random_string.len(), 32);
    }
}
