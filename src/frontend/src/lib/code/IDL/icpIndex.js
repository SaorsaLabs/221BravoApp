export const icpIndexIDL = ({ IDL }) => {
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
    const OverviewTuple = IDL.Tuple(IDL.Nat32, IDL.Nat64);
    const Overview = IDL.Record({
      'balance' : IDL.Nat64,
      'sent' : OverviewTuple,
      'last_active' : IDL.Nat64,
      'first_active' : IDL.Nat64,
      'received' : OverviewTuple,
    });
    const LinkDataResponse = IDL.Record({
      'net' : IDL.Int64,
      'linked_from' : IDL.Nat64,
      'linked_id' : IDL.Text,
      'number_txs' : IDL.Nat32,
      'gross' : IDL.Nat64,
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
    const FullDataResponse = IDL.Record({
      'overview' : Overview,
      'links' : IDL.Vec(LinkDataResponse),
      'account_ref' : IDL.Text,
      'blocks' : IDL.Vec(ProcessedTX),
    });
    const LinkData = IDL.Record({
      'net' : IDL.Int64,
      'linked_from' : IDL.Nat64,
      'linked_id' : IDL.Nat32,
      'number_txs' : IDL.Nat32,
      'gross' : IDL.Nat64,
    });
    const FullDataResponseRaw = IDL.Record({
      'overview' : Overview,
      'links' : IDL.Vec(LinkData),
      'account_ref' : IDL.Nat32,
      'blocks' : IDL.Vec(IDL.Nat32),
    });
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    const WorkingStats = IDL.Record({
      'timer_set' : IDL.Bool,
      'task_id' : IDL.Nat8,
      'total_downloaded' : IDL.Nat,
      'tx_completed_to' : IDL.Nat,
      'next_tx' : IDL.Nat,
      'is_upto_date' : IDL.Bool,
      'is_busy' : IDL.Bool,
    });
    return IDL.Service({
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'are_stats_public' : IDL.Func([], [IDL.Bool], ['query']),
      'check_and_start_processing_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_canister_logs' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
      'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_full_from_id' : IDL.Func([IDL.Text], [IDL.Opt(FullDataResponse)], []),
      'get_full_from_id_raw' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(FullDataResponseRaw)],
          ['query'],
        ),
      'get_full_from_ref' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(FullDataResponse)],
          [],
        ),
      'get_full_from_ref_raw' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(FullDataResponseRaw)],
          ['query'],
        ),
      'get_id_from_ref' : IDL.Func([IDL.Nat32], [IDL.Opt(IDL.Text)], ['query']),
      'get_latest_transactions' : IDL.Func(
          [IDL.Nat32],
          [IDL.Vec(ProcessedTX)],
          ['query'],
        ),
      'get_links_from_id' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(LinkDataResponse))],
          ['query'],
        ),
      'get_links_from_id_raw' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(LinkData))],
          ['query'],
        ),
      'get_links_from_ref' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(IDL.Vec(LinkDataResponse))],
          ['query'],
        ),
      'get_links_from_ref_raw' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(IDL.Vec(LinkData))],
          ['query'],
        ),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_multiple_tx' : IDL.Func(
          [IDL.Vec(IDL.Nat32)],
          [IDL.Vec(ProcessedTX)],
          [],
        ),
      'get_overview_by_id' : IDL.Func([IDL.Text], [IDL.Opt(Overview)], ['query']),
      'get_overview_by_ref' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(Overview)],
          ['query'],
        ),
      'get_ref_from_id' : IDL.Func([IDL.Text], [IDL.Opt(IDL.Nat32)], ['query']),
      'get_transactions_from_id' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(ProcessedTX))],
          [],
        ),
      'get_transactions_from_id_raw' : IDL.Func(
          [IDL.Text],
          [IDL.Opt(IDL.Vec(IDL.Nat32))],
          ['query'],
        ),
      'get_transactions_from_ref' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(IDL.Vec(ProcessedTX))],
          [],
        ),
      'get_transactions_from_ref_raw' : IDL.Func(
          [IDL.Nat32],
          [IDL.Opt(IDL.Vec(IDL.Nat32))],
          ['query'],
        ),
      'get_tx' : IDL.Func([IDL.Nat32], [IDL.Opt(ProcessedTX)], []),
      'get_working_stats' : IDL.Func([], [WorkingStats], ['query']),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
      'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
      'set_target_canister' : IDL.Func(
          [IDL.Text, IDL.Text, IDL.Text],
          [IDL.Text],
          [],
        ),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    });
  };