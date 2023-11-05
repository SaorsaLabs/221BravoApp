use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::ops::DerefMut;
use candid::{CandidType, encode_one, decode_one};
use ic_stable_memory::collections::SVec;
use serde::{ Deserialize, Serialize };
use ic_stable_memory::{
    retrieve_custom_data, stable_memory_init, stable_memory_post_upgrade,
    stable_memory_pre_upgrade, store_custom_data, SBox,
};
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType, CandidAsDynSizeBytes};
use crate::constants::MAX_LINKED_ACS_TO_RETURN;
use crate::custom_types::{ 
    LogEntry, CanisterSettings, BlockHolder, ProcessedTX, 
    Directory, SmallTX, AccountTree, IDKey, Overview, LinkData, FullDataResponseRaw, LinkDataResponse, FullDataResponse, WorkingStats };
use crate::utils::{string_to_idkey, log};
use ic_cdk_timers::TimerId;

// [][] ---------------------------------------- [][]
// [][] --- Main Stable and Runtime Elements --- [][]
// [][] ---------------------------------------- [][]

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub canister_data: CanisterSettings,
    pub processed_data: AccountTree,
    pub directory_data: Directory,
}
impl Main {
    pub fn set_target_canisters_and_fee(&mut self, target_id: String, store_id: String, self_id: String, fee: u64){
        let targ: IDKey = string_to_idkey(&target_id).unwrap();
        let store: IDKey = string_to_idkey(&store_id).unwrap();
        let self_idk:IDKey = string_to_idkey(&self_id).unwrap();

        self.canister_data.target_canister = targ;
        self.canister_data.stx_store_canister = store;
        self.canister_data.self_canister = self_idk;
        self.canister_data.target_canister_locked = true;
        self.processed_data.transaction_fee = fee;
    }

    pub fn get_overview_by_id(&self, id_string: &String) -> Option<Overview> {
        match self.directory_data.get_ref(id_string) {
            Some(ref_value) => {
                match self.processed_data.accounts.get(&ref_value) {
                    Some(ac_value) => { 
                        let ov = Overview{
                            first_active: ac_value.overview.first_active,
                            last_active: ac_value.overview.last_active,
                            sent: ac_value.overview.sent,
                            received: ac_value.overview.received,
                            balance: ac_value.overview.balance,
                        };
                        return Some(ov);
                    },
                    None => {return None}
                }
            },
            None => { return None },
        } 
    }

    pub fn get_transactions_by_id(&self, id_string: &String) -> Option<Vec<u32>> {
        match self.directory_data.get_ref(id_string) {
            Some(ref_value) => {
                match self.processed_data.accounts.get(&ref_value) {
                    Some(ac_value) => { 
                        let mut ret: Vec<u32> = Vec::new();
                        for tx in ac_value.data.blocks.iter() {
                            ret.push(*tx);
                        }
                        return Some(ret);
                    },
                    None => {return None}
                }
            },
            None => { return None },
        } 
    }

    pub fn get_fulldata_by_id_raw(&self, id_string: &String) -> Option<FullDataResponseRaw> {
        let ac_idkey = string_to_idkey(&id_string);
        match self.directory_data.get_ref(id_string) {
            Some(ref_value) => {
                match self.processed_data.accounts.get(&ref_value) {
                    Some(ac_value) => { 
                        let ov = Overview{
                            first_active: ac_value.overview.first_active,
                            last_active: ac_value.overview.last_active,
                            sent: ac_value.overview.sent,
                            received: ac_value.overview.received,
                            balance: ac_value.overview.balance,
                        };
                        let mut links:Vec<LinkData> = Vec::new();
                        for ld in ac_value.data.links.iter() {
                            let ld2 = LinkData{
                                linked_from: ld.1.linked_from,
                                linked_id: ld.1.linked_id,
                                number_txs: ld.1.number_txs,
                                gross: ld.1.gross,
                                net: ld.1.net,
                            };
                            links.push(ld2);  
                        }
                        let mut blocks:Vec<u32> = Vec::new();
                        for bd in ac_value.data.blocks.iter() {
                            blocks.push(*bd);
                        }
                        let res = FullDataResponseRaw{
                            account_ref: ref_value,
                            overview: ov,
                            links,
                            blocks,
                        };
                        return Some(res);
                    },
                    None => {return None}
                }
            },
            None => { return None }
        }
    }

    pub fn get_fulldata_by_id(&self, id_string: &String) -> Option<FullDataResponse> {
        let ac_idkey = string_to_idkey(&id_string);
        match self.directory_data.get_ref(id_string) {
            Some(ref_value) => {
                match self.processed_data.accounts.get(&ref_value) {
                    Some(ac_value) => { 
                        let ov = Overview{
                            first_active: ac_value.overview.first_active,
                            last_active: ac_value.overview.last_active,
                            sent: ac_value.overview.sent,
                            received: ac_value.overview.received,
                            balance: ac_value.overview.balance,
                        };
                        let mut links:Vec<LinkDataResponse> = Vec::new();
                        // 
                        for ld in ac_value.data.links.iter() {
                            match self.directory_data.get_id(&ld.1.linked_id) {
                                Some(id_string) => {
                                    let ld2 = LinkDataResponse{
                                        linked_from: ld.1.linked_from,
                                        linked_id: id_string,
                                        number_txs: ld.1.number_txs,
                                        gross: ld.1.gross,
                                        net: ld.1.net,
                                    };
                                    links.push(ld2);  
                                }
                                None => {
        
                                }
                            }   
                        }
                        // trim links if too many
                        if links.len() > MAX_LINKED_ACS_TO_RETURN { links.truncate(MAX_LINKED_ACS_TO_RETURN); }
                        // blocks
                        let mut blocks:Vec<ProcessedTX> = Vec::new(); // fetch + process in follow up call. 
                        let res = FullDataResponse{
                            account_ref: id_string.clone(),
                            overview: ov,
                            links,
                            blocks,
                        };
                        return Some(res);
                    },
                    None => {return None}
                }
            },
            None => { return None }
        }
    }

    pub fn get_overview_by_ref(&self, id_ref: &u32) -> Option<Overview> {
        match self.processed_data.accounts.get(&id_ref) {
            Some(ac_value) => { 
                let ov = Overview{
                    first_active: ac_value.overview.first_active,
                    last_active: ac_value.overview.last_active,
                    sent: ac_value.overview.sent,
                    received: ac_value.overview.received,
                    balance: ac_value.overview.balance,
                };
                return Some(ov);
            },
            None => {return None}
        }
    }

    pub fn get_transactions_by_ref(&self, id_ref: &u32) -> Option<Vec<u32>> {
        match self.processed_data.accounts.get(&id_ref) {
            Some(ac_value) => { 
                let mut ret: Vec<u32> = Vec::new();
                for tx in ac_value.data.blocks.iter() {
                    ret.push(*tx);
                }
                return Some(ret);
            },
            None => {return None}
        }
    }

    // linked acs/ blocks as refs not decoded to account string.
    pub fn get_fulldata_by_ref_raw(&self, id_ref: &u32) -> Option<FullDataResponseRaw> {
        match self.processed_data.accounts.get(&id_ref) {
            Some(ac_value) => { 
                let ov = Overview{
                    first_active: ac_value.overview.first_active,
                    last_active: ac_value.overview.last_active,
                    sent: ac_value.overview.sent,
                    received: ac_value.overview.received,
                    balance: ac_value.overview.balance,
                };
                let mut links:Vec<LinkData> = Vec::new();
                for ld in ac_value.data.links.iter() {
                    let ld2 = LinkData{
                        linked_from: ld.1.linked_from,
                        linked_id: ld.1.linked_id,
                        number_txs: ld.1.number_txs,
                        gross: ld.1.gross,
                        net: ld.1.net,
                    };
                    links.push(ld2);  
                }
                let mut blocks:Vec<u32> = Vec::new();
                for bd in ac_value.data.blocks.iter() {
                    blocks.push(*bd);
                }
                let res = FullDataResponseRaw{
                    account_ref: id_ref.clone(),
                    overview: ov,
                    links,
                    blocks,
                };
                return Some(res);
            },
            None => {return None}
        }
    }

    // blocks need to be fetched in follow up call to block-store and processed. 
    pub fn get_fulldata_by_ref(&self, id_ref: &u32) -> Option<FullDataResponse> {
        match self.processed_data.accounts.get(&id_ref) {
            Some(ac_value) => { 
                let ov = Overview{
                    first_active: ac_value.overview.first_active,
                    last_active: ac_value.overview.last_active,
                    sent: ac_value.overview.sent,
                    received: ac_value.overview.received,
                    balance: ac_value.overview.balance,
                };
                let mut links:Vec<LinkDataResponse> = Vec::new();
                for ld in ac_value.data.links.iter() {
                    match self.directory_data.get_id(&ld.1.linked_id) {
                        Some(id_string) => {
                            let ld2 = LinkDataResponse{
                                linked_from: ld.1.linked_from,
                                linked_id: id_string,
                                number_txs: ld.1.number_txs,
                                gross: ld.1.gross,
                                net: ld.1.net,
                            };
                            links.push(ld2);  
                        }
                        None => {

                        }
                    }   
                }
                // trim links if over Max to return
                if links.len() > MAX_LINKED_ACS_TO_RETURN { links.truncate(MAX_LINKED_ACS_TO_RETURN); }
                // blocks
                match self.directory_data.get_id(&id_ref) {
                    Some(ac_string) => {
                        let mut blocks:Vec<ProcessedTX> = Vec::new(); // fetch blocks in follow up call to block-store. 
                        let res = FullDataResponse{
                            account_ref: ac_string,
                            overview: ov,
                            links,
                            blocks,
                        };
                        return Some(res);
                    },
                    None => { return None; },
                }
            },
            None => {return None}
        }
    }
}

#[derive(CandidType, Default, Deserialize, Clone)]
pub struct RuntimeState{
    pub latest_txs: BlockHolder,
    pub canister_logs: Vec<LogEntry>,
    pub temp_vec_ptx: Vec<ProcessedTX>,
    pub temp_vec_stx: Vec<SmallTX>
}

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default(); 
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
    pub static TIMER_STATE: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}

pub fn state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data = Main::default();
    let default_authorised = string_to_idkey(&"DEV_PRINCIPAL_HERE".to_string()).unwrap();
    let saorsa_admin:IDKey = string_to_idkey(&"DEV_PRINCIPAL_HERE".to_string()).unwrap();
    let default_canister_name = string_to_idkey(&"Name Me Please!".to_string()).unwrap();
    stable_data.canister_data.authorised.push(default_authorised).expect("Out of memory");
    stable_data.canister_data.authorised.push(saorsa_admin.clone()).expect("Out of memory");
    stable_data.canister_data.admin.push(saorsa_admin).expect("Out of memory");
    stable_data.canister_data.canister_name = default_canister_name;
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let mut runtime_state = RuntimeState::default();
    runtime_state.latest_txs.init();
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
    log("Canister Initialised");
}

pub fn state_pre_upgrade(){
    // Check if busy to prevent upgrade during download of block chunks. 
    STABLE_STATE.with(|s|{
       if s.borrow().as_ref().unwrap().canister_data.working_stats.is_busy == true {
        ic_cdk::trap("Canister is busy with a task. Upgrade stopped.");
       }
    });
    
    // Stable Storage
    let state: Main = STABLE_STATE.with(|s| s.borrow_mut().take().unwrap());
    let boxed_state = SBox::new(state).expect("Out of memory");
    store_custom_data(0, boxed_state);

    // Runtime Storage
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
    log("Canister has been upgraded");
}