
use candid::Principal;
use crate::utils::{log, idkey_to_string, processedtx_to_smalltx};
use crate::state_management::{ STABLE_STATE, RUNTIME_STATE};
use crate::custom_types::{ SmallTX, SendTxToStoreArgs,};

pub fn process_to_small_tx() {
    RUNTIME_STATE.with(|state|{
        let st = &mut state.borrow_mut();
        let mut stx: Vec<SmallTX> = Vec::new();

        stx = processedtx_to_smalltx(&st.temp_vec_ptx);

        // put Small TX in runtime temp array.
        st.temp_vec_stx = stx; 
    })
}

pub async fn send_stx_to_store() -> bool {
    let store_id = STABLE_STATE.with(|s|{
        s.borrow().as_ref().unwrap().canister_data.stx_store_canister.clone()
    });
    let canister_id = idkey_to_string(&store_id);
    match canister_id {
        Some(id) => {
            let store_id = Principal::from_text(&id);
            match store_id {
                Ok(pr_id) => {
                    // args
                    let call_data = RUNTIME_STATE.with(|s|{
                        s.borrow().temp_vec_stx.clone()
                    });
                    let args2 = SendTxToStoreArgs(call_data);
                    // call
                    let (call_res,):(bool,)  = ic_cdk
                        ::call(pr_id, "add_txs_to_store", (args2,)).await
                        .map_err(|(code, str)| format!("code: {:#?} message: {}", code, str))
                        .unwrap();
                    return call_res;
                },
                Err(error) => {
                    log(format!("Error getting principal from string (send_stx_to_store) Err:{}", error));
                    return false;
                },
            }
        },
        None => {
            log("Unable to get string from IDKey - send_stx_to_store");
            return false;
        }
    }
}

pub async fn process_smtx_to_index(){ 
    let blocks = RUNTIME_STATE.with(|s|{s.borrow().temp_vec_stx.clone()});

    for tx in blocks {
        // process from accoung
        if let Some(from_ref) = tx.from {
            // Process TX
            match tx.tx_type {
                // transaction
                0  => {
                    // process overview
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                    .processed_data.process_transfer_from(&from_ref, &tx)});
                    
                    // process links
                    let linked_ref = tx.to.unwrap(); // safe as this should not be None in transaction type. 
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                    .processed_data.process_links(&from_ref, &linked_ref, -1_i8, &tx)});

                    // process blocks
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_block(&from_ref, tx.block)});
                }, 
                // Mint - do nothing tx is from ICP LEDGER
                1  => {}, 
                // Burn
                2  => {
                    // process overview
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_transfer_from(&from_ref, &tx)});
                    // process blocks
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_block(&from_ref, tx.block)});
                    // Note - No links to process as linked account is ICP LEDGER
                }, 
                _ =>  {log(format!("Error - unknown tx type (process_smtx_to_index). Type: {}", tx.tx_type))},
            }
        }
        
        // process to account
        if let Some(to_ref) = tx.to {
            match tx.tx_type {
                // Transaction
                0  => {
                    // process overview
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                    .processed_data.process_transfer_to(&to_ref, &tx)});

                    // process links
                    let linked_ref = tx.from.unwrap(); // safe as this should not be None in transaction type. 
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                    .processed_data.process_links(&to_ref, &linked_ref, 1_i8, &tx)});

                    // process blocks
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_block(&to_ref, tx.block)});
                    
                }, 
                // Mint
                1  => {
                    // process overview
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_transfer_to(&to_ref, &tx)});
                    // process blocks
                    STABLE_STATE.with(|s|{s.borrow_mut().as_mut().unwrap()
                        .processed_data.process_block(&to_ref, tx.block)});
                    // Note - No links to process as linked account is ICP LEDGER
                }, 
                // Burn - do nothing tx is to ICP LEDGER
                2  => {}, 
                _ =>  {log(format!("Error - unknown tx type (process_smtx_to_index). Type: {}", tx.tx_type))},
            }
        }
    }// for tx

    // Clear temp vecs
    RUNTIME_STATE.with(|s|{
        s.borrow_mut().temp_vec_ptx.clear();
        s.borrow_mut().temp_vec_stx.clear();
    });

}

