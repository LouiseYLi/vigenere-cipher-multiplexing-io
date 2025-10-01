use crate::globals::*;

pub fn decrypt(msg: String, shift_value: i32) -> String {
    let mut decrypted_msg = String::new();
    for c in msg.chars() {
        let new_c = if c.is_ascii_uppercase() {
            shift(TOTAL_LETTERS, c, -shift_value, UP_MIN_INDEX) as u8 as char
        } else if c.is_ascii_lowercase() {
            shift(TOTAL_LETTERS, c, -shift_value, LOW_MIN_INDEX) as u8 as char
        } else {
            c
        };
        decrypted_msg.push(new_c);
    }

    decrypted_msg
}

pub fn shift(total_letters: i32, letter: char, shift_value: i32, min_index: i32) -> char {
    let curr_index = letter as i32 - min_index;
    let shifted = modulo26(total_letters, curr_index + shift_value);
    (min_index + shifted) as u8 as char
}

fn modulo26(total_letters: i32, x: i32) -> i32 {
    let r = x % total_letters;
    if r < 0 { r + total_letters } else { r }
}
