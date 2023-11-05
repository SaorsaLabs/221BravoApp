export const backendCanisterIDL = ({ IDL }) => {
  const PublicAddressEntry = IDL.Tuple(IDL.Text, IDL.Text);
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
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'encrypt' : IDL.Func(
      [IDL.Text],
      [IDL.Text],
      ['query'],
    ),
    'add_new_user' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'add_public_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_user_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'get_all_user_named_accounts' : IDL.Func(
        [IDL.Text],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'get_multiple_account' : IDL.Func(
        [IDL.Text, IDL.Nat32, IDL.Nat32],
        [IDL.Vec(IDL.Text)],
        ['query'],
      ),
    'get_public_named_accounts' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'get_single_account' : IDL.Func(
        [IDL.Text, IDL.Nat32],
        [IDL.Text],
        ['query'],
      ),
    'get_user_data' : IDL.Func([IDL.Text], [IDL.Opt(UserData)], ['query']),
    'get_user_named_accounts' : IDL.Func(
        [IDL.Text, IDL.Vec(IDL.Text)],
        [IDL.Opt(IDL.Vec(PublicAddressEntry))],
        ['query'],
      ),
    'getauthorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'is_genesis_holder' : IDL.Func([IDL.Text], [IDL.Bool], []),
    'read_logs' : IDL.Func([], [IDL.Opt(IDL.Vec(LogEntry))], ['query']),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_public_named_account' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_user_named_account' : IDL.Func(
        [IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'update_user_tokens' : IDL.Func([IDL.Text, IDL.Nat32], [IDL.Bool], []),
    'update_username' : IDL.Func([IDL.Text, IDL.Text], [IDL.Bool], []),
  });
};