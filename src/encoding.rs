use sha2::{Digest, Sha256};

use crate::env_with_prefix;
use crate::utils::env::TT_ENV_PREFIX;

const ALPHABET: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";

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

pub fn hash_with_salt(salt: &str, data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(salt);
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}

pub fn hash_with_env_salt(data: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(env_with_prefix!("HASH_SALT"));
    hasher.update(data);
    let result = hasher.finalize();

    hex::encode(result)
}
