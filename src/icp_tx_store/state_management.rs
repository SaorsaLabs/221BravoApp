use std::cell::RefCell;
use std::ops::DerefMut;
use candid::CandidType;
use serde::{ Deserialize, Serialize };
use ic_stable_memory::{
    retrieve_custom_data, stable_memory_init, stable_memory_post_upgrade,
    stable_memory_pre_upgrade, store_custom_data, SBox,
};
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};
use crate::custom_types::{LogEntry, CanisterSettings, TxStore, IDKey};
use crate::utils::{string_to_idkey, log};

// [][] ---------------------------------------- [][]
// [][] --- Main Stable and Runtime Elements --- [][]
// [][] ---------------------------------------- [][]

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub canister_data: CanisterSettings,
    pub tx_store: TxStore,
}

#[derive(CandidType, Default, Clone)]
pub struct RuntimeState{
    pub canister_logs: Vec<LogEntry>,
}

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

pub fn state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data: Main = Main::default();
    let default_admin: IDKey = string_to_idkey(&"2vxsx-fae".to_string()).unwrap();
    let saorsa_admin:IDKey = string_to_idkey(&"e3uc3-o4g2j-bdkhp-yi4p4-wzfdy-glkas-zlhqf-n2jm2-ehxiv-fnjkc-2ae".to_string()).unwrap();
    let default_canister_name: IDKey = string_to_idkey(&"Name Me Please!".to_string()).unwrap();
    stable_data.canister_data.authorised.push(default_admin).expect("Out of memory");
    stable_data.canister_data.authorised.push(saorsa_admin).expect("Out of memory");
    stable_data.canister_data.canister_name = default_canister_name;
    STABLE_STATE.with(|state: &RefCell<Option<Main>>| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let runtime_date: RuntimeState = RuntimeState::default();
    RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
        *state.borrow_mut() = runtime_date;
    });
    log("Canister Initialised");
}

pub fn state_pre_upgrade(){
    let state: Main = STABLE_STATE.with(|s: &RefCell<Option<Main>>| s.borrow_mut().take().unwrap());
    let boxed_state: SBox<Main> = SBox::new(state).expect("Out of memory");
    store_custom_data(0, boxed_state);
    stable_memory_pre_upgrade().expect("Out of memory");
}

pub fn state_post_upgrade(){
    stable_memory_post_upgrade();
    let state: Main = retrieve_custom_data::<Main>(0).unwrap().into_inner();
    STABLE_STATE.with(|s: &RefCell<Option<Main>>| {
      *s.borrow_mut() = Some(state);
    });
}