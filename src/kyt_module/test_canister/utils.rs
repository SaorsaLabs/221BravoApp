use std::collections::BTreeMap;

use crate::types::{ProcessedTX, IDKey, KeyMap, EntityData};

pub fn string_to_key(input: &String) -> IDKey {
    let mut buffer: [u8; 2] = [0_u8; 2];
    let bytes: Vec<u8> = input.to_owned().into_bytes();
    for (i, &byte) in bytes.iter().enumerate().take(2) {
        buffer[i] = byte;
    }
    return buffer;
}

// creates 1296 keys 00 to zz
pub fn create_all_keys() -> KeyMap {
    let input_chars = "0123456789abcdefghijklmnopqrstuvwxyz";
    let mut bytes_vec: Vec<(IDKey, usize)> = Vec::new();
    let mut count: usize = 0;
    for c1 in input_chars.chars() {
        for c2 in input_chars.chars() {
            let s: String = format!("{}{}", c1, c2);
            let key: [u8; 2] = string_to_key(&s);
            bytes_vec.push((key, count));
            count += 1;
        }
    }
    let km: KeyMap = KeyMap { map: bytes_vec};
    return km;
}

// creates 1296 BtreeMaps for keys 00 to zz
pub fn create_master_tree() -> Vec<BTreeMap<String, EntityData>> {
    let mut mt_vec: Vec<BTreeMap<String, EntityData>> = Vec::new();
    for i in 0..1296 {
        mt_vec.push(BTreeMap::new());
    }
    return mt_vec;
}

pub fn nearest_past_hour(time_nano: u64) -> u64 {
    const NANO_PER_HOUR: u64 = 3600_000_000_000;
    let remainder = time_nano % NANO_PER_HOUR;
    let nearest_hour = time_nano - remainder;
    return nearest_hour;
}

pub fn nearest_day_start(time_nano: u64) -> u64 {
    const NANO_PER_DAY: u64 = 86_400_000_000_000;
    let remainder = time_nano % NANO_PER_DAY;
    let nearest_day_start = time_nano - remainder;
    return nearest_day_start;
}

pub fn top_x_by_txvalue(
    mut transactions: Vec<ProcessedTX>,
    result_length: usize
) -> Vec<ProcessedTX> {
    // decending
    transactions.sort_by(|a, b| b.tx_value.cmp(&a.tx_value));
    if transactions.len() > result_length {
        transactions.truncate(result_length);
    }
    return transactions;
}

pub fn top_x_txcount(
    mut transactions: Vec<(String, u64)>,
    result_length: usize
) -> Vec<(String, u64)> {
    // decending
    transactions.sort_by(|a, b| b.1.cmp(&a.1));
    if transactions.len() > result_length {
        transactions.truncate(result_length);
    }
    return transactions;
}
