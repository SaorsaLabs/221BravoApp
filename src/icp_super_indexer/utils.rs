
use crate::custom_types::{IDKey, LogEntry, ProcessedTX, SmallTX};
use crate::state_management::{RUNTIME_STATE, STABLE_STATE};
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

pub fn processedtx_to_smalltx(input_vec: &Vec<ProcessedTX>) -> Vec<SmallTX> {
    let mut stx:Vec<SmallTX> = Vec::new();
    for tx in input_vec {
        // get refs for from/ to accounts
        let fm_to = STABLE_STATE.with(|s| {
            let fm;
            let to;
            if tx.from_account.as_str() != "ICP_LEDGER" {
                fm = s.borrow_mut().as_mut().unwrap().directory_data.add_id(tx.from_account.clone());
            } else { fm = None };
            if tx.to_account.as_str() != "ICP_LEDGER" {
                to = s.borrow_mut().as_mut().unwrap().directory_data.add_id(tx.to_account.clone());
            } else { to = None }
            return (fm, to);
        });

        let tx_type: u8;
        match tx.tx_type.as_str() {
            "Transaction" => {tx_type = 0},
            "Mint" => {tx_type = 1},
            "Burn" => {tx_type = 2},
            _ => {tx_type = 99},
        }
        
        stx.push(SmallTX{
                    block: tx.block as u32,
                    time: tx.tx_time as u64,
                    from: fm_to.0, 
                    to: fm_to.1,
                    tx_type,
                    value: tx.tx_value as u64,
                    //pub hash: , // Option<IDKey> hash is 64 in len     
                    });
    } // for

    return stx;
}

// should only be called if processedtx to small tx has been called as this creates the directory to 
// convert u32 back to strings. 
pub fn smalltx_to_processedtx(input_vec: &Vec<SmallTX>) -> Vec<ProcessedTX> {
    let mut ptx:Vec<ProcessedTX> = Vec::new();

    for tx in input_vec {
        // get refs for from/ to accounts
        let fm_to = STABLE_STATE.with(|s| {
            let fm;
            let to;
            if tx.from != None {
                let fmac: u32 = tx.from.unwrap();
                fm = s.borrow_mut().as_mut().unwrap().directory_data.get_id(&fmac).unwrap();
            } else { fm = "ICP_LEDGER".to_string() };
            if tx.to != None {
                let toac = tx.to.unwrap();
                to = s.borrow_mut().as_mut().unwrap().directory_data.get_id(&toac).unwrap();
            } else { to = "ICP_LEDGER".to_string() }
            return (fm, to);
        });

        let mut tx_type: String = String::new();
        match tx.tx_type {
            0 => {tx_type = "Transaction".to_string()},
            1 => {tx_type = "Mint".to_string()},
            2 => {tx_type = "Burn".to_string()},
            _ => { log("Error - can't convert tx-type from smallTx to ProcessedTx. (smalltx_to_processedtx)")},
        }
        
        ptx.push(ProcessedTX { 
            block: tx.block as u128, 
            hash: "No-hash".to_string(), 
            tx_type, 
            from_account: fm_to.0, 
            to_account: fm_to.1, 
            tx_value: tx.value as u128, 
            tx_time: tx.time, 
        });
    } // for
    return ptx;
}

pub fn remove_none_ptx_values(vec: Vec<Option<ProcessedTX>>) -> Vec<ProcessedTX> {
    let mut ret: Vec<ProcessedTX> = Vec::new();
    for tx in vec {
        match tx {
            Some(v) => { ret.push(v) },
            None => {}
        }
    }
    return  ret;
}

// used for setting specific timers. 
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

