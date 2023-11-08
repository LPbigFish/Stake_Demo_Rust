use rand::RngCore;
use sha3::{Shake128, digest::{Update, ExtendableOutput, XofReader}, Keccak256};
use sha3::Digest;

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

pub fn new_keccak_from_bytes(input: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak256::default();
    Digest::update(&mut hasher, input);
    let mut hash = [0u8; 32];
    hash.copy_from_slice(hasher.finalize().as_slice());
    hash
}

pub fn join_arrays(a: &[u8], b: &[u8]) -> Vec<u8> {
    a.iter().cloned().chain(b.iter().cloned()).collect::<Vec<u8>>()
}