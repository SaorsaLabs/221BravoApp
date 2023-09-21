import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export type AddressEntry = [string, string, string];
export type HolderData = [number, string];
export interface LogEntry { 'text' : string, 'timestamp' : string }
export interface MemoryStats { 'memory' : bigint, 'heap_memory' : bigint }
export type PublicAddressEntry = [string, string];
export interface UserData {
  'user_name' : string,
  'user_rank' : UserRank,
  'user_account' : string,
  'user_tokens' : number,
}
export type UserRank = { 'MasterSleuth' : null } |
  { 'Padawan' : null } |
  { 'GrandMasterSleuth' : null } |
  { 'Admin' : null } |
  { 'DataDetective' : null };
export interface _SERVICE {
  'add_authorised' : ActorMethod<[string], string>,
  'add_new_user' : ActorMethod<[string], boolean>,
  'add_public_named_accounts' : ActorMethod<[string, string], string>,
  'add_user_named_accounts' : ActorMethod<[string, string, string], string>,
  'check_and_start_genesis_timer' : ActorMethod<[bigint], string>,
  'get_cycles_balance' : ActorMethod<[], bigint>,
  'get_memory_stats' : ActorMethod<[], MemoryStats>,
  'get_multiple_account' : ActorMethod<[string, number, number], Array<string>>,
  'get_public_named_accounts' : ActorMethod<
    [],
    [] | [Array<PublicAddressEntry>]
  >,
  'get_single_account' : ActorMethod<[string, number], string>,
  'get_user_data' : ActorMethod<[string], [] | [UserData]>,
  'get_user_named_accounts' : ActorMethod<[string], [] | [Array<AddressEntry>]>,
  'getauthorised' : ActorMethod<[], Array<string>>,
  'is_genesis_holder' : ActorMethod<[string], boolean>,
  'read_genesis_holders' : ActorMethod<[], Array<HolderData>>,
  'read_logs' : ActorMethod<[], [] | [Array<LogEntry>]>,
  'remove_authorised' : ActorMethod<[string], string>,
  'remove_public_named_account' : ActorMethod<[string], string>,
  'remove_user_named_account' : ActorMethod<[string, string], string>,
  'stop_all_timers' : ActorMethod<[], string>,
  'update_user_tokens' : ActorMethod<[string, number], boolean>,
  'update_username' : ActorMethod<[string, string], boolean>,
  'whoami' : ActorMethod<[], string>,
}
