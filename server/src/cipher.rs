use crate::globals::*;
use crate::math::*;

pub fn encrypt(msg: &str, _key: &str) -> String {
    let mut encrypted_msg = String::new();
    let key_len = _key.len();
    let mut key_it = 0;
    for c in msg.chars() {
        let new_c = if c.is_ascii_uppercase() {
            let c_index = get_char_index(c, UP_MIN_INDEX as u8);
            let key_char = _key.as_bytes()[key_it % key_len] as char;
            let key_index = get_char_index(key_char, UP_MIN_INDEX as u8);
            key_it += 1;
            (modulo26(TOTAL_LETTERS, c_index + key_index) + UP_MIN_INDEX) as u8 as char
        } else if c.is_ascii_lowercase() {
            let c_index = get_char_index(c, LOW_MIN_INDEX as u8);
            let key_char = _key.as_bytes()[key_it % key_len] as char;
            let key_index = get_char_index(key_char, UP_MIN_INDEX as u8);
            key_it += 1;
            (modulo26(TOTAL_LETTERS, c_index + key_index) + LOW_MIN_INDEX) as u8 as char
        } else {
            c
        };
        encrypted_msg.push(new_c);
    }
    println!("Encrypted msg: {}\n", encrypted_msg);
    encrypted_msg
}

pub fn get_char_index(c: char, index: u8) -> i32 {
    modulo26(TOTAL_LETTERS, (c.to_string().as_bytes()[0] - index) as i32)
}

// pub fn get_shift(c: char, key: &str) -> i32 {}
