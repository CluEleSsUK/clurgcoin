use sha2::{Sha256, Digest};

pub fn sha256(payload: String) -> Vec<u8> {
    let mut hasher = Sha256::new();
    hasher.update(payload);
    return hasher.finalize().as_slice().to_vec();
}