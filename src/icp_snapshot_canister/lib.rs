#[allow(non_snake_case)]
mod types;
mod utils;
mod constants;
mod memory;

use ic_stable_structures::{ writer::Writer, Memory as _ };
use candid::{ CandidType, Principal };
use ic_cdk_macros::*;
use serde::{ Deserialize, Serialize };
use std::cell::RefCell;
use std::collections::BTreeMap;
use std::time::Duration;
use ic_cdk_timers::TimerId;
use utils::{ nearest_day_start, string_to_key };
use constants::{ MAX_LOGS, DAY_AS_NANOS, MAX_RETAINED_LOGS };

use types::{
    MemoryData, // ignore unsued inport warning!
    WorkingStats,
    LogEntry,
    CanisterSettings,
    SnapshotData,
    IDKey,
    RetSaorsaStatsICP,
    QuickStats,
    TimeStats,
    TotalHoldersResponse,
    KEY_LENGTH, ExchangeCollection,
};

//[][] ---- State Manamgement ---- [][]
thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
    static LOGS_STATE: RefCell<LogsState> = RefCell::default();
}

#[derive(Serialize, Deserialize)]
struct RuntimeState {
    pub mgmt_data: Data,
    pub icp_standard_collections: BTreeMap<IDKey, SnapshotData>,
    pub icp_quickstats_collections: BTreeMap<IDKey, QuickStats>,
    pub icp_exchange_stats: Option<Vec<ExchangeCollection>>,
}
impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            mgmt_data: Data::default(),
            icp_standard_collections: BTreeMap::default(),
            icp_quickstats_collections: BTreeMap::default(),
            icp_exchange_stats: Some(Vec::new()),
        }
    }
}

// [][] --- Main Data Struct --- [][]
#[derive(CandidType, Deserialize, Serialize, Default)]
struct Data {
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    timer_active: bool,
    processing_data: bool,
    first_run: bool,
    working_stats: WorkingStats,
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
    // itit main state
    let mut mgmt_data = Data::default();
    let icp_standard_collections = BTreeMap::default();
    let icp_quickstats_collections = BTreeMap::default();
    let icp_exchange_stats = Some(Vec::new());
    // add state
    mgmt_data.authorised.push(
        "ADMIN_PRINCIPAL_HERE".to_string()
    ); 
    mgmt_data.authorised.push(
        "FRONTEND_PRINCIPAL_HERE".to_string()
    ); 
    mgmt_data.canister_settings.stats_are_public = true;
    mgmt_data.canister_settings.canister_name = "Name me please!".to_string();
    mgmt_data.first_run = true;
    let runtime_state = RuntimeState {
        mgmt_data,
        icp_standard_collections,
        icp_quickstats_collections,
        icp_exchange_stats
    };
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
    // Serialize the state.
    // This example is using CBOR, but you can use any data format you like.
    let mut state_bytes = vec![];
    RUNTIME_STATE.with(|s| ciborium::ser::into_writer(&*s.borrow(), &mut state_bytes)).expect(
        "failed to encode state"
    );

    // Write the length of the serialized bytes to memory, followed by the
    // by the bytes themselves.
    let len = state_bytes.len() as u32;
    let mut memory = memory::get_upgrades_memory();
    let mut writer = Writer::new(&mut memory, 0);
    writer.write(&len.to_le_bytes()).unwrap();
    writer.write(&state_bytes).unwrap()
}

#[post_upgrade]
fn post_upgrade() {
    let memory = memory::get_upgrades_memory();

    // Read the length of the state bytes.
    let mut state_len_bytes = [0; 4];
    memory.read(0, &mut state_len_bytes);
    let state_len = u32::from_le_bytes(state_len_bytes) as usize;

    // Read the bytes
    let mut state_bytes = vec![0; state_len];
    memory.read(4, &mut state_bytes);

    // Deserialize and set the state.
    let state = ciborium::de::from_reader(&*state_bytes).expect("failed to decode state");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = state;
    });
}

//[][] ---- Query/ Update Functions ---- [][]

#[update]
fn add_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        s.mgmt_data.add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        s.mgmt_data.remove_authorised(principal_id)
    })
}

#[query]
fn get_all_authorised() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        s.mgmt_data.get_all_authorised()
    })
}

#[update]
fn set_canister_name(name: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        s.mgmt_data.set_canister_name(name)
    })
}

#[update]
fn add_collection(key: String, canister_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let res;
        let mut s = state.borrow_mut();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        let key2 = key.to_owned();
        let k: IDKey = string_to_key(key);
        if let Some(_data) = s.icp_standard_collections.get(&k) {
            res = "Key already in use for a collection!".to_string();
            return res;
        } else {
            let sd: SnapshotData = SnapshotData::new(canister_id);
            s.icp_standard_collections.insert(k, sd);
            s.mgmt_data.working_stats.data_collections.push(key2);
            res = "Collection Added".to_string();
            return res;
        }
    })
}

#[update]
fn stop_all_timers() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
    });

    TIMER_IDS.with(|timer_ids: &RefCell<Vec<TimerId>>| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });
    log("[][] ---- Processing timer stopped ---- [][]");
    return String::from("Processing timer stopped");
}

#[update]
async fn check_and_start_processing_timer() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
    });
    set_midnight_timer().await;
    log("[][] ---- Processing timer started ---- [][]");
    return "Timer Started".to_string();
}

#[query]
fn get_working_stats() -> WorkingStats {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state| {
        let st = state.borrow();
        return st.mgmt_data.working_stats.to_owned();
    })
}

#[query]
fn get_canister_name() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());
        s.mgmt_data.get_canister_name()
    })
}

#[query]
fn get_cycles_balance() -> u64 {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text())
    });
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
fn get_logs() -> Option<Vec<LogEntry>> {
    // Is authorised?
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text())
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

#[query]
fn get_standard_snapshots(
    collection_id: String,
    max_to_return: u128
) -> Option<Vec<RetSaorsaStatsICP>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());

        let k: IDKey = string_to_key(collection_id);
        if let Some(data) = s.icp_standard_collections.get(&k) {
            let data_vec: Vec<RetSaorsaStatsICP> = data.retained_data.to_owned();
            let ret_data: Vec<RetSaorsaStatsICP> = data_vec
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();
            return Some(ret_data);
        } else {
            return None;
        }
    })
}

#[query]
fn get_exchange_snapshots( max_to_return: u32 ) -> Option<Vec<ExchangeCollection>> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());

        match s.icp_exchange_stats.clone() {
            Some(v) => {
                let ret_data: Vec<ExchangeCollection> = v
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();
                return Some(ret_data);
            },
            None => {
                return None;
            },

        }
    })
}

#[query]
fn get_quickstats(collection_id: String, max_to_return: u128) -> Option<QuickStats> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text());

        let k: IDKey = string_to_key(collection_id);
        if let Some(data) = s.icp_quickstats_collections.get(&k) {
            log(format!("DATA:: {:?}", data));

            let s1: Vec<(u128, u64)> = data.total_transaction_count
                .to_owned()
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();

            let s2: Vec<(u128, u64)> = data.total_transaction_value
                .to_owned()
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();

            let s3: Vec<(u128, u64)> = data.total_unique_accounts
                .to_owned()
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();

            let s4: Vec<(u128, u64)> = data.total_account_holders
                .to_owned()
                .iter()
                .rev()
                .take(max_to_return as usize)
                .cloned()
                .collect();

            let ret = QuickStats {
                total_transaction_count: s1,
                total_transaction_value: s2,
                total_unique_accounts: s3,
                total_account_holders: s4,
            };
            Some(ret)
        } else {
            return None;
        }
    })
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    // Is authorised?
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.mgmt_data.check_authorised(ic_cdk::caller().to_text())
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

// [][] --- functions --- [][]
async fn set_midnight_timer() {
    let time_now: u64 = ic_cdk::api::time();
    let since_midnight: u64 = time_now - nearest_day_start(time_now);
    let till_next_midnight: u64 = DAY_AS_NANOS - since_midnight;
    let nanos: Duration = Duration::from_nanos(till_next_midnight);
    let timer_id: TimerId = ic_cdk_timers::set_timer_interval(nanos, move ||
        ic_cdk::spawn(take_snapshots(time_now))
    );
    TIMER_IDS.with(|timer_ids: &RefCell<Vec<TimerId>>| timer_ids.borrow_mut().push(timer_id));
    log(
        format!(
            "[][] -- TIMER STARTED -- [][] Time Now:: {}, Till Midnight:: {}",
            time_now,
            till_next_midnight
        )
    );
}

async fn take_snapshots(time_nano: u64) {
    // process each collection
    let collection_canisters = RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        let collections: BTreeMap<
            [u8; KEY_LENGTH],
            SnapshotData
        > = s.icp_standard_collections.to_owned();
        let mut canisters: Vec<(String, String, IDKey)> = Vec::new();
        for (key, value) in collections {
            canisters.push((String::from_utf8_lossy(&key).to_string(), value.canister_id, key));
        }
        return canisters;
    });

    for id in collection_canisters {
        let mut full_stats: RetSaorsaStatsICP = RetSaorsaStatsICP::default();
        let ledger_id: Result<Principal, candid::types::principal::PrincipalError> = Principal::from_text(id.1);
        match ledger_id {
            Ok(pr_id) => {
  
                // GET TIME STATS (MAIN)
                let res: Result<
                    (TimeStats,),
                    (ic_cdk::api::call::RejectionCode, String)
                > = ic_cdk::call(pr_id, "get_hourly_stats", ()).await;
                match res {
                    Ok(v) => {
                        full_stats.snapshot_time = time_nano;
                        full_stats.total_transaction_count = v.0.total_transaction_count;
                        full_stats.total_transaction_value = v.0.total_transaction_value;
                        full_stats.total_transaction_average = v.0.total_transaction_average;
                        full_stats.total_unique_accounts = v.0.total_unique_accounts;
                        full_stats.most_active_accounts = v.0.most_active_accounts;
                        full_stats.burn_stats = v.0.burn_stats;
                        full_stats.mint_stats = v.0.mint_stats;
                        full_stats.transaction_stats = v.0.transaction_stats;  // Typo not fixed in deployed ICP stats canister
                        full_stats.count_over_time = v.0.count_over_time;
                        full_stats.top_mints = v.0.top_mints;
                        full_stats.top_burns = v.0.top_burns;
                        full_stats.top_transactions = v.0.top_transactions;
                    }
                    Err(error) => {
                        log(format!("Error getting data from ICP Stats Canister. {}", error.1));
                    }
                }

                //GET TOTAL HOLDERS
                let res: Result<
                    (TotalHoldersResponse,),
                    (ic_cdk::api::call::RejectionCode, String)
                > = ic_cdk::call(pr_id, "get_total_holders", ()).await;
                match res {
                    Ok(v) => {
                        full_stats.total_account_holders = v.0.accounts;
                    }
                    Err(error) => {
                        log(format!("Error getting data from ICP Stats Canister. {}", error.1));
                    }
                }

                // Populate Quick Stats
                RUNTIME_STATE.with(|state| {
                    let mut rs: std::cell::RefMut<'_, RuntimeState> = state.borrow_mut();
                    if let Some(qs) = rs.icp_quickstats_collections.get_mut(&id.2) {
                        qs.total_transaction_count.push((
                            full_stats.total_transaction_count,
                            time_nano,
                        ));
                        qs.total_transaction_value.push((
                            full_stats.total_transaction_value,
                            time_nano,
                        ));
                        qs.total_unique_accounts.push((
                            full_stats.total_unique_accounts as u128,
                            time_nano,
                        ));
                        qs.total_account_holders.push((
                            full_stats.total_account_holders as u128,
                            time_nano,
                        ));
                    } else {
                        let mut qs: QuickStats = QuickStats::new();
                        qs.total_transaction_count.push((
                            full_stats.total_transaction_count,
                            time_nano,
                        ));
                        qs.total_transaction_value.push((
                            full_stats.total_transaction_value,
                            time_nano,
                        ));
                        qs.total_unique_accounts.push((
                            full_stats.total_unique_accounts as u128,
                            time_nano,
                        ));
                        qs.total_account_holders.push((
                            full_stats.total_account_holders as u128,
                            time_nano,
                        ));
                        rs.icp_quickstats_collections.insert(id.2, qs);
                    }
                });

                // update last processed time
                RUNTIME_STATE.with(|state| {
                    let mut rs: std::cell::RefMut<'_, RuntimeState> = state.borrow_mut();
                    rs.mgmt_data.working_stats.last_processed_time = time_nano;
                });
            }
            Err(error) => {
                log(format!("Error - Cannot convert input canister ID to Principal: {}", error));
            }
        }

        RUNTIME_STATE.with(|state| {
            let mut s = state.borrow_mut();
            if let Some(v) = s.icp_standard_collections.get_mut(&id.2) {
                v.retained_data.push(full_stats);
                let len = v.retained_data.len();
                if len > MAX_RETAINED_LOGS {
                    let excess: usize = len - MAX_RETAINED_LOGS;
                    v.retained_data.drain(0..excess);
                }
            }
        });
        // update Map
        log(format!("Collection {} updated", id.0));
    }

    // Get Exchange Data
    let ex_id: Result<Principal, candid::types::principal::PrincipalError> = Principal::from_text("gnfso-uqaaa-aaaak-qclzq-cai");
    match ex_id {
        Ok(pr_id) => {
            let res: Result<
                (ExchangeCollection,),
                (ic_cdk::api::call::RejectionCode, String)
            > = ic_cdk::call(pr_id, "get_exchange_data", ()).await;
            match res {
                Ok(v) => {
                    log(format!("Got exchange data : {:?}", v.clone()));
                    let data = RUNTIME_STATE.with(|s|{
                        s.borrow().icp_exchange_stats.clone()
                    });
                    match data {
                        Some(d1) => {
                            let mut combined = d1;
                            combined.push(v.0);
                            RUNTIME_STATE.with(|s|{
                                s.borrow_mut().icp_exchange_stats = Some(combined)
                            });
                        },
                        None => {
                            RUNTIME_STATE.with(|s|{
                                let mut x = Vec::new();
                                x.push(v.0);
                                s.borrow_mut().icp_exchange_stats = Some(x)
                            });
                        },
                    }
                }
                Err(error) => {
                    log(format!("Error getting exchange stats from Tracking Canister. {}", error.1));
                }
            }
        },
        Err(error) => {
            log(format!("Error - Cannot convert input canister ID to Principal (exchange data): {}", error));
        },
    }

    // Clear all timers + Sent new timer for next midnight
    TIMER_IDS.with(|timer_ids: &RefCell<Vec<TimerId>>| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });
    set_midnight_timer().await;
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
        if len > MAX_LOGS {
            let excess: usize = len - MAX_LOGS;
            logs.drain(0..excess);
        }
    });
}
