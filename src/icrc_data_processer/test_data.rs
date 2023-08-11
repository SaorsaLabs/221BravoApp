#[allow(dead_code)]
use candid::Nat;
use num_traits::ToPrimitive;

use crate::{types::{ProcessedTX, TimeStats, StatsType, TimeChunkStats, TotCntAvg}};
use crate::{constants::{HOUR_AS_NANOS, DAY_AS_NANOS}};
use crate::{utils::{nearest_past_hour, nearest_day_start, top_x_by_txvalue, get_unique_string_values, top_x_txcount}};
use std::collections::{VecDeque, HashMap};

pub fn test_data() -> VecDeque<ProcessedTX> {
    // TEST DATA OVERVIEW
    // 
    // Test cases - mint, burn, transfer, self transfer, 0 values, sub-accounts. 
    //
    // Accounts used (10)
    // 2vxsx-fae (sub accounts 0 - 4)
    // 3xwpq-ziaaa-aaaah-qcn4a-cai (subs 0 - 2)
    // q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe (subs 0 + 1)
    //
    // 
    // ACCOUNT BALANCES/ TRANSACTIONS 
    // 
    // (1)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 269,530,001
    // txs: transfer 8, burn 0, mint 1;
    //
    // (2)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001
    // END Balance: 890_000 
    // txs: transfer 3, burn 1, mint 0;
    //
    // (3)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000002
    // END Balance: 20_000_000
    // txs: transfer 1, burn 0, mint 0;
    // 
    // (4)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000003
    // END Balance: 229_500_000
    // txs: transfer 1, burn 1, mint 2;
    //
    // (5)
    // 2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000004
    // END Balance: 0
    // txs: transfer 2, burn 0, mint 1;
    //
    // (6)
    // 3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 479_999
    // txs: transfer 3, burn 0, mint 0;
    //
    // (7)
    // 3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000001
    // END Balance: 589_990_000
    // txs: transfer 3, burn 0, mint 0;
    //
    // (8)
    // 3xwpq-ziaaa-aaaah-qcn4a-cai.0000000000000000000000000000000000000000000000000000000000000002
    // END Balance: 10_000_000
    // txs: transfer 1, burn 0, mint 0;
    //
    // (9)
    // q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000
    // END Balance: 30_000_000
    // txs: transfer 1, burn 0, mint 0;
    //
    // (10)
    // q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000001
    // END Balance: 0
    // txs: transfer 0, burn 1, mint 1;


    // -----------------------------------------------------------------------------------------------
    // calculate_time_stats data
    //  
    // Timing of transactions
    // Hourly tests - 10 transactions over 5 hours
    // Daily tests - 20 transactions over 5 days (inc hourly data)
    //
    // Hourly Stats
    // number 5,2,2,0,1
    //
    // Daily stats
    // number 10,0,3,2,5

    // -----------------------------------------------------------------------------------------------
    // Total Principal Txs (send to self is counted as 2 txs, one in and one out.)
    // 2vxsx-fae - 21
    // 3xwpq-ziaaa-aaaah-qcn4a-cai - 8
    // q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe - 3
    //
    // -----------------------------------------------------------------------------------------------

    let mut txs = VecDeque::new();
    // let start_time: u64 = 1_687_939_200_000_000_000; // Wednesday, 28 June 2023 08:00:00

    
    // ********* Hourly 
    // chunk 1 (5) - Wednesday, 28 June 2023 07:00:00 -> Wednesday, 28 June 2023 08:00:00
    // start nano : 1_687_935_600_000,000,000
    // end nano :   1_687_939_200_000_000_000
    
    txs.push_front(
        ProcessedTX { // 0 transaction??  Can TX BE 0 value? 
            block: Nat::from(19),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000004".to_string(),
            tx_value: Nat::from(0),
            tx_time: 1_687_938_999_999_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(18),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: Nat::from(500_000_000),
            tx_time: 1_687_938_221_575_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(17),
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "ICRC_LEDGER".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: Nat::from(79_000_000),
            tx_time: 1_687_937_821_875_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(16),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(400_000),
            tx_time: 1_687_936_125_123_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(15),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            tx_value: Nat::from(100_000_000),
            tx_time: 1_687_935_600_000_000_000
        }
    );

    // chunk 2 (2) - Wednesday, 28 June 2023 06:00:00 -> Wednesday, 28 June 2023 07:00:00
    // start nano : 1_687_932_000_000_000_000
    // end nano :   1_687_935_600_000,000,000
    
    txs.push_front(
        ProcessedTX {
            block: Nat::from(14),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(1),
            tx_time: 1_687_934_684_123_321_001
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(13),
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            to_principal: "ICRC_LEDGER".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: Nat::from(500_000),
            tx_time: 1_687_933_800_000_000_000
        }
    );

    // chunk 3 (2) - Wednesday, 28 June 2023 05:00:00 -> Wednesday, 28 June 2023 06:00:00
    // start nano : 1_687_928_400_000_000_000
    // end nano :   1_687_932_000_000_000_000

    txs.push_front(
        ProcessedTX {
            block: Nat::from(12),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(90_000),
            tx_time: 1_687_930_000_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(11),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000004".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            tx_value: Nat::from(80_000_000),
            tx_time: 1_687_928_400_000_000_001
        }
    );

    // chunk 4 (0) - Wednesday, 28 June 2023 04:00:00 -> Wednesday, 28 June 2023 05:00:00
    // start nano : 1_687_928_400_000_000_000
    // end nano :   1_687_924_800_000_000_000

    // NO TRANSACTIONS
    
    // chunk 5 (1) - Wednesday, 28 June 2023 03:00:00 -> Wednesday, 28 June 2023 04:00:00
    // start nano : 1_687_921_200_000_000_000
    // end nano :   1_687_928_400_000_000_000
    
    txs.push_front(
        ProcessedTX {
            block: Nat::from(10),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000004".to_string(),
            tx_value: Nat::from(80_010_000),
            tx_time: 1_687_923_200_000_000_000
        }
    );

    // DAILY TXS 
    // chunk 1 (0) - Tuesday, 27 June 2023 00:00:00 -> Tuesday, 27 June 2023 23:59:59

    // NO Transactions 

    // chunk 2 (3) - Monday, 26 June 2023 00:00:00 -> Tuesday, 26 June 2023 23:59:59
    // start : 1_687_737_600_000_000_000
    // end :   1_687_823_999_000_000_000

    txs.push_front(
        ProcessedTX {
            block: Nat::from(9),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(30_000_000),
            tx_time: 1_687_822_999_999_999_999
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(8),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            tx_value: Nat::from(10_000_000),
            tx_time: 1_687_800_333_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(7),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000003".to_string(),
            tx_value: Nat::from(50_000_000),
            tx_time: 1_687_737_600_000_000_000
        }
    );

    // chunk 3 (2) - Sunday, 25 June 2023 00:00:00 -> Sunday, 25 June 2023 23:59:59
    // start : 1_687_651_200_000_000_000
    // end :   1_687_737_599_000_000_000

    txs.push_front(
        ProcessedTX {
            block: Nat::from(6),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: Nat::from(100_000_000),
            tx_time: 1_687_730_285_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(5),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            tx_value: Nat::from(20_000_000),
            tx_time: 1_687_659_205_000_000_000
        }
    );

    // chunk 4 (5) - Saturday, 24 June 2023 00:00:00 -> Saturday, 24 June 2023 23:59:59
    // 2 mint, 2 transaction, 1 burn
    // start : 1_687_564_800_000_000_000
    // end :   1_687_651_199_000_000_000

    txs.push_front(
        ProcessedTX {
            block: Nat::from(4),
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_principal: "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "ICRC_LEDGER".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: Nat::from(1_000_000_000),
            tx_time: 1_687_649_199_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(3),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: Nat::from(1_000_000_000),
            tx_time: 1_687_620_899_999_999_123
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(2),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: Nat::from(100_000_000),
            tx_time: 1_687_569_420_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(1),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(500_000),
            tx_time: 1_687_567_222_000_000_000
        }
    );

    txs.push_front(
        ProcessedTX {
            block: Nat::from(0),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(1_000_000_000),
            tx_time: 1_687_564_900_000_000_000
        }
    );

    return txs;
}

pub fn test_TX_vec() -> Vec<ProcessedTX> {
    let mut txs: Vec<ProcessedTX> = Vec::new();
    txs.push(
        ProcessedTX {
            block: Nat::from(0),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "2vxsx-fae".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(1),
            tx_time: 1_687_934_684_123_321_001
        }
    );
    txs.push(
        ProcessedTX {
            block: Nat::from(1),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "2vxsx-fae".to_string(), 
            from_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000000".to_string(),
            tx_value: Nat::from(999_999),
            tx_time: 1_687_934_684_123_321_001
        }
    );
    txs.push(
        ProcessedTX {
            block: Nat::from(2),
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_principal: "ICRC_LEDGER".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_principal: "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            tx_value: Nat::from(799_123_123),
            tx_time: 1_687_620_899_999_999_123
        }
    );
    txs.push(
        ProcessedTX {
            block: Nat::from(3),
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_principal: "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "ICRC_LEDGER".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: Nat::from(500_123_321),
            tx_time: 1_687_649_199_000_000_000
        }
    );
    txs.push(
        ProcessedTX {
            block: Nat::from(4),
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            from_account: "0000000000000000000000000000000000000000000000000000000000000001".to_string(),
            to_principal: "3xwpq-ziaaa-aaaah-qcn4a-cai".to_string(),
            to_account: "0000000000000000000000000000000000000000000000000000000000000002".to_string(),
            tx_value: Nat::from(10_000_000),
            tx_time: 1_687_800_333_000_000_000
        }
    );
   return txs;
}

pub fn test_TX_count_vec() -> Vec<(String, u64)> {
    let mut txs = Vec::new();
    txs.push(
        (String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000000"), 999_999_u64)
    );
    txs.push(
        (String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000001"), 999_u64)
    );
    txs.push(
        (String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000002"), 123_999_999_u64)
    );
    txs.push(
        (String::from("2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000003"), 1_u64)
    );
    return txs;
}

// direct copy of function in lib.rs with state swapped for test state
pub fn test_calculate_time_stats(
    process_from: u64,
    mode: StatsType,
    time_now: u64
) -> Result<TimeStats, String> {
    let fn_return: Result<TimeStats, String>;

    // TEST STATE - RUNTIME_STATE removed for test.  
    let test_ret_state = test_data();
   // fn_return = RUNTIME_STATE.with(|state: &RefCell<RuntimeState>| {
      // unique accounts.
      //  let rts: &mut std::cell::RefMut<'_, RuntimeState> = &mut state.borrow_mut();
       
        let array: &VecDeque<ProcessedTX> = &test_ret_state;
        let mut all_accounts: Vec<String> = Vec::new();
        let mut all_principals: Vec<String> = Vec::new();
        let mut from_combined: String;
        let mut to_combined: String;
        let mut mint_count: u128 = 0_u128;
        let mut mint_value: u128 = 0_u128;
        let mut burn_count: u128 = 0_u128;
        let mut burn_value: u128 = 0_u128;
        let mut transaction_count: u128 = 0_u128;
        let mut transaction_value: u128 = 0_u128;
        let mut total_value: u128 = 0_u128;
        let mut total_txs: u128 = 0_u128;
        let mut error_output: String = String::new();
        let mut is_error = false;
        let mut all_mints: Vec<ProcessedTX> = Vec::new();
        let mut all_burns: Vec<ProcessedTX> = Vec::new();
        let mut all_transactions: Vec<ProcessedTX> = Vec::new();

        for tx in array {
            if tx.tx_time >= process_from {
                let value_u128: Result<u128, &str> = tx.tx_value.0
                    .to_u128()
                    .ok_or("Tip of Chain is not a valid u128");
                match value_u128 {
                    Ok(value_u128) => {
                        from_combined = format!("{}.{}", tx.from_principal, tx.from_account);
                        to_combined = format!("{}.{}", tx.to_principal, tx.to_account);
                        if tx.from_principal != "ICRC_LEDGER" {
                            all_accounts.push(from_combined);
                            all_principals.push(tx.from_principal.clone());
                        }
                        if tx.to_principal != "ICRC_LEDGER" {
                            all_accounts.push(to_combined);
                            all_principals.push(tx.to_principal.clone());
                        }
                        if tx.tx_type == "Mint" {
                            mint_count += 1_u128;
                            mint_value += &value_u128;
                            all_mints.push(tx.clone());
                        }
                        if tx.tx_type == "Burn" {
                            burn_count += 1_u128;
                            burn_value += &value_u128;
                            all_burns.push(tx.clone());
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count += 1_u128;
                            transaction_value += &value_u128;
                            all_transactions.push(tx.clone());
                        }
                        total_value += &value_u128;
                        total_txs += 1_u128;
                    }
                    Err(error) => {
                        is_error = true;
                        error_output = format!("Process Stats Error : {}", error);
                        //log(format!("Process Stats Error : {}", error));
                    }
                } // match
            } // if
        } // for
        // volumes per time-chunk
        let mut count_over_time = Vec::new();
        if mode == StatsType::Hourly {
            let chunks_needed = (
                ((time_now - process_from) as f64) / (HOUR_AS_NANOS as f64)
            ).ceil() as u32;
            let nearest_hour = nearest_past_hour(time_now);
            let mut start_chunk: u64 = 0_u64;
            let mut end_chunk: u64;
            let mut tx_count_chunk: u64;
            let mut mint_count_chunk: u64;
            let mut burn_count_chunk: u64;
            let mut transaction_count_chunk: u64;

            for i in 0..chunks_needed {
                if i == 0 {
                    start_chunk = if time_now == nearest_hour {
                        nearest_hour - HOUR_AS_NANOS
                    } else {
                        nearest_hour
                    };
                    end_chunk = time_now;
                } else {
                    end_chunk = start_chunk;
                    start_chunk = start_chunk - HOUR_AS_NANOS;
                }

                // reset
                tx_count_chunk = 0;
                mint_count_chunk = 0;
                burn_count_chunk = 0;
                transaction_count_chunk = 0;

                for tx in array {
                    if tx.tx_time >= start_chunk && tx.tx_time < end_chunk {
                        tx_count_chunk += 1;
                        if tx.tx_type == "Mint" {
                            mint_count_chunk += 1;
                        }
                        if tx.tx_type == "Burn" {
                            burn_count_chunk += 1;
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count_chunk += 1;
                        }
                    }
                    if tx.tx_time > end_chunk {
                        break;
                    }
                }

                let tcs: TimeChunkStats = TimeChunkStats {
                    start_time: start_chunk,
                    end_time: end_chunk,
                    total_count: tx_count_chunk,
                    mint_count: mint_count_chunk,
                    transaction_count: transaction_count_chunk,
                    burn_count: burn_count_chunk,
                };
                count_over_time.push(tcs);
            }
        } else if mode == StatsType::Daily {
            let chunks_needed: u32 = (
                ((time_now - process_from) as f64) / (DAY_AS_NANOS as f64)
            ).ceil() as u32;
            let nearest_day: u64 = nearest_day_start(time_now);
            let mut start_chunk: u64 = 0_u64;
            let mut end_chunk: u64;
            let mut tx_count_chunk: u64;
            let mut mint_count_chunk: u64;
            let mut burn_count_chunk: u64;
            let mut transaction_count_chunk: u64;

            for i in 0..chunks_needed {
                if i == 0 {
                    start_chunk = if time_now == nearest_day {
                        nearest_day - DAY_AS_NANOS
                    } else {
                        nearest_day
                    };
                    end_chunk = time_now;
                } else {
                    end_chunk = start_chunk;
                    start_chunk = start_chunk - DAY_AS_NANOS;
                }

                // reset
                tx_count_chunk = 0;
                mint_count_chunk = 0;
                burn_count_chunk = 0;
                transaction_count_chunk = 0;

                for tx in array {
                    if tx.tx_time >= start_chunk && tx.tx_time < end_chunk {
                        tx_count_chunk += 1;
                        if tx.tx_type == "Mint" {
                            mint_count_chunk += 1;
                        }
                        if tx.tx_type == "Burn" {
                            burn_count_chunk += 1;
                        }
                        if tx.tx_type == "Transaction" {
                            transaction_count_chunk += 1;
                        }
                    }
                    if tx.tx_time > end_chunk {
                        break;
                    }
                }

                let tcs: TimeChunkStats = TimeChunkStats {
                    start_time: start_chunk,
                    end_time: end_chunk,
                    total_count: tx_count_chunk,
                    mint_count: mint_count_chunk,
                    transaction_count: transaction_count_chunk,
                    burn_count: burn_count_chunk,
                };
                count_over_time.push(tcs);
            }
        }

        // largest burn/ tx/ transaction
        let ret_len: usize = 10_usize; // rts.data.canister_settings.stats_return_length.clone(); ** set for test!
        let top_mints: Vec<ProcessedTX> = top_x_by_txvalue(all_mints, ret_len);
        let top_burns: Vec<ProcessedTX> = top_x_by_txvalue(all_burns, ret_len);
        let top_transactions: Vec<ProcessedTX> = top_x_by_txvalue(all_transactions, ret_len);
        let unique_accounts: Vec<String> = get_unique_string_values(all_accounts);
        let unique_principals: Vec<String> = get_unique_string_values(all_principals);
        let ua: &usize = &unique_accounts.len();
        let up: &usize = &unique_principals.len();

        //output struct
        let ret = TimeStats {
            total_transaction_count: total_txs,
            total_transaction_value: total_value,
            total_transaction_average: (total_value as f64) / (total_txs as f64),
            total_unique_accounts: ua.to_owned() as u64,
            total_unique_principals: up.to_owned() as u64,
            most_active_accounts: Vec::new(),
            most_active_principals: Vec::new(),
            burn_stats: TotCntAvg {
                total_value: burn_value,
                count: burn_count,
                average: (burn_value as f64) / (burn_count as f64),
            },
            mint_stats: TotCntAvg {
                total_value: mint_value,
                count: mint_count,
                average: (mint_value as f64) / (mint_count as f64),
            },
            transaction_stats: TotCntAvg {
                total_value: transaction_value,
                count: transaction_count,
                average: (transaction_value as f64) / (transaction_count as f64),
            },
            count_over_time,
            top_mints,
            top_burns,
            top_transactions,
        };

        if is_error == false {
           // log("Stats calculation complete");
            return Ok(ret); // closure return
        } else {
          //  log("Error Calculating stats");
            return Err(error_output);
        }
   // });
    //return fn_return; //
}

// direct copy of function in lib.rs with state swapped for test state and return changed to tuple of processed data. 
pub fn test_most_active(process_from: u64, return_number: usize) -> ((Vec<(String, u64)>),(Vec<(String, u64)>)) {
   // RUNTIME_STATE.with(|state| {
        //let mut rts: std::cell::RefMut<'_, RuntimeState> = state.borrow_mut();
        let test_ret_state: VecDeque<ProcessedTX> = test_data(); // TEST STATE 
        let array: &VecDeque<ProcessedTX> = &test_ret_state; //&rts.data.retained_blocks;

        let mut from_combined: String;
        let mut to_combined: String;
        let mut most_active_acs: Vec<(String, u64)> = Vec::new();
        let mut most_active_prs: Vec<(String, u64)> = Vec::new();
        let mut all_acs: HashMap<String, u64> = HashMap::new();
        let mut all_prs: HashMap<String, u64> = HashMap::new();

        // process from 
        for tx in array {
            if tx.tx_time >= process_from {
                from_combined = format!("{}.{}", tx.from_principal, tx.from_account);
                
                let a = all_acs.entry(from_combined).or_insert(0);
                *a += 1; // add 1 to count

                let p = all_prs.entry(tx.from_principal.clone()).or_insert(0);
                *p += 1; // add 1 to count
            }
        }

        // process to 
        for tx in array {
            if tx.tx_time >= process_from {
                to_combined = format!("{}.{}", tx.to_principal, tx.to_account);
                
                let a = all_acs.entry(to_combined).or_insert(0);
                *a += 1; // add 1 to count

                let p = all_prs.entry(tx.to_principal.clone()).or_insert(0);
                *p += 1; // add 1 to count
            }
        }

        // accounts to vec
        for (ac, value) in &all_acs {
            if ac != "ICRC_LEDGER.ICRC_LEDGER" {
                most_active_acs.push((ac.to_owned(), value.to_owned()));
            }
        }

        // principals to vec
        for (pr, value) in &all_prs {
            if pr != "ICRC_LEDGER" {
                most_active_prs.push((pr.to_owned(), value.to_owned()));
            }
        }

        // most active accounts
        let top_active_acs: Vec<(String, u64)> = top_x_txcount(most_active_acs, return_number);
        let top_active_prs: Vec<(String, u64)> = top_x_txcount(most_active_prs, return_number);

        // update -- RETURNED in ret tuple 
        // rts.data.hourly_stats.data.most_active_accounts = top_active_acs;
        // rts.data.hourly_stats.data.most_active_principals = top_active_prs;

        // test return 
        let ret: ((Vec<(String, u64)>),(Vec<(String, u64)>)) = (top_active_acs, top_active_prs);
        return ret;
       // log("Most Active Accounts + Principals Updated");

   // });
    //return true;
}


// pub fn test_data_fail() -> VecDeque<ProcessedTX> {
//     let txs = VecDeque::new();
//     return ret;
// }