const DOMAIN =   "https://app221b.herokuapp.com"; // 'http://localhost:3000'; //
const MAX_TIME_LOGIN = 60 * 60 * 24 * 3; // 3 days in seconds
const MAX_CACHE_TIME = 60 * 15; // 15 minutes in seconds
const MAX_SYNC_WAIT = 60 * 5; // 5 minutes *** SOON DEPRICATED
const MAX_BLOCK_REQUEST = 10000;
const DEFAULT_SUBACCOUNT = '0000000000000000000000000000000000000000000000000000000000000000';

const backendCanisterID = "meped-laaaa-aaaak-qcbsa-cai";
const genesisNFTCanister = "t555s-uyaaa-aaaal-qbjsa-cai";
const dynamicContentCanister = "a5jy4-uyaaa-aaaak-qclma-cai";
const assetsCanister = "bxg2g-wiaaa-aaaak-qclla-cai";

const ckBTC_dataCanister = '26yxo-4iaaa-aaaak-qcfwa-cai';
const ckBTC_ledgerCanister = 'mxzaz-hqaaa-aaaar-qaada-cai';
const ckBTC_decimals = 8;
const ckBTC_index = '4lv6s-yiaaa-aaaak-qcjpa-cai';

const ICP_dataCanister = 'ltzyb-kqaaa-aaaak-qcg4q-cai';
const ICP_ledgerCanister = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
const ICP_decimals = 8;
const ICP_index = 'mx6pc-wyaaa-aaaak-qckaa-cai';

const CHAT_dataCanister = 'umtnw-jiaaa-aaaak-qciza-cai';
const CHAT_ledgerCanister = '2ouva-viaaa-aaaaq-aaamq-cai';
const CHAT_decimals = 8;
const CHAT_index = 'ly2wh-viaaa-aaaak-qckra-cai';

const SNS1_dataCanister = 'ulslc-eqaaa-aaaak-qcizq-cai';
const SNS1_ledgerCanister = 'zfcdd-tqaaa-aaaaq-aaaga-cai';
const SNS1_decimals = 8;
const SNS1_index = 'lk4b6-zyaaa-aaaak-qcksa-cai';

const KINIC_dataCanister = 'u6v2p-fyaaa-aaaak-qci2a-cai';
const KINIC_ledgerCanister = '73mez-iiaaa-aaaaq-aaasq-cai';
const KINIC_decimals = 8;
const KINIC_index = 'le6mw-ciaaa-aaaak-qckta-cai';

const HOT_dataCanister = 'uzu43-iaaaa-aaaak-qci2q-cai';
const HOT_ledgerCanister = '6rdgd-kyaaa-aaaaq-aaavq-cai';
const HOT_decimals = 8;
const HOT_index = 'korom-ayaaa-aaaak-qckua-cai';

const GHOST_dataCanister = 'v2yv5-4yaaa-aaaak-qci4a-cai';
const GHOST_ledgerCanister = '4c4fd-caaaa-aaaaq-aaa3a-cai';
const GHOST_decimals = 8;
const GHOST_index = 'katde-3iaaa-aaaak-qckva-cai';

const ModClub_dataCanister = 'cssbn-laaaa-aaaak-qclaq-cai';
const ModClub_ledgerCanister = 'xsi2v-cyaaa-aaaaq-aabfq-cai';
const ModClub_decimals = 8;
const ModClub_index = 'c4qmf-qqaaa-aaaak-qclbq-cai';

const CAT_dataCanister = 'fuvtu-6yaaa-aaaak-qclqa-cai';
const CAT_ledgerCanister = 'uf2wh-taaaa-aaaaq-aabna-cai';
const CAT_decimals = 8;
const CAT_index = 'f2x64-fiaaa-aaaak-qclra-cai';

const BOOM_dataCanister = 'gycdd-vyaaa-aaaak-qcl2a-cai';
const BOOM_ledgerCanister = 'vtrom-gqaaa-aaaaq-aabia-cai';
const BOOM_decimals = 8;
const BOOM_index = 'g7dfx-yaaaa-aaaak-qcl2q-cai';

const ICX_dataCanister = 'grbi7-dqaaa-aaaak-qcl3q-cai';
const ICX_ledgerCanister = 'rffwt-piaaa-aaaaq-aabqq-cai';
const ICX_decimals = 8;
const ICX_index = 'h3okf-baaaa-aaaak-qcl4q-cai';

const NUANCE_dataCanister = 'burde-oqaaa-aaaak-qcmwa-cai';
const NUANCE_ledgerCanister = 'rxdbk-dyaaa-aaaaq-aabtq-cai';
const NUANCE_decimals = 8;
const NUANCE_index = 'btqfq-diaaa-aaaak-qcmwq-cai';


const ICRC_SnapshotCanister = '7hcof-tiaaa-aaaak-qcjfa-cai';
const ICP_SnapshotCanister = '4qqcx-cqaaa-aaaak-qcjnq-cai';
const flagCanister = "mz4ck-niaaa-aaaak-qckba-cai";
const trackingCanister = "gnfso-uqaaa-aaaak-qclzq-cai";

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
	},
	{
		tokenPair: 'MOD/ICP',
		canister: 'iq6by-ryaaa-aaaak-ae5pq-cai'
	},
	{
		tokenPair: 'BOOM/ICP',
		canister: '2nmer-3aaaa-aaaak-ae6na-cai'
	},
	{
		tokenPair: 'HOT/ICP',
		canister: 'ntwyo-viaaa-aaaak-ae2pa-cai'
	},
	{
		tokenPair: 'CAT/ICP',
		canister: 'zu4fl-riaaa-aaaak-ae6eq-cai'
	},
	{
		tokenPair: 'NUANCE/ICP',
		canister: 'zu4fl-riaaa-aaaak-ae6eq-cai'
	}
];

let canister_ids = [
	{
		token: 'ICP',
		canister: ICP_dataCanister,
		decimals: ckBTC_decimals,
		ledger: ICP_ledgerCanister,
		index: ICP_index 
	},
	{
		token: 'CKBTC',
		canister: ckBTC_dataCanister,
		decimals: ckBTC_decimals,
		ledger: ckBTC_ledgerCanister,
		index: ckBTC_index
	},
	{
		token: 'CHAT',
		canister: CHAT_dataCanister,
		decimals: CHAT_decimals,
		ledger: CHAT_ledgerCanister,
		index: CHAT_index 
	},
	{
		token: 'SNS1',
		canister: SNS1_dataCanister,
		decimals: SNS1_decimals,
		ledger: SNS1_ledgerCanister,
		index: SNS1_index 
	},
	{
		token: 'KINIC',
		canister: KINIC_dataCanister,
		decimals: KINIC_decimals,
		ledger: KINIC_ledgerCanister,
		index: KINIC_index 
	},
	{ 
		token: 'HOT', 
		canister: HOT_dataCanister, 
		decimals: HOT_decimals, 
		ledger: HOT_ledgerCanister,
		index: HOT_index 
	},
	{
		token: 'GHOST',
		canister: GHOST_dataCanister,
		decimals: GHOST_decimals,
		ledger: GHOST_ledgerCanister,
		index: GHOST_index 
	},
	{
		token: 'MODCLUB',
		canister: ModClub_dataCanister,
		decimals: ModClub_decimals,
		ledger: ModClub_ledgerCanister,
		index: ModClub_index 
	},
	{
		token: 'CAT',
		canister: CAT_dataCanister,
		decimals: CAT_decimals,
		ledger: CAT_ledgerCanister,
		index: CAT_index 
	},
	{
		token: 'BOOM',
		canister: BOOM_dataCanister,
		decimals: BOOM_decimals,
		ledger: BOOM_ledgerCanister,
		index: BOOM_index 
	},
	{
		token: 'ICX',
		canister: ICX_dataCanister,
		decimals: ICX_decimals,
		ledger: ICX_ledgerCanister,
		index: ICX_index 
	},
	{
		token: 'NUANCE',
		canister: NUANCE_dataCanister,
		decimals: NUANCE_decimals,
		ledger: NUANCE_ledgerCanister,
		index: NUANCE_index 
	}
];

export {
	DOMAIN,
	MAX_TIME_LOGIN,
	MAX_SYNC_WAIT,
	MAX_CACHE_TIME,
	DEFAULT_SUBACCOUNT,
	MAX_BLOCK_REQUEST,
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
	ModClub_decimals,
	CAT_decimals,
	BOOM_decimals,
	ICX_decimals,
	NUANCE_decimals,
	canister_ids,
	ICRC_SnapshotCanister,
	ICP_SnapshotCanister,
	backendCanisterID,
	genesisNFTCanister,
	dynamicContentCanister,
	assetsCanister,
	flagCanister,
	trackingCanister,
};
