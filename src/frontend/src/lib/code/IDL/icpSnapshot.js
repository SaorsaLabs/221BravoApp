export const icpSnapshotIDL = ({ IDL }) => {
  const ExchangeOverviewTotal = IDL.Record({
    'num_sent' : IDL.Nat64,
    'num_received' : IDL.Nat64,
    'num_transactions' : IDL.Nat64,
    'name' : IDL.Text,
    'total_received' : IDL.Nat64,
    'total_sent' : IDL.Nat64,
    'total_balance' : IDL.Nat64,
  });
  const ExchangeCollection = IDL.Record({
    'huobi' : ExchangeOverviewTotal,
    'gate' : ExchangeOverviewTotal,
    'bitfinex' : ExchangeOverviewTotal,
    'kucoin' : ExchangeOverviewTotal,
    'binance' : ExchangeOverviewTotal,
    'kraken' : ExchangeOverviewTotal,
    'coinbase' : ExchangeOverviewTotal,
    'coinex' : ExchangeOverviewTotal,
  });
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const QsRecord = IDL.Tuple(IDL.Nat, IDL.Nat64);
  const QuickStats = IDL.Record({
    'total_unique_accounts' : IDL.Vec(QsRecord),
    'total_account_holders' : IDL.Vec(QsRecord),
    'total_transaction_value' : IDL.Vec(QsRecord),
    'total_transaction_count' : IDL.Vec(QsRecord),
  });
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
    'total_account_holders' : IDL.Nat64,
    'top_mints' : IDL.Vec(ProcessedTX),
    'total_transaction_value' : IDL.Nat,
    'top_transactions' : IDL.Vec(ProcessedTX),
    'most_active_accounts' : IDL.Vec(MostActive),
    'count_over_time' : IDL.Vec(TimeChunkStats),
    'total_transaction_count' : IDL.Nat,
    'burn_stats' : TotCntAvg,
    'snapshot_time' : IDL.Nat64,
  });
  const WorkingStats = IDL.Record({
    'data_collections' : IDL.Vec(IDL.Text),
    'last_processed_time' : IDL.Nat64,
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_collection' : IDL.Func([IDL.Text, IDL.Text], [IDL.Text], []),
    'check_and_start_processing_timer' : IDL.Func([], [IDL.Text], []),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_exchange_snapshots' : IDL.Func(
        [IDL.Nat32],
        [IDL.Opt(IDL.Vec(ExchangeCollection))],
        ['query'],
      ),
    'get_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_quickstats' : IDL.Func(
        [IDL.Text, IDL.Nat],
        [IDL.Opt(QuickStats)],
        ['query'],
      ),
    'get_standard_snapshots' : IDL.Func(
        [IDL.Text, IDL.Nat],
        [IDL.Opt(IDL.Vec(TimeStats))],
        ['query'],
      ),
    'get_working_stats' : IDL.Func([], [WorkingStats], ['query']),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
    'stop_all_timers' : IDL.Func([], [IDL.Text], []),
  });
};