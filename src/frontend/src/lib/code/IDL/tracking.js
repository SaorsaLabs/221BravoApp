export const trackingIDL = ({ IDL }) => {
    const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
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
    const MemoryStats = IDL.Record({
      'memory' : IDL.Nat64,
      'heap_memory' : IDL.Nat64,
    });
    const MixerWorkingStats = IDL.Record({
      'last_run_time' : IDL.Nat64,
      'last_tx_time' : IDL.Nat64,
      'awaiting_flagging' : IDL.Nat64,
      'is_upto_date' : IDL.Bool,
    });
    return IDL.Service({
      'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'are_stats_public' : IDL.Func([], [IDL.Bool], ['query']),
      'check_and_start_exchange_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
      'check_and_start_mixer_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
      'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
      'get_canister_logs' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
      'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
      'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
      'get_exchange_data' : IDL.Func([], [ExchangeCollection], ['query']),
      'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
      'get_mixer_workings_stats' : IDL.Func([], [MixerWorkingStats], ['query']),
      'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
      'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
      'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
      'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    });
  };