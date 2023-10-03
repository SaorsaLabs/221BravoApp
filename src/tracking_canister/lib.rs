mod state_management;
mod custom_types;
mod constants;
mod utils;
mod tx_tracking;
mod test_data;
mod exchanges;

use std::time::Duration;

use exchanges::{calculate_exchange_overview, fetch_exchange_data};
use ic_cdk_macros::*;
use ic_cdk_timers::TimerId;
use state_management::{ state_init, state_pre_upgrade, state_post_upgrade, STABLE_STATE, RUNTIME_STATE, TIMER_STATE };
use custom_types::{  LogEntry, MemoryData, MixerWorkingStats, ExchangeCollection };
use tx_tracking::{ check_for_new_spinner_links, process_mixer_flag_que};
use utils::log;

// [][] ---------------- [][]
// [][] ---  Methods --- [][]
// [][] ---------------- [][]
#[query]
fn get_mixer_workings_stats() -> MixerWorkingStats {
    // check auth
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{
        s.borrow().spinner_tracking.get_working_stats()
    })
}

#[query]
fn get_exchange_data() -> ExchangeCollection {
    // check auth
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|s|{
        s.borrow().exchange_tracking.clone()
    })
}

// [][] ----------------------- [][]
// [][] --- Timer Functions --- [][]
// [][] ----------------------- [][]
#[update]
fn stop_all_timers() -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // clear timers
    TIMER_STATE.with(|timer_ids| {
        let vec1: &mut std::cell::RefMut<Vec<TimerId>> = &mut timer_ids.borrow_mut();
        for i in vec1.iter() {
            ic_cdk_timers::clear_timer(*i);
        }
        vec1.clear();
    });
    
    // update mixer timer
    STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().canister_data.mixer_timer_active = false
    });

    // update mixer timer
    STABLE_STATE.with(|s|{
        s.borrow_mut().as_mut().unwrap().canister_data.exchange_timer_active = false
    });

    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]
fn check_and_start_mixer_timer(secs: u64) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });

    let ret: String;
    let is_running = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.mixer_timer_active
    });

    if is_running == true {
        ret = String::from("Mixer timer is already running");
    } else {
        start_mixer_timer(secs);
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.mixer_timer_active = true
        });
        ret = String::from("Mixer timer has been started");
        log("[][] ---- Starting Mixer Timer ---- [][]");
    }
    return ret;
}

fn start_mixer_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_mixer_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_mixer_processing() {
    // check if busy
    let busy = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.is_busy.clone()
    });
    if busy == true {
        return;
    } else {
        // set busy 
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.is_busy = true;
        });

        check_for_new_spinner_links().await;
        process_mixer_flag_que().await;

        // set not busy
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.is_busy = false;
        });
    }
}

#[update]
fn check_and_start_exchange_timer(secs: u64) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });

    let ret: String;
    let is_running = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.exchange_timer_active
    });

    if is_running == true {
        ret = String::from("Mixer timer is already running");
    } else {
        start_exchange_timer(secs);
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.exchange_timer_active = true
        });
        ret = String::from("Exchange timer has been started");
        log("[][] ---- Starting Exchange Timer ---- [][]");
    }
    return ret;
}

fn start_exchange_timer(secs: u64) {
    let secs = Duration::from_secs(secs);
    let timer_id = ic_cdk_timers::set_timer_interval(secs, ||
        ic_cdk::spawn(schedule_exchange_processing())
    );
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_exchange_processing() {
    // check if busy
    let busy = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.is_busy.clone()
    });
    if busy == true {
        return;
    } else {
        // set busy 
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.is_busy = true;
        });

        let fetch_data = fetch_exchange_data().await;
        let calc_data = calculate_exchange_overview(fetch_data);
        
        RUNTIME_STATE.with(|s|{
            s.borrow_mut().exchange_tracking = calc_data;
        });

        // set not busy
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.is_busy = false;
        });
    }
}

// [][] --------------------------- [][]
// [][] --- Canister Management --- [][]
// [][] --------------------------- [][]
#[update]
fn add_authorised(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .remove_authorised(principal_id)
    })
}

#[update] // not visible to public
fn add_admin(principal_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // add admin
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .add_admin(principal_id)
    })
}

#[update] // not visible to public
fn remove_admin(principal_id: String) -> String {
   // check admin
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_admin(ic_cdk::caller().to_text());
    });
    // remove admin
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .remove_admin(principal_id)
    })
}

#[query] // not visible to public
fn get_all_admins() -> Vec<String> {
   // check admin
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_admin(ic_cdk::caller().to_text());
    });
    // get all admins
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_all_admins();
        return ret;
    })
}

#[update]
fn set_canister_name(name: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // set canister name 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_canister_name(name)
    })
}

#[update]
fn set_stats_public(are_stats_public: bool) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // set stats public
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_stats_public(are_stats_public)
    })
}

#[query]
fn get_all_authorised() -> Vec<String> {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // get all authorised
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_all_authorised();
        return ret;
    })
}

#[query]
fn get_canister_name() -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get canister name
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_canister_name();
        return ret;
    })
}

#[query]
fn are_stats_public() -> bool {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // check if stats are public
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.are_stats_public();
        return ret;
    })
}

#[query]
fn get_canister_logs() -> Vec<LogEntry> {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|{
        state.borrow().canister_logs.to_owned()
    })
}

#[query]
fn get_cycles_balance() -> u64 {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // get cycles balance
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
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


// [][] -------------------------------- [][]
// [][] --- Canister Setup/ Upgrades --- [][]
// [][] -------------------------------- [][]
#[init]
fn init() {
    state_init();
}

#[pre_upgrade]
fn pre_upgrade() {
    state_pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    state_post_upgrade();
}



mod tests {
    use crate::{test_data::{ test_mixer_data }, custom_types::MixerLink};
    use crate::{tx_tracking::{ get_unique_mixer_flags }};

    #[test]
    fn test_get_unique_mixer_flags(){

        let data: Vec<MixerLink> = test_mixer_data();
        let process = get_unique_mixer_flags(data);
        //println!("{:?}", process); => manually checked and OK.
        assert_eq!(process.len(), 3);
        assert_eq!(process[0].from, 100); // id1 is 100 (lowest of id1 entries);
    }
}