use candid::{CandidType, Deserialize, Nat, Principal};
use serde::Serialize;
use std::marker::PhantomData;
use serde_bytes::ByteBuf;
use std::fmt;

// [][] --- Types for processed stats --- [][]
#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TotalHoldersResponse {
    pub principals: u64,
    pub accounts: u64
}

#[derive(CandidType, Serialize, Clone, Deserialize, Debug)]
pub struct TopHoldersResponse {
    pub top_principals: Vec<HolderBalance>,
    pub top_accounts: Vec<HolderBalance>
}
#[derive(CandidType, Serialize, Clone, Deserialize, Debug)]
pub struct HolderBalance {
    pub holder: String,
    pub balance: u128 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct WorkingStats {
    pub total_downloaded: u128,
    pub tx_completed_to: u128,
    pub next_tx: u128
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct ResultsData {
    pub work_in_progress: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug, Copy)]
pub struct EntityData {
    pub balance: u128,
    pub transactions: u64,
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct DailyStats {
    pub data: TimeStats,
    pub last_update: u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct HourlyStats {
    pub data: TimeStats,
    pub last_update: u64
}

#[derive(PartialEq)]
pub enum StatsType {
    Hourly,
    Daily
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TimeChunkStats {
    pub start_time: u64,
    pub end_time: u64,
    pub total_count: u64,
    pub mint_count: u64,
    pub transaction_count: u64,
    pub burn_count: u64
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct TotCntAvg{
    pub total_value: u128,
    pub count: u128,
    pub average: f64
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
    pub trasaction_stats: TotCntAvg,
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
    pub target_canister: String
}

#[derive(CandidType, Serialize, Deserialize, Clone)]
pub enum TransactionType {
    Transaction,
    Mint,
    Burn,
}
impl TransactionType {
    pub fn to_string (&self) -> String {
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
    pub from_principal: String,
    pub from_account: String,
    pub to_principal: String,
    pub to_account: String,
    pub tx_value: Nat,
    pub tx_time: u64
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
            self.tx_time,
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


// [][] --- ICRC Ledger Types --- [][]
// Defines types for interacting with the DFINITY implementation of the ICRC-1 / ICRC-3 fungible token standards
// https://github.com/dfinity/ic/tree/master/packages/icrc-ledger-types

pub type BlockIndex = Nat;
pub type GetTransactionsRequest = GetBlocksRequest;
pub type QueryTxArchiveFn = QueryArchiveFn<GetTransactionsRequest, TransactionRange>;

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetBlocksRequest {
    pub start: BlockIndex,
    pub length: Nat,
}

// Representation of a Transaction which supports the Icrc1 Standard functionalities
#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transaction {
    pub kind: String,
    pub mint: Option<Mint>,
    pub burn: Option<Burn>,
    pub transfer: Option<Transfer>,
    pub timestamp: u64,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Mint {
    pub amount: Nat,
    pub to: Account,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Burn {
    pub amount: Nat,
    pub from: Account,
    pub memo: Option<Memo>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Transfer {
    pub amount: Nat,
    pub from: Account,
    pub to: Account,
    pub memo: Option<Memo>,
    pub fee: Option<Nat>,
    pub created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct TransactionRange {
    pub transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetTransactionsResponse {
    pub log_length: Nat,
    pub first_index: BlockIndex,
    pub transactions: Vec<Transaction>,
    pub archived_transactions: Vec<ArchivedRange<QueryTxArchiveFn>>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct GetTransactionsArchiveResponse {
    pub transactions: Vec<Transaction>,
}

#[derive(CandidType, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ArchivedRange<Callback> {
    pub start: Nat,
    pub length: Nat,
    pub callback: Callback,
}

#[derive(Debug, Deserialize, PartialEq, Eq)]
#[serde(try_from = "candid::types::reference::Func")]
pub struct QueryArchiveFn<Input: CandidType, Output: CandidType> {
    pub canister_id: Principal,
    pub method: String,
    pub _marker: PhantomData<(Input, Output)>,
}

impl<Input: CandidType, Output: CandidType> QueryArchiveFn<Input, Output> {
    pub fn new(canister_id: Principal, method: impl Into<String>) -> Self {
        Self {
            canister_id,
            method: method.into(),
            _marker: PhantomData,
        }
    }
}

impl<Input: CandidType, Output: CandidType> Clone for QueryArchiveFn<Input, Output> {
    fn clone(&self) -> Self {
        Self {
            canister_id: self.canister_id,
            method: self.method.clone(),
            _marker: PhantomData,
        }
    }
}

impl<Input: CandidType, Output: CandidType> From<QueryArchiveFn<Input, Output>>
    for candid::types::reference::Func
{
    fn from(archive_fn: QueryArchiveFn<Input, Output>) -> Self {
        let p: &Principal = &Principal::try_from(archive_fn.canister_id.as_ref())
            .expect("could not deserialize principal");
        Self {
            principal: *p,
            method: archive_fn.method,
        }
    }
}

impl<Input: CandidType, Output: CandidType> TryFrom<candid::types::reference::Func>
    for QueryArchiveFn<Input, Output>
{
    type Error = String;
    fn try_from(func: candid::types::reference::Func) -> Result<Self, Self::Error> {
        let canister_id = Principal::try_from(func.principal.as_slice())
            .map_err(|e| format!("principal is not a canister id: {}", e))?;
        Ok(QueryArchiveFn {
            canister_id,
            method: func.method,
            _marker: PhantomData,
        })
    }
}

impl<Input: CandidType, Output: CandidType> CandidType for QueryArchiveFn<Input, Output> {
    fn _ty() -> candid::types::Type {
        candid::types::Type::Func(candid::types::Function {
            modes: vec![candid::parser::types::FuncMode::Query],
            args: vec![Input::_ty()],
            rets: vec![Output::_ty()],
        })
    }

    fn idl_serialize<S>(&self, serializer: S) -> Result<(), S::Error>
    where
        S: candid::types::Serializer,
    {
        candid::types::reference::Func::from(self.clone()).idl_serialize(serializer)
    }
}

pub type Subaccount = [u8; 32];
pub const DEFAULT_SUBACCOUNT: &Subaccount = &[0; 32];

// Account representation of ledgers supporting the ICRC1 standard
#[derive(Serialize, CandidType, Deserialize, Clone, Debug, Copy)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<Subaccount>,
}

impl Account {
    #[inline]
    pub fn effective_subaccount(&self) -> &Subaccount {
        self.subaccount.as_ref().unwrap_or(DEFAULT_SUBACCOUNT)
    }
}

impl PartialEq for Account {
    fn eq(&self, other: &Self) -> bool {
        self.owner == other.owner && self.effective_subaccount() == other.effective_subaccount()
    }
}

impl Eq for Account {}

impl std::cmp::PartialOrd for Account {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::cmp::Ord for Account {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.owner.cmp(&other.owner).then_with(|| {
            self.effective_subaccount()
                .cmp(other.effective_subaccount())
        })
    }
}

impl std::hash::Hash for Account {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.owner.hash(state);
        self.effective_subaccount().hash(state);
    }
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subaccount {
            None => write!(f, "{}", self.owner),
            Some(subaccount) => write!(f, "0x{}.{}", hex::encode(&subaccount[..]), self.owner),
        }
    }
}

impl From<Principal> for Account {
    fn from(owner: Principal) -> Self {
        Self {
            owner,
            subaccount: None,
        }
    }
}

#[derive(Serialize, Deserialize, CandidType, Clone, Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Default,)]
#[serde(transparent)]
pub struct Memo(pub ByteBuf);

impl From<u64> for Memo {
    fn from(num: u64) -> Self {
        Self(ByteBuf::from(num.to_be_bytes().to_vec()))
    }
}

impl From<ByteBuf> for Memo {
    fn from(b: ByteBuf) -> Self {
        Self(b)
    }
}

impl From<Vec<u8>> for Memo {
    fn from(v: Vec<u8>) -> Self {
        Self::from(ByteBuf::from(v))
    }
}

impl From<Memo> for ByteBuf {
    fn from(memo: Memo) -> Self {
        memo.0
    }
}

