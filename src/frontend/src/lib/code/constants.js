const DOMAIN = 'http://localhost:3000'; // "https://app221b.herokuapp.com"; //
const MAX_TIME_LOGIN = 60 * 60 * 24 * 3; // 3 days in seconds
const MAX_SYNC_WAIT = 60 * 5; // 5 minutes
const DEFAULT_SUBACCOUNT = '0000000000000000000000000000000000000000000000000000000000000000';

const ckBTC_dataCanister = '26yxo-4iaaa-aaaak-qcfwa-cai';
const ckBTC_ledgerCanister = 'mxzaz-hqaaa-aaaar-qaada-cai';
const ckBTC_decimals = 8;
const ICP_dataCanister = 'ltzyb-kqaaa-aaaak-qcg4q-cai';
const ICP_ledgerCanister = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
const ICP_decimals = 8;
const CHAT_dataCanister = 'umtnw-jiaaa-aaaak-qciza-cai';
const CHAT_ledgerCanister = '2ouva-viaaa-aaaaq-aaamq-cai';
const CHAT_decimals = 8;
const SNS1_dataCanister = 'ulslc-eqaaa-aaaak-qcizq-cai';
const SNS1_ledgerCanister = 'zfcdd-tqaaa-aaaaq-aaaga-cai';
const SNS1_decimals = 8;
const KINIC_dataCanister = 'u6v2p-fyaaa-aaaak-qci2a-cai';
const KINIC_ledgerCanister = '73mez-iiaaa-aaaaq-aaasq-cai';
const KINIC_decimals = 8;
const HOT_dataCanister = 'uzu43-iaaaa-aaaak-qci2q-cai';
const HOT_ledgerCanister = '6rdgd-kyaaa-aaaaq-aaavq-cai';
const HOT_decimals = 8;
const GHOST_dataCanister = 'v2yv5-4yaaa-aaaak-qci4a-cai';
const GHOST_ledgerCanister = '4c4fd-caaaa-aaaaq-aaa3a-cai';
const GHOST_decimals = 8;

const ICRC_SnapshotCanister = '7hcof-tiaaa-aaaak-qcjfa-cai';
const ICP_SnapshotCanister = '4qqcx-cqaaa-aaaak-qcjnq-cai';

const token_swap_canisters = [
	{
		tokenPair: 'CKBTC/ICP',
		canister: 'k7tml-iaaaa-aaaak-aecgq-cai'
	},
	{
		tokenPair: 'CHAT/ICP',
		canister: '3we4s-lyaaa-aaaak-aegrq-cai'
	},
	{
		tokenPair: 'SNS1/ICP',
		canister: '32fn4-qqaaa-aaaak-ad65a-cai'
	},
	{
		tokenPair: 'KINIC/ICP',
		canister: 'bog3h-7qaaa-aaaak-aexmq-cai'
	},
	{
		tokenPair: 'HOT/ICP',
		canister: 'ntwyo-viaaa-aaaak-ae2pa-cai'
	},
	{
		tokenPair: 'GHOST/ICP',
		canister: 'gyh35-piaaa-aaaak-ae3ta-cai'
	}
];

let icrc_canister_ids = [
	{
		token: 'CKBTC',
		canister: ckBTC_dataCanister,
		decimals: ckBTC_decimals,
		ledger: ckBTC_ledgerCanister
	},
	{
		token: 'CHAT',
		canister: CHAT_dataCanister,
		decimals: CHAT_decimals,
		ledger: CHAT_ledgerCanister
	},
	{
		token: 'SNS1',
		canister: SNS1_dataCanister,
		decimals: SNS1_decimals,
		ledger: SNS1_ledgerCanister
	},
	{
		token: 'KINIC',
		canister: KINIC_dataCanister,
		decimals: KINIC_decimals,
		ledger: KINIC_ledgerCanister
	},
	{ token: 'HOT', canister: HOT_dataCanister, decimals: HOT_decimals, ledger: HOT_ledgerCanister },
	{
		token: 'GHOST',
		canister: GHOST_dataCanister,
		decimals: GHOST_decimals,
		ledger: GHOST_ledgerCanister
	}
];

export {
	DOMAIN,
	MAX_TIME_LOGIN,
	MAX_SYNC_WAIT,
	DEFAULT_SUBACCOUNT,
	token_swap_canisters,
	ICP_dataCanister,
	ICP_ledgerCanister,
	ICP_decimals,
	ckBTC_decimals,
	CHAT_decimals,
	SNS1_decimals,
	KINIC_decimals,
	HOT_decimals,
	GHOST_decimals,
	icrc_canister_ids,
	ICRC_SnapshotCanister,
	ICP_SnapshotCanister
};
