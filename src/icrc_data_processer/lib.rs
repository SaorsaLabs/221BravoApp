#[allow(non_snake_case)]
mod types;
mod utils;
mod constants;
mod test_data;

use candid::{ CandidType, Nat, Principal };
use ic_cdk_macros::*;
use serde::Deserialize;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::collections::{ BTreeMap, VecDeque, HashMap };
use std::ops::Deref;
use std::time::Duration;
use utils::{
    get_unique_string_values,
    nearest_past_hour,
    nearest_day_start,
    top_x_by_txvalue,
    top_x_txcount,
};
use ic_cdk_timers::TimerId;
use num_traits::cast::ToPrimitive;

use constants::{
    MAX_TRANSACTION_BATCH_SIZE,
    MAX_TOTAL_DOWNLOAD,
    DAY_AS_NANOS,
    HOUR_AS_NANOS,
    MAX_DAYS,
    MAX_HOURS,
    STATS_RETURN_LENGTH,
    MAX_LOG_LENGTH,
    VERSION
};

use types::{
    GetTransactionsRequest,
    GetTransactionsResponse,
    MemoryData, // ignore unsued inport warning!
    WorkingStats,
    TransactionType,
    EntityData,
    LogEntry,
    CanisterSettings,
    ProcessedTX,
    ArchivedRange,
    QueryTxArchiveFn,
    GetTransactionsArchiveResponse,
    Burn,
    Mint,
    Transfer,
    DailyStats,
    HourlyStats,
    TotalHoldersResponse,
    TopHoldersResponse,
    HolderBalance,
    TotCntAvg,
    TimeStats,
    StatsType,
    TimeChunkStats, Approve,
};

//[][] ---- State Manamgement ---- [][]
thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
    static LOGS_STATE: RefCell<LogsState> = RefCell::default();
}

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

// [][] --- Main Data Struct --- [][]
#[derive(CandidType, Deserialize, Default)]
struct Data {
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    timer_active: bool,
    processing_data: bool,
    first_run: bool,
    working_stats: WorkingStats,
    canister_settings: CanisterSettings,
    retained_blocks: VecDeque<ProcessedTX>,
    principal_holders: BTreeMap<String, EntityData>,
    account_holders: BTreeMap<String, EntityData>,
    hourly_stats: HourlyStats,
    daily_stats: DailyStats,
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

    fn are_stats_public(&self) -> bool {
        let ret = &self.canister_settings.stats_are_public;
        return ret.to_owned();
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

fn with_runtime<R>(f: impl FnOnce(&RuntimeState) -> R) -> R {
    RUNTIME_STATE.with(|idx| f(idx.borrow().deref()))
}

fn with_runtime_mut<R>(f: impl FnOnce(&mut RuntimeState) -> R) -> R {
    RUNTIME_STATE.with(|idx| f(&mut *idx.borrow_mut()))
}

#[init]
fn init() {
    // init main data state
    let mut data = Data::default();
    data.authorised.push("2vxsx-fae".to_string());
    data.authorised.push(
        "DEV_PRINCIPAL_HERE".to_string()
    ); 
    data.authorised.push(
        "FRONTEND_PRINCIPAL_HERE".to_string()
    ); 
    data.canister_settings.stats_are_public = true;
    data.canister_settings.stats_return_length = STATS_RETURN_LENGTH;
    data.first_run = true;
    data.canister_settings.canister_name = "Name me please!".to_string();
    data.working_stats.version = VERSION.to_string();
    let runtime_state = RuntimeState { data };
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    // init canister logging state
    let log_data = LoggingData::default();
    let logs_state = LogsState { data: log_data };
    LOGS_STATE.with(|state| {
        *state.borrow_mut() = logs_state;
    });
    log("Canister Initialised");
}

#[pre_upgrade]
fn pre_upgrade() {
    RUNTIME_STATE.with(|state|{
        let b = state.borrow().data.working_stats.is_busy;
        if b == true { ic_cdk::trap("Canister Busy - Upgrade Stopped! Check working stats for status")}
    });
    RUNTIME_STATE.with(|state| ic_cdk::storage::stable_save((&state.borrow().data,)).unwrap());
}

#[post_upgrade]
fn post_upgrade() {
    let (mut data,): (Data,) = ic_cdk::storage::stable_restore().unwrap();
    data.working_stats.version = VERSION.to_string(); // update version 
    let runtime_state = RuntimeState { data };
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    log("Canister upgraded, remember to start any timers! ");
}

//[][] --------------------------------- [][]
//[][] ---- Query/ Update Functions ---- [][]
//[][] --------------------------------- [][]
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
fn get_working_stats() -> WorkingStats {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    with_runtime(|rts| { rts.data.working_stats.to_owned() })
}

#[query]
fn get_total_holders() -> TotalHoldersResponse {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });

    with_runtime(|rts| { 
        let principals =  rts.data.principal_holders.len();
        let accounts = rts.data.account_holders.len();
        let ret = TotalHoldersResponse {
            principals: principals as u64,
            accounts: accounts as u64,
        };
        return ret;
    })
}

#[query]
fn get_top_holders(top_x: usize) -> TopHoldersResponse {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });

    with_runtime(|rts| {
        let principals = &rts.data.principal_holders;
        let accounts = &rts.data.account_holders;
        let pr_len = if top_x > principals.len() { principals.len() } else { top_x };
        let ac_len = if top_x > accounts.len() { accounts.len() } else { top_x };
        
        // ACCOUNTS
        let mut ac_vec: Vec<HolderBalance> = vec![];
        for (hdr, ed) in accounts {
            ac_vec.push(HolderBalance { holder: hdr.clone(), balance: ed.balance.clone() });
        }
        ac_vec.sort_unstable_by_key(|element| element.balance);
        ac_vec.reverse();
        let mut top_ac: Vec<HolderBalance> = vec![];
        for i in 0..ac_len as usize {
            top_ac.push(ac_vec[i].to_owned());
        }
        // PRINCIPALS
        let mut pr_vec: Vec<HolderBalance> = vec![];
        for (hdr, ed) in principals {
            pr_vec.push(HolderBalance { holder: hdr.clone(), balance: ed.balance.clone() });
        }
        pr_vec.sort_unstable_by_key(|element| element.balance);
        pr_vec.reverse();
        let mut top_pr: Vec<HolderBalance> = vec![];
        for i in 0..pr_len as usize {
            top_pr.push(pr_vec[i].to_owned());
        }

        let res = TopHoldersResponse {
            top_accounts: top_ac,
            top_principals: top_pr,
        };
        return res;
    })
}

#[query]
fn get_account_balance(id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });

    let ac_balance: String = with_runtime(|rts| {
        let ret = rts.data.account_holders.get(&id);
        match ret {
            Some(v) => { return format!("{:?}",v.balance)},
            None => {return "Cannot find balance for that Account".to_string()},
        }
    });
    return ac_balance;
}

#[query]
fn get_principal_balance(id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });

    let pr_balance: String = with_runtime(|rts| {
        let ret = rts.data.principal_holders.get(&id);
        match ret {
            Some(v) => { return format!("{:?}",v.balance)},
            None => {return "Cannot find balance for that Principal".to_string()},
        }
    });
    return pr_balance;
}

#[query]
fn get_hourly_stats() -> TimeStats {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    let hs = with_runtime(|rts| { rts.data.hourly_stats.data.clone() });
    return hs;
}

#[query]
fn get_daily_stats() -> TimeStats {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    let ds: TimeStats = with_runtime(|rts| { rts.data.daily_stats.data.clone() });
    return ds;
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
async fn set_target_canister(canister_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    let is_first_run = with_runtime(|rts| { rts.data.first_run });
    if is_first_run == false {
        ic_cdk::trap(
            "Target canister cann't be changed after being set. Re-install canister to change."
        );
    }

    let ledger_id = ic_cdk::export::Principal::from_text(&canister_id);
    match ledger_id {
        Ok(pr_id) => {
            let (fee_call,): (Nat,) = ic_cdk
                ::call(pr_id, "icrc1_fee", ()).await
                .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                .unwrap();

            let fee_u64 = fee_call.0.to_u64().ok_or("Fee Result is not a valid u64");

            match fee_u64 {
                Ok(value) => {
                    log(format!("Target: {}", &canister_id));
                    with_runtime_mut(|rts| {
                        rts.data.canister_settings.transaction_fee = value;
                        rts.data.first_run = false;
                        rts.data.canister_settings.target_canister = canister_id;
                    });
                    log("[][] ---- Target Canister Set ---- [][]");
                    log(format!("Updated transfer fee: {}", &value));
                }
                Err(error) => {
                    log(format!("Error setting target canister: {}", error));
                    ic_cdk::trap(
                        "Cannot read fee from target canister. Check target canister is an ICRC ledger canister"
                    );
                }
            }
        }
        Err(error) => {
            log(format!("Can't get principal from text. Error {}", error));
        }
    }
    return "Target canister and fee set".to_string();
}

#[update]
fn set_canister_name(name: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.data.check_authorised(ic_cdk::caller().to_text());
        s.data.set_canister_name(name)
    })
}

#[update]
fn set_stats_timescales(hours_to_calculate: u64, days_to_calculate: u64) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    if hours_to_calculate == 0 || days_to_calculate == 0 {
        ic_cdk::trap("hours and days cannot be 0");
    }
    if hours_to_calculate > MAX_HOURS || days_to_calculate > MAX_DAYS {
        ic_cdk::trap("Hour or Days is greater than max allowed");
    }
    with_runtime_mut(|rts| {
        rts.data.canister_settings.days_to_calcualte = days_to_calculate;
        rts.data.canister_settings.hours_to_calculate = hours_to_calculate;
    });
    return "Updated Stats Gathering Timescales".to_string();
}

#[update]
fn set_stats_public(input_bool: bool) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let ret: String;
    match input_bool {
        true => {
            with_runtime_mut(|rts| {
                rts.data.canister_settings.stats_are_public = true;
            });
            ret = "Stats Made Public".to_string();
        }
        false => {
            with_runtime_mut(|rts| {
                rts.data.canister_settings.stats_are_public = false;
            });
            ret = "Stats Made Private".to_string();
        }
    }
    return ret;
}

#[update]
fn set_stats_return_length(length: usize) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });
    let ret: String;
    if length > 0 {
        with_runtime_mut(|rts| {
            rts.data.canister_settings.stats_return_length = length;
        });
        ret = format!("Return Length set to {}", length).to_string();
    } else {
        ret = "Return Length must be greater than 0".to_string();
    }
    return ret;
}

#[update]
fn stop_all_timers() -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    TIMER_IDS.with(|timer_ids: &RefCell<Vec<TimerId>>| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });
    RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
        state.borrow_mut().data.timer_active = false;
    });
    log("[][] ---- Processing timer stopped ---- [][]");
    return String::from("Processing timer stopped");
}

#[update]
fn check_and_start_processing_timer(secs: u64) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    // check target canister is set
    let canister_settings: CanisterSettings = with_runtime(|rts| {
        rts.data.canister_settings.to_owned()
    });
    if canister_settings.target_canister.is_empty() {
        ic_cdk::trap("Target Canister Not Set!");
    }
    // check hours/ days is set
    if canister_settings.days_to_calcualte == 0 || canister_settings.hours_to_calculate == 0 {
        ic_cdk::trap("Hours to calculate or Days to calculate cannot be 0");
    }

    let ret: String;
    let is_running = RUNTIME_STATE.with(|state| {
        return state.borrow().data.timer_active;
    });
    if is_running == true {
        ret = String::from("Processing timer is alraedy running");
    } else {
        start_processing_timer(secs);
        RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
            state.borrow_mut().data.timer_active = true;
        });
        ret = String::from("Processing timer has been started");
        log("[][] ---- Starting Processing Timer ---- [][]");
    }
    return ret;
}


async fn fetch_data() {
    // Calculate time windows (for retained blocks)
    let time_now = ic_cdk::api::time();
    let hour_start_time: u64 = with_runtime(|rts| {
        time_now - rts.data.canister_settings.hours_to_calculate * HOUR_AS_NANOS
    });
    let day_start_time: u64 = with_runtime(|rts| {
        time_now - rts.data.canister_settings.days_to_calcualte * DAY_AS_NANOS
    });
    let targ_canister = with_runtime(|rts| {
        rts.data.canister_settings.target_canister.to_owned()
    });

    // Download latest blocks
    let ledger_id = ic_cdk::export::Principal::from_text(targ_canister);
    match ledger_id {
        Ok(ledger_id) => {
            let tip_u128 = get_tip_u128(ledger_id).await;
            match tip_u128 {
                Ok(tip) => {
                    if tip > 0 {
                        let next_block = with_runtime(|rts| rts.data.working_stats.next_tx);
                        let blocks_needed = tip - next_block;
                        let chunks_needed = (
                            (blocks_needed as f32) / (MAX_TRANSACTION_BATCH_SIZE as f32)
                        ).ceil() as u32;

                        log("[][] ----- Starting ICRC Download ----- [][]");
                        log(
                            format!(
                                "Blocks Needed: {}, Chunks Needed: {}, Tip: {}, Next-Block: {}",
                                blocks_needed,
                                chunks_needed,
                                tip,
                                next_block
                            )
                        );

                        if blocks_needed > 0 {
                            with_runtime_mut(|rts| {
                                rts.data.working_stats.is_upto_date = false;
                            });
                            let mut start: u128;
                            let mut length: u128;
                            let mut remaining: u128;
                            let mut completed_this_run: u128 = 0_u128;
                            let mut temp_tx_array: Vec<ProcessedTX> = Vec::new();
                            let max_loops = (
                                (MAX_TOTAL_DOWNLOAD as f64) / (MAX_TRANSACTION_BATCH_SIZE as f64)
                            ).ceil() as u32;
                            let segment: u32 = if chunks_needed > max_loops {
                                max_loops
                            } else {
                                chunks_needed
                            };

                            // Download in chunks
                            for i in 0..segment {
                                start = if i == 0 {
                                    next_block
                                } else {
                                    next_block + completed_this_run
                                };
                                remaining = tip - start;
                                length = if remaining >= (MAX_TRANSACTION_BATCH_SIZE as u128) {
                                    MAX_TRANSACTION_BATCH_SIZE as u128
                                } else {
                                    remaining
                                };
                                // Get transactions
                                let txns: Option<Vec<ProcessedTX>> = icrc_transaction_download(
                                    start,
                                    length
                                ).await;
                                let mut txns_len = 0_u128;
                                if let Some(txns) = txns {
                                    txns_len = txns.len() as u128;
                                    for tx in txns {
                                        temp_tx_array.push(tx);
                                    }
                                } else {
                                    log("No transactions in this chunk!");
                                }
                                completed_this_run += txns_len;
                            }
                            log(format!("{} chunks downloaded", segment));

                            // Calculate and update balances
                            let ub_res = update_balances(&temp_tx_array).await;
                            if ub_res == false {
                                log("Error when updating balances");
                                ic_cdk::trap("Error when updating balances");
                            } else {
                                log("[][] --- Hodler Balances Updated --- [][]");
                            }

                            // clean old blocks from retained vecdeque
                            let mut ret_blocks = with_runtime(|rts| {
                                rts.data.retained_blocks.to_owned()
                            });
                            let clean_before = if day_start_time < hour_start_time {
                                day_start_time
                            } else {
                                hour_start_time
                            };
                            if ret_blocks.len() > 0 {
                                ret_blocks.retain(
                                    |transaction| transaction.tx_time >= clean_before
                                );
                            }
                            // add new blocks
                            let mut is_upto_date = false;
                            let tip_as_nat = Nat::from(tip) - Nat::from(1);
                            for tx in &temp_tx_array {
                                if tx.tx_time >= clean_before {
                                    ret_blocks.push_back(tx.clone());
                                }
                                if tx.block >= tip_as_nat {
                                    is_upto_date = true;
                                    with_runtime_mut(|rts| {
                                        rts.data.working_stats.task_id = 1;
                                    });
                                }
                            }
                            let ret_block_len: &usize = &ret_blocks.len();
                            with_runtime_mut(|rts| {
                                rts.data.retained_blocks = ret_blocks;
                            });
                            log("[][] --- Retained blocks updated --- [][]");
                            temp_tx_array.clear();

                            // update working stats state
                            if ub_res == true {
                                with_runtime_mut(|rts| {
                                    let mut s = rts.data.working_stats.borrow_mut();
                                    s.total_downloaded = next_block + completed_this_run;
                                    s.tx_completed_to = next_block + completed_this_run - 1;
                                    s.next_tx = next_block + completed_this_run;
                                    s.is_upto_date = is_upto_date;
                                }); // -1 to account for 0 block

                                log(
                                    format!(
                                        "Complete To {}; Retained Transactions {}; All transactions downloaded? = {};",
                                        next_block + completed_this_run - 1,
                                        ret_block_len,
                                        is_upto_date
                                    )
                                );
                            }
                        } // end if blocks_needed
                    } // end if tip
                } // end Ok tip
                Err(error) => {
                    log(format!("ERROR : {}", error));
                }
            } // end match tip
        }
        Err(error) => {
            log(format!("Cannot derive principal from target canister. Error {}", error));
        }
    } // match ledger id
    with_runtime_mut(|rts| {
        rts.data.working_stats.is_busy = false;
    });
    return;
}

async fn calc_hourly_stats() -> bool {
    let time_now: u64 = ic_cdk::api::time();
    let hour_start_time: u64 = with_runtime(|rts| {
        time_now - rts.data.canister_settings.hours_to_calculate * HOUR_AS_NANOS
    });
    if time_now == 0 || hour_start_time == 0 {
        return false;
    }

    let hourly_stats: Result<TimeStats, String> = calculate_time_stats(
        hour_start_time,
        StatsType::Hourly,
        time_now.clone()
    );
    match hourly_stats {
        Ok(v) => {
            with_runtime_mut(|rts: &mut RuntimeState| {
                rts.data.hourly_stats.data = v;
                rts.data.working_stats.is_busy = false;
            });
            return true;
        }
        Err(error) => {
            log(format!("Error calculating hourly stats. State is not updated. Error {}", error));
            with_runtime_mut(|rts: &mut RuntimeState| {
                rts.data.working_stats.is_busy = false;
            });
            return false;
        }
    }
}

async fn calc_daily_stats() -> bool {
    let processed_ok: bool;
    let time_now = ic_cdk::api::time();
    let day_start_time: u64 = with_runtime(|rts: &RuntimeState| {
        time_now - rts.data.canister_settings.days_to_calcualte * DAY_AS_NANOS
    });

    let daily_stats: Result<TimeStats, String> = calculate_time_stats(
        day_start_time,
        StatsType::Daily,
        time_now.clone()
    );
    match daily_stats {
        Ok(v) => {
            processed_ok = true;
            with_runtime_mut(|rts: &mut RuntimeState| {
                rts.data.daily_stats.data = v;
                rts.data.working_stats.is_busy = false;
            });
        }
        Err(error) => {
            processed_ok = false;
            log(format!("Error calculating daily stats. State is not updated. Error {}", error));
            with_runtime_mut(|rts: &mut RuntimeState| {
                rts.data.working_stats.is_busy = false;
            });
        }
    }
    return processed_ok;
}

async fn calc_most_active() -> bool {
    let time_now: u64 = ic_cdk::api::time();
    let hour_start_time: u64 = with_runtime(|rts| {
        time_now - rts.data.canister_settings.hours_to_calculate * HOUR_AS_NANOS
    });
    let ret_len: usize = with_runtime(|rts| { rts.data.canister_settings.stats_return_length });
    most_active(hour_start_time, ret_len).await;
    with_runtime_mut(|rts| {
        rts.data.working_stats.is_busy = false;
    });
    return true;
}


//[][] ----------------------------------- [][]
//[][] ---- Data Download/ Processing ---- [][]
//[][] ----------------------------------- [][]
async fn icrc_transaction_download(start: u128, length: u128) -> Option<Vec<ProcessedTX>> {
    // check target canister is set
    let canister_settings = with_runtime(|rts| { rts.data.canister_settings.to_owned() });
    if canister_settings.target_canister.is_empty() {
        log("Target Canister Not Set!");
        ic_cdk::trap("Target Canister Not Set!");
    }

    let ledger_id = ic_cdk::export::Principal
        ::from_text(canister_settings.target_canister)
        .unwrap();
    let mut processed_transactions: Vec<ProcessedTX> = vec![];
    let res: Result<GetTransactionsResponse, String> = get_transactions_from_ledger(
        ledger_id,
        start,
        length
    ).await;
    match res {
        Ok(value) => {
            match (value.transactions.is_empty(), value.archived_transactions.is_empty()) {
                (false, false) => {
                    // Ledger and Archive

                    //  Archive TXS
                    let mut block_master: Nat = Nat::from(0);
                    for archived in value.archived_transactions {
                        let mut block = archived.start.clone();

                        let archived = ArchivedRange::<QueryTxArchiveFn> {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                // loop through results
                                for transaction in data.transactions {
                                    if let Some(value) = transaction.mint {
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.burn {
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.transfer {
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.approve {
                                        processed_transactions.push(
                                            process_approve_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!("Error fetching archive transactions. Error : {}", err_text)
                                );
                            }
                        }
                        block_master = block;
                    }

                    // Ledger TXS
                    for transaction in value.transactions {
                        if let Some(value) = transaction.mint {
                            processed_transactions.push(
                                process_mint_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                        if let Some(value) = transaction.burn {
                            processed_transactions.push(
                                process_burn_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                        if let Some(value) = transaction.transfer {
                            processed_transactions.push(
                                process_transfer_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                        if let Some(value) = transaction.approve {
                            processed_transactions.push(
                                process_approve_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                    }

                    return Some(processed_transactions);
                }
                (false, true) => {
                    // Ledger TX only - no archive
                    let mut block = Nat::from(start);
                    for transaction in value.transactions {
                        if let Some(value) = transaction.mint {
                            processed_transactions.push(
                                process_mint_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                        if let Some(value) = transaction.burn {
                            processed_transactions.push(
                                process_burn_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                        if let Some(value) = transaction.transfer {
                            processed_transactions.push(
                                process_transfer_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                        if let Some(value) = transaction.approve {
                            processed_transactions.push(
                                process_approve_transaction(
                                    value,
                                    &block,
                                    &transaction.timestamp
                                )
                            );
                            block += Nat::from(1);
                        }
                    }

                    return Some(processed_transactions);
                }
                (true, false) => {
                    // Archive TXS ONLY
                    for archived in value.archived_transactions {
                        let archived = ArchivedRange::<QueryTxArchiveFn> {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = archived.start.clone();
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                // loop through results
                                for transaction in data.transactions {
                                    if let Some(value) = transaction.mint {
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.burn {
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.transfer {
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.approve {
                                        processed_transactions.push(
                                            process_approve_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!("Error fetching archive transactions. Error : {}", err_text)
                                );
                            }
                        }
                    }
                    return Some(processed_transactions);
                }
                (true, true) => {
                    log("No Data to fetch!".to_string());
                    return None;
                }
            }
        }
        Err(error) => ic_cdk::trap(&error),
    }
}

async fn get_transactions_from_ledger(
    ledger_id: Principal,
    start: u128,
    length: u128
) -> Result<GetTransactionsResponse, String> {
    let req = GetTransactionsRequest {
        start: Nat::from(start),
        length: Nat::from(length),
    };
    let call: Result<(GetTransactionsResponse,), _> = ic_cdk::call(ledger_id, "get_transactions", (
        req,
    )).await;
    match call {
        Ok(v) => { Ok(v.0) }
        Err(error) => {
            log(format!("Error getting transactions from ICRC ledger. {}", error.1));
            Err(format!("code: {:#?} message: {}", error.0, error.1))
        }
    }
}

async fn get_transactions_from_archive(
    archived: &ArchivedRange<QueryTxArchiveFn>
) -> Result<GetTransactionsArchiveResponse, String> {
    let req = GetTransactionsRequest {
        start: archived.start.clone(),
        length: archived.length.clone(),
    };
    let ledger_id = archived.callback.canister_id;
    let method = &archived.callback.method;
    let call: Result<(GetTransactionsArchiveResponse,), _> = ic_cdk::call(ledger_id, method, (
        req,
    )).await;
    match call {
        Ok(v) => { Ok(v.0) }
        Err(error) => {
            log(format!("Error getting transactions from ICRC Archive. {}", error.1));
            Err(format!("code: {:#?} message: {}", error.0, error.1))
        }
    }
}

fn process_mint_transaction(tx: Mint, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let to_ac = tx.to;
    let to_pr = to_ac.owner.to_string();
    let sub = to_ac.effective_subaccount();
    let sub_ac = hex::encode(sub);

    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Mint.to_string(),
        from_principal: "ICRC_LEDGER".to_string(),
        from_account: "ICRC_LEDGER".to_string(),
        to_principal: to_pr,
        to_account: sub_ac,
        tx_value: tx.amount,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_burn_transaction(tx: Burn, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let from_ac: types::Account = tx.from;
    let from_pr = from_ac.owner.to_string();
    let sub = from_ac.effective_subaccount();
    let sub_ac = hex::encode(sub);

    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Burn.to_string(),
        from_principal: from_pr,
        from_account: sub_ac,
        to_principal: "ICRC_LEDGER".to_string(),
        to_account: "ICRC_LEDGER".to_string(),
        tx_value: tx.amount,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_transfer_transaction(tx: Transfer, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let from_ac = tx.from;
    let from_pr = from_ac.owner.to_string();
    let from_sub = from_ac.effective_subaccount();
    let from_sub_ac = hex::encode(from_sub);

    let to_ac = tx.to;
    let to_pr = to_ac.owner.to_string();
    let to_sub = to_ac.effective_subaccount();
    let to_sub_ac = hex::encode(to_sub);

    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Transaction.to_string(),
        from_principal: from_pr,
        from_account: from_sub_ac,
        to_principal: to_pr,
        to_account: to_sub_ac,
        tx_value: tx.amount,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_approve_transaction(tx: Approve, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let from_ac = tx.from;
    let from_pr = from_ac.owner.to_string();
    let from_sub = from_ac.effective_subaccount();
    let from_sub_ac = hex::encode(from_sub);

    let to_ac = tx.spender;
    let to_pr = to_ac.owner.to_string();
    let to_sub = to_ac.effective_subaccount();
    let to_sub_ac = hex::encode(to_sub);

    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Approve.to_string(),
        from_principal: from_pr,
        from_account: from_sub_ac,
        to_principal: to_pr, // THIS IS THE SPENDER PRINCIPAL
        to_account: to_sub_ac, // THIS IS THE SPENDER SUB-ACCOUNT
        tx_value: tx.amount, // THIS IS THE APPROVAL VALUE
        tx_time: timestamp.to_owned(),
    };
    return ret;
} 

async fn get_tip_of_chain(ledger_id: Principal) -> Result<Nat, String> {
    let req = GetTransactionsRequest {
        start: Nat::from(0),
        length: Nat::from(1),
    };
    let (res,): (GetTransactionsResponse,) = ic_cdk
        ::call(ledger_id, "get_transactions", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res.log_length)
}

async fn get_tip_u128(ledger_id: Principal) -> Result<u128, String> {
    let tip_chain = get_tip_of_chain(ledger_id).await;
    match tip_chain {
        Ok(v) => {
            let tip_u128 = v.0.to_u128();
            match tip_u128 {
                Some(v) => { Ok(v) }
                None => {
                    let error = "Error getting u128 from Nat - tip of chain.".to_string();
                    log(&error);
                    Err(error)
                }
            }
        }
        Err(error) => {
            log(format!("Error getting tip of chain. {}", error));
            Err(error)
        }
    }
}

async fn update_balances(tx_array: &Vec<ProcessedTX>) -> bool {
    if tx_array.len() == 0 {
        return true;
    }

    let processed_ok_fn:bool;
    processed_ok_fn = RUNTIME_STATE.with(|state| {
        let rts = &mut state.borrow_mut();
        let data = rts.data.borrow_mut();
        let tx_fee = data.canister_settings.transaction_fee as u128;
        let mut from_combined_account: String;
        let mut to_combined_account: String;
        let mut processed_ok = true;

        for tx in tx_array {
            let tx_value_u128 = tx.tx_value.0.to_u128().ok_or("Tip of Chain is not a valid u128");
            match tx_value_u128 {
                Ok(tx_value_u128) => {
                    match tx.tx_type.as_str() {
                        "Transaction" => {
                            // ----- DEBIT FROM
                            // ----- account balance
                            from_combined_account = format!(
                                "{}.{}",
                                &tx.from_principal,
                                &tx.from_account
                            );
                            if let Some(ac) = data.account_holders.get(&from_combined_account) {
                                let tot_deduction;
                                if ac.balance < (tx_value_u128 + tx_fee) {
                                    tot_deduction = ac.balance.clone(); // catch overflows. cant spend more than ac balance.
                                    log(
                                        format!(
                                            "Caught overflow from transfer. Account: {}. Balance: {}, tx_total(inc fee): {}, fee: {}",
                                            &from_combined_account, &ac.balance, (tx_value_u128 + tx_fee), &tx_fee
                                        )
                                    );
                                } else {
                                    tot_deduction = tx_value_u128.clone() + tx_fee.clone();
                                }
                                // existing account
                                let ent = EntityData {
                                    balance: ac.balance - tot_deduction,
                                    transactions: ac.transactions + 1_u64,
                                };
                                data.account_holders.insert(from_combined_account.clone(), ent);
                            } else {
                                log(
                                    format!(
                                        "Error: Sent transaction from new unknown account {}",
                                        &from_combined_account
                                    )
                                );
                                processed_ok = false;
                            }

                            // principal balance
                            if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                                let tot_deduction;
                                if pr.balance < (tx_value_u128 + tx_fee) {
                                    tot_deduction = pr.balance; // catch overflows. cant spend more than ac balance.
                                    log(
                                        format!(
                                            "Caught overflow from transfer. Principal: {}",
                                            &tx.from_principal
                                        )
                                    );
                                } else {
                                    tot_deduction = tx_value_u128.clone() + tx_fee.clone();
                                }
                                // existing account
                                let ent = EntityData {
                                    balance: pr.balance - tot_deduction,
                                    transactions: pr.transactions + 1_u64,
                                };
                                data.principal_holders.insert(tx.from_principal.clone(), ent);
                            } else {
                                log(
                                    format!(
                                        "Error: Sent transaction from new unknown principal {}",
                                        &tx.from_principal
                                    )
                                );
                                processed_ok = false;
                            }

                            // ----- PAYMENT TO
                            // ----- account balance
                            to_combined_account = format!(
                                "{}.{}",
                                &tx.to_principal,
                                &tx.to_account
                            );
                            if let Some(ac) = data.account_holders.get(&to_combined_account) {
                                // existing account
                                let ent = EntityData {
                                    balance: ac.balance + tx_value_u128.clone(),
                                    transactions: ac.transactions + 1_u64,
                                };
                                data.account_holders.insert(to_combined_account.clone(), ent);
                            } else {
                                // new account
                                let ent = EntityData {
                                    balance: tx_value_u128.clone(),
                                    transactions: 1_u64,
                                };
                                data.account_holders.insert(to_combined_account, ent);
                            }

                            // principal balance
                            if let Some(pr) = data.principal_holders.get(&tx.to_principal) {
                                // existing account
                                let ent = EntityData {
                                    balance: pr.balance + tx_value_u128,
                                    transactions: pr.transactions + 1_u64,
                                };
                                data.principal_holders.insert(tx.to_principal.clone(), ent);
                            } else {
                                // new account
                                let ent = EntityData {
                                    balance: tx_value_u128,
                                    transactions: 1_u64,
                                };
                                data.principal_holders.insert(tx.to_principal.clone(), ent);
                            }
                        }
                        "Mint" => {
                            // account balance
                            to_combined_account = format!(
                                "{}.{}",
                                &tx.to_principal,
                                &tx.to_account
                            );
                            if let Some(ac) = data.account_holders.get(&to_combined_account) {
                                // existing account
                                let ent = EntityData {
                                    balance: ac.balance + tx_value_u128, // Nat
                                    transactions: ac.transactions + 1_u64,
                                };
                                data.account_holders.insert(to_combined_account.clone(), ent);
                            } else {
                                // new account
                                let ent = EntityData {
                                    balance: tx_value_u128,
                                    transactions: 1_u64,
                                };
                                data.account_holders.insert(to_combined_account, ent);
                            }
                            // principal balance
                            if let Some(pr) = data.principal_holders.get(&tx.to_principal) {
                                // existing principal
                                let ent = EntityData {
                                    balance: pr.balance + tx_value_u128, // Nat
                                    transactions: pr.transactions + 1_u64,
                                };
                                data.principal_holders.insert(tx.to_principal.clone(), ent);
                            } else {
                                // new principal
                                let ent = EntityData {
                                    balance: tx_value_u128,
                                    transactions: 1_u64,
                                };
                                data.principal_holders.insert(tx.to_principal.clone(), ent);
                            }
                        }
                        "Burn" => {
                            // account balance
                            from_combined_account = format!(
                                "{}.{}",
                                &tx.from_principal,
                                &tx.from_account
                            );
                            if let Some(ac) = data.account_holders.get(&from_combined_account) {
                                // existing account
                                let ent = EntityData {
                                    balance: ac.balance - tx_value_u128, // Nat
                                    transactions: ac.transactions + 1_u64,
                                };
                                data.account_holders.insert(from_combined_account.clone(), ent);
                            } else {
                                log(
                                    format!("Error: Burn transaction from new account {}", from_combined_account)
                                );
                            }
                            // principal balance
                            if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                                // existing principal
                                let ent = EntityData {
                                    balance: pr.balance - tx_value_u128, // Nat
                                    transactions: pr.transactions + 1_u64,
                                };
                                data.principal_holders.insert(tx.from_principal.clone(), ent);
                            } else {
                                log(
                                    format!(
                                        "Error: Burn transaction from new principal {}",
                                        tx.from_principal
                                    )
                                );
                                processed_ok = false;
                            }
                        }
                        "Approve" => {
                            // ----- DEBIT FEE ONLY (FROM)
                            // ----- account balance
                            from_combined_account = format!(
                                "{}.{}",
                                &tx.from_principal,
                                &tx.from_account
                            );
                            if let Some(ac) = data.account_holders.get(&from_combined_account) {
                                let tot_deduction;
                                if ac.balance < tx_fee {
                                    tot_deduction = ac.balance.clone(); // catch overflows. cant spend more than ac balance.
                                    log(
                                        format!(
                                            "Caught overflow from Approve Fee deduction. Account: {}",
                                            &from_combined_account
                                        )
                                    );
                                } else {
                                    tot_deduction = tx_fee.clone();
                                }
                                // existing account
                                let ent = EntityData {
                                    balance: ac.balance - tot_deduction,
                                    transactions: ac.transactions + 1_u64,
                                };
                                data.account_holders.insert(from_combined_account.clone(), ent);
                            } else {
                                log(
                                    format!(
                                        "Error: Approve Transaction from new unknown account {}",
                                        &from_combined_account
                                    )
                                );
                                processed_ok = false;
                            }

                            // principal balance
                            if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                                let tot_deduction;
                                if pr.balance < tx_fee {
                                    tot_deduction = pr.balance.clone(); // catch overflows. cant spend more than ac balance.
                                    log(
                                        format!(
                                            "Caught overflow from approve. Principal: {}",
                                            &tx.from_principal
                                        )
                                    );
                                } else {
                                    tot_deduction = tx_fee.clone();
                                }
                                // existing account
                                let ent = EntityData {
                                    balance: pr.balance - tot_deduction,
                                    transactions: pr.transactions + 1_u64,
                                };
                                data.principal_holders.insert(tx.from_principal.clone(), ent);
                            } else {
                                log(
                                    format!(
                                        "Error: Approve transaction from new unknown principal {}",
                                        &tx.from_principal
                                    )
                                );
                                processed_ok = false;
                            }
                        }
                        _ => {
                            log(
                                "Could not process transaction, type is not Mint, Burn, Approve or Transaction"
                            );
                            processed_ok = false;
                        }
                    }
                }
                Err(error) => {
                    log(
                        format!("Could not get tx_value_u128 from tx.tx_value (Nat). Error: {}", error)
                    );
                    processed_ok = false;
                }
            }
        }
        return processed_ok; // return closure
    });

    return processed_ok_fn; // return function
}

fn calculate_time_stats(
    process_from: u64,
    mode: StatsType,
    time_now: u64
) -> Result<TimeStats, String> {
    let fn_return: Result<TimeStats, String>;
    fn_return = RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
        // unique accounts.
        let rts: &mut std::cell::RefMut<'_, RuntimeState> = &mut state.borrow_mut();
        let array: &VecDeque<ProcessedTX> = &rts.data.retained_blocks;
        let mut all_accounts: Vec<String> = Vec::new();
        let mut all_principals: Vec<String> = Vec::new();
        let mut from_combined: String;
        let mut to_combined: String;
        let mut mint_count: u128 = 0_u128;
        let mut mint_value: u128 = 0_u128;
        let mut burn_count: u128 = 0_u128;
        let mut burn_value: u128 = 0_u128;
        let mut transaction_count: u128 = 0_u128;
        let mut transaction_value: u128 = 0_u128;
        let mut total_value: u128 = 0_u128;
        let mut total_txs: u128 = 0_u128;
        let mut error_output: String = String::new();
        let mut is_error = false;
        let mut all_mints: Vec<ProcessedTX> = Vec::new();
        let mut all_burns: Vec<ProcessedTX> = Vec::new();
        let mut all_transactions: Vec<ProcessedTX> = Vec::new();

        for tx in array {
            if tx.tx_time >= process_from {
                let value_u128: Result<u128, &str> = tx.tx_value.0
                    .to_u128()
                    .ok_or("Tip of Chain is not a valid u128");
                match value_u128 {
                    Ok(value_u128) => {
                        from_combined = format!("{}.{}", tx.from_principal, tx.from_account);
                        to_combined = format!("{}.{}", tx.to_principal, tx.to_account);
                        if tx.from_principal != "ICRC_LEDGER" {
                            all_accounts.push(from_combined);
                            all_principals.push(tx.from_principal.clone());
                        }
                        if tx.to_principal != "ICRC_LEDGER" && tx.tx_type != "Approve" {
                            all_accounts.push(to_combined);
                            all_principals.push(tx.to_principal.clone());
                        }
                        if tx.tx_type == "Mint" {
                            mint_count += 1_u128;
                            mint_value += &value_u128;
                            all_mints.push(tx.clone());
                        }
                        if tx.tx_type == "Burn" {
                            burn_count += 1_u128;
                            burn_value += &value_u128;
                            all_burns.push(tx.clone());
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count += 1_u128;
                            transaction_value += &value_u128;
                            all_transactions.push(tx.clone());
                        }
                        if tx.tx_type == "Approve" {
                            // Do Nothing at the moment. 
                        }
                        total_value += &value_u128;
                        total_txs += 1_u128;
                    }
                    Err(error) => {
                        is_error = true;
                        error_output = format!("Process Stats Error : {}", error);
                        log(format!("Process Stats Error : {}", error));
                    }
                } // match
            } // if
        } // for
        // volumes per time-chunk
        let mut count_over_time = Vec::new();
        if mode == StatsType::Hourly {
            let chunks_needed = (
                ((time_now - process_from) as f64) / (HOUR_AS_NANOS as f64)
            ).ceil() as u32;
            let nearest_hour = nearest_past_hour(time_now);
            let mut start_chunk: u64 = 0_u64;
            let mut end_chunk: u64;
            let mut tx_count_chunk: u64;
            let mut mint_count_chunk: u64;
            let mut burn_count_chunk: u64;
            let mut transaction_count_chunk: u64;

            for i in 0..chunks_needed {
                if i == 0 {
                    start_chunk = if time_now == nearest_hour {
                        nearest_hour - HOUR_AS_NANOS
                    } else {
                        nearest_hour
                    };
                    end_chunk = time_now;
                } else {
                    end_chunk = start_chunk;
                    start_chunk = start_chunk - HOUR_AS_NANOS;
                }

                // reset
                tx_count_chunk = 0;
                mint_count_chunk = 0;
                burn_count_chunk = 0;
                transaction_count_chunk = 0;

                for tx in array {
                    if tx.tx_time >= start_chunk && tx.tx_time < end_chunk {
                        tx_count_chunk += 1;
                        if tx.tx_type == "Mint" {
                            mint_count_chunk += 1;
                        }
                        if tx.tx_type == "Burn" {
                            burn_count_chunk += 1;
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count_chunk += 1;
                        }
                    }
                    if tx.tx_time > end_chunk {
                        break;
                    }
                }

                let tcs: TimeChunkStats = TimeChunkStats {
                    start_time: start_chunk,
                    end_time: end_chunk,
                    total_count: tx_count_chunk,
                    mint_count: mint_count_chunk,
                    transaction_count: transaction_count_chunk,
                    burn_count: burn_count_chunk,
                };
                count_over_time.push(tcs);
            }
        } else if mode == StatsType::Daily {
            let chunks_needed: u32 = (
                ((time_now - process_from) as f64) / (DAY_AS_NANOS as f64)
            ).ceil() as u32;
            let nearest_day: u64 = nearest_day_start(time_now);
            let mut start_chunk: u64 = 0_u64;
            let mut end_chunk: u64;
            let mut tx_count_chunk: u64;
            let mut mint_count_chunk: u64;
            let mut burn_count_chunk: u64;
            let mut transaction_count_chunk: u64;

            for i in 0..chunks_needed {
                if i == 0 {
                    start_chunk = if time_now == nearest_day {
                        nearest_day - DAY_AS_NANOS
                    } else {
                        nearest_day
                    };
                    end_chunk = time_now;
                } else {
                    end_chunk = start_chunk;
                    start_chunk = start_chunk - DAY_AS_NANOS;
                }

                // reset
                tx_count_chunk = 0;
                mint_count_chunk = 0;
                burn_count_chunk = 0;
                transaction_count_chunk = 0;

                for tx in array {
                    if tx.tx_time >= start_chunk && tx.tx_time < end_chunk {
                        tx_count_chunk += 1;
                        if tx.tx_type == "Mint" {
                            mint_count_chunk += 1;
                        }
                        if tx.tx_type == "Burn" {
                            burn_count_chunk += 1;
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count_chunk += 1;
                        }
                    }
                    if tx.tx_time > end_chunk {
                        break;
                    }
                }

                let tcs: TimeChunkStats = TimeChunkStats {
                    start_time: start_chunk,
                    end_time: end_chunk,
                    total_count: tx_count_chunk,
                    mint_count: mint_count_chunk,
                    transaction_count: transaction_count_chunk,
                    burn_count: burn_count_chunk,
                };
                count_over_time.push(tcs);
            }
        }

        // largest burn/ tx/ transaction
        let ret_len: usize = rts.data.canister_settings.stats_return_length.clone();
        let top_mints: Vec<ProcessedTX> = top_x_by_txvalue(all_mints, ret_len);
        let top_burns: Vec<ProcessedTX> = top_x_by_txvalue(all_burns, ret_len);
        let top_transactions: Vec<ProcessedTX> = top_x_by_txvalue(all_transactions, ret_len);
        let mut unique_accounts: Vec<String> = get_unique_string_values(all_accounts);
        unique_accounts.retain(|s| s != "ICRC_LEDGER.ICRC_LEDGER"); // remove txs made by the ledger
        let mut unique_principals: Vec<String> = get_unique_string_values(all_principals);
        unique_principals.retain(|s| s != "ICRC_LEDGER"); // remove txs made by the ledger
        let ua: &usize = &unique_accounts.len();
        let up: &usize = &unique_principals.len();

        //output struct
        let ret = TimeStats {
            total_transaction_count: total_txs,
            total_transaction_value: total_value,
            total_transaction_average: (total_value as f64) / (total_txs as f64),
            total_unique_accounts: ua.to_owned() as u64,
            total_unique_principals: up.to_owned() as u64,
            most_active_accounts: Vec::new(),
            most_active_principals: Vec::new(),
            burn_stats: TotCntAvg {
                total_value: burn_value,
                count: burn_count,
                average: (burn_value as f64) / (burn_count as f64),
            },
            mint_stats: TotCntAvg {
                total_value: mint_value,
                count: mint_count,
                average: (mint_value as f64) / (mint_count as f64),
            },
            transaction_stats: TotCntAvg {
                total_value: transaction_value,
                count: transaction_count,
                average: (transaction_value as f64) / (transaction_count as f64),
            },
            count_over_time,
            top_mints,
            top_burns,
            top_transactions,
        };

        if is_error == false {
            log("Stats calculation complete");
            return Ok(ret); // closure return
        } else {
            log("Error Calculating stats");
            return Err(error_output);
        }
    });
    return fn_return; //
}

async fn most_active(process_from: u64, return_number: usize) -> bool {
    RUNTIME_STATE.with(|state| {
        let mut rts: std::cell::RefMut<'_, RuntimeState> = state.borrow_mut();
        let array: &VecDeque<ProcessedTX> = &rts.data.retained_blocks;

        let mut from_combined: String;
        let mut to_combined: String;
        let mut most_active_acs: Vec<(String, u64)> = Vec::new();
        let mut most_active_prs: Vec<(String, u64)> = Vec::new();
        let mut all_acs: HashMap<String, u64> = HashMap::new();
        let mut all_prs: HashMap<String, u64> = HashMap::new();

        // process from 
        for tx in array {
            if tx.tx_time >= process_from {
                from_combined = format!("{}.{}", tx.from_principal, tx.from_account);
                
                let a = all_acs.entry(from_combined).or_insert(0);
                *a += 1; // add 1 to count

                let p = all_prs.entry(tx.from_principal.clone()).or_insert(0);
                *p += 1; // add 1 to count
            }
        }

        // process to 
        for tx in array {
            if tx.tx_time >= process_from && tx.tx_type != "Approve" {
                to_combined = format!("{}.{}", tx.to_principal, tx.to_account);
                
                let a = all_acs.entry(to_combined).or_insert(0);
                *a += 1; // add 1 to count

                let p = all_prs.entry(tx.to_principal.clone()).or_insert(0);
                *p += 1; // add 1 to count
            }
        }

        // accounts to vec
        for (ac, value) in &all_acs {
            if ac != "ICRC_LEDGER.ICRC_LEDGER" {
                most_active_acs.push((ac.to_owned(), value.to_owned()));
            }
        }

        // principals to vec
        for (pr, value) in &all_prs {
            if pr != "ICRC_LEDGER" {
                most_active_prs.push((pr.to_owned(), value.to_owned()));
            }
        }

        // most active accounts
        let top_active_acs: Vec<(String, u64)> = top_x_txcount(most_active_acs, return_number);
        let top_active_prs: Vec<(String, u64)> = top_x_txcount(most_active_prs, return_number);

        // update
        rts.data.hourly_stats.data.most_active_accounts = top_active_acs;
        rts.data.hourly_stats.data.most_active_principals = top_active_prs;

        log("Most Active Accounts + Principals Updated");
        return true;
    });
    return true;
}


//[][] ------------------------- [][]
//[][] ---- Timer Functions ---- [][]
//[][] ------------------------- [][]
fn start_processing_timer(secs: u64) {
    let secs: Duration = Duration::from_secs(secs);
    let timer_id: TimerId = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_IDS.with(|timer_ids: &RefCell<Vec<TimerId>>| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_data_processing() {
    let ws: WorkingStats = with_runtime(|rts: &RuntimeState| { rts.data.working_stats.to_owned() });
    if ws.is_busy == true {
        return;
    } else {
        with_runtime_mut(|rts: &mut RuntimeState| {
            rts.data.working_stats.is_busy = true;
        });
        if ws.task_id == 0 {
            fetch_data().await;
        } else if ws.is_upto_date == true && ws.task_id == 1 {
            log("[][] --- Calculating Hourly Stats --- [][]");
            let hs = calc_hourly_stats().await;
            if hs == true {
                with_runtime_mut(|rts: &mut RuntimeState| {
                    let ws: &mut WorkingStats = rts.data.working_stats.borrow_mut();
                    ws.task_id = 2;
                    ws.hr_stats_complete_to = ws.tx_completed_to;
                });
            }
        } else if ws.is_upto_date == true && ws.task_id == 2 {
            log("[][] --- Calculating Daily Stats --- [][]");
            let ds = calc_daily_stats().await;
            if ds == true {
                with_runtime_mut(|rts: &mut RuntimeState| {
                    let ws: &mut WorkingStats = rts.data.working_stats.borrow_mut();
                    ws.task_id = 3;
                    ws.day_stats_complete_to = ws.tx_completed_to;
                });
            }
        } else if ws.is_upto_date == true && ws.task_id == 3 {
            log("[][] --- Calculating Most Acitve (24hr) --- [][]");
            let ma: bool = calc_most_active().await;
            if ma == true {
                with_runtime_mut(|rts: &mut RuntimeState| {
                    rts.data.working_stats.task_id = 0;
                });
            }
        }
    }
}

// [][] ------------------------------ [][]
// [][] --- Canister Metrics/ Logs --- [][]
// [][] ------------------------------ [][]
#[query]
fn get_cycles_balance() -> u64 {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.data.are_stats_public() {
            s.data.check_authorised(ic_cdk::caller().to_text())
        }
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

#[query]
fn read_logs() -> Option<Vec<LogEntry>> {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.data.check_authorised(ic_cdk::caller().to_text());
    });

    let mut ret: Option<Vec<LogEntry>> = None;
    LOGS_STATE.with(|state: &RefCell<LogsState>| {
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
    LOGS_STATE.with(|state: &RefCell<LogsState>| {
        let logs: &mut Vec<LogEntry> = &mut state.borrow_mut().data.canister_logs;
        let nano_time: u64 = ic_cdk::api::time();
        let log_entry: LogEntry = LogEntry {
            timestamp: nano_time.to_string(),
            text: text.as_ref().to_string(),
        };
        logs.push(log_entry);

        // remove any logs over max length
        let len = logs.len();
        if len > MAX_LOG_LENGTH {
            let excess: usize = len - MAX_LOG_LENGTH;
            logs.drain(0..excess);
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nearest_past_hour() {
        //  Input already on an hour boundary
        let time_ms_1: u64 = 1687856400000000000; // 27/06/23 0900 gmt
        assert_eq!(nearest_past_hour(time_ms_1), time_ms_1);

        //  Input at half past hour
        let time_ms_2: u64 = 1687858200000000000; // 09:30:00 gmt
        assert_eq!(nearest_past_hour(time_ms_2), 1687856400000000000); // 27/06/23 0900 gmt

        // 1 nano before the hour
        let time_ms_3: u64 = 1687856399999999999;
        assert_eq!(nearest_past_hour(time_ms_3), 1687852800000000000); // 27/06/23 0800 gmt

        //  1 nano before end of the hour
        let time_ms_4: u64 = 1687859999999999999;
        assert_eq!(nearest_past_hour(time_ms_4), 1687856400000000000); // 27/06/23 0900 gmt
    }

    #[test]
    fn test_nearest_past_day() {
        //  Input already on an day boundary
        let time_ms_1: u64 = 1687824000000000000; // 27/06/23 0000 gmt
        assert_eq!(nearest_day_start(time_ms_1), time_ms_1);

        //  Input at mid-day (12:00)
        let time_ms_2: u64 = 1687867200000000000; 
        assert_eq!(nearest_day_start(time_ms_2), time_ms_1); // 27/06/23 0000 gmt

        // 1 nano after midnight
        let time_ms_3: u64 = 1687824000000000001;
        assert_eq!(nearest_day_start(time_ms_3), time_ms_1); // 27/06/23 0000 gmt

        //  1 nano before next day 
        let time_ms_4: u64 = 1687910399999999999;
        assert_eq!(nearest_day_start(time_ms_4), time_ms_1); // 27/06/23 0000 gmt
    }

    use crate::test_data::{ 
        test_data, 
        test_TX_vec, 
        test_TX_count_vec, 
        test_calculate_time_stats, 
        test_most_active, 
    };

    use crate::utils::get_unique_string_values;

    #[test]
    fn text_unique_string_values(){
        let mut data: Vec<String> = Vec::new();
        data.push(String::from("aaaa"));
        data.push(String::from("aaaa"));
        data.push(String::from("zzzz"));
        data.push(String::from("abaa"));
        data.push(String::from("acaa"));
        data.push(String::from("aaaa"));
        data.push(String::from("cccc"));
        data.push(String::from("bbbb"));
        data.push(String::from("cccc"));

        pub fn contains_string(vec: &Vec<String>, query: &str) -> bool {
            vec.iter().any(|s| s == query)
        }

        let unique: Vec<String> = get_unique_string_values(data);
        assert_eq!(unique.len(), 6);
        assert!(contains_string(&unique, "aaaa"));
        assert!(contains_string(&unique, "zzzz"));
        assert!(contains_string(&unique, "abaa"));
        assert!(contains_string(&unique, "acaa"));
        assert!(contains_string(&unique, "cccc"));
        assert!(contains_string(&unique, "bbbb"));
        println!("{:?}", unique);
    }

    #[test]
    fn test_top_tx_value() {
        let data: Vec<ProcessedTX> = test_TX_vec();
        let results: Vec<ProcessedTX> = top_x_by_txvalue(data,4);
        assert_eq!(results[0].tx_value, 799_123_123);
        assert_eq!(results[1].tx_value, 500_123_321);
        assert_eq!(results[2].tx_value, 10_000_000);
        assert_eq!(results[3].tx_value, 999_999);
    }
    
    #[test]
    fn text_top_tx_count() {
        let data: Vec<(String, u64)> = test_TX_count_vec();
        let results = top_x_txcount(data, 3);
        assert_eq!(results[0].1, 123_999_999);
        assert_eq!(results[1].1, 999_999);
        assert_eq!(results[2].1, 999);
    }

    struct TestData {
        principal_holders: BTreeMap<String, EntityData>,
        account_holders: BTreeMap<String, EntityData>,
    }

    #[test]
    fn test_update_balances() {
        let tx_array = test_data();
        //if tx_array.len() == 0 {return true}; removed for tests

        let mut data = TestData {
            principal_holders: BTreeMap::new(),
            account_holders: BTreeMap::new(),
        };

        //let data = rts.data.borrow_mut(); removed for tests
        let tx_fee = 10_000_u128; // change to 10_000 for tests!
        let mut from_combined_account: String;
        let mut to_combined_account: String;
        let mut processed_ok = true;

    for tx in tx_array {
        let tx_value_u128 = tx.tx_value.0.to_u128().ok_or("Tip of Chain is not a valid u128");
        match tx_value_u128 {
            Ok(tx_value_u128) => {
                match tx.tx_type.as_str() {
                    "Transaction" => {
                        // ----- DEBIT FROM
                        // ----- account balance
                        from_combined_account = format!(
                            "{}.{}",
                            &tx.from_principal,
                            &tx.from_account
                        );
                        if let Some(ac) = data.account_holders.get(&from_combined_account) {
                            let tot_deduction;
                            if ac.balance < tx_value_u128 + tx_fee {
                                tot_deduction = ac.balance; // catch overflows. cant spend more than ac balance.
                                log(
                                    format!(
                                        "Caught overflow from transfer. Account: {}",
                                        &from_combined_account
                                    )
                                );
                            } else {
                                tot_deduction = tx_value_u128 + tx_fee;
                            }
                            // existing account
                            let ent = EntityData {
                                balance: ac.balance - tot_deduction,
                                transactions: ac.transactions + 1_u64,
                            };
                            data.account_holders.insert(from_combined_account.clone(), ent);
                        } else {
                            log(
                                format!(
                                    "Error: Sent transaction from new unknown account {}",
                                    &from_combined_account
                                )
                            );
                            processed_ok = false;
                        }

                        // principal balance
                        if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                            let tot_deduction;
                            if pr.balance < tx_value_u128 + tx_fee {
                                tot_deduction = pr.balance; // catch overflows. cant spend more than ac balance.
                                log(
                                    format!(
                                        "Caught overflow from transfer. Principal: {}",
                                        &tx.from_principal
                                    )
                                );
                            } else {
                                tot_deduction = tx_value_u128 + tx_fee;
                            }
                            // existing account
                            let ent = EntityData {
                                balance: pr.balance - tot_deduction,
                                transactions: pr.transactions + 1_u64,
                            };
                            data.principal_holders.insert(tx.from_principal.clone(), ent);
                        } else {
                            log(
                                format!(
                                    "Error: Sent transaction from new unknown principal {}",
                                    &tx.from_principal
                                )
                            );
                            processed_ok = false;
                        }

                        // ----- PAYMENT TO
                        // ----- account balance
                        to_combined_account = format!(
                            "{}.{}",
                            &tx.to_principal,
                            &tx.to_account
                        );
                        if let Some(ac) = data.account_holders.get(&to_combined_account) {
                            // existing account
                            let ent = EntityData {
                                balance: ac.balance + tx_value_u128,
                                transactions: ac.transactions + 1_u64,
                            };
                            data.account_holders.insert(to_combined_account.clone(), ent);
                        } else {
                            // new account
                            let ent = EntityData {
                                balance: tx_value_u128,
                                transactions: 1_u64,
                            };
                            data.account_holders.insert(to_combined_account, ent);
                        }

                        // principal balance
                        if let Some(pr) = data.principal_holders.get(&tx.to_principal) {
                            // existing account
                            let ent = EntityData {
                                balance: pr.balance + tx_value_u128,
                                transactions: pr.transactions + 1_u64,
                            };
                            data.principal_holders.insert(tx.to_principal.clone(), ent);
                        } else {
                            // new account
                            let ent = EntityData {
                                balance: tx_value_u128,
                                transactions: 1_u64,
                            };
                            data.principal_holders.insert(tx.to_principal.clone(), ent);
                        }
                    }
                    "Mint" => {
                        // account balance
                        to_combined_account = format!(
                            "{}.{}",
                            &tx.to_principal,
                            &tx.to_account
                        );
                        if let Some(ac) = data.account_holders.get(&to_combined_account) {
                            // existing account
                            let ent = EntityData {
                                balance: ac.balance + tx_value_u128, // Nat
                                transactions: ac.transactions + 1_u64,
                            };
                            data.account_holders.insert(to_combined_account.clone(), ent);
                        } else {
                            // new account
                            let ent = EntityData {
                                balance: tx_value_u128,
                                transactions: 1_u64,
                            };
                            data.account_holders.insert(to_combined_account, ent);
                        }
                        // principal balance
                        if let Some(pr) = data.principal_holders.get(&tx.to_principal) {
                            // existing principal
                            let ent = EntityData {
                                balance: pr.balance + tx_value_u128, // Nat
                                transactions: pr.transactions + 1_u64,
                            };
                            data.principal_holders.insert(tx.to_principal.clone(), ent);
                        } else {
                            // new principal
                            let ent = EntityData {
                                balance: tx_value_u128,
                                transactions: 1_u64,
                            };
                            data.principal_holders.insert(tx.to_principal.clone(), ent);
                        }
                    }
                    "Burn" => {
                        // account balance
                        from_combined_account = format!(
                            "{}.{}",
                            &tx.from_principal,
                            &tx.from_account
                        );
                        if let Some(ac) = data.account_holders.get(&from_combined_account) {
                            // existing account
                            let ent = EntityData {
                                balance: ac.balance - tx_value_u128, // Nat
                                transactions: ac.transactions + 1_u64,
                            };
                            data.account_holders.insert(from_combined_account.clone(), ent);
                        } else {
                            log(
                                format!("Error: Burn transaction from new account {}", from_combined_account)
                            );
                        }
                        // principal balance
                        if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                            // existing principal
                            let ent = EntityData {
                                balance: pr.balance - tx_value_u128, // Nat
                                transactions: pr.transactions + 1_u64,
                            };
                            data.principal_holders.insert(tx.from_principal.clone(), ent);
                        } else {
                            log(
                                format!(
                                    "Error: Burn transaction from new principal {}",
                                    tx.from_principal
                                )
                            );
                            processed_ok = false;
                        }
                    }
                    "Approve" => {
                        // ----- DEBIT FEE ONLY (FROM)
                        // ----- account balance
                        from_combined_account = format!(
                            "{}.{}",
                            &tx.from_principal,
                            &tx.from_account
                        );
                        if let Some(ac) = data.account_holders.get(&from_combined_account) {
                            let tot_deduction;
                            if ac.balance < tx_fee {
                                tot_deduction = ac.balance; // catch overflows. cant spend more than ac balance.
                                log(
                                    format!(
                                        "Caught overflow from Approve Fee deduction. Account: {}",
                                        &from_combined_account
                                    )
                                );
                            } else {
                                tot_deduction = tx_fee;
                            }
                            // existing account
                            let ent = EntityData {
                                balance: ac.balance - tot_deduction,
                                transactions: ac.transactions + 1_u64,
                            };
                            data.account_holders.insert(from_combined_account.clone(), ent);
                        } else {
                            log(
                                format!(
                                    "Error: Approve Transaction from new unknown account {}",
                                    &from_combined_account
                                )
                            );
                            processed_ok = false;
                        }

                        // principal balance
                        if let Some(pr) = data.principal_holders.get(&tx.from_principal) {
                            let tot_deduction;
                            if pr.balance < tx_fee {
                                tot_deduction = pr.balance; // catch overflows. cant spend more than ac balance.
                                log(
                                    format!(
                                        "Caught overflow from approve. Principal: {}",
                                        &tx.from_principal
                                    )
                                );
                            } else {
                                tot_deduction = tx_fee;
                            }
                            // existing account
                            let ent = EntityData {
                                balance: pr.balance - tot_deduction,
                                transactions: pr.transactions + 1_u64,
                            };
                            data.principal_holders.insert(tx.from_principal.clone(), ent);
                        } else {
                            log(
                                format!(
                                    "Error: Approve transaction from new unknown principal {}",
                                    &tx.from_principal
                                )
                            );
                            processed_ok = false;
                        }
                    }
                    _ => {
                        log(
                            "Could not process transaction, type is not Mint, Burn, Approve or Transaction"
                        );
                        processed_ok = false;
                    }
                }
            }
            Err(error) => {
                log(
                    format!("Could not get tx_value_u128 from tx.tx_value (Nat). Error: {}", error)
                );
                processed_ok = false;
            }
        }
    }

        let key =
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let test1 = data.account_holders
            .get(&key)
            .unwrap_or(&(EntityData { transactions: 0, balance: 0 }));
        let res = 269_520_001;
        assert_eq!(test1.balance, res);

        let key =
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001".to_string();
        let test1 = data.account_holders
            .get(&key)
            .unwrap_or(&(EntityData { transactions: 0, balance: 0 }));
        let res = 890_000;
        assert_eq!(test1.balance, res);

        let key =
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000002".to_string();
        let test1 = data.account_holders
            .get(&key)
            .unwrap_or(&(EntityData { transactions: 0, balance: 0 }));
        let res = 20_000_000;
        assert_eq!(test1.balance, res);

        let key =
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000003".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 229_500_000;
        assert_eq!(test1.balance, res);

        let key =
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000004".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 0;
        assert_eq!(test1.balance, res);

        let key =
            "3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 479_999;
        assert_eq!(test1.balance, res);

        let key =
            "3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000001".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 589_990_000;
        assert_eq!(test1.balance, res);

        let key =
            "3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000002".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 10_000_000;
        assert_eq!(test1.balance, res);

        let key =
            "3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000002".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 10_000_000;
        assert_eq!(test1.balance, res);

        let key =
            "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 30_000_000;
        assert_eq!(test1.balance, res);

        let key =
            "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000001".to_string();
        let test1 = data.account_holders.get(&key).unwrap();
        let res = 0;
        assert_eq!(test1.balance, res);
    }

     #[test]
     fn test_time_stats(){
        let stats_result = test_calculate_time_stats(
                        1_687_921_200_000_000_000,
                        StatsType::Hourly,
                        1_687_939_200_000_000_000).unwrap();
        // total txs
        assert_eq!(stats_result.total_transaction_count, 10);

        // total_transaction_value
        assert_eq!(stats_result.total_transaction_value, 840_000_001);

        // total average
        assert_eq!(stats_result.total_transaction_average, 84_000_000.1);
        
        // burn_stats
        assert_eq!(stats_result.burn_stats.count, 2);
        assert_eq!(stats_result.burn_stats.total_value, 79_500_000);
        assert_eq!(stats_result.burn_stats.average, 39_750_000.0);

        // mint_stats
        assert_eq!(stats_result.mint_stats.count, 2);
        assert_eq!(stats_result.mint_stats.total_value, 180_010_000);
        assert_eq!(stats_result.mint_stats.average, 90_005_000.0);

        // transaction_stats
        assert_eq!(stats_result.transaction_stats.count, 6);
        assert_eq!(stats_result.transaction_stats.total_value, 580_490_001);
        assert_eq!(stats_result.transaction_stats.average, 96_748_333.5);

        // count_over_time
        assert_eq!(stats_result.count_over_time.len(), 5);
        assert_eq!(stats_result.count_over_time[0].total_count, 5);
        assert_eq!(stats_result.count_over_time[1].total_count, 2);
        assert_eq!(stats_result.count_over_time[2].total_count, 2);
        assert_eq!(stats_result.count_over_time[3].total_count, 0);
        assert_eq!(stats_result.count_over_time[4].total_count, 1);

        // top txs
        assert_eq!(stats_result.top_mints[0].tx_value, 100_000_000);
        assert_eq!(stats_result.top_burns[0].tx_value, 79_000_000);
        assert_eq!(stats_result.top_transactions[0].tx_value, 500_000_000);
     }

     #[test]
     fn test_most_active_stats(){
        // Tuple == (Accounts), (Principals)
        let data: (Vec<(String, u64)>, Vec<(String, u64)>) = 
            test_most_active(0, 99);
        // Most Active Account - 10 hits
        assert_eq!(
            data.0[0].0, 
            "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000"
        );
        assert_eq!(
            data.0[0].1, 
            10
        );

        // 7th place is unique with 2 hits
        assert_eq!(
            data.0[6].0, 
            "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000001"
        );
        assert_eq!(
            data.0[6].1, 
            2
        );

        // Most Active Principals
        assert_eq!(
            data.1[0].0, 
            "2vxsx-fae"
        );
        assert_eq!(
            data.1[0].1, 
            22
        );
        assert_eq!(
            data.1[1].0, 
            "3xwpq-ziaaa-aaaah-qcn4a-cai"
        );
        assert_eq!(
            data.1[1].1, 
            8
        );
        assert_eq!(
            data.1[2].0, 
            "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe"
        );
        assert_eq!(
            data.1[2].1, 
            3
        );
     }


}
