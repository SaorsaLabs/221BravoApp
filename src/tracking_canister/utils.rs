use crate::custom_types::{IDKey, LogEntry};
use crate::state_management::RUNTIME_STATE;
use crate::constants::MAX_LOGS;

// MAX 135 Bytes input length!! 
pub fn string_to_idkey(input: &String) -> Option<IDKey> {
    if input.len() > 134 {return None}
    let s = format!("{}:",input); // show end of string with :
    let bytes: Vec<u8> = s.to_owned().into_bytes();
    return Some(IDKey(bytes));
}

pub fn idkey_to_string(input: &IDKey) -> Option<String> {
    if let Some(pos) = input.0.iter().position(|&a| a == b':') {
        let id_slice = &input.0[..pos];
        let res_string = std::str::from_utf8(id_slice).map(|s| s.to_string());
        match res_string {
            Ok(output) => {
                return Some(output);
            }, 
            Err(error) => {
                return None;
            }
        }
    } else {
        return None;
    }
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

pub fn log(text: impl AsRef<str>) {
    RUNTIME_STATE.with(|state| {
        let max_logs = MAX_LOGS;
        let logs = &mut *state.borrow_mut();
        let nano_time = ic_cdk::api::time();
        let log_entry: LogEntry = LogEntry {
            timestamp: nano_time.to_string(),
            text: text.as_ref().to_string(),
        };
        logs.canister_logs.push(log_entry);
        if logs.canister_logs.len() > max_logs {
            logs.canister_logs.remove(0);
        }
    });
}

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

