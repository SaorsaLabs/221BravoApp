use std::borrow::BorrowMut;

use candid::{CandidType, Deserialize};
use ic_stable_memory::{derive::{StableType, AsFixedSizeBytes, CandidAsDynSizeBytes}, AsFixedSizeBytes, collections::{SHashMap, SVec}, SBox};

use crate::utils::{idkey_to_string, string_to_idkey, log};

// ID Key is limited to 135 bytes (max 134 input string and ':' at the end) 
#[derive(CandidType, Deserialize, StableType, Hash, Eq, PartialEq, PartialOrd, Ord, Clone, Debug)]
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
                    self.id_to_ref.insert(key.clone(), self.next_ref); 
                    self.ref_to_id.insert(self.next_ref, key);
                    let ret = self.next_ref.clone();
                    self.next_ref += 1_u32;
                    return Some(ret);
                }
            }
            },
            None => { return None}
        }
    }

    pub fn get_id(&self, id_ref: u32) -> Option<String> {
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

    pub fn get_ref(&self, id_string: String) -> Option<u32> {
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

// used for canister logging.
#[derive(CandidType, Deserialize, Debug, Default, Clone)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

// Stable storage of canister settings. 
#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct CanisterSettings {
    //pub transaction_fee: u64,
    //pub target_canister: IDKey,
    pub canister_name: IDKey, 
    pub stats_are_public: bool,
    pub authorised: SVec<IDKey>,
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
                        let rtn: String = String::from("Admin Added");
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
        return format!("are_stats_public updated to: {}", are_stats_public);
    }

}

// struct for returning memory query
#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
    pub memory: u64,
    pub heap_memory: u64,
}

#[derive(CandidType, Deserialize, Clone, Default, Debug)]
pub struct FraudReport {
    pub account: String, 
    pub evidence: String,
    pub urls: String,
    pub submitter: String
}

#[derive(CandidType, Deserialize, Clone, Debug)]
pub struct FlagData {
    pub flags: Vec<Flags>
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub enum Flags {
    GenesisFlag(GenesisFlag),
    FraudFlag(FraudFlag),
    MixerFlag(MixerFlag),
    CommunityFlag(CommunityFlag),
    SARFlag(SARFlag)
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GenesisFlag {
    pub id: String, 
    pub flag_from: u64,
    pub time_added: u64,
    pub text: String,
}

#[derive(CandidType, Deserialize, Clone,Debug, PartialEq, Eq)]
pub struct FraudFlag {
    pub id: String, 
    pub flag_from: u64,
    pub time_added: u64,
    pub text: String,
    pub link: String,
    pub flagged_by: String,
}

#[derive(CandidType, Deserialize, Clone,Debug, PartialEq, Eq)]
pub struct SARFlag {
    pub id: String, 
    pub flag_from: u64,
    pub time_added: u64,
    pub text: String,
    pub link: String,
    pub flagged_by: String,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MixerFlag {
    pub id: String, 
    pub flag_from: u64,
    pub time_added: u64,
    pub text: String,
    pub level: u8, // 0 is directly linked, 1 = 1 step removed. IE funds a directly linked account.. etc
}

#[derive(CandidType, Deserialize, Default, Clone, Debug, PartialEq, Eq)]
pub struct FlagStats {
    pub genesis_count: u64,
    pub fraud_count: u64,
    pub mixer_count: u64,
    pub community_count: u64,
    pub sar_count: u64
}

// Input format for add multiple mixer flags Vec<MixerLinkInput>
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MixerLinkInput {
    pub id: String, 
    pub from: u64,
    pub text: String,
    pub level: u8, 
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct CommunityFlag {
    pub id: String, 
    pub flag_from: u64,
    pub time_added: u64,
    pub text: String,
    pub link: String,
    pub number_of_flags: u32,
}
