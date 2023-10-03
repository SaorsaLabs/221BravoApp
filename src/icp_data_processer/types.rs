use candid::{ CandidType, Deserialize, Nat, Principal };
use serde::Serialize;
use serde_bytes::ByteBuf;
use std::fmt;
use icrc_ledger_types;

// [][] --- Types for processed stats --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TotalHoldersResponse {
    pub accounts: u64,
}

#[derive(CandidType, Serialize, Clone, Deserialize, Debug)]
pub struct TopHoldersResponse {
    pub top_accounts: Vec<HolderBalance>,
}
#[derive(CandidType, Serialize, Clone, Deserialize, Debug)]
pub struct HolderBalance {
    pub holder: String,
    pub balance: u128,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct WorkingStats {
    pub version: String,
    pub total_downloaded: u128,
    pub tx_completed_to: u128,
    pub next_tx: u128,
    pub hr_stats_complete_to: u128,
    pub day_stats_complete_to: u128,
    pub is_upto_date: bool,
    pub is_busy: bool,
    pub task_id: u32,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug, Copy)]
pub struct EntityData {
    pub balance: u128,
    pub transactions: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct DailyStats {
    pub data: TimeStats,
    pub last_update: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct HourlyStats {
    pub data: TimeStats,
    pub last_update: u64,
}

#[derive(PartialEq)]
pub enum StatsType {
    Hourly,
    Daily,
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

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug,)]
pub struct TimeStats {
    pub total_transaction_count: u128,
    pub total_transaction_value: u128,
    pub total_transaction_average: f64,
    pub total_unique_accounts: u64,
    pub most_active_accounts: Vec<(String, u64)>,
    pub burn_stats: TotCntAvg,
    pub mint_stats: TotCntAvg,
    pub transaction_stats: TotCntAvg,
    pub count_over_time: Vec<TimeChunkStats>,
    pub top_mints: Vec<ProcessedTX>,
    pub top_burns: Vec<ProcessedTX>,
    pub top_transactions: Vec<ProcessedTX>,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct CanisterSettings {
    pub hours_to_calculate: u64,
    pub days_to_calcualte: u64,
    pub transaction_fee: u64,
    pub target_canister: String,
    pub canister_name: String,
    pub stats_are_public: bool,
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Transaction,
    Mint,
    Burn,
}
impl TransactionType {
    pub fn to_string(&self) -> String {
        match self {
            TransactionType::Transaction => "Transaction".to_string(),
            TransactionType::Mint => "Mint".to_string(),
            TransactionType::Burn => "Burn".to_string(),
        }
    }
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct ProcessedTX {
    pub block: Nat,
    pub hash: String,
    pub tx_type: String,
    pub from_account: String,
    pub to_account: String,
    pub tx_value: Nat,
    pub tx_time: u64,
}
impl fmt::Display for ProcessedTX {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Block: {}\nHash: {}\nType: {}\nFrom Account: {}\nTo Account: {}\nValue: {}\nTime: {}",
            self.block,
            self.hash,
            self.tx_type,
            self.from_account,
            self.to_account,
            self.tx_value,
            self.tx_time
        )
    }
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

// [][] -- DFINITY ICRC TYPES -- [][]
// Types created by Dfinity for interacting with ICP Ledger
// https://github.com/dfinity/ic/blob/master/rs/rosetta-api/icp_ledger/src/lib.rs#L739

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct GetTransactionsRequest {
    pub start: u64,
    pub length: u64,
}

pub type BlockIndex = u64;

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct Tokens {
    /// Number of 10^-8 Tokens.
    /// Named because the equivalent part of a Bitcoin is called a Satoshi
    pub e8s: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Debug)]
pub struct TimeStamp {
    pub timestamp_nanos: u64,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct Memo(pub u64);

pub type Certification = Option<Vec<u8>>;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct Transaction {
    pub operation: Option<OperationEnum>,
    pub memo: u64,
    pub created_at_time: TimeStamp,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub enum OperationEnum {
    Burn {
        from: Vec<u8>,
        amount: Tokens,
    },
    Mint {
        to: Vec<u8>,
        amount: Tokens,
    },
    Transfer {
        from: Vec<u8>,
        to: Vec<u8>,
        amount: Tokens,
        fee: Tokens,
    },
    Approve {
        fee: Tokens,
        from: Vec<u8>,
        allowance_e8s: i128,
        expires_at: Option<TimeStamp>,
        spender: Vec<u8>,
    },
    TransferFrom {
        to: Vec<u8>,
        fee: Tokens,
        from: Vec<u8>,
        amount: Tokens,
        spender: Vec<u8>,
    },
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct GetBlocksArgs {
    pub start: BlockIndex,
    pub length: usize,
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct Block {
    pub parent_hash: Option<[u8; 32]>,
    pub transaction: Transaction,
    pub timestamp: TimeStamp,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct BlockRange {
    pub blocks: Vec<Block>,
}

pub type GetBlocksResult = Result<BlockRange, GetBlocksError>;

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub enum GetBlocksError {
    BadFirstBlockIndex {
        requested_index: BlockIndex,
        first_valid_index: BlockIndex,
    },
    Other {
        error_code: u64,
        error_message: String,
    },
}

#[derive(Serialize, Deserialize, CandidType, Clone, Debug)]
pub struct IterBlocksArgs {
    pub start: usize,
    pub length: usize,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct ArchivedBlocksRange {
    pub start: BlockIndex,
    pub length: u64,
    pub callback: QueryArchiveBlocksFn,
}

#[derive(Deserialize, CandidType, Clone, Debug)]
pub struct QueryBlocksResponse {
    pub chain_length: u64,
    pub certificate: Option<serde_bytes::ByteBuf>,
    pub blocks: Vec<Block>,
    pub first_block_index: BlockIndex,
    pub archived_blocks: Vec<ArchivedBlocksRange>,
}

pub type QueryArchiveBlocksFn = icrc_ledger_types::icrc3::archive::QueryArchiveFn<
    GetBlocksArgs,
    GetBlocksResult
>;
