use sha1::{Digest, Sha1};

pub fn hash_object(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    hex::encode(hasher.finalize())
}
