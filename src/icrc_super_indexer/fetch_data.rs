use ic_cdk::api::call;
use num_traits::cast::ToPrimitive;
use candid::{ Nat, Principal };

use crate::utils::{idkey_to_string, string_to_idkey, log};
use crate::state_management::{ STABLE_STATE, RUNTIME_STATE };
use crate::constants::{ MAX_TOTAL_DOWNLOAD, MAX_TRANSACTION_BATCH_SIZE, MAX_BLOCKS_RETAINED };
use crate::custom_types::{
    ProcessedTX, 
    GetTransactionsRequest, 
    TransactionType, GetTxFromStoreArgs, SmallTX, GetMultipleTxFromStoreArgs, 
    GetTransactionsResponse, QueryTxArchiveFn, ArchivedRange, 
    GetTransactionsArchiveResponse, Mint, Burn, Account, Transfer,
};

// Set target canister, call target and tx fee to stable memory
pub async fn impl_set_target_canister(canister_id: String, store_id: String) -> String {
    let s = STABLE_STATE.with(|state|{ 
        state.borrow().as_ref().unwrap().canister_data.target_canister_locked});
    // check if already set
    if  &s == &true {
        ic_cdk::trap(
            "Target canister cann't be changed after being set. Re-install canister to change."
        );
    } else {
        // call ledger, get fee and save
        let ledger_id = Principal::from_text(&canister_id);
        match ledger_id {
            Ok(pr_id) => {
                // call
                let (fee_call,): (Nat,) = ic_cdk
                    ::call(pr_id, "icrc1_fee", ()).await
                    .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                    .unwrap();
                
                let fee_u64 = fee_call.0.to_u64().ok_or("Fee Result is not a valid u64");
                match fee_u64 {
                    Ok(fee_value) => {
                        log(format!("Target: {}", &canister_id));

                        // update target canisters and fee into stable memory
                        STABLE_STATE.with(|state|{
                            state.borrow_mut().as_mut().unwrap()
                            .set_target_canisters_and_fee(canister_id, store_id, fee_value);
                        });
                        
                        log("[][] ---- Target Canister Set ---- [][]");
                        log(format!("Updated transfer fee: {}", &fee_value));
                    }
                    Err(error) => {
                        log(format!("Error setting target canister: {}", error));
                        ic_cdk::trap(
                            "Cannot read fee from target canister. Check target canister is an ICRC ledger canister"
                        );
                    }
                }
            }
            Err(error) => {
                log(format!("Can't get principal from text. Error {}", error));
            }
        }
    }
    return "Target canister and fee set".to_string();
}

pub async fn download_and_process_txs(){
    // get target canister id from stable memory
    let tc = STABLE_STATE.with(|state|{ 
        state.borrow().as_ref().unwrap().canister_data.target_canister.clone()});
    let targ_canister = idkey_to_string(&tc).unwrap();

    let ledger_id = Principal::from_text(&targ_canister);
    match ledger_id {
        Ok(ledger_id) => {
            let tip_chain = get_tip_of_chain(ledger_id).await
                .map_err(|err_string| format!("Can't fetch tip of chain : {}", err_string))
                .unwrap();

            let tip_u128 = tip_chain.0.to_u128().ok_or("Cannot cast to u128");
            match tip_u128 {
                Ok(tip) => {
                    if tip > 0 {
                        let next_block = STABLE_STATE.with(|s|{
                            s.borrow().as_ref().unwrap().canister_data.working_stats
                            .next_tx
                        });
                        let blocks_needed = tip - next_block;
                        let chunks_needed = (
                            (blocks_needed as f32) / (MAX_TRANSACTION_BATCH_SIZE as f32)
                        ).ceil() as u32;

                        log("[][] ----- Starting ICP Download ----- [][]");
                        log(
                            format!(
                                "Blocks Needed: {}, Chunks Needed: {}, Tip: {}, Next-Block: {}",
                                blocks_needed,
                                chunks_needed,
                                tip,
                                next_block
                            )
                        );

                        if blocks_needed > 0 {
                            STABLE_STATE.with(|s|{ s.borrow_mut().as_mut()
                                .unwrap().canister_data.working_stats.is_upto_date = false;
                            });
                            let mut start: u128 = 0;
                            let mut length: u128 = 0;
                            let mut remaining: u128;
                            let mut completed_this_run: u128 = 0_u128;
                            let mut temp_tx_array: Vec<ProcessedTX> = Vec::new();
                            let max_loops = (
                                (MAX_TOTAL_DOWNLOAD as f64) / (MAX_TRANSACTION_BATCH_SIZE as f64)
                            ).ceil() as u32;
                            let segment: u32 = if chunks_needed > max_loops {
                                max_loops
                            } else {
                                chunks_needed
                            };

                            // Download in chunks
                            for i in 0..segment {
                                start = if i == 0 {
                                    next_block
                                } else {
                                    next_block + completed_this_run
                                };
                                remaining = tip - start;
                                length = if remaining > (MAX_TRANSACTION_BATCH_SIZE as u128) {
                                    MAX_TRANSACTION_BATCH_SIZE as u128
                                } else {
                                    remaining
                                };
                                // Get transactions
                                let txns: Option<Vec<ProcessedTX>> = icrc_transaction_download(
                                    start,
                                    length,
                                    targ_canister.clone()
                                ).await;
                                let mut txns_len = 0_u128;
                                if let Some(txns) = txns {
                                    txns_len = txns.len() as u128;
                                    for tx in txns {
                                        temp_tx_array.push(tx);
                                    }
                                } else {
                                    log("No transactions in this chunk!");
                                }
                                completed_this_run += txns_len;
                            }

                            // Retain latest processed transactions
                            let mut is_upto_date = false;
                            if (start+length) > (tip - MAX_BLOCKS_RETAINED as u128) {
                                RUNTIME_STATE.with(|s|{
                                    let mut highest_block: u128 = 0;
                                    for tx in &temp_tx_array {
                                        s.borrow_mut().latest_txs.push_tx(tx.clone());
                                        if tx.block >= highest_block { highest_block = tx.block }
                                    }         
                                    let bh_tip = s.borrow().latest_txs.tip;
                                    if highest_block > bh_tip { s.borrow_mut().latest_txs.tip = highest_block };
                                    if highest_block == tip-1 { is_upto_date = true; }
                                });
                            };

                            // moved into runtimestate to pass to next task
                            RUNTIME_STATE.with(|state|{
                                state.borrow_mut().temp_vec_ptx = temp_tx_array;
                            });

                            // update working stats state
                            STABLE_STATE.with(|s|{
                                s.borrow_mut().as_mut()
                                .unwrap().canister_data.working_stats
                                .update_downloaded(
                                    next_block + completed_this_run, 
                                    next_block + completed_this_run - 1, 
                                    is_upto_date);
                            }); 

                            // move onto next task
                            STABLE_STATE.with(|s|{
                                s.borrow_mut().as_mut()
                                .unwrap().canister_data.working_stats
                                .task_id = 1;
                            }); 
                            

                            log(
                                format!(
                                    "Complete To {}; All transactions downloaded? = {};",
                                    next_block + completed_this_run - 1,
                                    is_upto_date
                                )
                            );
                        } // end if blocks_needed
                    } // end if tip
                } // end Ok tip
                Err(error) => {
                    log(format!("ERROR : {}", error));
                }
            } // end match tip
        }
        Err(error) => {
            log(format!("Cannot derive principal from target canister. Error {}", error));
        }
    } // match ledger id
}

async fn icrc_transaction_download(start: u128, length: u128, target_canister: String) -> Option<Vec<ProcessedTX>> {

    let ledger_id = Principal
        ::from_text(target_canister)
        .unwrap();
    let mut processed_transactions: Vec<ProcessedTX> = vec![];
    let res: Result<GetTransactionsResponse, String> = get_transactions_from_ledger(
        ledger_id,
        start,
        length
    ).await;
    match res {
        Ok(value) => {
            match (value.transactions.is_empty(), value.archived_transactions.is_empty()) {
                (false, false) => {
                    // Ledger and Archive

                    //  Archive TXS
                    let mut block_master: Nat = Nat::from(0);
                    for archived in value.archived_transactions {
                        let mut block = archived.start.clone();

                        let archived = ArchivedRange::<QueryTxArchiveFn> {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                // loop through results
                                for transaction in data.transactions {
                                    if let Some(value) = transaction.mint {
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.burn {
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.transfer {
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!("Error fetching archive transactions. Error : {}", err_text)
                                );
                            }
                        }
                        block_master = block;
                    }

                    // Ledger TXS
                    for transaction in value.transactions {
                        if let Some(value) = transaction.mint {
                            processed_transactions.push(
                                process_mint_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                        if let Some(value) = transaction.burn {
                            processed_transactions.push(
                                process_burn_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                        if let Some(value) = transaction.transfer {
                            processed_transactions.push(
                                process_transfer_transaction(
                                    value,
                                    &block_master,
                                    &transaction.timestamp
                                )
                            );
                            block_master += Nat::from(1);
                        }
                    }

                    return Some(processed_transactions);
                }
                (false, true) => {
                    // Ledger TX only - no archive
                    let mut block = Nat::from(start);
                    for transaction in value.transactions {
                        if let Some(value) = transaction.mint {
                            processed_transactions.push(
                                process_mint_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                        if let Some(value) = transaction.burn {
                            processed_transactions.push(
                                process_burn_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                        if let Some(value) = transaction.transfer {
                            processed_transactions.push(
                                process_transfer_transaction(value, &block, &transaction.timestamp)
                            );
                            block += Nat::from(1);
                        }
                    }

                    return Some(processed_transactions);
                }
                (true, false) => {
                    // Archive TXS ONLY
                    for archived in value.archived_transactions {
                        let archived = ArchivedRange::<QueryTxArchiveFn> {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = archived.start.clone();
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                // loop through results
                                for transaction in data.transactions {
                                    if let Some(value) = transaction.mint {
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.burn {
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    if let Some(value) = transaction.transfer {
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                value,
                                                &block,
                                                &transaction.timestamp
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!("Error fetching archive transactions. Error : {}", err_text)
                                );
                            }
                        }
                    }
                    return Some(processed_transactions);
                }
                (true, true) => {
                    log("No Data to fetch!".to_string());
                    return None;
                }
            }
        }
        Err(error) => ic_cdk::trap(&error),
    }
}

async fn get_transactions_from_ledger(
    ledger_id: Principal,
    start: u128,
    length: u128
) -> Result<GetTransactionsResponse, String> {
    let req = GetTransactionsRequest {
        start: Nat::from(start),
        length: Nat::from(length),
    };
    let call: Result<(GetTransactionsResponse,), _> = ic_cdk::call(ledger_id, "get_transactions", (
        req,
    )).await;
    match call {
        Ok(v) => { Ok(v.0) }
        Err(error) => {
            log(format!("Error getting transactions from ICRC ledger. {}", error.1));
            Err(format!("code: {:#?} message: {}", error.0, error.1))
        }
    }
}

async fn get_transactions_from_archive(
    archived: &ArchivedRange<QueryTxArchiveFn>
) -> Result<GetTransactionsArchiveResponse, String> {
    let req = GetTransactionsRequest {
        start: archived.start.clone(),
        length: archived.length.clone(),
    };
    let ledger_id = archived.callback.canister_id;
    let method = &archived.callback.method;
    let call: Result<(GetTransactionsArchiveResponse,), _> = ic_cdk::call(ledger_id, method, (
        req,
    )).await;
    match call {
        Ok(v) => { Ok(v.0) }
        Err(error) => {
            log(format!("Error getting transactions from ICRC Archive. {}", error.1));
            Err(format!("code: {:#?} message: {}", error.0, error.1))
        }
    }
}

fn process_mint_transaction(tx: Mint, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let to_ac: Account = tx.to;
    let to_pr: String = to_ac.owner.to_string();
    let sub: &[u8; 32] = to_ac.effective_subaccount();
    let sub_ac: String = hex::encode(sub);
    let block_u128: u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let value_u128: u128 = tx.amount.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let to_combined: String = format!("{}.{}", to_pr, sub_ac);

    let ret = ProcessedTX {
        block: block_u128,
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Mint.to_string(),
        from_account: "ICRC_LEDGER".to_string(),
        to_account: to_combined,
        tx_value: value_u128,
        tx_time: timestamp.to_owned(),
    };
    
    return ret;
}

fn process_burn_transaction(tx: Burn, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let from_ac: Account = tx.from;
    let from_pr: String = from_ac.owner.to_string();
    let sub: &[u8; 32] = from_ac.effective_subaccount();
    let sub_ac: String = hex::encode(sub);
    let from_combined: String = format!("{}.{}", from_pr, sub_ac);
    let block_u128: u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let value_u128: u128 = tx.amount.0.to_u128().ok_or("cant parse to u128").unwrap(); 

    let ret = ProcessedTX {
        block: block_u128,
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Burn.to_string(),
        from_account: from_combined,
        to_account: "ICRC_LEDGER".to_string(),
        tx_value: value_u128,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_transfer_transaction(tx: Transfer, block: &Nat, timestamp: &u64) -> ProcessedTX {
    let from_ac: Account = tx.from;
    let from_pr: String = from_ac.owner.to_string();
    let from_sub: &[u8; 32] = from_ac.effective_subaccount();
    let from_sub_ac: String = hex::encode(from_sub);
    let from_combined: String = format!("{}.{}", from_pr, from_sub_ac);
 

    let to_ac: Account = tx.to;
    let to_pr: String = to_ac.owner.to_string();
    let to_sub: &[u8; 32] = to_ac.effective_subaccount();
    let to_sub_ac: String = hex::encode(to_sub);
    let to_combined: String = format!("{}.{}", to_pr, to_sub_ac);

    let block_u128: u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let value_u128: u128 = tx.amount.0.to_u128().ok_or("cant parse to u128").unwrap();


    let ret: ProcessedTX = ProcessedTX {
        block: block_u128,
        hash: "no-hash".to_string(),
        tx_type: TransactionType::Transaction.to_string(),
        from_account: from_combined,
        to_account: to_combined,
        tx_value: value_u128,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

async fn get_tip_of_chain(ledger_id: Principal) -> Result<Nat, String> {
    let req = GetTransactionsRequest {
        start: Nat::from(0),
        length: Nat::from(1),
    };
    let (res,): (GetTransactionsResponse,) = ic_cdk
        ::call(ledger_id, "get_transactions", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res.log_length)
}

async fn get_tip_u128(ledger_id: Principal) -> Result<u128, String> {
    let tip_chain = get_tip_of_chain(ledger_id).await;
    match tip_chain {
        Ok(v) => {
            let tip_u128 = v.0.to_u128();
            match tip_u128 {
                Some(v) => { Ok(v) }
                None => {
                    let error = "Error getting u128 from Nat - tip of chain.".to_string();
                    log(&error);
                    Err(error)
                }
            }
        }
        Err(error) => {
            log(format!("Error getting tip of chain. {}", error));
            Err(error)
        }
    }
}


// [][] ------------------------------ [][]
// [][] --- Fetch from Block Store --- [][]
// [][] ------------------------------ [][]
pub async fn get_single_tx_from_store(block: u32) -> Option<ProcessedTX> {
    let store_id = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.stx_store_canister.clone()
    });
    let canister_id = idkey_to_string(&store_id);
    match canister_id {
        Some(id) => {
            let store_id = Principal::from_text(&id);
            match store_id {
                Ok(pr_id) => {

                    let args = GetTxFromStoreArgs(block);
                    // call
                    let (call_res,):(Option<SmallTX>,)  = ic_cdk
                        ::call(pr_id, "get_tx_from_store", (args,)).await
                        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                        .unwrap();

                    log(format!("{:?}", call_res));

                    match call_res {
                        Some(tx) => {
                            // process from SmallTX to ProcessedTX
                            let fm_to = STABLE_STATE.with(|s| {
                                let mut fm = String::new();
                                let mut to= String::new();

                                // from
                                if let Some(fm_res) =  tx.from {
                                    match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&fm_res) {
                                        Some(fm_value) => {fm = fm_value},
                                        None => {log("Errror getting string from fm_res. (get_processed_tx)")}
                                    }
                                } else {
                                    fm = "ICRC_LEDGER".to_string();
                                }

                                // To
                                if let Some(to_res) =  tx.to {
                                    match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&to_res) {
                                        Some(to_value) => {to = to_value},
                                        None => {log("Errror getting string from to_res. (get_processed_tx)")}
                                    }
                                } else {
                                    to = "ICRC_LEDGER".to_string();
                                }
                                return (fm, to);
                            });

                            let mut tx_type: String;
                            match tx.tx_type {
                                0  => {tx_type = "Transaction".to_string()},
                                1  => {tx_type = "Mint".to_string()},
                                2  => {tx_type = "Burn".to_string()},
                                _ =>  {tx_type = "Unknown".to_string()},
                            }

                            let res = ProcessedTX{
                                block: tx.block as u128,
                                hash: String::from("no-hash"),
                                tx_type,
                                from_account: fm_to.0,
                                to_account: fm_to.1,
                                tx_value: tx.value as u128,
                                tx_time: tx.time,
                            };
                            return Some(res);
                        },
                        None => { return None;}
                    }
                },
                Err(error) => {
                    log(format!("Error getting principal from string (send_stx_to_store) Err:{}", error));
                    return None;
                },
            }
        },
        None => {
            log("Unable to get string from IDKey - send_stx_to_store");
            return None;
        }
    }
}


pub async fn get_multiple_txs_from_store(block: Vec<u32>) -> Vec<Option<ProcessedTX>> {
    let store_id = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.stx_store_canister.clone()
    });
    let mut return_vec: Vec<Option<ProcessedTX>> = Vec::new();
    let canister_id = idkey_to_string(&store_id);
    match canister_id {
        Some(id) => {
            let store_id = Principal::from_text(&id);
            match store_id {
                Ok(pr_id) => {

                    let args = GetMultipleTxFromStoreArgs(block);
                    // call
                    let (call_res,):(Vec<Option<SmallTX>>,)  = ic_cdk
                        ::call(pr_id, "get_multiple_tx_from_store", (args,)).await
                        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                        .unwrap();

                    log(format!("{:?}", call_res));

                    for stx in call_res{
                        match stx {
                            Some(tx) => {
                                // process from SmallTX to ProcessedTX
                                let fm_to = STABLE_STATE.with(|s| {
                                    let mut fm = String::new();
                                    let mut to= String::new();
    
                                    // from
                                    if let Some(fm_res) =  tx.from {
                                        match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&fm_res) {
                                            Some(fm_value) => {fm = fm_value},
                                            None => {log("Errror getting string from fm_res. (get_processed_tx)")}
                                        }
                                    } else {
                                        fm = "ICRC_LEDGER".to_string();
                                    }
    
                                    // To
                                    if let Some(to_res) =  tx.to {
                                        match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&to_res) {
                                            Some(to_value) => {to = to_value},
                                            None => {log("Errror getting string from to_res. (get_processed_tx)")}
                                        }
                                    } else {
                                        to = "ICRC_LEDGER".to_string();
                                    }
                                    return (fm, to);
                                });
    
                                let mut tx_type: String;
                                match tx.tx_type {
                                    0  => {tx_type = "Transaction".to_string()},
                                    1  => {tx_type = "Mint".to_string()},
                                    2  => {tx_type = "Burn".to_string()},
                                    _ =>  {tx_type = "Unknown".to_string()},
                                }
    
                                let res = ProcessedTX{
                                    block: tx.block as u128,
                                    hash: String::from("no-hash"),
                                    tx_type,
                                    from_account: fm_to.0,
                                    to_account: fm_to.1,
                                    tx_value: tx.value as u128,
                                    tx_time: tx.time,
                                };
                               return_vec.push(Some(res));
                            },
                            None => { return_vec.push(None);}
                        }
                    }
                    return return_vec;
                },
                Err(error) => {
                    log(format!("Error getting principal from string (send_stx_to_store) Err:{}", error));
                    return_vec.push(None);
                    return return_vec;
                },
            }
        },
        None => {
            log("Unable to get string from IDKey - send_stx_to_store");
            return_vec.push(None);
            return return_vec;
        }
    }
}

