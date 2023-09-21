mod state_management;
mod custom_types;
mod constants;
mod utils;

use ic_cdk_macros::*;
use state_management::{ state_init, state_pre_upgrade, state_post_upgrade, STABLE_STATE, RUNTIME_STATE };
use custom_types::{ MemoryData, LogEntry, SmallTX, GetMultipleTxFromStoreTimeArgs };
use utils::log;

// [][] ---------------- [][]
// [][] ---  Methods --- [][]
// [][] ---------------- [][]

#[update]
fn add_txs_to_store(tx_vec: Vec<SmallTX>) -> bool {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add txs
    let mut process_error: bool = false;
    for tx in tx_vec {
        let res = STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().tx_store.add_tx(tx)
        });
        if res == false {
            log("Error adding tx - aborted processing txs");
            process_error = true;
            break;
        }
    }
    if process_error == true { return false}
    else { return true }
}

#[query]
fn get_tx_from_store(block_number: u32) -> Option<SmallTX> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let res = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().tx_store.get_tx(block_number)
    });
    return res;
}

#[query]
fn get_multiple_tx_from_store(block_vec: Vec<u32>) -> Vec<Option<SmallTX>> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let res = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().tx_store.get_multiple_tx(block_vec)
    });
    return res;
}

#[query]
fn get_multiple_tx_from_store_time(args: GetMultipleTxFromStoreTimeArgs) -> Option<Vec<SmallTX>> {
    // catch empty input
    if args.blocks.len() == 0 {
        return None;
    }

    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let res = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().tx_store.get_multiple_tx(args.blocks)
    });
    let mut ret_values: Vec<SmallTX> = Vec::new();
    let mut hits: bool = false;
    // time filter 
    for tx in res {
        match tx {
            Some(v) => {
                if v.time >= args.start && v.time <= args.end {
                    ret_values.push(v);
                    hits = true;
                }
            },
            None => {},
        }
    }
    
    if hits == true {
        ret_values.truncate(args.max_return as usize);
        return Some(ret_values);
    } else {
        return None
    }
}

#[query]
fn get_total_transactions() -> u32 {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    // total tx stored
    STABLE_STATE.with(|state|{
        state.borrow().as_ref().unwrap().tx_store.get_count()
    })
}

#[update] // not public. Can only be called once. 
fn canister_init() -> bool {
    let caller = ic_cdk::caller().to_text();
    let is_locked = STABLE_STATE.with(|s| {
        s.borrow().as_ref().unwrap().canister_data.init_lock
    });
    if is_locked == true {
        return false;
    } else {
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.add_admin(caller.clone());
            s.borrow_mut().as_mut().unwrap().canister_data.add_authorised(caller);
            s.borrow_mut().as_mut().unwrap().canister_data.init_lock = true;
        });
        return true;
    }
}

// [][] --------------------------- [][]
// [][] --- Canister Management --- [][]
// [][] --------------------------- [][]
#[update]
fn add_authorised(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .remove_authorised(principal_id)
    })
}

#[update] // not visible to public
fn add_admin(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add admin
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .add_admin(principal_id)
    })
}

#[update] // not visible to public
fn remove_admin(principal_id: String) -> String {
   // check admin
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_admin(ic_cdk::caller().to_text());
    });
    // remove admin
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .remove_admin(principal_id)
    })
}

#[query] // not visible to public
fn get_all_admins() -> Vec<String> {
   // check admin
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_admin(ic_cdk::caller().to_text());
    });
    // get all admins
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_all_admins();
        return ret;
    })
}

#[update]
fn set_canister_name(name: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // set canister name 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_canister_name(name)
    })
}

#[update]
fn set_stats_public(are_stats_public: bool) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // set stats public
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_stats_public(are_stats_public)
    })
}

#[query]
fn get_all_authorised() -> Vec<String> {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // get all authorised
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_all_authorised();
        return ret;
    })
}

#[query]
fn get_canister_name() -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get canister name
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_canister_name();
        return ret;
    })
}

#[query]
fn are_stats_public() -> bool {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // check if stats are public
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.are_stats_public();
        return ret;
    })
}

#[query]
fn get_canister_logs() -> Vec<LogEntry> {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|{
        state.borrow().canister_logs.to_owned()
    })
}

#[query]
fn get_cycles_balance() -> u64 {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // get cycles balance
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });

    let wasm_page_size: u64 = 65536;
    let m: u64 =
        (ic_cdk::api::stable::stable64_size() as u64) * wasm_page_size +
        (core::arch::wasm32::memory_size(0) as u64) * wasm_page_size;
    let m2: u64 = (core::arch::wasm32::memory_size(0) as u64) * wasm_page_size;
    let ret = MemoryData {
        memory: m,
        heap_memory: m2,
    };
    return ret;
}


// [][] -------------------------------- [][]
// [][] --- Canister Setup/ Upgrades --- [][]
// [][] -------------------------------- [][]
#[init]
fn init() {
    state_init();
}

#[pre_upgrade]
fn pre_upgrade() {
    state_pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    state_post_upgrade();
}
