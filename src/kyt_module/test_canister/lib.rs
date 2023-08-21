#[allow(non_snake_case)]
mod types;
mod utils;
mod constants;
mod memory;

use ic_stable_structures::{ writer::Writer, Memory as _ }; // 
use candid::{ CandidType, Nat, Principal };
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use std::borrow::{ BorrowMut };
use std::cell::RefCell;
use std::collections::{ BTreeMap, VecDeque, HashMap };
use std::ops::Deref;
use std::time::Duration;
use utils::{
    string_to_key,
    create_all_keys, 
    create_master_tree
};
use ic_cdk_timers::TimerId;
use ic_cdk::api::instruction_counter;
use num_traits::cast::ToPrimitive;

use constants::{
    MAX_TRANSACTION_BATCH_SIZE,
    MAX_TOTAL_DOWNLOAD,
    MAX_BLOCKS_RETAINED,
};

use types::{
    MemoryData, // ignore unsued inport warning!
    WorkingStats,
    TransactionType,
    EntityData,
    LogEntry,
    CanisterSettings,
    ProcessedTX,
    TotalHoldersResponse,
    TopHoldersResponse,
    HolderBalance,
    QueryBlocksResponse,
    GetTransactionsRequest,
    OperationEnum,
    ArchivedBlocksRange,
    GetBlocksResult,
    IDKey,
    KeyMap
};

//[][] ---- State Manamgement ---- [][]
thread_local! {
    static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    static TIMER_IDS: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
    static LOGS_STATE: RefCell<LogsState> = RefCell::default();
}

#[derive(CandidType, Deserialize, Serialize)]
struct RuntimeState {
    pub all_data: Data,
}
impl Default for RuntimeState {
    fn default() -> Self {
        RuntimeState {
            all_data: Data::default(),
        }
    }
}
#[derive(CandidType, Deserialize, Serialize)]
struct Directory {
    directory: KeyMap, // starts at 1. 
}
impl Default for Directory {
    fn default() -> Self {
        Directory { directory: KeyMap { map: Vec::new() } }
    }
}
impl Directory {
    fn lookup_directory(&mut self, id_key: IDKey) -> Option<usize> {
        for (k, v) in &self.directory.map {
            if *k == id_key {
                return Some(*v);
            }
        }
        None
    }
}


#[derive(CandidType, Deserialize, Serialize, Default)]
struct BlockHolder {
    blocks: VecDeque<ProcessedTX>,
    tip: Nat,
}
impl BlockHolder {
    const MAX_SIZE: usize = MAX_BLOCKS_RETAINED;

    fn create() -> Self{
        Self {
            blocks: VecDeque::with_capacity(Self::MAX_SIZE),
            tip: Nat::from(0),
        }
    }

    fn push_tx(&mut self, tx: ProcessedTX) {
        if self.blocks.len() ==  Self::MAX_SIZE {
            self.blocks.pop_back();
        }
        self.blocks.push_front(tx);
    }
}


// [][] --- Main Data Struct --- [][]
#[derive(CandidType, Deserialize, Serialize, Default)]
struct Data {
    authorised: Vec<String>,
    canister_logs: Vec<LogEntry>,
    timer_active: bool,
    processing_data: bool,
    total_holders: u128,
    first_run: bool,
    working_stats: WorkingStats,
    canister_settings: CanisterSettings,
    tree_master: Vec<BTreeMap<String, EntityData>>, 
    account_directory: Directory,
    latest_blocks: BlockHolder,
    awaiting_indexing: Vec<ProcessedTX>,
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

#[derive(CandidType, Deserialize, Serialize)]
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
#[derive(CandidType, Deserialize, Serialize, Default)]
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
    // itit main all_data state
    let mut all_data: Data = Data::default();
    all_data.tree_master = create_master_tree();
    all_data.account_directory.directory = create_all_keys(); 
    all_data.authorised.push("2vxsx-fae".to_string()); // ***************************  TESTING ONLY!!
    all_data.authorised.push(
        "e3uc3-o4g2j-bdkhp-yi4p4-wzfdy-glkas-zlhqf-n2jm2-ehxiv-fnjkc-2ae".to_string()
    ); // Saorsa Dev
    all_data.authorised.push(
        "ztewi-mzfkq-w57f2-xtl6i-kacap-n2gg6-dxyzu-p3oql-aikxf-rsivy-aqe".to_string()
    ); // frontend
    all_data.first_run = true;
    all_data.canister_settings.stats_are_public = true;
    all_data.canister_settings.canister_name = "Name me please!".to_string();
    let runtime_state = RuntimeState { all_data, };
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
        s.all_data.check_authorised(ic_cdk::caller().to_text());
        s.all_data.add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
        s.all_data.remove_authorised(principal_id)
    })
}

#[query]
fn get_all_authorised() -> Vec<String> {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
        s.all_data.get_all_authorised()
    })
}

#[update]
fn set_canister_name(name: String) -> String {
    RUNTIME_STATE.with(|state| {
        let mut s = state.borrow_mut();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
        s.all_data.set_canister_name(name)
    })
}

#[update]
async fn set_target_canister() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
    });

    let canister_id = "ryjl3-tyaaa-aaaaa-aaaba-cai".to_string();
    let is_first_run = with_runtime(|rts| { rts.all_data.first_run });
    if is_first_run == false {
        ic_cdk::trap(
            "Target canister cann't be changed after being set. Re-install canister to change."
        );
    }

    let ledger_id = Principal::from_text(&canister_id);
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
                        rts.all_data.canister_settings.transaction_fee = value;
                    });
                    with_runtime_mut(|rts| {
                        rts.all_data.first_run = false;
                    });
                    with_runtime_mut(|rts| {
                        rts.all_data.canister_settings.target_canister = canister_id;
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
fn set_stats_public(input_bool: bool) -> String {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
    });
    let ret: String;
    match input_bool {
        true => {
            with_runtime_mut(|rts| {
                rts.all_data.canister_settings.stats_are_public = true;
            });
            ret = "Stats Made Public".to_string();
        }
        false => {
            with_runtime_mut(|rts| {
                rts.all_data.canister_settings.stats_are_public = false;
            });
            ret = "Stats Made Private".to_string();
        }
    }
    return ret;
}

#[query]
fn get_canister_name() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
        s.all_data.get_canister_name()
    })
}

#[query]
fn get_working_stats() -> WorkingStats {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.all_data.are_stats_public() {
            s.all_data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    with_runtime(|rts| { rts.all_data.working_stats.to_owned() })
}

#[query]
fn get_total_holders() -> (){ /// TotalHoldersResponse
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.all_data.are_stats_public() {
            s.all_data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    // let accounts = with_runtime(|rts| { rts.all_data.account_holders.len() });
    // let ret = TotalHoldersResponse {
    //     accounts: accounts as u64,
    // };
    // return ret;
}

#[query]
fn get_top_holders(top_x: usize) -> () { // TopHoldersResponse
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.all_data.are_stats_public() {
            s.all_data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    // let accounts = with_runtime(|rts| { rts.all_data.account_holders.to_owned() });
    // let ac_len = if top_x > accounts.len() { accounts.len() } else { top_x };

    // // ACCOUNTS
    // let mut ac_vec: Vec<HolderBalance> = vec![];
    // for (hdr, ed) in accounts {
    //     ac_vec.push(HolderBalance { holder: hdr, balance: ed.balance });
    // }
    // ac_vec.sort_unstable_by_key(|element| element.balance);
    // ac_vec.reverse();
    // let mut top_ac: Vec<HolderBalance> = vec![];
    // for i in 0..ac_len as usize {
    //     top_ac.push(ac_vec[i].to_owned());
    // }
    // let res: TopHoldersResponse = TopHoldersResponse {
    //     top_accounts: top_ac,
    // };
    // return res;
}

#[query]
fn get_account_balance(id: String) -> () { //String 
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.all_data.are_stats_public() {
            s.all_data.check_authorised(ic_cdk::caller().to_text())
        }
    });

    // let accounts: BTreeMap<String, EntityData> = with_runtime(|rts| { rts.all_data.account_holders.to_owned() });
    // if let Some(item) = accounts.iter().find(|(&ref std, &_ed)| std.to_owned() == id.to_owned()) {
    //     return format!("{:?}", item);
    // } else {
    //     return "not found".to_string();
    // }
}

async fn fetch_data() {
    //Check target canister set
    with_runtime(|rts| {
        if rts.all_data.canister_settings.target_canister.is_empty() {
            ic_cdk::trap("Target Canister Not Set!");
        }
    });

    let targ_canister = with_runtime(|rts| {
        rts.all_data.canister_settings.target_canister.to_owned()
    });

    // Download latest blocks, calculate balances, and save any transactions within timewindow.
    let ledger_id: Result<Principal, candid::types::principal::PrincipalError> = Principal::from_text(targ_canister);
    match ledger_id {
        Ok(ledger_id) => {
            let tip_chain = get_tip_of_chain(ledger_id).await
                .map_err(|err_string| format!("Can't fetch tip of chain : {}", err_string))
                .unwrap();

            let tip_u128: Result<u128, &str> = tip_chain.to_u128().ok_or("Cannot cast to u128");
            match tip_u128 {
                Ok(tip) => {
                    if tip > 0 {
                        let next_block = with_runtime(|rts| rts.all_data.working_stats.next_tx);
                        let blocks_needed = tip - next_block;
                        let chunks_needed = (
                            (blocks_needed as f32) / (MAX_TRANSACTION_BATCH_SIZE as f32)
                        ).ceil() as u32;

                        log("[][] ----- Starting ICP Download ----- [][]");
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
                                rts.all_data.working_stats.is_upto_date = false;
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
                                length = if remaining > (MAX_TRANSACTION_BATCH_SIZE as u128) {
                                    MAX_TRANSACTION_BATCH_SIZE as u128
                                } else {
                                    remaining
                                };
                                // Get transactions
                                let txns: Option<Vec<ProcessedTX>> = icp_transaction_download(
                                    start as u64,
                                    length as u64
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

                            with_runtime_mut(|rts| {
                                rts.all_data.awaiting_indexing = temp_tx_array;
                            });

                            with_runtime_mut(|rts| {
                                rts.all_data.working_stats.total_downloaded = next_block + completed_this_run;
                                rts.all_data.working_stats.tx_completed_to = next_block + completed_this_run - 1;
                                rts.all_data.working_stats.next_tx = next_block + completed_this_run;
                            }); // -1 to account for 0 block
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
        rts.all_data.working_stats.is_busy = false;
    });
    return;
}

//[][] ----------------------------------- [][]
//[][] ---- Data Download/ Processing ---- [][]
//[][] ----------------------------------- [][]
async fn icp_transaction_download(start: u64, length: u64) -> Option<Vec<ProcessedTX>> {
    // check target canister is set
    let canister_settings = with_runtime(|rts| { rts.all_data.canister_settings.to_owned() });
    if canister_settings.target_canister.is_empty() {
        ic_cdk::trap("Target Canister Not Set!");
    }

    let ledger_id = Principal
        ::from_text(canister_settings.target_canister)
        .unwrap();
    let mut processed_transactions: Vec<ProcessedTX> = vec![];
    let res: Result<QueryBlocksResponse, String> = get_transactions_from_ledger(
        ledger_id,
        start,
        length
    ).await;
    match res {
        Ok(value) => {
            match (value.blocks.is_empty(), value.archived_blocks.is_empty()) {
                (false, false) => {
                    // Ledger and Archive
                    let mut block_master: Nat = Nat::from(0);
                    for archived in value.archived_blocks {
                        let archived = ArchivedBlocksRange {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = Nat::from(archived.start.clone());
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                match data {
                                    Ok(v) => {
                                        // loop through results
                                        for block_data in v.blocks {
                                            match block_data.transaction.operation {
                                                Some(value) => {
                                                    let hash: String = "no-hash".to_string();
                                                    // hash is wrong??
                                                    // match block_data.parent_hash {
                                                    //     Some(v) => {
                                                    //         hash = hex::encode(v);
                                                    //     },
                                                    //     _ => {
                                                    //         hash = "no-hash".to_string();
                                                    //     },
                                                    // }

                                                    match value {
                                                        OperationEnum::Burn { from, amount } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_burn_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Mint { to, amount } => {
                                                            let input = (
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_mint_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Transfer {
                                                            from,
                                                            to,
                                                            amount,
                                                            fee: _,
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_transfer_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Approve {
                                                            fee,
                                                            from,
                                                            allowance_e8s,
                                                            expires_at,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Approve opersion. Block: {}", block)
                                                            );
                                                        }
                                                        OperationEnum::TransferFrom {
                                                            to,
                                                            fee,
                                                            from,
                                                            amount,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Transfer from operation. Block: {}", block)
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    log(
                                                        format!("Transaction does not contain operation! Block: {}", block)
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        log(
                                            format!(
                                                "Error trying to unwrap Archive blocks. Err : {:?}",
                                                error
                                            )
                                        );
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!(
                                        "Error fetching archive transactions. Error : {:?}",
                                        err_text
                                    )
                                );
                            }
                        }
                        block_master = block;
                    }

                    // Ledger, non-archive blocks
                    for block_data in value.blocks {
                        // contains
                        match block_data.transaction.operation {
                            Some(value) => {
                                let hash: String = "no-hash".to_string();
                                // hash is wrong??
                                // match block_data.parent_hash {
                                //     Some(v) => {
                                //         hash = hex::encode(v);
                                //     },
                                //     _ => {
                                //         hash = "no-hash".to_string();
                                //     },
                                // }

                                match value {
                                    OperationEnum::Burn { from, amount } => {
                                        let input = (hex::encode(from), amount.e8s);
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Mint { to, amount } => {
                                        let input = (hex::encode(to), amount.e8s);
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Transfer { from, to, amount, fee } => {
                                        let input = (
                                            hex::encode(from),
                                            hex::encode(to),
                                            amount.e8s,
                                        );
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Approve {
                                        fee,
                                        from,
                                        allowance_e8s,
                                        expires_at,
                                        spender,
                                    } => {
                                        log(format!("Approve opersion. Block: {}", block_master));
                                    }
                                    OperationEnum::TransferFrom {
                                        to,
                                        fee,
                                        from,
                                        amount,
                                        spender,
                                    } => {
                                        log(
                                            format!("Transfer from operation. Block: {}", block_master)
                                        );
                                    }
                                }
                            }
                            _ => {
                                log("Transaction does not contain operation!");
                            }
                        }
                    }

                    return Some(processed_transactions);
                }
                (false, true) => {
                    // Ledger TX only - no archive
                    let mut block = Nat::from(start);
                    for block_data in value.blocks {
                        // contains
                        match block_data.transaction.operation {
                            Some(value) => {
                                let hash: String = "no-hash".to_string();
                                // hash is wrong??
                                // match block_data.parent_hash {
                                //     Some(v) => {
                                //         hash = hex::encode(v);
                                //     },
                                //     _ => {
                                //         hash = "no-hash".to_string();
                                //     },
                                // }

                                match value {
                                    OperationEnum::Burn { from, amount } => {
                                        let input = (hex::encode(from), amount.e8s);
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Mint { to, amount } => {
                                        let input = (hex::encode(to), amount.e8s);
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Transfer { from, to, amount, fee } => {
                                        let input = (
                                            hex::encode(from),
                                            hex::encode(to),
                                            amount.e8s,
                                        );
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Approve {
                                        fee,
                                        from,
                                        allowance_e8s,
                                        expires_at,
                                        spender,
                                    } => {
                                        log(format!("Approve opersion. Block: {}", block));
                                    }
                                    OperationEnum::TransferFrom {
                                        to,
                                        fee,
                                        from,
                                        amount,
                                        spender,
                                    } => {
                                        log(format!("Transfer from operation. Block: {}", block));
                                    }
                                }
                            }
                            _ => {
                                log("Transaction does not contain operation!");
                            }
                        }
                    }

                    return Some(processed_transactions);
                }
                (true, false) => {
                    // Archive TXS ONLY
                    for archived in value.archived_blocks {
                        let archived = ArchivedBlocksRange {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = Nat::from(archived.start.clone());
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                match data {
                                    Ok(v) => {
                                        // loop through results
                                        for block_data in v.blocks {
                                            match block_data.transaction.operation {
                                                Some(value) => {
                                                    let hash: String = "no-hash".to_string();
                                                    // hash is wrong??
                                                    // match block_data.parent_hash {
                                                    //     Some(v) => {
                                                    //         hash = hex::encode(v);
                                                    //     },
                                                    //     _ => {
                                                    //         hash = "no-hash".to_string();
                                                    //     },
                                                    // }

                                                    match value {
                                                        OperationEnum::Burn { from, amount } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_burn_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Mint { to, amount } => {
                                                            let input = (
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_mint_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Transfer {
                                                            from,
                                                            to,
                                                            amount,
                                                            fee,
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_transfer_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Approve {
                                                            fee,
                                                            from,
                                                            allowance_e8s,
                                                            expires_at,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Approve opersion. Block: {}", block)
                                                            );
                                                        }
                                                        OperationEnum::TransferFrom {
                                                            to,
                                                            fee,
                                                            from,
                                                            amount,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Transfer from operation. Block: {}", block)
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    log("Transaction does not contain operation!");
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        log(
                                            format!(
                                                "Error trying to unwrap Archive blocks. Err : {:?}",
                                                error
                                            )
                                        );
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!(
                                        "Error fetching archive transactions. Error : {:?}",
                                        err_text
                                    )
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
    start: u64,
    length: u64
) -> Result<QueryBlocksResponse, String> {
    let req: GetTransactionsRequest = GetTransactionsRequest {
        start: start,
        length: length,
    };
    let (res,): (QueryBlocksResponse,) = ic_cdk
        ::call(ledger_id, "query_blocks", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res)
}

async fn get_transactions_from_archive(
    archived: &ArchivedBlocksRange
) -> Result<GetBlocksResult, String> {
    let req = GetTransactionsRequest {
        start: archived.start.clone(),
        length: archived.length.clone(),
    };
    let ledger_id = archived.callback.canister_id;
    let method = &archived.callback.method;
    let (res,): (GetBlocksResult,) = ic_cdk
        ::call(ledger_id, method, (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res)
}

fn process_mint_transaction(
    data: (String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (to_account, tx_value) = data;
    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: hash.to_owned(),
        tx_type: TransactionType::Mint.to_string(),
        from_account: "ICP_LEDGER".to_string(),
        to_account: to_account,
        tx_value: Nat::from(tx_value),
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_burn_transaction(
    data: (String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (from_ac, tx_value) = data;
    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: hash.to_owned(),
        tx_type: TransactionType::Burn.to_string(),
        from_account: from_ac,
        to_account: "ICP_LEDGER".to_string(),
        tx_value: Nat::from(tx_value),
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_transfer_transaction(
    data: (String, String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (from_account, to_account, tx_value) = data;
    let ret = ProcessedTX {
        block: block.to_owned(),
        hash: hash.to_owned(),
        tx_type: TransactionType::Transaction.to_string(),
        from_account: from_account,
        to_account: to_account,
        tx_value: Nat::from(tx_value),
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

async fn get_tip_of_chain(ledger_id: Principal) -> Result<u64, String> {
    let req = GetTransactionsRequest {
        start: 0_u64,
        length: 1_u64,
    };
    let (res,): (QueryBlocksResponse,) = ic_cdk
        ::call(ledger_id, "query_blocks", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res.chain_length)
}

async fn update_balances(tx_array: &Vec<ProcessedTX>) -> bool {
    if tx_array.len() == 0 {
        return true;
    }
    let processed_ok_fn: bool;
    processed_ok_fn = RUNTIME_STATE.with(|state| {
        let rts = &mut state.borrow_mut();
        let tx_fee = rts.all_data.canister_settings.transaction_fee.clone() as u128;
        let mut processed_ok = true;

        for tx in tx_array {
            let tx_value_u128 = tx.tx_value.0.to_u128().ok_or("Tip of Chain is not a valid u128");
            match tx_value_u128 {
                Ok(tx_value_u128) => {
                    match tx.tx_type.as_str() {
                        "Transaction" => {
                            // ----- DEBIT FROM
                            let fm_key: IDKey = string_to_key(&tx.from_account);
                            // ----- lookup key
                            if let Some(inner_tree_key) = rts.all_data.account_directory.lookup_directory(fm_key){
                                // ----- main tree lookup
                                if let Some(ac) = rts.all_data.tree_master[inner_tree_key].get(&tx.from_account) {
                                    // ----- update balance and entity Data
                                    let tot_deduction;
                                    if ac.balance < tx_value_u128 + tx_fee {
                                        tot_deduction = ac.balance; // catch overflows. cant spend more than ac balance.
                                        log(
                                            format!(
                                                "Caught overflow from transfer. Account: {}",
                                                &tx.from_account
                                            )
                                        );
                                    } else {
                                        tot_deduction = tx_value_u128 + tx_fee;
                                    }
                                    // existing account
                                    let ent: EntityData = EntityData {
                                        balance: ac.balance - tot_deduction,
                                        transactions: ac.transactions + 1_u64,
                                    };
                                    rts.all_data.tree_master[inner_tree_key].insert(tx.from_account.clone(), ent);
                                } else {
                                    log(
                                        format!(
                                            "Error: Cant find tree key: {:?} - Sent transactions cannot be from unknown account: {}",
                                            &fm_key,
                                            &tx.from_account
                                        )
                                    );
                                    processed_ok = false;
                                }
                            } else {
                                log(
                                    format!(
                                        "Error: Lookup Key Directory returned None. Key: {:?}, Account: {}",
                                        &fm_key,
                                        &tx.from_account
                                    )
                                );
                                processed_ok = false;
                            }
                        
                            // ----- PAYMENT TO
                            let to_key: IDKey = string_to_key(&tx.to_account);
                            // ----- lookup key
                            if let Some(inner_tree_key) = rts.all_data.account_directory.lookup_directory(to_key){
                                // ----- main tree lookup
                                if let Some(ac) = rts.all_data.tree_master[inner_tree_key].get(&tx.to_account) {
                                        // existing account
                                        let ent = EntityData {
                                            balance: ac.balance + tx_value_u128,
                                            transactions: ac.transactions + 1_u64,
                                        };
                                        rts.all_data.tree_master[inner_tree_key].insert(tx.to_account.clone(), ent);
                                }else{
                                    // new account
                                    let ent = EntityData {
                                        balance: tx_value_u128,
                                        transactions: 1_u64,
                                    };
                                    rts.all_data.tree_master[inner_tree_key].insert(tx.to_account.clone(), ent);
                                }
                            } else {
                                log(
                                    format!(
                                        "Error: Lookup Key Directory returned None. Key: {:?}, Account: {}",
                                        &to_key,
                                        &tx.to_account
                                    )
                                );
                                processed_ok = false;
                            }
                        }
                        "Mint" => {
                            let to_key: IDKey = string_to_key(&tx.to_account);
                            // ----- lookup key
                            if let Some(inner_tree_key) = rts.all_data.account_directory.lookup_directory(to_key){
                                // ----- main tree lookup
                                if let Some(ac) = rts.all_data.tree_master[inner_tree_key].get(&tx.to_account) {
                                        // existing account
                                        let ent = EntityData {
                                            balance: ac.balance + tx_value_u128,
                                            transactions: ac.transactions + 1_u64,
                                        };
                                        rts.all_data.tree_master[inner_tree_key].insert(tx.to_account.clone(), ent);
                                }else{
                                    // new account
                                    let ent = EntityData {
                                        balance: tx_value_u128,
                                        transactions: 1_u64,
                                    };
                                    rts.all_data.tree_master[inner_tree_key].insert(tx.to_account.clone(), ent);
                                }
                            } else {
                                log(
                                    format!(
                                        "Error: Lookup Key Directory returned None. Key: {:?}, Account: {}",
                                        &to_key,
                                        &tx.to_account
                                    )
                                );
                                processed_ok = false;
                            }
                        }
                        "Burn" => {
                                let fm_key: IDKey = string_to_key(&tx.from_account);
                                // ----- lookup key
                                if let Some(inner_tree_key) = rts.all_data.account_directory.lookup_directory(fm_key){
                                    // ----- main tree lookup
                                    if let Some(ac) = rts.all_data.tree_master[inner_tree_key].get(&tx.from_account) {
                                        // ----- update balance and entity Data
                                        let tot_deduction;
                                        if ac.balance < tx_value_u128 + tx_fee {
                                            tot_deduction = ac.balance; // catch overflows. cant spend more than ac balance.
                                            log(
                                                format!(
                                                    "Caught overflow from transfer. Account: {}",
                                                    &tx.from_account
                                                )
                                            );
                                        } else {
                                            tot_deduction = tx_value_u128 + tx_fee;
                                        }
                                        // existing account
                                        let ent: EntityData = EntityData {
                                            balance: ac.balance - tot_deduction,
                                            transactions: ac.transactions + 1_u64,
                                        };
                                        rts.all_data.tree_master[inner_tree_key].insert(tx.from_account.clone(), ent);
                                    } else {
                                        log(
                                            format!(
                                                "Error: Cant find tree key: {:?} - Sent transactions cannot be from unknown account: {}",
                                                &fm_key,
                                                &tx.from_account
                                            )
                                        );
                                        processed_ok = false;
                                    }
                                } else {
                                    log(
                                        format!(
                                            "Error: Lookup Key Directory returned None. Key: {:?}, Account: {}",
                                            &fm_key,
                                            &tx.from_account
                                        )
                                    );
                                    processed_ok = false;
                                }
                            }
                        _ => {
                            log(
                                "Could not process transaction, type is not Mint, Burn or Transaction"
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

//[][] ------------------------- [][]
//[][] ---- Timer Functions ---- [][]
//[][] ------------------------- [][]
#[update]
fn stop_all_timers() -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
    });

    TIMER_IDS.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });
    RUNTIME_STATE.with(|state| {
        state.borrow_mut().all_data.timer_active = false;
    });
    log("[][] ---- Processing timer stopped ---- [][]");
    return String::from("Processing timer stopped");
}

#[update]
fn check_and_start_processing_timer(secs: u64) -> String {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
    });

    // check target canister is set
    let canister_settings = with_runtime(|rts| { rts.all_data.canister_settings.to_owned() });
    if canister_settings.target_canister.is_empty() {
        ic_cdk::trap("Target Canister Not Set!");
    }
    // check hours/ days is set
    if canister_settings.days_to_calcualte == 0 || canister_settings.hours_to_calculate == 0 {
        ic_cdk::trap("Hours to calculate or Days to calculate cannot be 0");
    }

    let ret: String;
    let is_running = RUNTIME_STATE.with(|state| {
        return state.borrow().all_data.timer_active;
    });
    if is_running == true {
        ret = String::from("Processing timer is alraedy running");
    } else {
        start_processing_timer(secs);
        RUNTIME_STATE.with(|state| {
            state.borrow_mut().all_data.timer_active = true;
        });
        ret = String::from("Processing timer has been started");
        log("[][] ---- Starting Processing Timer ---- [][]");
    }
    return ret;
}

fn start_processing_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_data_processing())
    );
    TIMER_IDS.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_data_processing() {
    let ws = with_runtime(|rts| { rts.all_data.working_stats.to_owned() });
    if ws.is_busy == true {
        return;
    } else {
        with_runtime_mut(|rts| {
            rts.all_data.working_stats.is_busy = true;
        });

        fetch_data().await;

        // if ws.task_id == 0 {
           
        // } else if ws.is_upto_date == true && ws.task_id == 1 {
            
        // } else if ws.is_upto_date == true && ws.task_id == 2 {
           
        // } else if ws.is_upto_date == true && ws.task_id == 3 {
            
        // }
    }
}

#[update]
async fn test_call() -> String{
    schedule_data_processing().await;

    let potato = instruction_counter();
    return format!("USED :: {}", potato);
}

#[update]
async fn test_call2() -> String{

    let mut temp_tx_array = with_runtime_mut(|rts|{rts.all_data.awaiting_indexing.to_owned()});

    // Calculate and update balances 
    let ub_res = update_balances(&temp_tx_array).await;
    if ub_res == false {
        log("Error when updating balances");
        ic_cdk::trap("Error when updating balances");
    } else {
        log("[][] --- Hodler Balances Updated --- [][]");
    }
    let btl = with_runtime(|rts|{rts.all_data.tree_master[0].len()});
    log(format!("Btree LEN:: {}", btl));
    temp_tx_array.clear();

    // update working stats state
    // if ub_res == true {
    //     with_runtime_mut(|rts| {
    //         let ws = rts.all_data.working_stats.borrow_mut(); 
    //         ws.total_downloaded = next_block + completed_this_run;
    //         ws.tx_completed_to = next_block + completed_this_run - 1; // -1 to account for 0 block
    //         ws.next_tx = next_block + completed_this_run;
    //         ws.is_upto_date = false;
    //     }); 

    //     log(
    //         format!(
    //             "Complete To {}; All transactions downloaded? = {};",
    //             next_block + completed_this_run - 1,
    //             is_upto_date
    //         )
    //     );
    // }

    let potato = instruction_counter();
    return format!("USED :: {}", potato);
}

// [][] ------------------------------ [][]
// [][] --- Canister Metrics/ Logs --- [][]
// [][] ------------------------------ [][]
#[query]
fn get_cycles_balance() -> u64 {
    RUNTIME_STATE.with(|state| {
        let s: std::cell::Ref<'_, RuntimeState> = state.borrow();
        if !s.all_data.are_stats_public() {
            s.all_data.check_authorised(ic_cdk::caller().to_text())
        }
    });
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    RUNTIME_STATE.with(|state| {
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
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
        let s = state.borrow();
        s.all_data.check_authorised(ic_cdk::caller().to_text());
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
        let max_logs = 500;
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
