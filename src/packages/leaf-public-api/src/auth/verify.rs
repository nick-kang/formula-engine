use sha2::{Digest, Sha512};

pub fn sha512(password: &str, hash: &str) -> bool {
    // hash password
    let mut hasher = Sha512::new();
    hasher.update(password);
    let result = hasher.finalize();

    match hex::decode(hash) {
        Err(_) => false,
        Ok(hash_bytes) => {
            if hash_bytes.len() != result.len() {
                return false;
            }

            // verification logic taken from bcrypt
            // https://github.com/Keats/rust-bcrypt/blob/58da52dd8f4964c8bd403a843eda19d46b3b30e5/src/lib.rs#L193
            let mut diff = 0;

            for (a, b) in result.into_iter().zip(hash_bytes) {
                diff |= a ^ b;
            }

            diff == 0
        }
    }
}
