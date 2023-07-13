#[allow(dead_code)]
use candid::Nat;

use crate::{types::ProcessedTX};
use std::collections::{VecDeque};

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

// pub fn test_data_fail() -> VecDeque<ProcessedTX> {
//     let txs = VecDeque::new();
//     return ret;
// }