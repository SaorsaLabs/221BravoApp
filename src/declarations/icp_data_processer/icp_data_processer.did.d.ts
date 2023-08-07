import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';

export interface HolderBalance {
	balance: bigint;
	holder: string;
}
export type HolderData = [number, string];
export interface LogEntry {
	text: string;
	timestamp: string;
}
export interface MemoryStats {
	memory: bigint;
	heap_memory: bigint;
}
export type MostActive = [string, bigint];
export interface ProcessedTX {
	hash: string;
	to_account: string;
	tx_value: bigint;
	from_account: string;
	block: bigint;
	tx_time: bigint;
	tx_type: string;
}
export interface TimeChunkStats {
	mint_count: bigint;
	end_time: bigint;
	start_time: bigint;
	burn_count: bigint;
	transaction_count: bigint;
	total_count: bigint;
}
export interface TimeStats {
	trasaction_stats: TotCntAvg;
	total_unique_accounts: bigint;
	top_burns: Array<ProcessedTX>;
	mint_stats: TotCntAvg;
	total_transaction_average: number;
	top_mints: Array<ProcessedTX>;
	total_transaction_value: bigint;
	top_transactions: Array<ProcessedTX>;
	most_active_accounts: Array<MostActive>;
	count_over_time: Array<TimeChunkStats>;
	total_transaction_count: bigint;
	burn_stats: TotCntAvg;
}
export interface TopHolderResponse {
	top_accounts: Array<HolderBalance>;
}
export interface TotCntAvg {
	count: bigint;
	average: number;
	total_value: bigint;
}
export interface TotalHolderResponse {
	accounts: bigint;
}
export interface WorkingStats {
	total_downloaded: bigint;
	tx_completed_to: bigint;
	next_tx: bigint;
}
export interface _SERVICE {
	add_authorised: ActorMethod<[string], string>;
	check_and_start_processing_timer: ActorMethod<[bigint], string>;
	get_account_balance: ActorMethod<[string], string>;
	get_cycles_balance: ActorMethod<[], bigint>;
	get_daily_stats: ActorMethod<[], TimeStats>;
	get_hourly_stats: ActorMethod<[], TimeStats>;
	get_memory_stats: ActorMethod<[], MemoryStats>;
	get_top_holders: ActorMethod<[bigint], TopHolderResponse>;
	get_total_holders: ActorMethod<[], TotalHolderResponse>;
	get_working_stats: ActorMethod<[], WorkingStats>;
	getauthorised: ActorMethod<[], Array<string>>;
	isauthorised: ActorMethod<[], string>;
	read_logs: ActorMethod<[], [] | [Array<LogEntry>]>;
	remove_authorised: ActorMethod<[string], string>;
	set_target_canister: ActorMethod<[], string>;
	stop_all_timers: ActorMethod<[], string>;
	update_stats_timescales: ActorMethod<[bigint, bigint], string>;
}
