use crate::types::{ KEY_LENGTH, IDKey };

// NOTHING YET
pub fn nearest_day_start(time_nano: u64) -> u64 {
    const NANO_PER_DAY: u64 = 86_400_000_000_000;
    let remainder = time_nano % NANO_PER_DAY;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn string_to_key(input: String) -> IDKey {
    let mut buffer: [u8; KEY_LENGTH] = [0_u8; KEY_LENGTH];
    let bytes: Vec<u8> = input.into_bytes();
    for (i, &byte) in bytes.iter().enumerate().take(KEY_LENGTH) {
        buffer[i] = byte;
    }
    return buffer;
}
