use crate::globals::*;
use crate::math::*;

pub fn encrypt(msg: &str, _key: &str) -> String {
    let mut encrypted_msg = String::new();
    let key_len: usize = _key.len();
    let mut key_it: usize = 0;
    for c in msg.chars() {
        let new_c = if c.is_ascii_uppercase() {
            let c_index = get_char_index(c, UP_MIN_INDEX as u8);
            let key_index = get_char_index(_key.as_bytes()[key_it] as char, UP_MIN_INDEX as u8);
            // println!(
            //     "Char: {}, Index: {}... Key Char: {}, Key Index: {}",
            //     c,
            //     c_index,
            //     _key.as_bytes()[key_it] as char,
            //     key_index
            // );
            // c.to_ascii_uppercase()
            (modulo26(TOTAL_LETTERS, c_index + key_index) + UP_MIN_INDEX) as u8 as char
        } else if c.is_ascii_lowercase() {
            let c_index = get_char_index(c, LOW_MIN_INDEX as u8);
            let key_index = get_char_index(_key.as_bytes()[key_it] as char, UP_MIN_INDEX as u8);
            // println!(
            //     "Char: {}, Index: {}... Key Char: {}, Key Index: {}",
            //     c,
            //     c_index,
            //     _key.as_bytes()[key_it] as char,
            //     key_index
            // );
            // c.to_ascii_lowercase()
            (modulo26(TOTAL_LETTERS, c_index + key_index) + LOW_MIN_INDEX) as u8 as char
        } else {
            c
        };
        encrypted_msg.push(new_c);
        if key_it < (key_len - 1) {
            key_it += 1;
        } else {
            key_it = 0;
        }
    }
    println!("Encrypted msg: {}", encrypted_msg);
    encrypted_msg
}

pub fn get_char_index(c: char, index: u8) -> i32 {
    modulo26(TOTAL_LETTERS, (c.to_string().as_bytes()[0] - index) as i32)
}

// pub fn get_shift(c: char, key: &str) -> i32 {}
