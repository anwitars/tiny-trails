use sha2::{Digest, Sha256};

use crate::prefixed_env;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

/// Basic Base62 encoding function.
pub fn encode_base62(mut number: u64) -> String {
    let mut result = String::new();
    let base = ALPHABET.len() as u64;

    while number > 0 {
        let remainder = number % base;
        number /= base;
        result.push(ALPHABET.chars().nth(remainder as usize).unwrap());
    }

    result.chars().rev().collect()
}

/// Hashes the given data with the given salt using Sha256.
pub fn hash_with_salt(salt: &str, data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}

/// Hashes the given data with the environment salt using Sha256.
pub fn hash_with_env_salt(data: &str) -> String {
    hash_with_salt(prefixed_env!("HASH_SALT"), data)
}
