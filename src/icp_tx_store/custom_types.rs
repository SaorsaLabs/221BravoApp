use std::{borrow::BorrowMut, fmt::format};

use candid::{CandidType, Deserialize};
use ic_stable_memory::{derive::{StableType, AsFixedSizeBytes}, AsFixedSizeBytes, collections::{SHashMap, SVec, SBTreeMap}};

use crate::utils::{idkey_to_string, string_to_idkey, log};

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

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct TxStore {
    blocks: SBTreeMap<u32, SmallTX>,
    next_block: u32,
}
impl TxStore {
    pub fn add_tx(&mut self, small_tx: SmallTX) -> bool {
        let nb: u32 = self.next_block;
        match self.blocks.insert(nb, small_tx) {
            Ok(prev) => {
                self.next_block += 1_u32;
                return true;
            },
            Err((k, v)) => {
                log(format!("Out of memory. Unable to insert pair: {}, {:?}",k, v));
                return false;
            }
        };
    }

    pub fn get_tx(&self, block: u32) -> Option<SmallTX> {
        match self.blocks.get(&block) {
            Some(value) => {
                let tx: SmallTX = *value;
                return Some(tx);
            },
            None => { return None}
        };
    }

    pub fn get_multiple_tx(&self, block_vec: Vec<u32>) -> Vec<Option<SmallTX>> {
        let mut ret_vec:Vec<Option<SmallTX>> = Vec::new();
        for id_ref in &block_vec {
            match self.blocks.get(&id_ref) {
                Some(value) => {
                    let tx: SmallTX = *value;
                    ret_vec.push(Some(tx));
                },
                None => { ret_vec.push(None)}
            };
        }
       return  ret_vec;
    }

    pub fn get_count(&self) -> u32 {
        self.next_block // is +1 but this accounts for 0 starting index
    }

}

#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct GetMultipleTxFromStoreTimeArgs {
    pub blocks: Vec<u32>, 
    pub start: u64, 
    pub end: u64, 
    pub max_return: u64
}

#[derive(CandidType, StableType, AsFixedSizeBytes, Debug, Deserialize, Default, Clone, Copy)]
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
#[derive(CandidType, Debug, Default, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

// Stable storage of canister settings. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct CanisterSettings {
    pub canister_name: IDKey, 
    pub stats_are_public: bool,
    pub authorised: SVec<IDKey>,
    pub admin: SVec<IDKey>,
    pub init_lock: bool,
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
                self.canister_name.borrow_mut().0 = caller_key.0;
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
        self.borrow_mut().stats_are_public = are_stats_public;
        if are_stats_public == true { self.add_authorised("2vxsx-fae".to_string()); } 
        else { self.remove_authorised("2vxsx-fae".to_string()); } 
        return format!("are_stats_public updated to: {}", are_stats_public);
    }
}

// struct for returning memory query
#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
    pub memory: u64,
    pub heap_memory: u64,
}


