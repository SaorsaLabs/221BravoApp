use std::collections::HashSet;
use crate::types::ProcessedTX;

pub fn get_unique_string_values(vec: Vec<String>) -> Vec<String> {
    if vec.len() == 0 {return Vec::new()};
    
    let mut working_array: Vec<String> = vec.to_owned();
    let mut keepers: Vec<String> = Vec::new();
    working_array.sort();
    keepers.push(working_array[0].to_owned()); // 1st is always a keeeper
    for i in 1..working_array.len() {
        if working_array[i] != working_array[i-1] {
            keepers.push(working_array[i].to_owned());
        }
    }
    return keepers;
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
