use rand::RngCore;
use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}};

pub fn random_hash() -> [u8; 16] {
    let mut rng = rand::thread_rng();
    let mut hash = [0u8; 16];
    rng.fill_bytes(&mut hash);
    hash
}

pub fn new_hash_from_bytes(input: &[u8]) -> [u8; 16] {
    let mut hasher = Shake128::default();
    hasher.update(input);
    let mut hash = [0u8; 16];
    let mut reader = hasher.finalize_xof();
    reader.read(&mut hash);
    hash
}

pub fn join_arrays(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().cloned().chain(b.iter().cloned()).collect::<Vec<u8>>()
}