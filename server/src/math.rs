pub fn shift(total_letters: i32, letter: char, shift_value: i32, min_index: i32) -> char {
    let curr_index = letter as i32 - min_index;
    let shifted = modulo26(total_letters, curr_index + shift_value);
    (min_index + shifted) as u8 as char
}

fn modulo26(total_letters: i32, x: i32) -> i32 {
    let r = x % total_letters;
    if r < 0 { r + total_letters } else { r }
}
