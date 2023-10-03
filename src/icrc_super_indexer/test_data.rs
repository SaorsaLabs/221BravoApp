#[allow(dead_code)]
use crate::custom_types::ProcessedTX;
use crate::state_management::{RuntimeState, Main, STABLE_STATE, RUNTIME_STATE};
use std::cell::RefCell;
use ic_stable_memory::stable_memory_init;


pub fn test_state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data = Main::default();
        // init Transfer Fee - set canister/ fee method not called in testing. 
        stable_data.processed_data.transaction_fee = 10_000; 
        STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let mut runtime_state = RuntimeState::default();
        RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_state;
    });
}


pub fn test_data() -> Vec<ProcessedTX> {
    // TEST DATA OVERVIEW
    // 
    // Test cases - mint, burn, transfer, self transfer, 0 values, sub-accounts. 
    //
    // Accounts used (10) - 
    // 1. 220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5
    // 2. 24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6
    // 3. 9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59
    // 4. f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4
    // 5. 0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c
    // 6. 646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2
    // 7. 1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6
    // 8. 4ada598f0657a86ed700d82a07f1308d2d42c953a1d5d79367491b6c09875289
    // 9. b9050eec17b91fb42c125c3c4d1136d7ebe129771a1b6bd781be86d95bdd59cb
    // 10. d2ff145968b3889873f1ec6689f0fc72deac6ca526d2257a44248de48d6207f4
    //
    // ACCOUNT BALANCES/ TRANSACTIONS 
    // 
    // (1)
    // 220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5
    // END Balance: 100_269_520_001
    // OVERVIEW
    //    first_active: 1687939200000000000, 
    //    last_active: 1688888888888888888, 
    //    sent: (7, 730570000), --1 is approve 
    //    received: (4, 101000090001),
    //    balance: 100269520001, 
    // txs: transfer 8, burn 0, mint 2, 1 approve;
    //
    // (2)
    // 24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6
    // END Balance: 
    // txs: transfer 3, burn 1, mint 1;
    //
    // (3)
    // 9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1;
    // 
    // (4)
    // f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4
    // END Balance: 
    // txs: transfer 1, burn 1, mint 3;
    //
    // (5)
    // 0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c
    // END Balance: 
    // txs: transfer 2, burn 0, mint 1;
    //
    // (6)
    // 646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2
    // END Balance: 
    // txs: transfer 3, burn 0, mint 1;
    //
    // (7)
    // 1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6
    // END Balance: 
    // txs: transfer 3, burn 0, mint 1;
    //
    // (8)
    // 4ada598f0657a86ed700d82a07f1308d2d42c953a1d5d79367491b6c09875289
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1;
    //
    // (9)
    // b9050eec17b91fb42c125c3c4d1136d7ebe129771a1b6bd781be86d95bdd59cb
    // END Balance: 
    // txs: transfer 1, burn 0, mint 1;
    //
    // (10)
    // d2ff145968b3889873f1ec6689f0fc72deac6ca526d2257a44248de48d6207f4
    // END Balance: 
    // txs: transfer 0, burn 1, mint 2;


    let mut txs = Vec::new();
    // let start_time: u64 = 1_687_939_200_000_000_000; // Wednesday, 28 June 2023 08:00:00

    // *** init Mints 
    txs.push(
        ProcessedTX {
            block: 0,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_939_200_000_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 1,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_939_750_000_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 2,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_455_000_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 3,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_459_000_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 4,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_940_459_000_010_001
        }
    );

    txs.push(
        ProcessedTX {
            block: 5,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_944_466_000_010_001
        }
    );

    txs.push(
        ProcessedTX {
            block: 6,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_444_040_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 7,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "4ada598f0657a86ed700d82a07f1308d2d42c953a1d5d79367491b6c09875289".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_444_040_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 8,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "b9050eec17b91fb42c125c3c4d1136d7ebe129771a1b6bd781be86d95bdd59cb".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_448_040_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 9,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "d2ff145968b3889873f1ec6689f0fc72deac6ca526d2257a44248de48d6207f4".to_string(),
            tx_value: 100_000_000_000,
            tx_time: 1_687_980_449_040_000_000
        }
    );
    
    txs.push(
        ProcessedTX { // 0 transaction??  Can TX BE 0 value? 
            block: 10,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c".to_string(),
            tx_value: 0,
            tx_time: 1_687_980_500_040_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 11,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6".to_string(),
            tx_value: 500_000_000,
            tx_time: 1_687_980_700_040_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 12,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: 79_000_000,
            tx_time: 1_687_988_700_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 13,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2".to_string(),
            to_account: "646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2".to_string(),
            tx_value: 400_000,
            tx_time: 1_687_988_701_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 14,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_703_540_000_000
        }
    );
    
    txs.push(
        ProcessedTX {
            block: 15,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2".to_string(),
            to_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            tx_value: 1,
            tx_time: 1_687_988_705_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 16,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: 500_000,
            tx_time: 1_687_988_707_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 17,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6".to_string(),
            to_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            tx_value: 90_000,
            tx_time: 1_687_988_709_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 18,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c".to_string(),
            to_account: "f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4".to_string(),
            tx_value: 80_000_000,
            tx_time: 1_687_988_712_540_000_000
        }
    );
    
    txs.push(
        ProcessedTX {
            block: 19,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "0a1c18f29bda699bd646acb47f518b864cb2b35b944dc920b73325680b00b03c".to_string(),
            tx_value: 80_010_000,
            tx_time: 1_687_988_714_540_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 20,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "b9050eec17b91fb42c125c3c4d1136d7ebe129771a1b6bd781be86d95bdd59cb".to_string(),
            tx_value: 30_000_000,
            tx_time: 1_687_988_718_000_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 21,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6".to_string(),
            to_account: "4ada598f0657a86ed700d82a07f1308d2d42c953a1d5d79367491b6c09875289".to_string(),
            tx_value: 10_000_000,
            tx_time: 1_687_988_724_666_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 22,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "f51cb73a607e22971cf01ca7143edc86557003b7d787806544da97d4dcf054d4".to_string(),
            tx_value: 50_000_000,
            tx_time: 1_687_988_728_132_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 23,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "1046c1fe0868175957d413149774cd6ea01ecb7b8e7cf78d906fd3ba6d44d1e6".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_766_132_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 24,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6".to_string(),
            to_account: "9e62737aab36f0baffc1faac9edd92a99279723eb3feb2e916fa99bb7fe54b59".to_string(),
            tx_value: 20_000_000,
            tx_time: 1_687_988_787_872_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 25,
            hash: "No-hash".to_string(),
            tx_type: "Burn".to_string(),
            from_account: "d2ff145968b3889873f1ec6689f0fc72deac6ca526d2257a44248de48d6207f4".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_687_988_788_872_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 26,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "d2ff145968b3889873f1ec6689f0fc72deac6ca526d2257a44248de48d6207f4".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_687_988_788_972_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 27,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "24879af686568e7e95846e91dc364ee910b7156ddcca9882e0e42b6cd1273da6".to_string(),
            tx_value: 100_000_000,
            tx_time: 1_687_988_888_972_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 28,
            hash: "No-hash".to_string(),
            tx_type: "Transaction".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "646ca9c5071136d07cf9b8b1a2f09e8bf8c4a1ab00f867194bb955281224a9d2".to_string(),
            tx_value: 500_000,
            tx_time: 1_688_888_888_888_000_000
        }
    );

    txs.push(
        ProcessedTX {
            block: 29,
            hash: "No-hash".to_string(),
            tx_type: "Mint".to_string(),
            from_account: "ICRC_LEDGER".to_string(),
            to_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_688_888_888_888_888_888
        }
    );

    txs.push(
        ProcessedTX {
            block: 30,
            hash: "No-hash".to_string(),
            tx_type: "Approve".to_string(),
            from_account: "220c3a33f90601896e26f76fa619fe288742df1fa75426edfaf759d39f2455a5".to_string(),
            to_account: "ICRC_LEDGER".to_string(),
            tx_value: 1_000_000_000,
            tx_time: 1_698_888_888_888_888_888
        }
    );

    return txs;
}
