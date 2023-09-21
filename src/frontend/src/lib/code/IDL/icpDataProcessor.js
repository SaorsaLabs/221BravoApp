export const icpDataIDL = ({ IDL }) => {
	const TotCntAvg = IDL.Record({
	  'count' : IDL.Nat,
	  'average' : IDL.Float64,
	  'total_value' : IDL.Nat,
	});
	const ProcessedTX = IDL.Record({
	  'hash' : IDL.Text,
	  'to_account' : IDL.Text,
	  'tx_value' : IDL.Nat,
	  'from_account' : IDL.Text,
	  'block' : IDL.Nat,
	  'tx_time' : IDL.Nat64,
	  'tx_type' : IDL.Text,
	});
	const MostActive = IDL.Tuple(IDL.Text, IDL.Nat64);
	const TimeChunkStats = IDL.Record({
	  'mint_count' : IDL.Nat64,
	  'end_time' : IDL.Nat64,
	  'start_time' : IDL.Nat64,
	  'burn_count' : IDL.Nat64,
	  'transaction_count' : IDL.Nat64,
	  'total_count' : IDL.Nat64,
	});
	const TimeStats = IDL.Record({
	  'transaction_stats' : TotCntAvg,
	  'total_unique_accounts' : IDL.Nat64,
	  'top_burns' : IDL.Vec(ProcessedTX),
	  'mint_stats' : TotCntAvg,
	  'total_transaction_average' : IDL.Float64,
	  'top_mints' : IDL.Vec(ProcessedTX),
	  'total_transaction_value' : IDL.Nat,
	  'top_transactions' : IDL.Vec(ProcessedTX),
	  'most_active_accounts' : IDL.Vec(MostActive),
	  'count_over_time' : IDL.Vec(TimeChunkStats),
	  'total_transaction_count' : IDL.Nat,
	  'burn_stats' : TotCntAvg,
	});
	const MemoryStats = IDL.Record({
	  'memory' : IDL.Nat64,
	  'heap_memory' : IDL.Nat64,
	});
	const HolderBalance = IDL.Record({
	  'balance' : IDL.Nat,
	  'holder' : IDL.Text,
	});
	const TopHolderResponse = IDL.Record({
	  'top_accounts' : IDL.Vec(HolderBalance),
	});
	const TotalHolderResponse = IDL.Record({ 'accounts' : IDL.Nat64 });
	const WorkingStats = IDL.Record({
	  'task_id' : IDL.Nat32,
	  'hr_stats_complete_to' : IDL.Nat,
	  'total_downloaded' : IDL.Nat,
	  'tx_completed_to' : IDL.Nat,
	  'version' : IDL.Text,
	  'next_tx' : IDL.Nat,
	  'is_upto_date' : IDL.Bool,
	  'day_stats_complete_to' : IDL.Nat,
	  'is_busy' : IDL.Bool,
	});
	const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
	return IDL.Service({
	  'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
	  'check_and_start_processing_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
	  'get_account_balance' : IDL.Func([IDL.Text], [IDL.Text], ['query']),
	  'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
	  'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
	  'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
	  'get_daily_stats' : IDL.Func([], [TimeStats], ['query']),
	  'get_hourly_stats' : IDL.Func([], [TimeStats], ['query']),
	  'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
	  'get_top_holders' : IDL.Func([IDL.Nat64], [TopHolderResponse], ['query']),
	  'get_total_holders' : IDL.Func([], [TotalHolderResponse], ['query']),
	  'get_working_stats' : IDL.Func([], [WorkingStats], ['query']),
	  'read_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
	  'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
	  'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
	  'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
	  'set_target_canister' : IDL.Func([], [IDL.Text], []),
	  'stop_all_timers' : IDL.Func([], [IDL.Text], []),
	  'update_stats_timescales' : IDL.Func(
		  [IDL.Nat64, IDL.Nat64],
		  [IDL.Text],
		  [],
		),
	});
  };