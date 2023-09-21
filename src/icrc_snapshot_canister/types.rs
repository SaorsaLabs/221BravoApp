use core::fmt;
use serde::Serialize;
use candid::{ CandidType, Nat };
use serde::Deserialize;

// [][] -- -- [][]
pub const KEY_LENGTH: usize = 10;
pub type IDKey = [u8; KEY_LENGTH];

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct SnapshotData {
    pub canister_id: String,
    pub retained_data: Vec<RetSaorsaStatsIcrc>,
    pub processed_data: Vec<QuickStats>,
    pub snapshots_taken: u64,
}

impl SnapshotData {
    pub fn new(canister_id: String) -> Self {
        Self {
            canister_id,
            retained_data: Vec::<RetSaorsaStatsIcrc>::new(),
            processed_data: Vec::<QuickStats>::new(),
            snapshots_taken: 0,
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug, Default)]
pub struct RetSaorsaStatsIcrc {
    pub snapshot_time: u64,
    pub total_transaction_count: u128,
    pub total_transaction_value: u128,
    pub total_transaction_average: f64,
    pub total_unique_accounts: u64,
    pub total_unique_principals: u64,
    pub most_active_accounts: Vec<(String, u64)>,
    pub most_active_principals: Vec<(String, u64)>,
    pub burn_stats: TotCntAvg,
    pub mint_stats: TotCntAvg,
    pub transaction_stats: TotCntAvg,
    pub count_over_time: Vec<TimeChunkStats>,
    pub top_mints: Vec<ProcessedTX>,
    pub top_burns: Vec<ProcessedTX>,
    pub top_transactions: Vec<ProcessedTX>,
    pub total_principal_holders: u64,
    pub total_account_holders: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct QuickStats {
    pub total_transaction_count: Vec<(u128, u64)>,
    pub total_transaction_value: Vec<(u128, u64)>,
    pub total_unique_accounts: Vec<(u128, u64)>,
    pub total_unique_principals: Vec<(u128, u64)>,
    pub total_principal_holders: Vec<(u128, u64)>,
    pub total_account_holders: Vec<(u128, u64)>,
}
impl QuickStats {
    pub fn new() -> Self {
        QuickStats {
            total_transaction_count: Vec::new(),
            total_transaction_value: Vec::new(),
            total_unique_accounts: Vec::new(),
            total_unique_principals: Vec::new(),
            total_principal_holders: Vec::new(),
            total_account_holders: Vec::new(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct WorkingStats {
    pub data_collections: Vec<String>,
    pub last_processed_time: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct CanisterSettings {
    pub max_snapshots: u64,
    pub canister_paused: bool,
    pub stats_are_public: bool,
    pub canister_name: String,
}

// [][] --- Types for Utils --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct MemoryData {
    pub memory: u64,
    pub heap_memory: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TimeChunkStats {
    pub start_time: u64,
    pub end_time: u64,
    pub total_count: u64,
    pub mint_count: u64,
    pub transaction_count: u64,
    pub burn_count: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TotCntAvg {
    pub total_value: u128,
    pub count: u128,
    pub average: f64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TimeStats {
    pub total_transaction_count: u128,
    pub total_transaction_value: u128,
    pub total_transaction_average: f64,
    pub total_unique_accounts: u64,
    pub total_unique_principals: u64,
    pub most_active_accounts: Vec<(String, u64)>,
    pub most_active_principals: Vec<(String, u64)>,
    pub burn_stats: TotCntAvg,
    pub mint_stats: TotCntAvg,
    pub transaction_stats: TotCntAvg,
    pub count_over_time: Vec<TimeChunkStats>,
    pub top_mints: Vec<ProcessedTX>,
    pub top_burns: Vec<ProcessedTX>,
    pub top_transactions: Vec<ProcessedTX>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TotalHoldersResponse {
    pub principals: u64,
    pub accounts: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct ProcessedTX {
    pub block: Nat,
    pub hash: String,
    pub tx_type: String,
    pub from_principal: String,
    pub from_account: String,
    pub to_principal: String,
    pub to_account: String,
    pub tx_value: Nat,
    pub tx_time: u64,
}
impl fmt::Display for ProcessedTX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block: {}\nHash: {}\nType: {}\nFrom Principal: {}\nFrom Account: {}\nTo Principal: {}\nTo Account: {}\nValue: {}\nTime: {}",
            self.block,
            self.hash,
            self.tx_type,
            self.from_principal,
            self.from_account,
            self.to_principal,
            self.to_account,
            self.tx_value,
            self.tx_time
        )
    }
}
