use std::{fmt, collections::VecDeque};
use std::borrow::BorrowMut;
use candid::{CandidType, Deserialize, Nat};
use ic_stable_memory::collections::SBTreeMap;
use ic_stable_memory::derive::CandidAsDynSizeBytes;
use ic_stable_memory::primitive::s_ref::SRef;
use num_traits::ops::overflowing;
use serde::Serialize;
use ic_stable_memory::{derive::{StableType, AsFixedSizeBytes}, AsFixedSizeBytes, collections::{SHashMap, SVec}};
use ic_stable_memory::SBox;

use crate::utils::{idkey_to_string, string_to_idkey, log};
use crate::constants::{ MAX_BLOCKS_RETAINED, VERSION };

// ID Key is limited to 135 bytes (max 134 input string and ':' at the end) 
#[derive(CandidType, Deserialize, Serialize, StableType, Hash, Eq, PartialEq, Clone, Debug)]
pub struct IDKey(pub Vec<u8>);
impl AsFixedSizeBytes for IDKey {
    const SIZE: usize = 135;
    type Buf =  Vec<u8>; // use for generics  
    
    fn as_fixed_size_bytes(&self, buf: &mut [u8]) {
        let key_bytes = self.0.as_slice();
        buf[0] =  key_bytes.len() as u8;
        buf[1..(1 + key_bytes.len())].copy_from_slice(key_bytes);
    }
    
    fn from_fixed_size_bytes(buf: &[u8]) -> Self {
        let key_len = buf[0] as usize;
        let key: &[u8] = &buf[1..(1 + key_len)];
        return IDKey(key.try_into().unwrap());
    }
}
impl Default for IDKey {
    fn default() -> Self {
        IDKey(Vec::new()) 
    }
}

// Directory used for converting IDKeys (ICRC/ICP Accounts) to and from a unique u32 referece number. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Directory {
    pub id_to_ref: SHashMap<IDKey, u32>,
    pub ref_to_id: SHashMap<u32, IDKey>,
    pub next_ref: u32,
}
impl Directory {
    pub fn add_id(&mut self, id_string: String) -> Option<u32> {
        let id_bytes = string_to_idkey(&id_string);
        match id_bytes {
            Some(key) => {
                let v = self.id_to_ref.get(&key);
            match v {
                Some(v) => {
                    return Some(v.to_owned());
                },
                None => {
                    // not handling result on inserts as 
                    // id_bytes match handles 'entry exists already' scenario.
                    self.id_to_ref.insert(key.clone(), self.next_ref).expect("Storage is full"); 
                    self.ref_to_id.insert(self.next_ref, key).expect("Storage is full");
                    let ret = self.next_ref.clone();
                    self.next_ref += 1_u32;
                    return Some(ret);
                }
            }
            },
            None => { return None}
        }
    }

    pub fn get_id(&self, id_ref: &u32) -> Option<String> {
        let v = &self.ref_to_id.get(&id_ref);
        match v {
            Some(v) => {
                let id_string = idkey_to_string(&v);
                match id_string {
                    Some(id) => {
                        return Some(id);
                    },
                    None => { 
                        log("Error - Cannot parse key to string for 'get_id'");
                        return None
                    }
                }
            },
            None => {return None}
        }
    }

    pub fn get_ref(&self, id_string: &String) -> Option<u32> {
        let opt_key = string_to_idkey(&id_string);
        match opt_key {
            Some(key) => {
                let v = &self.id_to_ref.get(&key);
                match v {
                    Some(v) => {
                        return Some(**v);
                    },
                    None => {return None}
                }
            },
            None => {
                log("Error - unable to parse string to key for 'get_ref'");
                return None;
            },
        }
    }

    pub fn get_total_entries(&self) -> u32 {
        let res = &self.next_ref;
        return res.to_owned();
    }
}


#[derive(CandidType, StableType, AsFixedSizeBytes, Debug, Default, Serialize, Deserialize, Clone)]
pub struct SmallTX {
    pub block: u32,
    pub time: u64,
    pub from: Option<u32>, 
    pub to: Option<u32>,
    pub tx_type: u8,
    pub value: u64,
    //pub hash: , // Option<IDKey> hash is 64 in len 
}

// used for canister logging.
#[derive(CandidType, StableType, Debug, Deserialize, Default, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

// used for sending blocks to store canister
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct SendTxToStoreArgs(pub Vec<SmallTX>);

// used for getting tx from tx store
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct GetTxFromStoreArgs(pub u32);

// used for getting mutiple tx from tx store
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct GetMultipleTxFromStoreArgs(pub Vec<u32>);

// Stable storage of canister settings. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct CanisterSettings {
    pub target_canister: IDKey,
    pub stx_store_canister: IDKey,
    pub self_canister: IDKey,
    pub canister_name: IDKey, 
    pub stats_are_public: bool,
    pub target_canister_locked: bool,
    pub authorised: SVec<IDKey>,
    pub admin: SVec<IDKey>,
    pub working_stats: WorkingStats,
}

impl CanisterSettings {
    pub fn check_authorised(&self, principal_id: String) {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&SVec<IDKey> = &self.authorised;
                let mut auth: bool = false;
                for idk in auth_vec.iter() {
                    if idk.0 == caller_key.0 {auth = true}
                }
                match auth {
                    true => (),
                    _ => ic_cdk::trap("Caller Not Authorised"),
                }
            },
            None => {
                ic_cdk::trap("Caller Not Authorised")
            }
        }
    }

    pub fn check_admin(&self, principal_id: String) {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&SVec<IDKey> = &self.admin;
                let mut auth: bool = false;
                for idk in auth_vec.iter() {
                    if idk.0 == caller_key.0 {auth = true}
                }
                match auth {
                    true => (),
                    _ => ic_cdk::trap("Caller Not Admin"),
                }
            },
            None => {
                ic_cdk::trap("Caller Not Admin")
            }
        }
    }

    pub fn add_authorised(&mut self, principal_id: String) -> String {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&mut SVec<IDKey> = &mut self.authorised;
                let mut auth: bool = false;
                for idk in auth_vec.iter() {
                    if idk.0 == caller_key.0 {auth = true}
                }
                match auth {
                    true => return "Principal is already authorised".to_string(),
                    false => {
                        auth_vec.push(caller_key).expect("out of memory");
                        let rtn: String = String::from("Authorised user added");
                        return rtn;
                    },
                }
            },
            None => {
                return "Cannot convert Principal Id to IDKey".to_string();
            }
        }
    }

    pub fn add_admin(&mut self, principal_id: String) -> String {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&mut SVec<IDKey> = &mut self.admin;
                let mut auth: bool = false;
                for idk in auth_vec.iter() {
                    if idk.0 == caller_key.0 {auth = true}
                }
                match auth {
                    true => return "Principal is already admin".to_string(),
                    false => {
                        auth_vec.push(caller_key).expect("out of memory");
                        let rtn: String = String::from("Admin added");
                        return rtn;
                    },
                }
            },
            None => {
                return "Cannot convert Principal Id to IDKey".to_string();
            }
        }
    }

    pub fn remove_authorised(&mut self, principal_id: String) -> String {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&mut SVec<IDKey> = self.authorised.borrow_mut();
                let mut pos: usize = 0_usize;
                let mut present: bool = false; 
                for (i,idk) in auth_vec.iter().enumerate() {
                    if idk.0 == caller_key.0 {
                        pos = i;
                        present = true;
                    }
                }
                match present {
                    true => {
                        auth_vec.remove(pos);
                        return "Authorised user removed".to_string();
                    },
                    false => {
                        return "Principal not part of authorised list".to_string();
                    },
                }
            },
            None => {
                return "Cannot convert Principal Id to IDKey".to_string();
            }
        }
    }

    pub fn remove_admin(&mut self, principal_id: String) -> String {
        let key = string_to_idkey(&principal_id);
        match key {
            Some(caller_key) => {
                let auth_vec:&mut SVec<IDKey> = self.admin.borrow_mut();
                let mut pos: usize = 0_usize;
                let mut present: bool = false; 
                for (i,idk) in auth_vec.iter().enumerate() {
                    if idk.0 == caller_key.0 {
                        pos = i;
                        present = true;
                    }
                }
                match present {
                    true => {
                        auth_vec.remove(pos);
                        return "Admin removed".to_string();
                    },
                    false => {
                        return "Principal not part of admin list".to_string();
                    },
                }
            },
            None => {
                return "Cannot convert Principal Id to IDKey".to_string();
            }
        }
    }

    pub fn get_all_authorised(&self) -> Vec<String> {
        let auth_vec:&SVec<IDKey> = &self.authorised;
        let mut ret_vec: Vec<String> = Vec::new();
        for idk in auth_vec.iter() {
            let vec_key = idk.0.clone();
            let id = IDKey(vec_key);
            let st = idkey_to_string(&id);
            match st {
                Some(value) => {
                    ret_vec.push(value);
                },
                None => ()
            }
        }
        return ret_vec.to_owned();
    }

    pub fn get_all_admins(&self) -> Vec<String> {
        let auth_vec:&SVec<IDKey> = &self.admin;
        let mut ret_vec: Vec<String> = Vec::new();
        for idk in auth_vec.iter() {
            let vec_key = idk.0.clone();
            let id = IDKey(vec_key);
            let st = idkey_to_string(&id);
            match st {
                Some(value) => {
                    ret_vec.push(value);
                },
                None => ()
            }
        }
        return ret_vec.to_owned();
    }

    pub fn set_canister_name(&mut self, name: String) -> String {
        let key = string_to_idkey(&name);
        match key {
            Some(caller_key) => {
                self.canister_name.0 = caller_key.0;
                return "Canister name set".to_string();
            }
            None => {
                return "Cannot convert name into IDKey to save".to_string();
            }
        }
    }

    pub fn get_canister_name(&self) -> String {
        let name = &self.canister_name;
        let st = idkey_to_string(&name);
        match st {
            Some(value) => return value,
            None => return "Could not parse IDKey to String".to_string()
        }
    }

    pub fn are_stats_public(&self) -> bool {
        let ret = &self.stats_are_public;
        return ret.to_owned();
    }

    pub fn set_stats_public(&mut self, are_stats_public: bool) -> String {
        self.stats_are_public = are_stats_public;
        if are_stats_public == true { self.add_authorised("2vxsx-fae".to_string()); } 
        else { self.remove_authorised("2vxsx-fae".to_string());}
        return format!("are_stats_public updated to: {}", are_stats_public);
    }

}

// struct for returning memory query
#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
    pub memory: u64,
    pub heap_memory: u64,
}

// sub-struct of canister settings
#[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, AsFixedSizeBytes, Debug)]
pub struct WorkingStats {
    pub total_downloaded: u128,
    pub tx_completed_to: u128,
    pub next_tx: u128,
    pub timer_set: bool,
    pub is_upto_date: bool,
    pub is_busy: bool,
    pub task_id: u8,
}
impl WorkingStats {
    pub fn update_downloaded(&mut self, total_downloaded: u128, complete_to: u128, is_upto_date: bool){
        self.total_downloaded = total_downloaded;
        self.tx_completed_to = complete_to;
        self.is_upto_date = is_upto_date;
        self.next_tx = complete_to+1;
    }
    pub fn read_stats(&self) -> WorkingStatsResponse {
        let ret: WorkingStatsResponse = WorkingStatsResponse{
            version: VERSION.to_string(),
            total_downloaded: self.total_downloaded,
            tx_completed_to: self.tx_completed_to,
            next_tx: self.next_tx,
            timer_set: self.timer_set,
            is_upto_date: self.is_upto_date,
            is_busy: self.is_busy,
            task_id: self.task_id,
        };
        return ret;
    }
}

// Adds version number to the response
#[derive(CandidType, Deserialize, Serialize, Clone, Default, Debug)]
pub struct WorkingStatsResponse {
    pub version: String,
    pub total_downloaded: u128,
    pub tx_completed_to: u128,
    pub next_tx: u128,
    pub timer_set: bool,
    pub is_upto_date: bool,
    pub is_busy: bool,
    pub task_id: u8,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct GetMultipleTxFromStoreTimeArgs {
    pub blocks: Vec<u32>, 
    pub start: u64, 
    pub end: u64, 
    pub max_return: u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TimeSearchArgs {
    pub id: String,
    pub start: u64, 
    pub end: u64, 
}

#[derive(CandidType, StableType, CandidAsDynSizeBytes, Deserialize, Serialize, Clone, Default, Debug, PartialEq, Eq)]
pub struct ProcessedTX {
    pub block: u128,
    pub hash: String,
    pub tx_type: String,
    pub from_account: String,
    pub to_account: String,
    pub tx_value: u128,
    pub tx_time: u64,
}
impl fmt::Display for ProcessedTX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block: {}\nHash: {}\nType: {}\nFrom Account: {}\nTo Account: {}\nValue: {}\nTime: {}",
            self.block,
            self.hash,
            self.tx_type,
            self.from_account,
            self.to_account,
            self.tx_value,
            self.tx_time
        )
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Transaction,
    Mint,
    Burn,
}
impl TransactionType {
    pub fn to_string(&self) -> String {
        match self {
            TransactionType::Transaction => "Transaction".to_string(),
            TransactionType::Mint => "Mint".to_string(),
            TransactionType::Burn => "Burn".to_string(),
        }
    }
}

#[derive(CandidType, Deserialize, Serialize, Default, Clone)]
pub struct BlockHolder {
    pub blocks: VecDeque<ProcessedTX>,
    pub tip: u128,
}
impl BlockHolder {
    pub const MAX_SIZE: usize = MAX_BLOCKS_RETAINED;

    pub fn init(&mut self){
            self.blocks = VecDeque::with_capacity(Self::MAX_SIZE);
            self.tip = 0_u128;
    }

    pub fn push_tx(&mut self, tx: ProcessedTX) {
        if self.blocks.len() ==  Self::MAX_SIZE {
            self.blocks.pop_back();
        }
        self.blocks.push_front(tx);
    }

    pub fn get_txs(&self, number_txs: usize) -> Vec<ProcessedTX> {
        let n = if number_txs > Self::MAX_SIZE { Self::MAX_SIZE } else { number_txs };
        let vec: Vec<ProcessedTX> = self.blocks.iter().take(n).cloned().collect();
        return vec
    }
}

// Stable Store of account indexed data
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct AccountTree{
    pub accounts: SBTreeMap<u32, AccountData>,
    count: u64,
    last_updated: u64,
    pub transaction_fee: u64,
}
impl AccountTree {

    pub fn process_transfer_to(&mut self, account_ref: &u32, stx: &SmallTX){
        if !self.accounts.contains_key(account_ref) {
            let acd = AccountData {
                overview: Overview { 
                    first_active: stx.time, 
                    last_active: stx.time, 
                    sent: (0_u32, 0_u64), 
                    received: (1_u32, stx.value), 
                    balance: stx.value 
                },
                data: IndexData::default(),
            };
            self.accounts.insert(*account_ref, acd).expect("Storage is full");
        } else if let Some(mut ac) = self.accounts.get_mut(account_ref) {
            ac.overview.credit_account(stx.time, stx.value);
        }
    }

    pub fn process_transfer_from(&mut self, account_ref: &u32, stx: &SmallTX ){
        match self.accounts.get_mut(account_ref) {
            Some(mut ac) => {
                ac.overview.debit_account(stx.time, stx.value, self.transaction_fee);
            },
            None => { log("Error - cannot send from a non-existent account (process_transfer_from)"); },
        }
    }

    // only call this after process transfers - this ensures links/ blocks are init.. NEEDED??
    // ** direction 1 = inbound (ie account_ref is to) -1 = outbound (ie account_ref is from) ** IMPORTANT! 
    pub fn process_links(&mut self, account_ref: &u32, linked_ref: &u32, direction: i8, stx: &SmallTX ){
        match self.accounts.get_mut(account_ref) {
            Some(mut ac) => {
                ac.data.update_links(linked_ref,  direction, stx); 
            },
            None => { log("Error - Cannot find account to update links"); },
        }
    }

    pub fn process_block(&mut self, account_ref: &u32, block_ref: u32){
        match self.accounts.get_mut(account_ref) {
            Some(mut ac) => {
                ac.data.update_blocks(block_ref); 
            },
            None => { log("Error - Cannot find account to update blocks"); },
        }
    }
}

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct AccountData {
    pub overview: Overview,
    pub data: IndexData,
 }


 
 #[derive(StableType, Default, AsFixedSizeBytes, Debug)]
 pub struct IndexData {
    pub links: SBTreeMap<u32, LinkData>,
    pub blocks: SVec<u32>
 }

 impl IndexData {
    // call this after transfer to or transfer from, to ensure account + indexData is created
    pub fn update_links(&mut self, linked_ref: &u32, direction: i8, stx: &SmallTX){

        match self.links.contains_key(linked_ref){
                true => {
                    match self.links.get_mut(linked_ref) {
                        Some(mut v) => {
                            v.update(stx.time, *linked_ref, stx.value, direction)
                        },
                        None => {} // should never get here as contains_key called first. 
                    }
                },
                false => {
                    // insert new
                    let calc_net: i64 = if direction == 1 { stx.value as i64 } else { stx.value as i64 *-1 };
                    let ld = LinkData{
                        linked_from: stx.time,
                        linked_id: *linked_ref,
                        number_txs: 1_u32,
                        gross: stx.value,
                        net: calc_net,
                    };
                    self.links.insert(*linked_ref, ld).expect("Storage is full");
                }
        }
    }

    pub fn update_blocks(&mut self, block_ref: u32) {
        self.blocks.push(block_ref);
    }

 }

 #[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, AsFixedSizeBytes, Debug)]
 pub struct Overview {
    pub first_active: u64,
    pub last_active: u64,
    pub sent: (u32, u64), // count, value
    pub received: (u32, u64), // count, value
    pub balance: u64,
 }
 impl Overview {
    pub fn debit_account(&mut self, time:u64, value: u64, tx_fee: u64){
        let total_deduction: u64;
        if self.first_active == 0 || time < self.first_active { self.first_active = time }
        if self.last_active < time { self.last_active = time }
        if self.balance < (value+tx_fee) { total_deduction = self.balance } 
        else { total_deduction = value+tx_fee }

        // update balances
        self.balance -= total_deduction;
        let (mut s1, mut s2) = self.sent;
        s1 += 1;
        s2 += value+tx_fee;
        self.sent = (s1,s2);
    }

    pub fn credit_account(&mut self, time:u64, value: u64){
        if self.first_active == 0 || time < self.first_active { self.first_active = time }
        if self.last_active < time {self.last_active = time}

        // update balances
        self.balance += value;
        let (mut r1, mut r2) = self.received;
        r1 += 1;
        r2 += value;
        self.received = (r1,r2);
    }
 }

 #[derive(CandidType, StableType, Deserialize, Serialize, Clone, Default, AsFixedSizeBytes, Debug, PartialEq, Eq)]
 pub struct LinkData {
    pub linked_from: u64,
    pub linked_id: u32,
    pub number_txs: u32,
    pub gross: u64,
    pub net: i64
 }
 impl LinkData {
    pub fn update (&mut self, time: u64, linked_id: u32, value: u64, direction: i8) {  
        self.linked_from = if self.linked_from == 0 || time < self.linked_from { time } else { self.linked_from };
        self.linked_id = linked_id;
        if direction == 1 { 
            self.gross += value;
            self.net += value as i64;
            self.number_txs += 1;
        } else if direction == -1 {
            self.gross += value;
            self.net -= value as i64;
            self.number_txs += 1;
        }
    }
 }

 #[derive(CandidType, Deserialize, Serialize, Clone, Default, Debug)]
 pub struct LinkDataResponse {
    pub linked_from: u64,
    pub linked_id: String,
    pub number_txs: u32,
    pub gross: u64,
    pub net: i64
 }

 #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
 pub struct FullDataResponseRaw {
    pub account_ref: u32,
    pub overview: Overview,
    pub links: Vec<LinkData>, 
    pub blocks: Vec<u32>,
 }

 #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
 pub struct FullDataResponse {
    pub account_ref: String,
    pub overview: Overview,
    pub links: Vec<LinkDataResponse>, 
    pub blocks: Vec<ProcessedTX>,
 }

// [][] -- DFINITY ICRC TYPES -- [][]
 // Types created by Dfinity for interacting with ICP Ledger
 // https://github.com/dfinity/ic/blob/master/rs/rosetta-api/icp_ledger/src/lib.rs#L739
 
 #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
 pub struct GetTransactionsRequest {
     pub start: u64,
     pub length: u64,
 }
 
 pub type BlockIndex = u64;
 
 #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
 pub struct Tokens {
     /// Number of 10^-8 Tokens.
     /// Named because the equivalent part of a Bitcoin is called a Satoshi
     pub e8s: u64,
 }
 
 #[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
 pub struct TimeStamp {
     pub timestamp_nanos: u64,
 }
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub struct Memo(pub u64);
 
 pub type Certification = Option<Vec<u8>>;
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub struct Transaction {
     pub operation: Option<OperationEnum>,
     pub memo: u64,
     pub created_at_time: TimeStamp,
 }
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub enum OperationEnum {
     Burn {
         from: Vec<u8>,
         amount: Tokens,
     },
     Mint {
         to: Vec<u8>,
         amount: Tokens,
     },
     Transfer {
         from: Vec<u8>,
         to: Vec<u8>,
         amount: Tokens,
         fee: Tokens,
     },
     Approve {
         fee: Tokens,
         from: Vec<u8>,
         allowance_e8s: i128,
         expires_at: Option<TimeStamp>,
         spender: Vec<u8>,
     },
     TransferFrom {
         to: Vec<u8>,
         fee: Tokens,
         from: Vec<u8>,
         amount: Tokens,
         spender: Vec<u8>,
     },
 }
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub struct GetBlocksArgs {
     pub start: BlockIndex,
     pub length: usize,
 }
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub struct Block {
     pub parent_hash: Option<[u8; 32]>,
     pub transaction: Transaction,
     pub timestamp: TimeStamp,
 }
 
 #[derive(Deserialize, CandidType, Clone, Debug)]
 pub struct BlockRange {
     pub blocks: Vec<Block>,
 }
 
 pub type GetBlocksResult = Result<BlockRange, GetBlocksError>;
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub enum GetBlocksError {
     BadFirstBlockIndex {
         requested_index: BlockIndex,
         first_valid_index: BlockIndex,
     },
     Other {
         error_code: u64,
         error_message: String,
     },
 }
 
 #[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
 pub struct IterBlocksArgs {
     pub start: usize,
     pub length: usize,
 }
 
 #[derive(Deserialize, CandidType, Clone, Debug)]
 pub struct ArchivedBlocksRange {
     pub start: BlockIndex,
     pub length: u64,
     pub callback: QueryArchiveBlocksFn,
 }
 
 #[derive(Deserialize, CandidType, Clone, Debug)]
 pub struct QueryBlocksResponse {
     pub chain_length: u64,
     pub certificate: Option<serde_bytes::ByteBuf>,
     pub blocks: Vec<Block>,
     pub first_block_index: BlockIndex,
     pub archived_blocks: Vec<ArchivedBlocksRange>,
 }
 
 pub type QueryArchiveBlocksFn = icrc_ledger_types::icrc3::archive::QueryArchiveFn<
     GetBlocksArgs,
     GetBlocksResult
 >;


// CANDID 0.9 IMPL
//  impl<Input: CandidType, Output: CandidType> CandidType for QueryArchiveFn<Input, Output> {
//     fn _ty() -> candid::types::Type {
//         candid::types::internal::TypeInner::Func(candid::types::internal::Function {
//             modes: vec![candid::types::internal::FuncMode::Query],
//             args: vec![Input::_ty()],
//             rets: vec![Output::_ty()],
//         })
//         .into()
//     }

//     fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
//     where
//         S: candid::types::Serializer,
//     {
//         candid::types::reference::Func::from(self.clone()).idl_serialize(serializer)
//     }
// }


// candid::define_function!(pub QueryBlockArchiveFn : (GetBlocksRequest) -> (BlockRange) query);
// candid::define_function!(pub QueryTxArchiveFn : (GetTransactionsRequest) -> (TransactionRange) query);
 