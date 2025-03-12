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
