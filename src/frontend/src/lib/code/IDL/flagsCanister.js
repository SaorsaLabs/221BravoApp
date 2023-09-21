export const flagCanisterIDL = ({ IDL }) => {
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const FlagData = IDL.Variant({
    'FraudFlag' : IDL.Record({
      'id' : IDL.Text,
      'flag_from' : IDL.Nat64,
      'link' : IDL.Text,
      'time_added' : IDL.Nat64,
      'flagged_by' : IDL.Text,
    }),
    'MixerFlag' : IDL.Record({
      'id' : IDL.Text,
      'flag_from' : IDL.Nat64,
      'text' : IDL.Text,
      'level' : IDL.Nat8,
      'time_added' : IDL.Nat64,
    }),
    'CommunityFlag' : IDL.Record({
      'id' : IDL.Text,
      'flag_from' : IDL.Nat64,
      'link' : IDL.Text,
      'text' : IDL.Text,
      'time_added' : IDL.Nat64,
      'number_of_flags' : IDL.Nat32,
    }),
    'SARFlag' : IDL.Record({
      'id' : IDL.Text,
      'flag_from' : IDL.Nat64,
      'link' : IDL.Text,
      'time_added' : IDL.Nat64,
      'flagged_by' : IDL.Text,
    }),
    'GenesisFlag' : IDL.Record({
      'id' : IDL.Text,
      'flag_from' : IDL.Nat64,
      'text' : IDL.Text,
      'time_added' : IDL.Nat64,
    }),
  });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const FraudReport = IDL.Record({
    'submitter' : IDL.Text,
    'urls' : IDL.Text,
    'evidence' : IDL.Text,
    'account' : IDL.Text,
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_community_flag' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Text, IDL.Text, IDL.Nat32],
        [IDL.Text],
        [],
      ),
    'add_fraud_flag' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_fraud_report' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_genesis_flag' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_mixer_flag' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Text, IDL.Nat8],
        [IDL.Text],
        [],
      ),
    'add_sarf_flag' : IDL.Func(
        [IDL.Text, IDL.Nat64, IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'are_stats_public' : IDL.Func([], [IDL.Bool], ['query']),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_canister_logs' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
    'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_flags' : IDL.Func([IDL.Text], [IDL.Opt(IDL.Vec(FlagData))], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'read_fraud_reports' : IDL.Func([], [IDL.Vec(FraudReport)], ['query']),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_community_flag' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_fraud_flag' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_fraud_report' : IDL.Func([IDL.Nat64], [IDL.Text], []),
    'remove_genesis_flag' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_mixer_flag' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_sarf_flag' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
  });
};