mod types;
mod utils;
mod account_identifier;

use ic_cdk_macros::*;
use candid::{ CandidType, Principal };
use serde::Deserialize;
use std::cell::RefCell;
use std::collections::BTreeMap;
use types::{ UserData, UserRank, MemoryData, LogEntry, CanisterSettings };
use utils::{
    validate_caller,
    get_subaccount_from_principal,
    get_multiple_subaccounts_from_principal,
};
use ic_cdk_timers::TimerId;
use std::time::Duration;

//[][] ---- State Manamgement ---- [][]
thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    static LOGS_STATE: RefCell<LogsState> = RefCell::default();
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}

//[][] --- Main Data Structs --- [][]
struct RuntimeState {
    pub data: Data,
}
impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            data: Data::default(),
        }
    }
}

#[derive(CandidType, Deserialize, Default, Clone)]
struct Data {
    authorised: Vec<String>,
    users: BTreeMap<String, UserData>,
    genesis_holders: Vec<(u32, String)>,
    public_accounts: BTreeMap<String, String>, // account, name
    genesis_updating: bool,
    canister_settings: CanisterSettings, 
}

impl Data {
    fn check_authorised(&self, principal_id: String) {
        let auth_vec: &Vec<String> = &self.authorised;
        let mut auth: bool = false;
        if auth_vec.contains(&principal_id) {
            auth = true;
        }
        match auth {
            true => (),
            _ => ic_cdk::trap("Caller Not Authorised"),
        }
    }

    fn add_authorised(&mut self, principal_id: String) -> String {
        let auth_vec: &mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id) {
            let rtn: String = String::from("Principal is already authorised");
            return rtn;
        } else {
            auth_vec.push(principal_id);
        }
        let rtn: String = String::from("Admin Added");
        return rtn;
    }

    fn remove_authorised(&mut self, principal_id: String) -> String {
        let auth_vec: &mut Vec<String> = &mut self.authorised;
        if auth_vec.contains(&principal_id) {
            auth_vec.retain(|x: &String| x != &principal_id);
        } else {
            let rtn: String = String::from("Can't remove - Principal isn't in the list of admins");
            return rtn;
        }
        let rtn: String = String::from("Admins Principal Removed");
        return rtn;
    }

    fn get_all_authorised(&self) -> Vec<String> {
        let auth_vec: &Vec<String> = &self.authorised;
        return auth_vec.to_owned();
    }

    fn set_canister_name(&mut self, name: String) -> String {
        self.canister_settings.canister_name = name;
        return "Canister name set".to_string();
    }

    fn get_canister_name(&self) -> String {
        let name = &self.canister_settings.canister_name;
        return name.to_owned();
    }

}


struct LogsState {
    pub data: LoggingData,
}
impl Default for LogsState {
    fn default() -> Self {
        LogsState {
            data: LoggingData::default(),
        }
    }
}
#[derive(CandidType, Deserialize, Default, Clone)]
struct LoggingData {
    canister_logs: Vec<LogEntry>,
}

#[init]
fn init() {
    // init main data state
    let mut data = Data::default();
    data.authorised.push(
        "DEVELOPER_PRINCIPAL_HERE".to_string()
    ); // Saorsa Dev
    data.authorised.push(
        "FRONTEND_PRINCIPAL_ID_HERE".to_string()
    ); // frontend
    data.genesis_updating = false;
    let runtime_state = RuntimeState { data };
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    // init logging data state
    let log_data = LoggingData::default();
    let logs_state = LogsState { data: log_data };
    LOGS_STATE.with(|state| {
        *state.borrow_mut() = logs_state;
    });
    log("Canister Initialised");
}

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state| ic_cdk::storage::stable_save((&state.borrow().data,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (data,): (Data,) = ic_cdk::storage::stable_restore().unwrap();
    let runtime_state = RuntimeState { data };
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    //start_genesis_holder_timer(300);
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().data.genesis_updating = true;
    });
    log("Canister upgraded");
}

// [][] --------------------------------- [][]
// [][] ---- Query/ Update Functions ---- [][]
// [][] --------------------------------- [][]

#[query]
fn get_user_data(user_account: String) -> Option<UserData> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| get_user_data_impl(user_account, &state.borrow()))
}
fn get_user_data_impl(user_account: String, runtime_state: &RuntimeState) -> Option<UserData> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let mut ud: UserData = UserData::default();
    let mut result: bool = false;
    match runtime_state.data.users.get(&user_account) {
        Some(value) => {
            ud = value.to_owned();
            result = true;
        }
        None => {
            result = false;
        }
    }

    if result == true {
        Some(ud)
    } else {
        None
    }
}

// Get user named accounts
#[query]
fn get_user_named_accounts(owner_account: String, query_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| get_user_named_accounts_impl(owner_account, query_vec, &state.borrow()))
}
fn get_user_named_accounts_impl(owner_account: String, query_vec: Vec<String>, runtime_state: &RuntimeState) -> Option<Vec<(String, String)>> {
   let mut ret: Vec<(String, String)> = Vec::new();
   let mut hits: bool = false;
   match runtime_state.data.users.get(&owner_account) {
        Some(value) => {
            for id in query_vec {
                match value.user_saved_accounts.get(&id){
                    Some(inner_value) => {
                                ret.push((id.clone(), inner_value.clone()));
                                hits = true;
                    }
                    None => {}
                }
           }
        },
        None => { return None}, 
   }
   if hits == true {return Some(ret);} else { return None };
}

// Get ALL user named accounts
#[query]
fn get_all_user_named_accounts(owner_account: String) -> Option<Vec<(String, String)>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| get_all_user_named_accounts_impl(owner_account, &state.borrow()))
}
fn get_all_user_named_accounts_impl(owner_account: String, runtime_state: &RuntimeState) -> Option<Vec<(String, String)>> {
   let mut ret: Vec<(String, String)> = Vec::new();
   let mut hits: bool = false;
   match runtime_state.data.users.get(&owner_account) {
        Some(usr_data) => {
            hits = true;
            for (key, value) in usr_data.user_saved_accounts.iter() {
                ret.push((key.to_owned(), value.to_owned()));
            }
        },
        None => { return None}, 
   }
   if hits == true {return Some(ret);} else { return None };
}


// add user named account
#[update]
fn add_user_named_accounts(owner_account: String, save_account: String, save_name:String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| add_user_named_accounts_impl(owner_account, save_account, save_name, &mut state.borrow_mut()))
}
fn add_user_named_accounts_impl(owner_account: String, save_account: String, save_name: String, runtime_state: &mut RuntimeState) -> String {
    
    let user_ac = runtime_state.data.users.get_mut(&owner_account);
    match user_ac {
        Some(value) => {
            value.user_saved_accounts.insert(save_account, save_name);
            return "Address book updated with new entry".to_string();
        }
        None => {
            return "Unknown User Account".to_string();
        }
    }
}

// delete user named account
#[update]
fn remove_user_named_account(owner_account: String, save_account: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| remove_user_named_accounts_impl(owner_account, save_account, &mut state.borrow_mut()))
}
fn remove_user_named_accounts_impl(owner_account: String, save_account: String, runtime_state: &mut RuntimeState) -> String {
    match runtime_state.data.users.get_mut(&owner_account) {
        Some(value) => {
           value.user_saved_accounts.remove(&save_account);
           return "Account removed from directory".to_string();
        }
        None => {
            return "Unknown User Account".to_string();
        }
    }
}

// get public named accounts
#[query]
fn get_public_named_accounts(input_vec: Vec<String>) -> Option<Vec<(String, String)>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| get_public_named_accounts_impl(input_vec, &state.borrow()))
}
fn get_public_named_accounts_impl(input_vec: Vec<String>, runtime_state: &RuntimeState) -> Option<Vec<(String, String)>> {
    let mut ret:Vec<(String, String)> = Vec::new();
    let mut hits: bool = false;
    for id in input_vec {
        let res = runtime_state.data.public_accounts.get(&id);
        match res {
            Some(v) => {
                ret.push((id.clone(), v.clone()));
                hits = true;
            }, 
            None => {}
        }
    }
    if hits == true { return Some(ret); } else { return None };
}

// add public named account
#[update]
fn add_public_named_accounts(save_account: String, save_name:String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| add_public_named_accounts_impl(save_account, save_name, &mut state.borrow_mut()))
}
fn add_public_named_accounts_impl(save_account: String, save_name: String, runtime_state: &mut RuntimeState) -> String {
    runtime_state.data.public_accounts.insert(save_account, save_name);
    return "ID added to public account list".to_string();
}

// delete public named account
#[update]
fn remove_public_named_account(save_account: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| remove_public_named_accounts_impl(save_account, &mut state.borrow_mut()))
}
fn remove_public_named_accounts_impl(save_account: String, runtime_state: &mut RuntimeState) -> String {
    let del: Option<String> = runtime_state.data.public_accounts.remove(&save_account);
    match del {
        Some(v) => {  return "ID Removed from public account list".to_string(); },
        None => { return "ID isnt in the public list - cannot remove".to_string(); }
    }
}

#[update]
fn add_new_user(user_account: String) -> bool {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| add_new_user_impl(user_account, &mut state.borrow_mut()))
}
fn add_new_user_impl(user_account: String, runtime_state: &mut RuntimeState) -> bool {
    let v = runtime_state.data.authorised.clone();
    validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
    let mut ud: UserData;
    match runtime_state.data.users.get(&user_account) {
        Some(value) => {
            return false;
        }
        None => {
            ud = UserData {
                user_account: String::from(&user_account),
                user_name: String::from("Unknown User"),
                user_tokens: 0,
                user_rank: UserRank::Padawan,
                user_saved_accounts: BTreeMap::new()
            };
            runtime_state.data.users.insert(user_account.clone(), ud);
            return true;
        }
    }
}

#[update]
fn update_username(user_account: String, user_name: String) -> bool {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|
        update_username_impl(user_account, user_name, &mut state.borrow_mut())
    )
}
fn update_username_impl(
    user_account: String,
    user_name: String,
    runtime_state: &mut RuntimeState
) -> bool {
    let v = runtime_state.data.authorised.clone();
    validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
    let mut ud: UserData;
    match runtime_state.data.users.get(&user_account) {
        Some(value) => {
            ud = value.to_owned();
            ud.user_name = user_name;
            runtime_state.data.users.insert(user_account, ud);
            return true;
        }
        None => {
            return false;
        }
    }
}

#[update]
fn update_user_tokens(user_account: String, user_tokens: u32) -> bool {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|
        update_user_tokens_impl(user_account, user_tokens, &mut state.borrow_mut())
    )
}
fn update_user_tokens_impl(
    user_account: String,
    user_tokens: u32,
    runtime_state: &mut RuntimeState
) -> bool {
    let v = runtime_state.data.authorised.clone();
    validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
    let mut ud: UserData;
    match runtime_state.data.users.get(&user_account) {
        Some(value) => {
            ud = value.to_owned();
            ud.user_tokens = user_tokens;
            runtime_state.data.users.insert(user_account, ud);
            return true;
        }
        None => {
            return false;
        }
    }
}

#[query]
fn get_all_authorised() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.get_all_authorised()
    })
}

#[update]
fn add_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.remove_authorised(principal_id)
    })
}

#[query]
fn get_canister_name() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.get_canister_name()
    })
}

#[update]
fn set_canister_name(name: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.set_canister_name(name)
    })
}

#[query]
fn get_single_account(input_principal: String, input_subaccount: u32) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|
        get_single_account_impl(input_principal, input_subaccount as u8, &state.borrow())
    )
}
fn get_single_account_impl(
    inpt_principal: String,
    inpt_subaccount: u8,
    runtime_state: &RuntimeState
) -> String {
    let v = runtime_state.data.authorised.clone();
    validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
    let sub_ac = get_subaccount_from_principal(inpt_principal, inpt_subaccount);
    return sub_ac;
}

#[query]
fn get_multiple_account(input_principal: String, start: u32, get_number: u32) -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|
        get_multiple_account_impl(input_principal, start as u8, get_number as u8, &state.borrow())
    )
}
fn get_multiple_account_impl(
    inpt_principal: String,
    inpt_start: u8,
    inpt_number: u8,
    runtime_state: &RuntimeState
) -> Vec<String> {
    let v = runtime_state.data.authorised.clone();
    validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
    let sub_ac = get_multiple_subaccounts_from_principal(inpt_principal, inpt_start, inpt_number);
    return sub_ac;
}

#[update]
async fn is_genesis_holder(input_id: String) -> bool {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let res: bool = is_genesis_holder_impl(input_id).await;
    return res;
}
async fn is_genesis_holder_impl(input_id: String) -> bool {
    let user_id: String;
    if input_id.contains("-") {
        user_id = get_subaccount_from_principal(input_id, 0);
    } else {
        user_id = input_id;
    }

    // get all holders
    let target: Principal = candid::Principal
    ::from_text("t555s-uyaaa-aaaal-qbjsa-cai")
    .unwrap();
    let (vec1,): (Vec<(u32, String)>,) = ic_cdk::api::call
    ::call(target, "getRegistry", ()).await
    .unwrap();

    let is_holder: bool = vec1.iter().any(|(_, value)| *value == user_id);
    return is_holder;
}


#[query]// not public 
fn get_user_count() -> usize {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{
        s.borrow().data.users.len()
    })
}


// #[query]
// fn read_genesis_holders() -> Vec<(u32, String)> {
//     RUNTIME_STATE.with(|state| {
//         let s = state.borrow();
//         s.data.check_authorised(ic_cdk::caller().to_text());
//     });
//     RUNTIME_STATE.with(|state| read_genesis_holders_impl(&state.borrow()))
// }
// fn read_genesis_holders_impl(runtime_state: &RuntimeState) -> Vec<(u32, String)> {
//     let v = runtime_state.data.authorised.clone();
//     validate_caller(ic_cdk::caller().to_text(), v); // is authorised?
//     let ret = runtime_state.data.genesis_holders.to_owned();
//     return ret;
// }

// [][] ------------------------- [][]
// [][] ---- Timer Functions ---- [][]
// [][] ------------------------- [][]

// #[update]
// fn stop_all_timers() -> String {
//     RUNTIME_STATE.with(|state| {
//         let s = state.borrow();
//         s.data.check_authorised(ic_cdk::caller().to_text());
//     });

//     TIMER_IDS.with(|timer_ids| {
//         let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
//         for i in vec1.iter() {
//             ic_cdk_timers::clear_timer(*i);
//             // ic_cdk::println!("Length : {}", vec1.len());
//             // ic_cdk::println!("{:?}", i)
//         }
//         vec1.clear();
//     });
//     RUNTIME_STATE.with(|state| {
//         state.borrow_mut().data.genesis_updating = false;
//     });
//     log("All timers stopped");
//     return String::from("All Timers Stopped");
// }

// #[update]
// fn check_and_start_genesis_timer(secs: u64) -> String {
//     RUNTIME_STATE.with(|state| {
//         let s = state.borrow();
//         s.data.check_authorised(ic_cdk::caller().to_text());
//     });

//     let ret: String;
//     let is_running = RUNTIME_STATE.with(|state| {
//         return state.borrow().data.genesis_updating;
//     });

//     if is_running == true {
//         ret = String::from("Genesis holder update timer is already running");
//     } else {
//         start_genesis_holder_timer(secs);
//         RUNTIME_STATE.with(|state| {
//             state.borrow_mut().data.genesis_updating = true;
//         });
//         ret = String::from("Genesis holder update timer has been started");
//         let lg = format!("Started Timer to fetch Genesis holders: {} seconds interval", secs);
//         log(lg);
//         log(format!("Cycles Balance : {}", get_cycles_balance()));
//     }
//     return ret;
// }

// fn start_genesis_holder_timer(secs: u64) {
//     RUNTIME_STATE.with(|state| {
//         let s = state.borrow();
//         s.data.check_authorised(ic_cdk::caller().to_text());
//     });

//     let secs = Duration::from_secs(secs);
//     let timer_id = ic_cdk_timers::set_timer_interval(secs, || ic_cdk::spawn(get_genesis_holders()));
//     TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
//     // To drive an async function to completion inside the timer handler,
//     // use `ic_cdk::spawn()`, for example:
//     // Non async = //ic_cdk_timers::set_timer_interval(secs, get_genesis_holders);
// }

// async fn get_genesis_holders() -> () {
//     let target: Principal = candid::Principal
//         ::from_text("t555s-uyaaa-aaaal-qbjsa-cai")
//         .unwrap();
//     let (vec1,): (Vec<(u32, String)>,) = ic_cdk::api::call
//         ::call(target, "getRegistry", ()).await
//         .unwrap();
//     ic_cdk::println!("{:?}", vec1);
//     RUNTIME_STATE.with(|state| update_genesis_holders(vec1, &mut state.borrow_mut()))
// }

// fn update_genesis_holders(latest_holders: Vec<(u32, String)>, runtime_state: &mut RuntimeState) {
//     runtime_state.data.genesis_holders = latest_holders;
//     log("Genesis II holders updated");
//     log(format!("Cycles Balance : {}", get_cycles_balance()));
// }

// [][] ------------------------ [][]
// [][] --- Canister Metrics --- [][]
// [][] ------------------------ [][]
#[query]
fn get_cycles_balance() -> u64 {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    let WASM_PAGE_SIZE: u64 = 65536;
    let m: u64 =
        (ic_cdk::api::stable::stable64_size() as u64) * WASM_PAGE_SIZE +
        (core::arch::wasm32::memory_size(0) as u64) * WASM_PAGE_SIZE;
    let m2: u64 = (core::arch::wasm32::memory_size(0) as u64) * WASM_PAGE_SIZE;
    let ret = MemoryData {
        memory: m,
        heap_memory: m2,
    };
    return ret;
}

#[query]
fn read_logs() -> Option<Vec<LogEntry>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let mut ret: Option<Vec<LogEntry>> = None;
    LOGS_STATE.with(|state| {
        let logs: &Vec<LogEntry> = &state.borrow().data.canister_logs;
        if logs.len() > 0 {
            ret = Some(logs.to_owned());
        } else {
            ret = None;
        }
    });
    return ret;
}

pub fn log(text: impl AsRef<str>) {
    LOGS_STATE.with(|state| {
        let max_logs = 200;
        let logs = &mut state.borrow_mut().data.canister_logs;
        let nano_time = ic_cdk::api::time();
        let log_entry: LogEntry = LogEntry {
            timestamp: nano_time.to_string(),
            text: text.as_ref().to_string(),
        };
        logs.push(log_entry);
        if logs.len() > max_logs {
            logs.to_owned().remove(0);
        }
    });
}
