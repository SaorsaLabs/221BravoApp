import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface LogEntry { 'text' : string, 'timestamp' : string }
export interface MemoryStats { 'memory' : bigint, 'heap_memory' : bigint }
export type MostActive = [string, bigint];
export interface ProcessedTX {
  'to_principal' : string,
  'hash' : string,
  'from_principal' : string,
  'to_account' : string,
  'tx_value' : bigint,
  'from_account' : string,
  'block' : bigint,
  'tx_time' : bigint,
  'tx_type' : string,
}
export type QsRecord = [bigint, bigint];
export interface QuickStats {
  'total_unique_accounts' : Array<QsRecord>,
  'total_account_holders' : Array<QsRecord>,
  'total_transaction_value' : Array<QsRecord>,
  'total_transaction_count' : Array<QsRecord>,
  'total_unique_principals' : Array<QsRecord>,
  'total_principal_holders' : Array<QsRecord>,
}
export interface TimeChunkStats {
  'mint_count' : bigint,
  'end_time' : bigint,
  'start_time' : bigint,
  'burn_count' : bigint,
  'transaction_count' : bigint,
  'total_count' : bigint,
}
export interface TimeStats {
  'transaction_stats' : TotCntAvg,
  'total_unique_accounts' : bigint,
  'top_burns' : Array<ProcessedTX>,
  'mint_stats' : TotCntAvg,
  'total_transaction_average' : number,
  'most_active_principals' : Array<MostActive>,
  'total_account_holders' : bigint,
  'top_mints' : Array<ProcessedTX>,
  'total_transaction_value' : bigint,
  'top_transactions' : Array<ProcessedTX>,
  'most_active_accounts' : Array<MostActive>,
  'count_over_time' : Array<TimeChunkStats>,
  'total_transaction_count' : bigint,
  'total_unique_principals' : bigint,
  'total_principal_holders' : bigint,
  'burn_stats' : TotCntAvg,
  'snapshot_time' : bigint,
}
export interface TotCntAvg {
  'count' : bigint,
  'average' : number,
  'total_value' : bigint,
}
export interface WorkingStats {
  'data_collections' : Array<string>,
  'last_processed_time' : bigint,
}
export interface _SERVICE {
  'add_authorised' : ActorMethod<[string], string>,
  'add_collection' : ActorMethod<[string, string], string>,
  'check_and_start_processing_timer' : ActorMethod<[], string>,
  'get_all_authorised' : ActorMethod<[], Array<string>>,
  'get_canister_name' : ActorMethod<[], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_logs' : ActorMethod<[], [] | [Array<LogEntry>]>,
  'get_memory_stats' : ActorMethod<[], MemoryStats>,
  'get_quickstats' : ActorMethod<[string, bigint], [] | [QuickStats]>,
  'get_standard_snapshots' : ActorMethod<
    [string, bigint],
    [] | [Array<TimeStats>]
  >,
  'get_working_stats' : ActorMethod<[], WorkingStats>,
  'remove_authorised' : ActorMethod<[string], string>,
  'set_canister_name' : ActorMethod<[string], string>,
  'stop_all_timers' : ActorMethod<[], string>,
}
