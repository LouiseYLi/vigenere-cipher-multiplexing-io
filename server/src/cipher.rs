use crate::globals::*;
use crate::math::*;

pub fn encrypt(msg: &str, shift_value: i32) -> String {
    let mut encrypted_msg = String::new();
    for c in msg.chars() {
        let new_c = if c.is_ascii_uppercase() {
            shift(TOTAL_LETTERS, c, shift_value, UP_MIN_INDEX) as u8 as char
        } else if c.is_ascii_lowercase() {
            shift(TOTAL_LETTERS, c, shift_value, LOW_MIN_INDEX) as u8 as char
        } else {
            c
        };
        encrypted_msg.push(new_c);
    }
    encrypted_msg
}
