
use crate::{
custom_types::{ProcessedTX, TimeSearchArgs, Direction, MixerLink, StringNumTuple}, 
constants::{CKBTC, ICP, CHAT, SNS1, KINIC, HOT, GHOST, MODCLUB, CAT, FLAG_CANISTER, spr_icp, MAX_CHUNK_TIME, MAX_UPDATE_CHUNK}, 
utils::log, state_management::RUNTIME_STATE
};

use candid::Principal;

pub async fn get_transaction_data(id: String, start: u64, end: u64, token: &str) -> Option<Vec<ProcessedTX>> {
    let mut canister: String = String::new(); 
    match token {
        "CKBTC" => { canister = CKBTC.to_string() },
        "ICP" => { canister = ICP.to_string() },
        "CHAT" => { canister = CHAT.to_string() },
        "SNS1" => { canister = SNS1.to_string() },
        "KINIC" => { canister = KINIC.to_string() },
        "HOT" => { canister = HOT.to_string() },
        "GHOST" => { canister = GHOST.to_string() },
        "MODCLUB" => { canister = MODCLUB.to_string() },
        "CAT" => { canister = CAT.to_string() },
        _ => {},
    }
    let store_id = Principal::from_text(&canister);
    match store_id {
        Ok(pr_id) => {
            // call
            let args = TimeSearchArgs{
                id,
                start,
                end
            };
            

            let (call_res,):(Option<Vec<ProcessedTX>>,)  = ic_cdk
            ::call(pr_id, "get_transactions_time_id", (args,)).await
            .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
            .unwrap();

            match call_res {
                Some(tx) => {
                    log(format!("RES LEN :: {}", tx.len()));
                    return Some(tx);
                },
                None => {
                    return None;
                },
            }
        },
        Err(error) => {
            log(format!("Error getting principal from string (tx_tracking.rs) Err:{}", error));
            return None;
        }
    }
}

// [][] -- SPINNER TRACKING FUNCTIONS -- [][]
pub fn process_mixer_links(root_id: String, check_txs: Vec<ProcessedTX>, direction: Direction, min_value: u64) -> Option<Vec<MixerLink>> {
    let mut temp_vec: Vec<MixerLink> = Vec::new();
    let mut dir_check: Direction;
    if check_txs.len() == 0 { return None };

    for tx in check_txs {
        if tx.from_account == root_id { dir_check = Direction::Outbound } else { dir_check = Direction::Inbound };

        // link to account
        if dir_check == Direction::Outbound || direction == Direction::Both {
            if tx.tx_value as u64 >= min_value {
                temp_vec.push( MixerLink{
                    id: tx.to_account,
                    from: tx.tx_time,
                    level: 0,
                    text: "Spinner".to_string() 
                }
                ); //  //tx.to_account
            }
        }

        // link from account
        if dir_check == Direction::Inbound || direction == Direction::Both {
            if tx.tx_value as u64 >= min_value {
                temp_vec.push( MixerLink{
                    id: tx.from_account,
                    from: tx.tx_time,
                    level: 0,
                    text: "Spinner".to_string() 
                });
            }
        }
    }

    // remove duplicates
    let ret_vec: Vec<MixerLink> = get_unique_mixer_flags(temp_vec.clone());
    if ret_vec.len() > 0 {
        return Some(ret_vec);
    } else {
        return None
    } 
}

pub async fn send_mixer_flags_to_store (array: Vec<MixerLink>){
    let store_id = Principal::from_text(&FLAG_CANISTER);
    match store_id {
        Ok(pr_id) => {
            // call
            let args = array;
            let (call_res,):(String,)  = ic_cdk
            ::call(pr_id, "add_multiple_mixer_flags", (args,)).await
            .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
            .unwrap();
        
            log(format!("MIXER FLAGS - {}", call_res));
        },
        Err(error) => {
            log(format!("Error getting principal from string (send_stx_to_store) Err:{}", error));
        }
    }
}

pub async fn process_mixer_flag_que (){
    // chunk and send
    let awaiting = RUNTIME_STATE.with(|s|{
        s.borrow().spinner_tracking.awaiting_flagging.len()
    });
    let chunks_needed = (
        (awaiting as f32) / (MAX_UPDATE_CHUNK as f32)
    ).ceil() as u32;
    let mut start_position:usize = 0;
    let mut end_position: usize = 0;
    let mut slice: Vec<MixerLink> = Vec::new();
    for i in 0..chunks_needed { 
        if start_position+MAX_UPDATE_CHUNK < awaiting { end_position = start_position+MAX_UPDATE_CHUNK } 
        else { end_position = awaiting }
        slice = RUNTIME_STATE.with(|s|{
            s.borrow().spinner_tracking.awaiting_flagging[start_position..end_position].to_vec()
        });
        send_mixer_flags_to_store(slice).await;
        start_position+= MAX_UPDATE_CHUNK; 
    }
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().spinner_tracking.awaiting_flagging.clear()
    });
}

pub async fn check_for_new_spinner_links(){
    //  5c21e9ec78aec661aba9722f80c09b50d1e2332aa80bfdb768eec3602f79d61b
    let id = spr_icp.to_string();
    let start = RUNTIME_STATE.with(|s|{
        s.borrow().spinner_tracking.last_tx_time.clone()
    }) +1; // note the +1 
    let mut end = ic_cdk::api::time();
    if (end - start) > MAX_CHUNK_TIME { end = start+MAX_CHUNK_TIME }; 
    let txs = get_transaction_data(id.clone(), start, end, "ICP").await;

    match txs {
        Some(vec_tx) => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().spinner_tracking.is_upto_date = false;
            });
            let links = process_mixer_links(id, vec_tx, Direction::Both, 100000000);
            match links {
                Some(v) => { 
                   for ac in v {
                        RUNTIME_STATE.with(|s|{
                        s.borrow_mut().spinner_tracking.awaiting_flagging.push(ac)
                       });
                   }
                },
                None => {
                    log("Spinner Search - No linked accounts found");
                }
            }
        },
        None => {
            RUNTIME_STATE.with(|s|{
                s.borrow_mut().spinner_tracking.is_upto_date = true
            });
            log("No new Spinner Transactions to index");
        },
    }
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().spinner_tracking.last_run_time = ic_cdk::api::time();
        s.borrow_mut().spinner_tracking.last_tx_time = end;
    });
    //
}

pub fn get_unique_mixer_flags(mut inpt_vec: Vec<MixerLink>) -> Vec<MixerLink> {
    let mut op_vec: Vec<MixerLink> = Vec::new();
    let inpt_len: usize = inpt_vec.len();
    if inpt_len == 0 || inpt_len == 1 { return inpt_vec }

    // get earlist flag for each id
    let mut first_hit_vec: Vec<StringNumTuple> = Vec::new();
    for id1 in &inpt_vec {
        let mut earliest: u64 = id1.from;
        for id2 in &inpt_vec {
            if id1.id == id2.id && id2.from < earliest { earliest = id2.from }
        }
        first_hit_vec.push( StringNumTuple{ st: id1.id.clone(), num: earliest.clone() } );
    }
    // de-duplicate earlist hits
    first_hit_vec.sort_by(|a, b| a.st.cmp(&b.st));
    first_hit_vec.dedup_by(|a, b| a.st == b.st);

    // de-duplicate input vec
    inpt_vec.sort_by(|a, b| a.id.cmp(&b.id)); // assending
    inpt_vec.dedup_by(|a, b| a.id == b.id);

    // update earliest hits
    for flg in inpt_vec {
        let mut earliest: u64 = 0;
        for tm in &first_hit_vec {
            if flg.id == tm.st { earliest = tm.num.clone() }
        }
        op_vec.push(MixerLink { 
            id: flg.id, 
            from: earliest, 
            level: flg.level, 
            text: flg.text 
        });
    }

    return op_vec;
}