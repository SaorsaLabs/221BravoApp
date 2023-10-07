use std::cell::RefCell;
use std::collections::BTreeMap;
use std::ops::DerefMut;
use candid::{CandidType, encode_one, decode_one};
use serde::{ Deserialize, Serialize };
use ic_stable_memory::{
    retrieve_custom_data, stable_memory_init, stable_memory_post_upgrade,
    stable_memory_pre_upgrade, store_custom_data, SBox,
};
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};
use crate::custom_types::{ LogEntry, CanisterSettings, IDKey, FraudReport, FlagData, 
    Flags, MixerLinkInput, MixerFlag, FlagStats};
use crate::utils::{string_to_idkey, log};

// [][] ---------------------------------------- [][]
// [][] --- Main Stable and Runtime Elements --- [][]
// [][] ---------------------------------------- [][]

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub canister_data: CanisterSettings,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct RuntimeState{
    pub canister_logs: Vec<LogEntry>,
    pub fraud_reports: Vec<FraudReport>,
    pub flag_tree: BTreeMap<String, FlagData>,
    pub flag_stats: FlagStats,
}
impl RuntimeState {
    // Fraud Reports
    pub fn add_report(&mut self, account:String, evidence:String, urls:String, submitter:String) -> String {
        let data:FraudReport = FraudReport {
            account, 
            evidence,
            urls,
            submitter
        };
        self.fraud_reports.push(data);
        return "Fraud Report Added".to_string();
    }
    pub fn remove_report(&mut self, index: usize) -> String {
        self.fraud_reports.remove(index);
        return "Fraud Report Removed".to_string();
    }
    pub fn get_all_reports(&self) -> Vec<FraudReport> {
        return self.fraud_reports.clone();
    }

    // Flags BTREE
    pub fn add_flag(&mut self, account: String, flag: Flags) -> String{
        match self.flag_tree.get(&account){
            Some(value) => {
                // account exists
                let mut flag_vec: Vec<Flags> = value.flags.clone();
                flag_vec.push(flag.clone());
                let data: FlagData = FlagData { flags: flag_vec };
                self.flag_tree.insert(account, data);
                self.add_to_flag_stats(flag);
                return "Flag Added".to_string();
            },
            None => {
                // account doesn't exist
                let mut flag_vec:Vec<Flags> = Vec::new();
                flag_vec.push(flag.clone());
                let data: FlagData = FlagData { flags: flag_vec };
                self.flag_tree.insert(account, data);
                self.add_to_flag_stats(flag);
                return "Flag Added".to_string();
            },
        }
    }
    pub fn remove_flag(&mut self, account: String, variant: Flags) -> String {
        if let Some(flag_data) = self.flag_tree.get(&account) {
            let mut temp_vec: Vec<Flags> = flag_data.flags.clone();
            match variant {
                Flags::GenesisFlag(_) => {
                    temp_vec.retain(|flag| !matches!(flag, Flags::GenesisFlag(_)));
                    self.remove_from_flag_stats(variant);
                }
                Flags::FraudFlag(_) => {
                    temp_vec.retain(|flag| !matches!(flag, Flags::FraudFlag(_)));
                    self.remove_from_flag_stats(variant);
                }
                Flags::MixerFlag(_) => {
                    temp_vec.retain(|flag| !matches!(flag, Flags::MixerFlag(_)));
                    self.remove_from_flag_stats(variant);
                }
                Flags::CommunityFlag(_) => {
                    temp_vec.retain(|flag| !matches!(flag, Flags::CommunityFlag(_)));
                    self.remove_from_flag_stats(variant);
                }
                Flags::SARFlag(_) => {
                    temp_vec.retain(|flag| !matches!(flag, Flags::SARFlag(_)));
                    self.remove_from_flag_stats(variant);
                }
                _ => {return "flag not found on account!".to_string()},
            }
            let data: FlagData = FlagData { flags: temp_vec };
            self.flag_tree.insert(account, data);
            return "flag removed".to_string();
        } else {
            return "can't remove flag - account doesn't exist".to_string();
        }
    }
    pub fn get_all_flags(&self, account: String) -> Option<Vec<Flags>> {
        if let Some(flag_data) = self.flag_tree.get(&account) {
            return Some(flag_data.flags.clone());
        } else {
            return None;
        }
    }

    // add multi mixer flags
    pub fn add_multi_mixer_flags(&mut self, input_vec: Vec<MixerLinkInput>) -> String {

        for ML in input_vec {
            match self.flag_tree.get(&ML.id){
                Some(value) => {
                    // check if flagged already 
                    let mut is_already_flagged = false;
                    for FD in &value.flags {
                        match FD {
                            Flags::MixerFlag(v) => {
                                // check flag source is the same (eg Spinner)
                                if v.text == ML.text {is_already_flagged = true}
                            },
                            _ => {},
                        }
                    }
                    // exists but no mixer flag
                    if is_already_flagged == false {
                        let mut flag_vec: Vec<Flags> = value.flags.clone();
                        let nano_time = ic_cdk::api::time();
                        let mixer_flag: Flags = Flags::MixerFlag( MixerFlag{
                            id: ML.id.clone(), 
                            flag_from: ML.from,
                            time_added: nano_time,
                            text: ML.text,
                            level: ML.level,
                        });
                        flag_vec.push(mixer_flag);
                        let data: FlagData = FlagData { flags: flag_vec };
                        self.flag_tree.insert(ML.id, data);
                        self.flag_stats.mixer_count += 1;
                    } 
                },
                None => {
                    // account doesn't exist
                    let mut flag_vec:Vec<Flags> = Vec::new();
                    let nano_time = ic_cdk::api::time();
                    let mixer_flag: Flags = Flags::MixerFlag( MixerFlag{
                        id: ML.id.clone(), 
                        flag_from: ML.from,
                        time_added: nano_time,
                        text: ML.text,
                        level: ML.level,
                    });
                    flag_vec.push(mixer_flag);
                    let data: FlagData = FlagData { flags: flag_vec };
                    self.flag_tree.insert(ML.id, data);
                    self.flag_stats.mixer_count += 1;
                },
            }
        }
        return "Mixer Vec Processed".to_string();
    }

    // get flag stats 
    pub fn get_flag_stats(&self) -> FlagStats {
        return self.flag_stats.clone();
    }

    pub fn add_to_flag_stats(&mut self, flag: Flags){
        match flag {
            Flags::GenesisFlag(_) => {
               self.flag_stats.genesis_count += 1;
            }
            Flags::FraudFlag(_) => {
                self.flag_stats.fraud_count += 1;
            }
            Flags::MixerFlag(_) => {
                self.flag_stats.mixer_count += 1;
            }
            Flags::CommunityFlag(_) => {
                self.flag_stats.community_count += 1;
            }
            Flags::SARFlag(_) => {
                self.flag_stats.sar_count += 1;
            }
            _ => {},
        }
    }

    pub fn remove_from_flag_stats(&mut self, flag: Flags){
        match flag {
            Flags::GenesisFlag(_) => {
               self.flag_stats.genesis_count -= 1;
            }
            Flags::FraudFlag(_) => {
                self.flag_stats.fraud_count -= 1;
            }
            Flags::MixerFlag(_) => {
                self.flag_stats.mixer_count -= 1;
            }
            Flags::CommunityFlag(_) => {
                self.flag_stats.community_count -= 1;
            }
            Flags::SARFlag(_) => {
                self.flag_stats.sar_count -= 1;
            }
            _ => {},
        }
    }

}

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

pub fn state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data = Main::default();
    let default_admin: IDKey = string_to_idkey(&"ADMIN_PRINCIPAL_HERE".to_string()).unwrap();
    let frontend: IDKey = string_to_idkey(&"FRONTEND_PRINCIPAL_HERE".to_string()).unwrap(); 
    let default_canister_name = string_to_idkey(&"Name Me Please!".to_string()).unwrap();
    stable_data.canister_data.authorised.push(default_admin).expect("Out of memory");
    stable_data.canister_data.authorised.push(frontend).expect("Out of memory");
    stable_data.canister_data.canister_name = default_canister_name;
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let runtime_date = RuntimeState::default();
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_date;
    });
    log("Canister Initialised");
}

pub fn state_pre_upgrade(){
    // stable state
    let state: Main = STABLE_STATE.with(|s| s.borrow_mut().take().unwrap());
    let boxed_state = SBox::new(state).expect("Out of memory");
    store_custom_data(0, boxed_state);

    // runtime state
    let rstate = RUNTIME_STATE.with(|s|{s.borrow_mut().to_owned()});
    let bytes = encode_one(rstate).expect("Unable to candid encode");
    let boxed_bytes = SBox::new(bytes).expect("Out of memory");
    store_custom_data(1, boxed_bytes);

    stable_memory_pre_upgrade().expect("Out of memory");
}

pub fn state_post_upgrade(){
    stable_memory_post_upgrade();
    let state: Main = retrieve_custom_data::<Main>(0).unwrap().into_inner();
    STABLE_STATE.with(|s| {
      *s.borrow_mut() = Some(state);
    });

    // Runtime Storage 
    let bytes: Vec<u8> = retrieve_custom_data::<Vec<u8>>(1).unwrap().into_inner();
    let rstate: RuntimeState = decode_one(&bytes).expect("Unable to candid decode");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = rstate;
    });
}