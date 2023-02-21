use std::{io::BufReader, fs::File};

use sha2::{Digest, Sha256};


fn get_hash(filepath: &str) -> String {
    let mut f = BufReader::new(File::open(filepath).unwrap());
    let mut h = Sha256::new();
    std::io::copy(&mut f, &mut h).unwrap();
    hex::encode(h.finalize())
}

fn main() {
    let filepath = "testfile";
    get_hash(filepath);
}
