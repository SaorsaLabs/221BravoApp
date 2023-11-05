use std::collections::BTreeMap;

use candid::{ CandidType, Principal };
use serde::{ Deserialize, Serialize };


#[derive(CandidType, Serialize, Deserialize, Clone, Default)]
pub struct UserData {
    pub user_account: String,
    pub user_name: String,
    pub user_tokens: u32,
    pub user_rank: UserRank,
    pub user_saved_accounts: BTreeMap<String, String> // account, name. 
}

#[derive(CandidType, Serialize, Deserialize, Clone, Default, Debug)]
pub struct CanisterSettings {
    pub canister_name: String,
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


// [][] -- TYPES FOR CANISTER HISTORY (Lookup any canister on the IC) -- [][]


#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct CanisterInfoRequest {
    /// Principle of the canister.
    pub canister_id: Principal,
    /// Number of most recent changes requested to be retrieved from canister history.
    /// No changes are retrieved if this field is null.
    pub num_requested_changes: Option<u64>,
}

/// Return type of [canister_info](super::canister_info).
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct CanisterInfoResponse {
    /// Total number of changes ever recorded in canister history.
    /// This might be higher than the number of canister changes in `recent_changes`
    /// because the IC might drop old canister changes from its history
    /// (with `20` most recent canister changes to always remain in the list).
    pub total_num_changes: u64,
    /// The canister changes stored in the order from the oldest to the most recent.
    pub recent_changes: Vec<CanisterChange>,
    /// A SHA256 hash of the module installed on the canister. This is null if the canister is empty.
    pub module_hash: Option<Vec<u8>>,
    /// Controllers of the canister.
    pub controllers: Vec<Principal>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct CanisterChange {
    /// The system timestamp (in nanoseconds since Unix Epoch) at which the change was performed
    pub timestamp_nanos: u64,
    /// The canister version after performing the change.
    pub canister_version: u64,
    /// The change's origin (a user or a canister).
    pub origin: CanisterChangeOrigin,
    /// The change's details.
    pub details: CanisterChangeDetails,
}

/// Provides details on who initiated a canister change.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub enum CanisterChangeOrigin {
    /// See [FromUserRecord].
    #[serde(rename = "from_user")]
    FromUser(FromUserRecord),
    /// See [FromCanisterRecord].
    #[serde(rename = "from_canister")]
    FromCanister(FromCanisterRecord),
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct FromUserRecord {
    /// Principle of the user.
    pub user_id: Principal,
}

/// Details about a canister change initiated by a canister (called _originator_).
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct FromCanisterRecord {
    /// Principle of the originator.
    pub canister_id: Principal,
    /// Canister version of the originator when the originator initiated the change.
    /// This is null if the original does not include its canister version
    /// in the field `sender_canister_version` of the management canister payload.
    pub canister_version: Option<u64>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub enum CanisterChangeDetails {
    /// See [CreationRecord].
    #[serde(rename = "creation")]
    Creation(CreationRecord),
    /// Uninstalling canister's module.
    #[serde(rename = "code_uninstall")]
    CodeUninstall,
    /// See [CodeDeploymentRecord].
    #[serde(rename = "code_deployment")]
    CodeDeployment(CodeDeploymentRecord),
    /// See [ControllersChangeRecord].
    #[serde(rename = "controllers_change")]
    ControllersChange(ControllersChangeRecord),
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct CreationRecord {
    /// Initial set of canister controllers.
    pub controllers: Vec<Principal>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct CodeDeploymentRecord {
    /// See [CanisterInstallMode].
    pub mode: CanisterInstallMode,
    /// A SHA256 hash of the new module installed on the canister.
    pub module_hash: Vec<u8>,
}

/// Details about updating canister controllers.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct ControllersChangeRecord {
    /// The full new set of canister controllers.
    pub controllers: Vec<Principal>,
}

/// The mode with which a canister is installed.
#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
// #[serde(rename_all = "lowercase")]
pub enum CanisterInstallMode {
    /// A fresh install of a new canister.
    #[serde(rename = "install")]
    Install,
    /// Reinstalling a canister that was already installed.
    #[serde(rename = "reinstall")]
    Reinstall,
    /// Upgrade an existing canister.
    #[serde(rename = "upgrade")]
    Upgrade,
}