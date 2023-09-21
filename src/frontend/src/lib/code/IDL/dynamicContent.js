export const dynamicContentIDL = ({ IDL }) => {
  const LogEntry = IDL.Record({ 'text' : IDL.Text, 'timestamp' : IDL.Text });
  const MemoryStats = IDL.Record({
    'memory' : IDL.Nat64,
    'heap_memory' : IDL.Nat64,
  });
  const ProjectCard = IDL.Record({
    'title' : IDL.Text,
    'project_url' : IDL.Text,
    'image_url' : IDL.Text,
    'sub_title' : IDL.Text,
  });
  const ProjectCollection = IDL.Record({
    'bucket1' : IDL.Vec(ProjectCard),
    'bucket2' : IDL.Vec(ProjectCard),
    'bucket3' : IDL.Vec(ProjectCard),
    'bucket4' : IDL.Vec(ProjectCard),
    'bucket5' : IDL.Vec(ProjectCard),
    'bucket6' : IDL.Vec(ProjectCard),
    'bucket7' : IDL.Vec(ProjectCard),
    'bucket8' : IDL.Vec(ProjectCard),
  });
  const NewsItem = IDL.Record({
    'title' : IDL.Text,
    'article_url' : IDL.Text,
    'image_url' : IDL.Text,
    'sub_title' : IDL.Text,
  });
  return IDL.Service({
    'add_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'add_news_item' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'add_project' : IDL.Func(
        [IDL.Nat8, IDL.Text, IDL.Text, IDL.Text, IDL.Text],
        [IDL.Text],
        [],
      ),
    'are_stats_public' : IDL.Func([], [IDL.Bool], ['query']),
    'get_all_authorised' : IDL.Func([], [IDL.Vec(IDL.Text)], ['query']),
    'get_canister_logs' : IDL.Func([], [IDL.Vec(LogEntry)], ['query']),
    'get_canister_name' : IDL.Func([], [IDL.Text], ['query']),
    'get_cycles_balance' : IDL.Func([], [IDL.Nat64], ['query']),
    'get_memory_stats' : IDL.Func([], [MemoryStats], ['query']),
    'read_all_project_buckets' : IDL.Func([], [ProjectCollection], ['query']),
    'read_news_items' : IDL.Func([], [IDL.Vec(NewsItem)], ['query']),
    'read_single_project_bucket' : IDL.Func(
        [IDL.Nat8],
        [IDL.Vec(ProjectCard)],
        ['query'],
      ),
    'remove_authorised' : IDL.Func([IDL.Text], [IDL.Text], []),
    'remove_news_item' : IDL.Func([IDL.Nat64], [IDL.Text], []),
    'remove_project' : IDL.Func([IDL.Nat64, IDL.Nat8], [IDL.Text], []),
    'set_canister_name' : IDL.Func([IDL.Text], [IDL.Text], []),
    'set_stats_public' : IDL.Func([IDL.Bool], [IDL.Text], []),
  });
};