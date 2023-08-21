mod state_management;
mod custom_types;
mod constants;
mod utils;
mod fetch_data;
mod process_data;
mod test_data;

use std::time::Duration;

use ic_cdk::api::instruction_counter;
use ic_cdk_macros::*;
use ic_cdk_timers::TimerId;
use state_management::{ state_init, state_pre_upgrade, state_post_upgrade, STABLE_STATE, RUNTIME_STATE, TIMER_STATE };
use custom_types::{ MemoryData, ProcessedTX, LogEntry, FullDataResponse, Overview, FullDataResponseRaw, LinkDataResponse, LinkData, WorkingStats,};
use utils::{remove_none_ptx_values, string_to_idkey, log};
use fetch_data::*;
use process_data::{ send_stx_to_store, process_smtx_to_index, process_to_small_tx};
use constants::MAX_BLOCKS_TO_RETURN;


// [][] ---------------- [][]
// [][] ---  Methods --- [][]
// [][] ---------------- [][]

// get block by block number ✔️
// get multi blocks by block number ✔️
// get latest blocks (x upto 20k) ✔️
// get overview ✔️
// get full account raw/ processed ✔️
// get ac links (+Raw) ✔️
// get ac transactions ✔️
// get working stats
// set target canisters ✔️
// ref to id?  ✔️
// id to ref? ✔️
// set timer ✔️
// stop timer ✔️


// Set target canister and tx store canister
#[update]
async fn set_target_canister(principal_id: String, store_id: String) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });
    // set target and save fee
    let ret = impl_set_target_canister(principal_id, store_id).await;
    return ret;
}

// get latest blocks (from this canister's cache not tx store. Max 20K)
#[query]
fn get_latest_transactions(number_txs: u32) -> Vec<ProcessedTX> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|{
        state.borrow().latest_txs.get_txs(number_txs as usize)
    })
}

// get single tx from store (all accounts)
#[update]//#[query(composite = true)]
async fn get_tx(block: u32) -> Option<ProcessedTX> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let x:Option<ProcessedTX> = get_single_tx_from_store(block).await;
    return x;
}

// get multiple tx from store (all accounts)
#[update]//#[query(composite = true)]
async fn get_multiple_tx(block_vec: Vec<u32>) -> Vec<ProcessedTX> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    // fetch from tx store
    let res: Vec<Option<ProcessedTX>> = get_multiple_txs_from_store(block_vec).await;
    // remove any NONE
    let ret: Vec<ProcessedTX> = remove_none_ptx_values(res);
    return ret;
}

// get full account info by u32 ref
#[update]//#[query(composite = true)]
async fn get_full_from_ref(id_ref: u32) -> Option<FullDataResponse> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });

    // get blocks 
    let block_refs = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_transactions_by_ref(&id_ref)
    });
    match block_refs {
        Some(mut vec_refs) => {
            // trim blocks to 
            vec_refs.reverse();
            vec_refs.truncate(MAX_BLOCKS_TO_RETURN);
            // fetch blocks
            let ptx: Vec<Option<ProcessedTX>> = get_multiple_txs_from_store(vec_refs).await;
            let ptx2 = remove_none_ptx_values(ptx);
            // get rest of data
            let overview_and_links: Option<FullDataResponse> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_ref(&id_ref)
            });
            match overview_and_links {
                Some(ovlnk) => {
                    let mut ret: FullDataResponse = ovlnk;
                    ret.blocks = ptx2; 
                    return Some(ret);
                },
                None => {
                     return None
                } 
            }
        },
        None => {
             return None
        } 
    }
}

// full response from u32 Ref in unprocessed format. Note blocks aren't reversed. 
#[query]//#[query(composite = true)]
async fn get_full_from_ref_raw(id_ref: u32) -> Option<FullDataResponseRaw> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_fulldata_by_ref_raw(&id_ref)
    })
}

// get full account info by ID (account string)
#[update]//#[query(composite = true)]
async fn get_full_from_id(id_string: String) -> Option<FullDataResponse> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    // get blocks 
    let block_refs = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_transactions_by_id(&id_string)
    });
    match block_refs {
        Some(mut vec_refs) => {
            // trim blocks to 
            vec_refs.reverse();
            vec_refs.truncate(MAX_BLOCKS_TO_RETURN);
            // fetch blocks
            let ptx: Vec<Option<ProcessedTX>> = get_multiple_txs_from_store(vec_refs).await;
            let ptx2 = remove_none_ptx_values(ptx);
            // get rest of data
            let overview_and_links: Option<FullDataResponse> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_id(&id_string)
            });
            match overview_and_links {
                Some(ovlnk) => {
                    let mut ret: FullDataResponse = ovlnk;
                    ret.blocks = ptx2; 
                    return Some(ret);
                },
                None => {return None} 
            }
        },
        None => {return None}
    }
}

// full response from ID String in unprocessed format. Note blocks aren't reversed.  
#[query]//#[query(composite = true)]
async fn get_full_from_id_raw(id_string: String) -> Option<FullDataResponseRaw> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_fulldata_by_id_raw(&id_string)
    })
}

// Overview by ID string
#[query]
fn get_overview_by_id(id_string: String) -> Option<Overview> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_overview_by_id(&id_string)        
    })
}

// Overview by u32 ref
#[query]
fn get_overview_by_ref(id_ref: u32) -> Option<Overview> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_overview_by_ref(&id_ref)        
    })
}

// Account Links by ID
#[query]
fn get_links_from_id(id_string: String) -> Option<Vec<LinkDataResponse>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let overview_and_links: Option<FullDataResponse> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_id(&id_string)
    });
    match overview_and_links {
        Some(v) => {
            return Some(v.links);
        },
        None => { return None}
    }
}

// Account Links by Ref
#[query]
fn get_links_from_ref(id_ref: u32) -> Option<Vec<LinkDataResponse>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let overview_and_links: Option<FullDataResponse> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_ref(&id_ref)
    });
    match overview_and_links {
        Some(v) => {
            return Some(v.links);
        },
        None => { return None}
    }
}

// Account Links by ID (RAW)
#[query]
fn get_links_from_id_raw(id_string: String) -> Option<Vec<LinkData>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    let overview_and_links: Option<FullDataResponseRaw> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_id_raw(&id_string)
    });
    match overview_and_links {
        Some(v) => {
            return Some(v.links);
        },
        None => { return None}
    }
}

// Account Links by Ref (RAW)
#[query]
fn get_links_from_ref_raw(id_ref: u32) -> Option<Vec<LinkData>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let overview_and_links: Option<FullDataResponseRaw> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_fulldata_by_ref_raw(&id_ref)
    });
    match overview_and_links {
        Some(v) => {
            return Some(v.links);
        },
        None => { return None}
    }
}

// Account transactions by ID (Max Return value - MAX_BLOCKS_TO_RETURN)
#[update]
async fn get_transactions_from_id(id_string: String) -> Option<Vec<ProcessedTX>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let block_refs: Option<Vec<u32>> = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_transactions_by_id(&id_string)
    });
    match block_refs {
        Some(mut vec_refs) => {
            // trim blocks to 
            vec_refs.truncate(MAX_BLOCKS_TO_RETURN);
            // fetch blocks
            let ptx: Vec<Option<ProcessedTX>> = get_multiple_txs_from_store(vec_refs).await;
            let ptx2: Vec<ProcessedTX> = remove_none_ptx_values(ptx);
            return Some(ptx2);
        },
        None => {return None}
    }
}

// Account transactions ID (raw)
#[query]
fn get_transactions_from_id_raw(id_string: String) -> Option<Vec<u32>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let txs: Option<Vec<u32>> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_transactions_by_id(&id_string)
    });
    match txs {
        Some(v) => {
            return Some(v);
        },
        None => { return None}
    }
}

// Account transactions by Ref (Max Return value - MAX_BLOCKS_TO_RETURN)
#[update]
async fn get_transactions_from_ref(id_ref: u32) -> Option<Vec<ProcessedTX>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let block_refs: Option<Vec<u32>> = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().get_transactions_by_ref(&id_ref)
    });
    match block_refs {
        Some(mut vec_refs) => {
            // trim blocks to 
            vec_refs.truncate(MAX_BLOCKS_TO_RETURN);
            // fetch blocks
            let ptx: Vec<Option<ProcessedTX>> = get_multiple_txs_from_store(vec_refs).await;
            let ptx2: Vec<ProcessedTX> = remove_none_ptx_values(ptx);
            return Some(ptx2);
        },
        None => {return None}
    }
}

// Account transactions by Ref (raw)
#[query]
fn get_transactions_from_ref_raw(id_ref: u32) -> Option<Vec<u32>> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });

    let txs: Option<Vec<u32>> = STABLE_STATE.with(|s|{
                s.borrow().as_ref().unwrap().get_transactions_by_ref(&id_ref)
    });
    match txs {
        Some(v) => {
            return Some(v);
        },
        None => { return None}
    }
}

// ID to Ref 
#[query]
fn get_id_from_ref(id_ref: u32) -> Option<String> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get ID
    let ID = STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().directory_data.get_id(&id_ref)  
    });
    match ID {
        Some(v) => { return Some(v)},
        None => {return None},
    }
}

// Ref to ID
#[query]
fn get_ref_from_id(id_string: String) -> Option<u32> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get ID
    let ID = STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().directory_data.get_ref(&id_string)  
    });
    match ID {
        Some(v) => { return Some(v)},
        None => {return None},
    }
}

// Get working stats
#[query]
fn get_working_stats() -> WorkingStats {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get working stats
    let ws: WorkingStats = STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data.working_stats.clone()
    });
    return ws;
}


// *************** Test Calls

// Download txs from ICP Ledger
#[update]
async fn test_call_1() -> String {
    download_and_process_txs().await;
    return "Complete".to_string();
}

// Add Test TXS from test_data.rs 
#[update]
fn test_call_1A(tx_store: String) -> String {
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().temp_vec_ptx = test_data::test_data();
     });

    STABLE_STATE.with(|s|{
    s.borrow_mut().as_mut().unwrap()
    .canister_data.stx_store_canister= string_to_idkey(&tx_store).unwrap() ;
    });
     return "Done".to_string();
}

// Test 2 - Process to Small TX
#[update]
async fn test_Small_tx_2() -> String {
    process_to_small_tx(); 
    let counter = instruction_counter();
    return format!("DONE! Instructions Used :: {},", counter);
}

#[update]
async fn test_send_tx() -> String {
    let x = send_stx_to_store().await;
    let counter = instruction_counter();
    return format!("DONE! Instructions Used :: {}, Res :: {}", counter, x);
}

#[update]
fn test_index_stx_3() -> String {
    process_smtx_to_index();
    let counter = instruction_counter();
    return format!("DONE! Instructions Used :: {}", counter);
}

#[query]
fn test_get_alldata(id_ref: u32) -> String {
    let ov = STABLE_STATE.with(|s|{s.borrow().as_ref().unwrap()
        .get_fulldata_by_ref_raw(&id_ref)});
    let res;
    match ov {
        Some(v) => {res = format!("{:?}", v)},
        None => {res = "nothing".to_string()}
    }
    let counter = instruction_counter();
    return format!("DONE! Instructions Used :: {}, Result {}", counter, res);
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
    // update working stats
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .working_stats.timer_set = false;
    });
    log("[][] ---- All timers stopped ---- [][]");
    return String::from("All timers stopped");
}

#[update]
fn check_and_start_processing_timer(secs: u64) -> String {
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
    });

    // check target canister is set
    let target_set = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.target_canister_locked
    });
    if target_set == false {
        ic_cdk::trap("Target Canister Not Set!");
    }

    let ret: String;
    let is_running = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.working_stats.timer_set
    });

    if is_running == true {
        ret = String::from("Processing timer is alraedy running");
    } else {
        start_processing_timer(secs);
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.working_stats.timer_set = true;
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
    TIMER_STATE.with(|timer_ids| timer_ids.borrow_mut().push(timer_id));
}

async fn schedule_data_processing() {
    // check if busy
    let busy = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.working_stats.is_busy
    });
    if busy == true {
        return;
    } else {
        // set busy 
        STABLE_STATE.with(|s|{
            s.borrow_mut().as_mut().unwrap().canister_data.working_stats.is_busy = true;
        });
        let working_stats =  STABLE_STATE.with(|s|{
            s.borrow().as_ref().unwrap().canister_data.working_stats.clone()
        });

        if working_stats.task_id == 0 {
            //[][] --- TASK 1 --- [][]
            download_and_process_txs().await;
            // set busy false
            STABLE_STATE.with(|s|{
                s.borrow_mut().as_mut().unwrap().canister_data.working_stats.is_busy = false;
            });
        } else if working_stats.task_id == 1 {
            //[][] --- TASK 2 --- [][]
            log("[][] --- Process SmallTX and send to store --- [][]");
            process_to_small_tx(); 
            process_smtx_to_index();
            let update_store = send_stx_to_store().await;
            match update_store {
                true => {
                    log("[][] --- Store Updated with latest blocks --- [][]");
                    // set busy false + go to next task
                    STABLE_STATE.with(|s|{
                        s.borrow_mut().as_mut().unwrap().canister_data.working_stats.is_busy = false;
                        s.borrow_mut().as_mut().unwrap().canister_data.working_stats.task_id = 0;
                    });
                },
                false => {
                    log("ERROR - Store canister returned an error. Check store canister logs!");
                }
            }
        }            
        // } else if working_stats.task_id == 2 {
        //     //[][] --- TASK 3 --- [][]
        //     // NOT USED
        //     // set busy false + go to next task
        //     STABLE_STATE.with(|s|{
        //         s.borrow_mut().as_mut().unwrap().canister_data.working_stats.is_busy = false;
        //         s.borrow_mut().as_mut().unwrap().canister_data.working_stats.task_id = 0;
        //     });
        //     log("[][] --- Index Updated with latest transactions --- [][]");
        // } 
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
    // remove authorised
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
    // set canister name 
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
    // check admin
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_admin(ic_cdk::caller().to_text());
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

// [][] ------------- [][]
// [][] --- Tests --- [][]
// [][] ------------- [][]


mod tests {
    use crate::custom_types::{ IDKey, FullDataResponseRaw, LinkData, ProcessedTX };
    use crate::process_data::{process_to_small_tx, process_smtx_to_index};
    use crate::state_management::{RUNTIME_STATE, STABLE_STATE};
    use crate::utils::{ string_to_idkey, idkey_to_string, smalltx_to_processedtx };
    use crate::test_data::{ test_data, test_state_init };

    #[test]
    fn test_string_to_key(){
        let input: String = "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000004".to_string();
        let as_key: IDKey = string_to_idkey(&input).unwrap();
        let output: String = idkey_to_string(&as_key).unwrap();
        assert_eq!(input, output);

        let input2: String = "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000".to_string();
        let as_key2:IDKey = string_to_idkey(&input2).unwrap();
        let output2: String = idkey_to_string(&as_key2).unwrap();
        assert_eq!(input2, output2);

        let input3: String = "q6osm".to_string();
        let as_key3:IDKey  = string_to_idkey(&input3).unwrap();
        let output3: String = idkey_to_string(&as_key3).unwrap();
        assert_eq!(input3, output3);
    }

    #[test]
    fn test_process_to_small_tx_format(){

        // init test Stable/ Runtime state
        test_state_init();

        // add vec<ProcessedTX> to test state
        RUNTIME_STATE.with(|s|{
           s.borrow_mut().temp_vec_ptx = test_data();
        });

        // process to Small TX - output is temp_vec_stx in TEST_RUNTIME
        RUNTIME_STATE.with(|state| {
            process_to_small_tx();
        }); 

        // TRANSACTION TYPE
        // Processed TX 10
        let first_ptx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_ptx[10].clone()
        });

        // Small TX 0
        let first_stx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_stx[10].clone()
        });

        // from account to u32 ref (using Directory)
        let id_ref_from = STABLE_STATE.with(|s|{
            let ac = first_ptx.from_account;
             s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).unwrap().clone()
        });

        // to account to u32 ref (using Directory)
        let id_ref_to = STABLE_STATE.with(|s|{
            let ac = first_ptx.to_account;
                s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).unwrap().clone()
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(first_stx.from.unwrap(), id_ref_from);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(first_stx.to.unwrap(), id_ref_to);
        // check time
        assert_eq!(first_stx.time, first_stx.time);
        // check type
        assert_eq!(first_stx.tx_type, 0_u8); // 0 = transfer, 1 = Mint, 2 = Burn.
        // check value
        assert_eq!(first_stx.value, first_ptx.tx_value as u64);
        // check block
        assert_eq!(first_stx.block, first_ptx.block as u32);

        // MINT TYPE
        let mint_ptx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_ptx[14].clone()
        });

        // Small TX 0
        let mint_stx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_stx[14].clone()
        });


        // from account to u32 ref (using Directory)
        let id_ref_from2 = STABLE_STATE.with(|s|{
            let ac = mint_ptx.from_account;
             s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).clone()
        });

        // to account to u32 ref (using Directory)
        let id_ref_to2 = STABLE_STATE.with(|s|{
            let ac = mint_ptx.to_account;
                s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).clone()
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(mint_stx.from, None);
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(mint_stx.to.unwrap(), id_ref_to2.unwrap());
        // check time
        assert_eq!(mint_stx.time, mint_stx.time);
        // check type
        assert_eq!(mint_stx.tx_type, 1_u8); // 0 = transfer, 1 = Mint, 2 = Burn.
        // check value
        assert_eq!(mint_stx.value, mint_ptx.tx_value as u64);
        // check block
        assert_eq!(mint_stx.block, mint_ptx.block as u32);

        // BURN TYPE
        let burn_ptx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_ptx[16].clone()
        });

        // Small TX 0
        let burn_stx = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_stx[16].clone()
        });


        // from account to u32 ref (using Directory)
        let id_ref_from3 = STABLE_STATE.with(|s|{
            let ac = burn_ptx.from_account;
             s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).clone()
        });

        // to account to u32 ref (using Directory)
        let id_ref_to3 = STABLE_STATE.with(|s|{
            let ac = burn_ptx.to_account;
                s.borrow().as_ref().unwrap()
            .directory_data.get_ref(&ac).clone()
        });
        
        // check from ac on Small TX = from ac on Processed TX
        assert_eq!(burn_stx.from.unwrap(), id_ref_from3.unwrap());
       
        // check to ac on Small TX = to ac on Processed TX
        assert_eq!(burn_stx.to, None);
        // check time
        assert_eq!(burn_stx.time, burn_stx.time);
        // check type
        assert_eq!(burn_stx.tx_type, 2_u8); // 0 = transfer, 1 = Mint, 2 = Burn.
        // check value
        assert_eq!(burn_stx.value, burn_ptx.tx_value as u64);
        // check block
        assert_eq!(burn_stx.block, burn_ptx.block as u32);

        // check input length == output length.
        // Processed TX 0
        let ptx_len = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_ptx.len()
        });

        // Small TX 0
        let stx_len = RUNTIME_STATE.with(|state| {
            state.borrow().temp_vec_stx.len()
        });
        assert_eq!(ptx_len, stx_len); 
    }

    #[test]
    fn test_calculate_balances(){
        // init test Stable/ Runtime state
        test_state_init();

        // add vec<ProcessedTX> to test state
        RUNTIME_STATE.with(|s|{
           s.borrow_mut().temp_vec_ptx = test_data();
        });
        
        // process to Small TX - output is temp_vec_stx in RUNTIME_STATE
        process_to_small_tx();

        // process balances
        process_smtx_to_index();

        //data for account 1
        let ac1 = "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string();
        let res1 = STABLE_STATE.with(|s| s.borrow().as_ref().unwrap()
        .get_fulldata_by_id_raw(&ac1)
        );

        // index exists
        let sm;
        match &res1 {
            Some(v) => { sm = 1 },
            None => { sm = -1 }
        }
        assert!(sm == 1);
        // First Active 
        assert_eq!(&res1.as_ref().unwrap().overview.first_active, &1_687_939_200_000_000_000);
        // Last Active 
        assert_eq!(&res1.as_ref().unwrap().overview.last_active, &1_688_888_888_888_888_888);
        // Sent Count
        assert_eq!(&res1.as_ref().unwrap().overview.sent.0, &6);
        // Sent Value
        assert_eq!(&res1.as_ref().unwrap().overview.sent.1, &730560000);
        // Received Count
        assert_eq!(&res1.as_ref().unwrap().overview.received.0, &4);
        // Received Value
        assert_eq!(&res1.as_ref().unwrap().overview.received.1, &101000090001);
        // Balance
        assert_eq!(&res1.as_ref().unwrap().overview.balance, &100_269_530_001);
        // Link Data
        let LD1 = LinkData{ 
            linked_from: 1687988709540000000, linked_id: 1, number_txs: 2, gross: 100090000, net: -99910000 };
        assert_eq!(&res1.as_ref().unwrap().links[0], &LD1);
        let LD2 = LinkData{ 
            linked_from: 1687980500040000000, linked_id: 4, number_txs: 1, gross: 0, net: 0 };
        assert_eq!(&res1.as_ref().unwrap().links[1], &LD2);
        let LD3 = LinkData{ 
            linked_from: 1687988705540000000, linked_id: 5, number_txs: 2, gross: 500001, net: -499999 };
        assert_eq!(&res1.as_ref().unwrap().links[2], &LD3);
        let LD4 = LinkData{ 
            linked_from: 1687980700040000000, linked_id: 6, number_txs: 2, gross: 600000000, net: -600000000 };
        assert_eq!(&res1.as_ref().unwrap().links[3], &LD4);
        let LD5 = LinkData{ 
            linked_from: 1687988718000000000, linked_id: 8, number_txs: 1, gross: 30000000, net: -30000000 };
        assert_eq!(&res1.as_ref().unwrap().links[4], &LD5);
        // Blocks
        let blocks = Vec::from([0, 10, 11, 15, 17, 20, 23, 27, 28, 29]);
        assert_eq!(&res1.as_ref().unwrap().blocks, &blocks);
    }

    #[test]
    fn full_cycle() { 
        // processedTx => smallTx => processedTx

          // init test Stable/ Runtime state
          test_state_init();

          // add vec<ProcessedTX> to test state
          let inpt_data = test_data();
          RUNTIME_STATE.with(|s|{
             s.borrow_mut().temp_vec_ptx = test_data(); // add same to runtime state.
          });
          
          // process to Small TX - output is temp_vec_stx in RUNTIME_STATE
          process_to_small_tx();

          // process back from small TX to Processed TX
          let reverse = RUNTIME_STATE.with(|s|{
            let ret = s.borrow().temp_vec_stx.to_owned();
            let res = smalltx_to_processedtx(&ret);
            return res;
          });
          
          assert_eq!(inpt_data[0], reverse[0]); // mint
          assert_eq!(inpt_data[10], reverse[10]); // transfer
          assert_eq!(inpt_data[16], reverse[16]); // burn
    }

}

    