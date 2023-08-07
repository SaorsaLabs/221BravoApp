use std::collections::HashSet;
use crate::types::ProcessedTX;

// Check if admin/authorised
pub fn validate_caller(principal_id: String, admins: Vec<String>) {
    let mut auth: bool = false;
    if admins.contains(&principal_id) {
        auth = true;
    }
    match auth {
        true => (),
        _ => ic_cdk::trap("Caller Not Authorised"),
    }
}

pub fn get_unique_string_values(vec: Vec<String>) -> Vec<String> {
    let mut unique_vec = Vec::new();
    let mut set = HashSet::new();
    for item in vec {
        if set.insert(item.clone()) {
            unique_vec.push(item);
        }
    }
    return unique_vec;
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
