use ic_cdk::api::call;
use num_traits::cast::ToPrimitive;
use candid::{ Nat, Principal };

use crate::utils::{idkey_to_string, string_to_idkey, log};
use crate::state_management::{ STABLE_STATE, RUNTIME_STATE };
use crate::constants::{ MAX_TOTAL_DOWNLOAD, MAX_TRANSACTION_BATCH_SIZE, MAX_BLOCKS_RETAINED };
use crate::custom_types::{
    ProcessedTX, 
    GetTransactionsRequest, 
    QueryBlocksResponse, ArchivedBlocksRange, GetBlocksResult, OperationEnum, TransactionType, GetTxFromStoreArgs, SmallTX, GetMultipleTxFromStoreArgs, GetMultipleTxFromStoreTimeArgs 
};

// Set target canister, call target and tx fee to stable memory
pub async fn impl_set_target_canister(canister_id: String, store_id: String, self_id: String) -> String {
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

                        // CHECK/ INIT STORE CANISTER 
                        let store_pr = Principal::from_text(&store_id);
                        match store_pr {
                            Ok(spr) => {
                                // call
                                let res: Result<
                                    (bool,), 
                                    (ic_cdk::api::call::RejectionCode, String)> 
                                    = ic_cdk::call(spr, "canister_init", ()).await;
                                match res {
                                    Ok(v) => {
                                        if v.0 == true {
                                            // update target canisters and fee into stable memory
                                            STABLE_STATE.with(|state|{
                                                state.borrow_mut().as_mut().unwrap()
                                                .set_target_canisters_and_fee(canister_id, store_id, self_id, fee_value);
                                            });
                                            log("[][] ---- Target Canister Set ---- [][]");
                                            log(format!("Updated transfer fee: {}", &fee_value));
                                            log("[][] ---- TX Store Admin Set ---- [][]");
                                        } else {
                                            ic_cdk::trap("Error adding ICRC Index canister as admin on TX Store Canister");
                                        }
                                    }
                                    Err(error) => {
                                        log(format!("Error doing init on ICRC TX Store. {}", error.1));
                                    }
                                }
                            }
                            Err(error) => {
                                log(format!("Can't get principal from text. Error {}", error));
                            }
                        }
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
    return "Target canister, Store Canister and fee set".to_string();
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

            let tip_u128 = tip_chain.to_u128().ok_or("Cannot cast to u128");
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
                                let txns: Option<Vec<ProcessedTX>> = icp_transaction_download(
                                    start as u64,
                                    length as u64,
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
                            if (start+length) > (tip - MAX_BLOCKS_RETAINED as u128) || tip <=  MAX_BLOCKS_RETAINED as u128 {
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

async fn get_tip_of_chain(ledger_id: Principal) -> Result<u64, String> {
    let req = GetTransactionsRequest {
        start: 0_u64,
        length: 1_u64,
    };
    let (res,): (QueryBlocksResponse,) = ic_cdk
        ::call(ledger_id, "query_blocks", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res.chain_length)
}

async fn icp_transaction_download(start: u64, length: u64, target_canister: String) -> Option<Vec<ProcessedTX>> {

    let ledger_id = Principal
        ::from_text(target_canister)
        .unwrap();
    let mut processed_transactions: Vec<ProcessedTX> = vec![];
    let res: Result<QueryBlocksResponse, String> = get_transactions_from_ledger(
        ledger_id,
        start,
        length
    ).await;
    match res {
        Ok(value) => {
            match (value.blocks.is_empty(), value.archived_blocks.is_empty()) {
                (false, false) => {
                    // Ledger and Archive
                    let mut block_master: Nat = Nat::from(0);
                    for archived in value.archived_blocks {
                        let archived = ArchivedBlocksRange {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = Nat::from(archived.start.clone());
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                match data {
                                    Ok(v) => {
                                        // loop through results
                                        for block_data in v.blocks {
                                            match block_data.transaction.operation {
                                                Some(value) => {
                                                    let hash: String = "no-hash".to_string();
                                                    // hash is wrong??
                                                    // match block_data.parent_hash {
                                                    //     Some(v) => {
                                                    //         hash = hex::encode(v);
                                                    //     },
                                                    //     _ => {
                                                    //         hash = "no-hash".to_string();
                                                    //     },
                                                    // }

                                                    match value {
                                                        OperationEnum::Burn { from, amount } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_burn_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Mint { to, amount } => {
                                                            let input = (
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_mint_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Transfer {
                                                            from,
                                                            to,
                                                            amount,
                                                            fee: _,
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_transfer_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Approve {
                                                            from,
                                                            spender,
                                                            allowance,
                                                            allowance_e8s,
                                                            expected_allowance,
                                                            expires_at,
                                                            fee
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                allowance.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_approve_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::TransferFrom {
                                                            to,
                                                            fee,
                                                            from,
                                                            amount,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Transfer from operation. Block: {}", block)
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    log(
                                                        format!("Transaction does not contain operation! Block: {}", block)
                                                    );
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        log(
                                            format!(
                                                "Error trying to unwrap Archive blocks. Err : {:?}",
                                                error
                                            )
                                        );
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!(
                                        "Error fetching archive transactions. Error : {:?}",
                                        err_text
                                    )
                                );
                            }
                        }
                        block_master = block;
                    }

                    // Ledger, non-archive blocks
                    for block_data in value.blocks {
                        // contains
                        match block_data.transaction.operation {
                            Some(value) => {
                                let hash: String = "no-hash".to_string();
                                // hash is wrong??
                                // match block_data.parent_hash {
                                //     Some(v) => {
                                //         hash = hex::encode(v);
                                //     },
                                //     _ => {
                                //         hash = "no-hash".to_string();
                                //     },
                                // }

                                match value {
                                    OperationEnum::Burn { from, amount } => {
                                        let input = (hex::encode(from), amount.e8s);
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Mint { to, amount } => {
                                        let input = (hex::encode(to), amount.e8s);
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Transfer { from, to, amount, fee } => {
                                        let input = (
                                            hex::encode(from),
                                            hex::encode(to),
                                            amount.e8s,
                                        );
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::Approve {
                                        from,
                                        spender,
                                        allowance,
                                        allowance_e8s,
                                        expected_allowance,
                                        expires_at,
                                        fee
                                    } => {
                                        let input = (
                                            hex::encode(from),
                                            allowance.e8s,
                                        );
                                        processed_transactions.push(
                                            process_approve_transaction(
                                                input,
                                                &block_master,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block_master += Nat::from(1);
                                    }
                                    OperationEnum::TransferFrom {
                                        to,
                                        fee,
                                        from,
                                        amount,
                                        spender,
                                    } => {
                                        log(
                                            format!("Transfer from operation. Block: {}", block_master)
                                        );
                                    }
                                }
                            }
                            _ => {
                                log("Transaction does not contain operation!");
                            }
                        }
                    }

                    return Some(processed_transactions);
                }
                (false, true) => {
                    // Ledger TX only - no archive
                    let mut block = Nat::from(start);
                    for block_data in value.blocks {
                        // contains
                        match block_data.transaction.operation {
                            Some(value) => {
                                let hash: String = "no-hash".to_string();
                                // hash is wrong??
                                // match block_data.parent_hash {
                                //     Some(v) => {
                                //         hash = hex::encode(v);
                                //     },
                                //     _ => {
                                //         hash = "no-hash".to_string();
                                //     },
                                // }

                                match value {
                                    OperationEnum::Burn { from, amount } => {
                                        let input = (hex::encode(from), amount.e8s);
                                        processed_transactions.push(
                                            process_burn_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Mint { to, amount } => {
                                        let input = (hex::encode(to), amount.e8s);
                                        processed_transactions.push(
                                            process_mint_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Transfer { from, to, amount, fee } => {
                                        let input = (
                                            hex::encode(from),
                                            hex::encode(to),
                                            amount.e8s,
                                        );
                                        processed_transactions.push(
                                            process_transfer_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::Approve {
                                        from,
                                        spender,
                                        allowance,
                                        allowance_e8s,
                                        expected_allowance,
                                        expires_at,
                                        fee
                                    } => {
                                        let input = (
                                            hex::encode(from),
                                            allowance.e8s,
                                        );
                                        processed_transactions.push(
                                            process_approve_transaction(
                                                input,
                                                &block,
                                                &block_data.timestamp.timestamp_nanos,
                                                &hash
                                            )
                                        );
                                        block += Nat::from(1);
                                    }
                                    OperationEnum::TransferFrom {
                                        to,
                                        fee,
                                        from,
                                        amount,
                                        spender,
                                    } => {
                                        log(format!("Transfer from operation. Block: {}", block));
                                    }
                                }
                            }
                            _ => {
                                log("Transaction does not contain operation!");
                            }
                        }
                    }

                    return Some(processed_transactions);
                }
                (true, false) => {
                    // Archive TXS ONLY
                    for archived in value.archived_blocks {
                        let archived = ArchivedBlocksRange {
                            start: archived.start.clone(),
                            length: archived.length.clone(),
                            callback: archived.callback.clone(),
                        };
                        let mut block = Nat::from(archived.start.clone());
                        let arc_res = get_transactions_from_archive(&archived).await;
                        match arc_res {
                            Ok(data) => {
                                match data {
                                    Ok(v) => {
                                        // loop through results
                                        for block_data in v.blocks {
                                            match block_data.transaction.operation {
                                                Some(value) => {
                                                    let hash: String = "no-hash".to_string();
                                                    // hash is wrong??
                                                    // match block_data.parent_hash {
                                                    //     Some(v) => {
                                                    //         hash = hex::encode(v);
                                                    //     },
                                                    //     _ => {
                                                    //         hash = "no-hash".to_string();
                                                    //     },
                                                    // }

                                                    match value {
                                                        OperationEnum::Burn { from, amount } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_burn_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Mint { to, amount } => {
                                                            let input = (
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_mint_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Transfer {
                                                            from,
                                                            to,
                                                            amount,
                                                            fee,
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                hex::encode(to),
                                                                amount.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_transfer_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::Approve {
                                                            from,
                                                            spender,
                                                            allowance,
                                                            allowance_e8s,
                                                            expected_allowance,
                                                            expires_at,
                                                            fee
                                                        } => {
                                                            let input = (
                                                                hex::encode(from),
                                                                allowance.e8s,
                                                            );
                                                            processed_transactions.push(
                                                                process_approve_transaction(
                                                                    input,
                                                                    &block,
                                                                    &block_data.timestamp.timestamp_nanos,
                                                                    &hash
                                                                )
                                                            );
                                                            block += Nat::from(1);
                                                        }
                                                        OperationEnum::TransferFrom {
                                                            to,
                                                            fee,
                                                            from,
                                                            amount,
                                                            spender,
                                                        } => {
                                                            log(
                                                                format!("Transfer from operation. Block: {}", block)
                                                            );
                                                        }
                                                    }
                                                }
                                                _ => {
                                                    log("Transaction does not contain operation!");
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        log(
                                            format!(
                                                "Error trying to unwrap Archive blocks. Err : {:?}",
                                                error
                                            )
                                        );
                                    }
                                }
                            }
                            Err(err_text) => {
                                log(
                                    format!(
                                        "Error fetching archive transactions. Error : {:?}",
                                        err_text
                                    )
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
    start: u64,
    length: u64
) -> Result<QueryBlocksResponse, String> {
    let req: GetTransactionsRequest = GetTransactionsRequest {
        start: start,
        length: length,
    };
    let (res,): (QueryBlocksResponse,) = ic_cdk
        ::call(ledger_id, "query_blocks", (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res)
}

async fn get_transactions_from_archive(
    archived: &ArchivedBlocksRange
) -> Result<GetBlocksResult, String> {
    let req = GetTransactionsRequest {
        start: archived.start.clone(),
        length: archived.length.clone(),
    };
    let ledger_id = archived.callback.canister_id;
    let method = &archived.callback.method;
    let (res,): (GetBlocksResult,) = ic_cdk
        ::call(ledger_id, method, (req,)).await
        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))?;
    Ok(res)
}

fn process_mint_transaction(
    data: (String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (to_account, tx_value) = data;
    let block_u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let ret = ProcessedTX {
        block: block_u128,
        hash: hash.to_owned(),
        tx_type: TransactionType::Mint.to_string(),
        from_account: "ICP_LEDGER".to_string(),
        to_account: to_account,
        tx_value: tx_value as u128,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_burn_transaction(
    data: (String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (from_ac, tx_value) = data;
    let block_u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let ret = ProcessedTX {
        block: block_u128,
        hash: hash.to_owned(),
        tx_type: TransactionType::Burn.to_string(),
        from_account: from_ac,
        to_account: "ICP_LEDGER".to_string(),
        tx_value: tx_value as u128,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_transfer_transaction(
    data: (String, String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {
    let (from_account, to_account, tx_value) = data;
    let block_u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let ret = ProcessedTX {
        block: block_u128,
        hash: hash.to_owned(),
        tx_type: TransactionType::Transaction.to_string(),
        from_account: from_account,
        to_account: to_account,
        tx_value: tx_value as u128,
        tx_time: timestamp.to_owned(),
    };
    return ret;
}

fn process_approve_transaction(
    data: (String, u64),
    block: &Nat,
    timestamp: &u64,
    hash: &String
) -> ProcessedTX {

    let (from_account, tx_value) = data;
    let block_u128 = block.0.to_u128().ok_or("cant parse to u128").unwrap(); 
    let ret = ProcessedTX {
        block: block_u128,
        hash: hash.to_owned(),
        tx_type: TransactionType::Approve.to_string(),
        from_account: from_account, 
        to_account: "ICP_LEDGER".to_string(),
        tx_value: tx_value as u128, // This is the approve value
        tx_time: timestamp.to_owned(),
    };
    return ret;
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
                                    fm = "ICP_LEDGER".to_string();
                                }

                                // To
                                if let Some(to_res) =  tx.to {
                                    match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&to_res) {
                                        Some(to_value) => {to = to_value},
                                        None => {log("Errror getting string from to_res. (get_processed_tx)")}
                                    }
                                } else {
                                    to = "ICP_LEDGER".to_string();
                                }
                                return (fm, to);
                            });

                            let mut tx_type: String;
                            match tx.tx_type {
                                0  => {tx_type = "Transaction".to_string()},
                                1  => {tx_type = "Mint".to_string()},
                                2  => {tx_type = "Burn".to_string()},
                                3  => {tx_type = "Approve".to_string()},
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
                                        fm = "ICP_LEDGER".to_string();
                                    }
    
                                    // To
                                    if let Some(to_res) =  tx.to {
                                        match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&to_res) {
                                            Some(to_value) => {to = to_value},
                                            None => {log("Errror getting string from to_res. (get_processed_tx)")}
                                        }
                                    } else {
                                        to = "ICP_LEDGER".to_string();
                                    }
                                    return (fm, to);
                                });
    
                                let mut tx_type: String;
                                match tx.tx_type {
                                    0  => {tx_type = "Transaction".to_string()},
                                    1  => {tx_type = "Mint".to_string()},
                                    2  => {tx_type = "Burn".to_string()},
                                    3  => {tx_type = "Approve".to_string()},
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

pub async fn get_multiple_txs_from_store_time(block: Vec<u32>, start: u64, end: u64, max_return: u64) -> Option<Vec<ProcessedTX>> {
    let store_id = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.stx_store_canister.clone()
    });
    let mut return_vec: Vec<ProcessedTX> = Vec::new();
    let canister_id = idkey_to_string(&store_id);
    match canister_id {
        Some(id) => {
            let store_id = Principal::from_text(&id);
            match store_id {
                Ok(pr_id) => {

                    let args = GetMultipleTxFromStoreTimeArgs{
                        blocks: block,
                        start,
                        end,
                        max_return,
                    };

                    // call
                    let (call_res,):(Option<Vec<SmallTX>>,)  = ic_cdk
                        ::call(pr_id, "get_multiple_tx_from_store_time", (args,)).await
                        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                        .unwrap();

                    match call_res {
                        Some(txar) => {
                            for tx in txar {
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
                                        fm = "ICP_LEDGER".to_string();
                                    }

                                    // To
                                    if let Some(to_res) =  tx.to {
                                        match s.borrow_mut().as_mut().unwrap().directory_data.get_id(&to_res) {
                                            Some(to_value) => {to = to_value},
                                            None => {log("Errror getting string from to_res. (get_processed_tx)")}
                                        }
                                    } else {
                                        to = "ICP_LEDGER".to_string();
                                    }
                                    return (fm, to);
                                });

                                let mut tx_type: String;
                                match tx.tx_type {
                                    0  => {tx_type = "Transaction".to_string()},
                                    1  => {tx_type = "Mint".to_string()},
                                    2  => {tx_type = "Burn".to_string()},
                                    3  => {tx_type = "Approve".to_string()},
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
                            return_vec.push(res);
                            }
                            return Some(return_vec);
                        },
                        None => {
                            return None;
                        },
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