use sha2::{Digest, Sha256};

pub fn hash_password(password: String) -> String {
    let mut hasher = Sha256::new();
    hasher.update(password.as_bytes());
    let hashed_pass = hasher.finalize();
    hex::encode(hashed_pass)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hash_password_should_work() {
        let ret = hash_password("hunter42".to_string());
        assert_eq!(
            ret,
            "9decfdd44a1c0d753f4a766127fd92b0f649a4e345696edf876f62c0a428770e"
        );
    }
}
