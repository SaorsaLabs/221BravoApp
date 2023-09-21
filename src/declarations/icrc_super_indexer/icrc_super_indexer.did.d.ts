import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface FullDataResponse {
  'overview' : Overview,
  'links' : Array<LinkDataResponse>,
  'account_ref' : string,
  'blocks' : Array<ProcessedTX>,
}
export interface FullDataResponseRaw {
  'overview' : Overview,
  'links' : Array<LinkData>,
  'account_ref' : number,
  'blocks' : Uint32Array | number[],
}
export interface LinkData {
  'net' : bigint,
  'linked_from' : bigint,
  'linked_id' : number,
  'number_txs' : number,
  'gross' : bigint,
}
export interface LinkDataResponse {
  'net' : bigint,
  'linked_from' : bigint,
  'linked_id' : string,
  'number_txs' : number,
  'gross' : bigint,
}
export interface LogEntry { 'text' : string, 'timestamp' : string }
export interface MemoryStats { 'memory' : bigint, 'heap_memory' : bigint }
export interface Overview {
  'balance' : bigint,
  'sent' : OverviewTuple,
  'last_active' : bigint,
  'first_active' : bigint,
  'received' : OverviewTuple,
}
export type OverviewTuple = [number, bigint];
export interface ProcessedTX {
  'hash' : string,
  'to_account' : string,
  'tx_value' : bigint,
  'from_account' : string,
  'block' : bigint,
  'tx_time' : bigint,
  'tx_type' : string,
}
export interface WorkingStats {
  'timer_set' : boolean,
  'task_id' : number,
  'total_downloaded' : bigint,
  'tx_completed_to' : bigint,
  'next_tx' : bigint,
  'is_upto_date' : boolean,
  'is_busy' : boolean,
}
export interface _SERVICE {
  'add_authorised' : ActorMethod<[string], string>,
  'are_stats_public' : ActorMethod<[], boolean>,
  'check_and_start_processing_timer' : ActorMethod<[bigint], string>,
  'get_all_authorised' : ActorMethod<[], Array<string>>,
  'get_canister_logs' : ActorMethod<[], Array<LogEntry>>,
  'get_canister_name' : ActorMethod<[], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_full_from_id' : ActorMethod<[string], [] | [FullDataResponse]>,
  'get_full_from_id_raw' : ActorMethod<[string], [] | [FullDataResponseRaw]>,
  'get_full_from_ref' : ActorMethod<[number], [] | [FullDataResponse]>,
  'get_full_from_ref_raw' : ActorMethod<[number], [] | [FullDataResponseRaw]>,
  'get_id_from_ref' : ActorMethod<[number], [] | [string]>,
  'get_latest_transactions' : ActorMethod<[number], Array<ProcessedTX>>,
  'get_links_from_id' : ActorMethod<[string], [] | [Array<LinkDataResponse>]>,
  'get_links_from_id_raw' : ActorMethod<[string], [] | [Array<LinkData>]>,
  'get_links_from_ref' : ActorMethod<[number], [] | [Array<LinkDataResponse>]>,
  'get_links_from_ref_raw' : ActorMethod<[number], [] | [Array<LinkData>]>,
  'get_memory_stats' : ActorMethod<[], MemoryStats>,
  'get_multiple_tx' : ActorMethod<[Uint32Array | number[]], Array<ProcessedTX>>,
  'get_overview_by_id' : ActorMethod<[string], [] | [Overview]>,
  'get_overview_by_ref' : ActorMethod<[number], [] | [Overview]>,
  'get_ref_from_id' : ActorMethod<[string], [] | [number]>,
  'get_transactions_from_id' : ActorMethod<[string], [] | [Array<ProcessedTX>]>,
  'get_transactions_from_id_raw' : ActorMethod<
    [string],
    [] | [Uint32Array | number[]]
  >,
  'get_transactions_from_ref' : ActorMethod<
    [number],
    [] | [Array<ProcessedTX>]
  >,
  'get_transactions_from_ref_raw' : ActorMethod<
    [number],
    [] | [Uint32Array | number[]]
  >,
  'get_tx' : ActorMethod<[number], [] | [ProcessedTX]>,
  'get_working_stats' : ActorMethod<[], WorkingStats>,
  'remove_authorised' : ActorMethod<[string], string>,
  'set_canister_name' : ActorMethod<[string], string>,
  'set_stats_public' : ActorMethod<[boolean], string>,
  'set_target_canister' : ActorMethod<[string, string], string>,
  'stop_all_timers' : ActorMethod<[], string>,
  'test_Small_tx_2' : ActorMethod<[], string>,
  'test_call_1' : ActorMethod<[], string>,
  'test_call_1A' : ActorMethod<[string], string>,
  'test_get_alldata' : ActorMethod<[number], string>,
  'test_index_stx_3' : ActorMethod<[], string>,
  'test_send_tx' : ActorMethod<[], string>,
}
