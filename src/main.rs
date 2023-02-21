use sha2::{Digest, Sha256};


fn get_hash(filepath: &str) -> String {
    let bytes = std::fs::read(filepath).unwrap();
    let mut h = Sha256::new();
    h.update(&bytes[..]);
    hex::encode(h.finalize())
}

fn main() {
    let filepath = "testfile";
    get_hash(filepath);
}
