use candid::{CandidType};
use serde::{Deserialize, Serialize};

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
    pub user_account: String,
    pub user_name: String,
    pub user_tokens: u32,
    pub user_rank: UserRank
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub enum UserRank {
    #[default] Padawan,
    DataDetective,
    MasterSleuth,
    GrandMasterSleuth,
    Admin,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub struct MemoryData {
   pub memory: u64,
   pub heap_memory: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct LogEntry {
    pub timestamp: String,
    pub text: String,
}
// user token rank