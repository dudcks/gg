use sha2::{Sha256, Digest};

pub fn encrypt_pwd(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let result = hasher.finalize();
    
    format!("{:x}", result)
}
