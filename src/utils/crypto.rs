use hmac::{Hmac, Mac};
use sha2::{Digest, Sha256};

type HmacSha256 = Hmac<Sha256>;

/// 计算 SHA256 哈希
pub fn sha256(data: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// 生成 HMAC 签名（返回十六进制字符串）
pub fn sign_data(data: &[u8], key: &[u8]) -> String {
    let mut mac = HmacSha256::new_from_slice(key).expect("HMAC can take key of any size");
    mac.update(data);
    let result = mac.finalize();
    hex::encode(result.into_bytes())
}

/// 验证 HMAC 签名
pub fn verify_signature(data: &[u8], signature: &str, key: &[u8]) -> bool {
    let expected = sign_data(data, key);
    expected == signature
}
