use std::cell::RefCell;
use std::ops::DerefMut;
use candid::{CandidType, decode_one, encode_one};
use serde::{ Deserialize, Serialize };
use ic_stable_memory::{
    retrieve_custom_data, stable_memory_init, stable_memory_post_upgrade,
    stable_memory_pre_upgrade, store_custom_data, SBox,
};
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};
use crate::custom_types::{LogEntry, CanisterSettings, IDKey, SpinnerTracking, ExchangeCollection};
use crate::utils::{string_to_idkey, log};
use ic_cdk_timers::TimerId;

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
    pub spinner_tracking: SpinnerTracking,
    pub exchange_tracking: ExchangeCollection,
}

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
    pub static TIMER_STATE: RefCell<Vec<TimerId>> = RefCell::new(Vec::new());
}

pub fn state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data: Main = Main::default();
    let saorsa_admin:IDKey = string_to_idkey(&"ADMIN_PRINCIPAL_HERE".to_string()).unwrap();
    let default_canister_name = string_to_idkey(&"Name Me Please!".to_string()).unwrap();
    stable_data.canister_data.authorised.push(saorsa_admin.clone()).expect("Out of memory");
    stable_data.canister_data.admin.push(saorsa_admin).expect("Out of memory");
    stable_data.canister_data.canister_name = default_canister_name;
    STABLE_STATE.with(|state: &RefCell<Option<Main>>| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let runtime_sate: RuntimeState = RuntimeState::default();
    RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
        *state.borrow_mut() = runtime_sate;
    });

    // init spinner tracking 
    RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
        state.borrow_mut().spinner_tracking.init()
    });

    log("Canister Initialised");
}

pub fn state_pre_upgrade(){
    let state: Main = STABLE_STATE.with(|s: &RefCell<Option<Main>>| s.borrow_mut().take().unwrap());
    let boxed_state: SBox<Main> = SBox::new(state).expect("Out of memory");
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
    STABLE_STATE.with(|s: &RefCell<Option<Main>>| {
      *s.borrow_mut() = Some(state);
    });

    // Runtime Storage 
    let bytes: Vec<u8> = retrieve_custom_data::<Vec<u8>>(1).unwrap().into_inner();
    let rstate: RuntimeState = decode_one(&bytes).expect("Unable to candid decode");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = rstate;
    });
    log("Canister has been ungraded");
}