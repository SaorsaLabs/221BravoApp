use std::marker::PhantomData;
use std::{fmt, collections::VecDeque};
use std::borrow::BorrowMut;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_stable_memory::collections::SBTreeMap;
use ic_stable_memory::derive::CandidAsDynSizeBytes;
use serde::Serialize;
use ic_stable_memory::{derive::{StableType, AsFixedSizeBytes}, AsFixedSizeBytes, collections::{SHashMap, SVec}};
use serde_bytes::ByteBuf;

use crate::utils::{idkey_to_string, string_to_idkey, log};
use crate::constants::MAX_BLOCKS_RETAINED;

// ID Key is limited to 135 bytes (max 134 input string and ':' at the end) 
#[derive(CandidType, Deserialize, StableType, Hash, Eq, PartialEq, Clone, Debug)]
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
#[derive(CandidType, StableType, Deserialize, Clone, Default, AsFixedSizeBytes, Debug)]
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
        self.blocks.push(block_ref).expect("Storage is full");
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
 
pub type BlockIndex = Nat;
pub type GetTransactionsRequest = GetBlocksRequest;
pub type QueryTxArchiveFn = QueryArchiveFn<GetTransactionsRequest, TransactionRange>;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetBlocksRequest {
    pub start: BlockIndex,
    pub length: Nat,
}

// Representation of a Transaction which supports the Icrc1 Standard functionalities
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub kind: String,
    pub mint: Option<Mint>,
    pub burn: Option<Burn>,
    pub transfer: Option<Transfer>,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Mint {
    pub amount: Nat,
    pub to: Account,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Burn {
    pub amount: Nat,
    pub from: Account,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub amount: Nat,
    pub from: Account,
    pub to: Account,
    pub memo: Option<Memo>,
    pub fee: Option<Nat>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TransactionRange {
    pub transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetTransactionsResponse {
    pub log_length: Nat,
    pub first_index: BlockIndex,
    pub transactions: Vec<Transaction>,
    pub archived_transactions: Vec<ArchivedRange<QueryTxArchiveFn>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetTransactionsArchiveResponse {
    pub transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArchivedRange<Callback> {
    pub start: Nat,
    pub length: Nat,
    pub callback: Callback,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(try_from = "candid::types::reference::Func")]
pub struct QueryArchiveFn<Input: CandidType, Output: CandidType> {
    pub canister_id: Principal,
    pub method: String,
    pub _marker: PhantomData<(Input, Output)>,
}

impl<Input: CandidType, Output: CandidType> QueryArchiveFn<Input, Output> {
    pub fn new(canister_id: Principal, method: impl Into<String>) -> Self {
        Self {
            canister_id,
            method: method.into(),
            _marker: PhantomData,
        }
    }
}

impl<Input: CandidType, Output: CandidType> Clone for QueryArchiveFn<Input, Output> {
    fn clone(&self) -> Self {
        Self {
            canister_id: self.canister_id,
            method: self.method.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Input: CandidType, Output: CandidType> From<QueryArchiveFn<Input, Output>>
for candid::types::reference::Func {
    fn from(archive_fn: QueryArchiveFn<Input, Output>) -> Self {
        let p: &Principal = &Principal::try_from(archive_fn.canister_id.as_ref()).expect(
            "could not deserialize principal"
        );
        Self {
            principal: *p,
            method: archive_fn.method,
        }
    }
}

impl<Input: CandidType, Output: CandidType> TryFrom<candid::types::reference::Func>
for QueryArchiveFn<Input, Output> {
    type Error = String;
    fn try_from(func: candid::types::reference::Func) -> Result<Self, Self::Error> {
        let canister_id = Principal::try_from(func.principal.as_slice()).map_err(|e|
            format!("principal is not a canister id: {}", e)
        )?;
        Ok(QueryArchiveFn {
            canister_id,
            method: func.method,
            _marker: PhantomData,
        })
    }
}

impl<Input: CandidType, Output: CandidType> CandidType for QueryArchiveFn<Input, Output> {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Func(candid::types::Function {
            modes: vec![candid::parser::types::FuncMode::Query],
            args: vec![Input::_ty()],
            rets: vec![Output::_ty()],
        })
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
        where S: candid::types::Serializer
    {
        candid::types::reference::Func::from(self.clone()).idl_serialize(serializer)
    }
}

pub type Subaccount = [u8; 32];
pub const DEFAULT_SUBACCOUNT: &Subaccount = &[0; 32];

// Account representation of ledgers supporting the ICRC1 standard
#[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

impl Account {
    #[inline]
    pub fn effective_subaccount(&self) -> &Subaccount {
        self.subaccount.as_ref().unwrap_or(DEFAULT_SUBACCOUNT)
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.effective_subaccount() == other.effective_subaccount()
    }
}

impl Eq for Account {}

impl std::cmp::PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Account {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.owner
            .cmp(&other.owner)
            .then_with(|| { self.effective_subaccount().cmp(other.effective_subaccount()) })
    }
}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.effective_subaccount().hash(state);
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subaccount {
            None => write!(f, "{}", self.owner),
            Some(subaccount) => write!(f, "0x{}.{}", hex::encode(&subaccount[..]), self.owner),
        }
    }
}

impl From<Principal> for Account {
    fn from(owner: Principal) -> Self {
        Self {
            owner,
            subaccount: None,
        }
    }
}

#[derive(
    Serialize,
    Deserialize,
    CandidType,
    Clone,
    Hash,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Default
)]
#[serde(transparent)]
pub struct Memo(pub ByteBuf);

impl From<u64> for Memo {
    fn from(num: u64) -> Self {
        Self(ByteBuf::from(num.to_be_bytes().to_vec()))
    }
}

impl From<ByteBuf> for Memo {
    fn from(b: ByteBuf) -> Self {
        Self(b)
    }
}

impl From<Vec<u8>> for Memo {
    fn from(v: Vec<u8>) -> Self {
        Self::from(ByteBuf::from(v))
    }
}

impl From<Memo> for ByteBuf {
    fn from(memo: Memo) -> Self {
        memo.0
    }
}
