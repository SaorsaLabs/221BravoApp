import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface LogEntry { 'text' : string, 'timestamp' : string }
export interface MemoryStats { 'memory' : bigint, 'heap_memory' : bigint }
export interface SmallTX {
  'to' : [] | [number],
  'value' : bigint,
  'from' : [] | [number],
  'time' : bigint,
  'block' : number,
  'tx_type' : number,
}
export interface _SERVICE {
  'add_authorised' : ActorMethod<[string], string>,
  'add_txs_to_store' : ActorMethod<[Array<SmallTX>], boolean>,
  'are_stats_public' : ActorMethod<[], boolean>,
  'get_all_authorised' : ActorMethod<[], Array<string>>,
  'get_canister_logs' : ActorMethod<[], Array<LogEntry>>,
  'get_canister_name' : ActorMethod<[], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_memory_stats' : ActorMethod<[], MemoryStats>,
  'get_multiple_tx_from_store' : ActorMethod<
    [Uint32Array | number[]],
    Array<[] | [SmallTX]>
  >,
  'get_total_transactions' : ActorMethod<[], number>,
  'get_tx_from_store' : ActorMethod<[number], [] | [SmallTX]>,
  'remove_authorised' : ActorMethod<[string], string>,
  'set_canister_name' : ActorMethod<[string], string>,
  'set_stats_public' : ActorMethod<[boolean], string>,
}
