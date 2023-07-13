export const idlFactory = ({ IDL }) => {
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const UserRank = IDL.Variant({
    'MasterSleuth' : IDL.Null,
    'Padawan' : IDL.Null,
    'GrandMasterSleuth' : IDL.Null,
    'Admin' : IDL.Null,
    'DataDetective' : IDL.Null,
  });
  const UserData = IDL.Record({
    'user_name' : IDL.Text,
    'user_rank' : UserRank,
    'user_account' : IDL.Text,
    'user_tokens' : IDL.Nat32,
  });
  const HolderData = IDL.Tuple(IDL.Nat32, IDL.Text);
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_new_user' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'check_and_start_genesis_timer' : IDL.Func([IDL.Nat64], [IDL.Text], []),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_multiple_account' : IDL.Func(
        [IDL.Text, IDL.Nat32, IDL.Nat32],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'get_single_account' : IDL.Func(
        [IDL.Text, IDL.Nat32],
        [IDL.Text],
        ['query'],
      ),
    'get_user_data' : IDL.Func([IDL.Text], [IDL.Opt(UserData)], ['query']),
    'getauthorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'is_genesis_holder' : IDL.Func([IDL.Text], [IDL.Bool], ['query']),
    'isauthorised' : IDL.Func([], [IDL.Text], ['query']),
    'read_genesis_holders' : IDL.Func([], [IDL.Vec(HolderData)], ['query']),
    'read_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'stop_all_timers' : IDL.Func([], [IDL.Text], []),
    'update_user_tokens' : IDL.Func([IDL.Text, IDL.Nat32], [IDL.Bool], []),
    'update_username' : IDL.Func([IDL.Text, IDL.Text], [IDL.Bool], []),
    'whoami' : IDL.Func([], [IDL.Text], ['query']),
  });
};
export const init = ({ IDL }) => { return []; };
