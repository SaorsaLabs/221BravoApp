export const idlFactory = ({ IDL }) => {
  const SmallTX = IDL.Record({
    'to' : IDL.Opt(IDL.Nat32),
    'value' : IDL.Nat64,
    'from' : IDL.Opt(IDL.Nat32),
    'time' : IDL.Nat64,
    'block' : IDL.Nat32,
    'tx_type' : IDL.Nat8,
  });
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_txs_to_store' : IDL.Func([IDL.Vec(SmallTX)], [IDL.Bool], []),
    'are_stats_public' : IDL.Func([], [IDL.Bool], ['query']),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_canister_logs' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
    'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_multiple_tx_from_store' : IDL.Func(
        [IDL.Vec(IDL.Nat32)],
        [IDL.Vec(IDL.Opt(SmallTX))],
        ['query'],
      ),
    'get_total_transactions' : IDL.Func([], [IDL.Nat32], ['query']),
    'get_tx_from_store' : IDL.Func([IDL.Nat32], [IDL.Opt(SmallTX)], ['query']),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
