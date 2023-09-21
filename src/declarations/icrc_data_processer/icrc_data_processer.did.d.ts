import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HolderBalance { 'balance' : bigint, 'holder' : string }
export type HolderData = [number, string];
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
  'top_mints' : Array<ProcessedTX>,
  'total_transaction_value' : bigint,
  'top_transactions' : Array<ProcessedTX>,
  'most_active_accounts' : Array<MostActive>,
  'count_over_time' : Array<TimeChunkStats>,
  'total_transaction_count' : bigint,
  'total_unique_principals' : bigint,
  'burn_stats' : TotCntAvg,
}
export interface TopHolderResponse {
  'top_accounts' : Array<HolderBalance>,
  'top_principals' : Array<HolderBalance>,
}
export interface TotCntAvg {
  'count' : bigint,
  'average' : number,
  'total_value' : bigint,
}
export interface TotalHolderResponse {
  'accounts' : bigint,
  'principals' : bigint,
}
export interface WorkingStats {
  'task_id' : number,
  'hr_stats_complete_to' : bigint,
  'total_downloaded' : bigint,
  'tx_completed_to' : bigint,
  'stats_return_length' : number,
  'next_tx' : bigint,
  'is_upto_date' : boolean,
  'day_stats_complete_to' : bigint,
  'is_busy' : boolean,
}
export interface _SERVICE {
  'add_authorised' : ActorMethod<[string], string>,
  'check_and_start_processing_timer' : ActorMethod<[bigint], string>,
  'get_account_balance' : ActorMethod<[string], string>,
  'get_all_authorised' : ActorMethod<[], Array<string>>,
  'get_canister_name' : ActorMethod<[], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_daily_stats' : ActorMethod<[], TimeStats>,
  'get_hourly_stats' : ActorMethod<[], TimeStats>,
  'get_memory_stats' : ActorMethod<[], MemoryStats>,
  'get_principal_balance' : ActorMethod<[string], string>,
  'get_top_holders' : ActorMethod<[bigint], TopHolderResponse>,
  'get_total_holders' : ActorMethod<[], TotalHolderResponse>,
  'get_working_stats' : ActorMethod<[], WorkingStats>,
  'read_logs' : ActorMethod<[], [] | [Array<LogEntry>]>,
  'remove_authorised' : ActorMethod<[string], string>,
  'set_canister_name' : ActorMethod<[string], string>,
  'set_stats_public' : ActorMethod<[boolean], string>,
  'set_stats_return_length' : ActorMethod<[bigint], string>,
  'set_stats_timescales' : ActorMethod<[bigint, bigint], string>,
  'set_target_canister' : ActorMethod<[string], string>,
  'stop_all_timers' : ActorMethod<[], string>,
}
