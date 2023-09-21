import { getIdentity, icActor } from './icAgent.js';
import {
	canister_ids,
	token_swap_canisters,
	ICP_dataCanister,
	ICP_ledgerCanister,
	ICP_decimals,
	ICRC_SnapshotCanister,
	ICP_SnapshotCanister
} from './constants.js';
import { parsePrincipalSubAccountString, shortenString, processPromises } from './utils.js';
import { icrcDataIDL } from './IDL/icrcDataProcessor.js';
import { icpDataIDL } from './IDL/icpDataProcessor.js';
import { icrcLedgerIDL } from './IDL/icrcLedger.js';
import { icpLedgerIDL } from './IDL/icpLedger.js';
import { swapsIDL } from './IDL/icLightSwaps.js';
import { snapshotIDL } from './IDL/snapshot.js';
import { icpSnapshotIDL } from './IDL/icpSnapshot.js';

const canisterIDS = canister_ids;

// ICRC Stats
async function getICRC_Stats(token, timescale) {
	const ID = getIdentity();
	let known = false;
	let searchCanister;
	for (let i = 0; i < canisterIDS.length; i++) {
		if (token == canisterIDS[i].token) {
			searchCanister = canisterIDS[i].canister;
			known = true;
		}
	}
	if (known == false) {
		console.log(`Error - Cannot find ${token} in known Stats Canisters list`);
		return {
			burn_stats: {},
			count_over_time: [],
			mint_stats: {},
			most_active_accounts: [],
			most_active_principals: [],
			top_burns: [],
			top_mints: [],
			top_transactions: [],
			total_transaction_average: 0,
			total_transaction_count: 0,
			total_transaction_value: 0,
			total_unique_accounts: 0,
			total_unique_principals: 0,
			transaction_stats: {}
		};
	}
	if (timescale == 'hourly') {
		let actor = icActor(searchCanister, icrcDataIDL, ID);
		let stats = await actor.get_hourly_stats();
		return stats;
	}
	if (timescale == 'daily') {
		let actor = icActor(searchCanister, icrcDataIDL, ID);
		let stats = await actor.get_daily_stats();
		return stats;
	}
}

async function getICRC_TotalHolders(token) {
	const ID = getIdentity();
	let known = false;
	let searchCanister;
	for (let i = 0; i < canisterIDS.length; i++) {
		if (token == canisterIDS[i].token) {
			searchCanister = canisterIDS[i].canister;
			known = true;
		}
	}
	if (known == false) {
		console.log(`Error - Cannot find ${token} in known Stats Canisters list`);
		return 0;
	}
	let actor = icActor(searchCanister, icrcDataIDL, ID);
	let total = await actor.get_total_holders();
	return total;
}

async function getICRC_TopHolders(token, numberOfReturns) {
	if (numberOfReturns == 0 || !numberOfReturns) numberOfReturns = 10;
	let stats;
	let vPower;
	const ID = getIdentity();
	let known = false;
	let searchCanister;
	let searchDecimals;
	for (let i = 0; i < canisterIDS.length; i++) {
		if (token == canisterIDS[i].token) {
			searchCanister = canisterIDS[i].canister;
			searchDecimals = canisterIDS[i].decimals;
			known = true;
		}
	}
	if (known == false) {
		console.log(`Error - Cannot find ${token} in known Stats Canisters list`);
		return { topAccounts: [], topPrincipals: [] };
	}
	let actor = icActor(searchCanister, icrcDataIDL, ID);
	stats = await actor.get_top_holders(numberOfReturns);
	vPower = 1 / Math.pow(10, searchDecimals);

	let processed = [];
	let i, k;
	let statsLen;
	let hasSubAccount;
	let P, SA, split;
	let targID;
	let balance;
	let topACS = [];
	let topPRS = [];
	let count;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (k = 0; k < 2; k++) {
		if (k == 0) statsLen = stats.top_accounts.length ?? 0;
		if (k == 1) statsLen = stats.top_principals.length ?? 0;

		count = 1;
		for (i = 0; i < statsLen; i++) {
			if (k == 0) {
				targID = stats.top_accounts[i].holder;
				balance = (Number(stats.top_accounts[i].balance) * vPower).toLocaleString(
					'en-US',
					fmtOptionsValue
				);
			}
			if (k == 1) {
				targID = stats.top_principals[i].holder;
				balance = (Number(stats.top_principals[i].balance) * vPower).toLocaleString(
					'en-US',
					fmtOptionsValue
				);
			}

			if (targID.includes('.')) {
				split = parsePrincipalSubAccountString(targID);
				P = split.principal;
				SA = split.subaccount;
				if (
					split.subaccount == '0000000000000000000000000000000000000000000000000000000000000000'
				) {
					hasSubAccount = false;
				} else {
					hasSubAccount = true;
				}
			} else {
				P = targID;
				SA = '';
				hasSubAccount = false;
			}
			if (k == 0) {
				topACS[i] = {
					holderID: count,
					hasSubAccount,
					balance: balance,
					principal: P,
					subAccount: SA
				};
			} else {
				topPRS[i] = {
					holderID: count,
					hasSubAccount,
					balance: balance,
					principal: P,
					subAccount: SA
				};
			}
			count++;
		} //i
	} //k
	processed = { topAccounts: topACS, topPrincipals: topPRS };
	return processed;
}

async function getICRC_TotalSupply(token) {
	let ret = 0;
	let stats, vPower;
	let known = false;
	let searchCanister;
	let searchDecimals;
	for (let i = 0; i < canisterIDS.length; i++) {
		if (token == canisterIDS[i].token) {
			searchCanister = canisterIDS[i].ledger;
			searchDecimals = canisterIDS[i].decimals;
			known = true;
		}
	}
	if (known == false) {
		console.log(`Error - Cannot find ${token} in known Stats Canisters list`);
		return 0;
	}
	let actor = icActor(searchCanister, icrcLedgerIDL);
	stats = await actor.icrc1_total_supply();
	vPower = 1 / Math.pow(10, searchDecimals);
	ret = Number(stats) * vPower;
	return ret;
}

async function getICRC_SnapshotQuickStats(token, return_length) {
	const ID = getIdentity();
	let stats;
	let actor = icActor(ICRC_SnapshotCanister, snapshotIDL, ID);
	stats = await actor.get_quickstats(token, return_length);
	return stats;
}

async function getICP_SnapshotQuickStats(token, return_length) {
	const ID = getIdentity();
	let stats;
	let actor = icActor(ICP_SnapshotCanister, icpSnapshotIDL, ID);
	stats = await actor.get_quickstats(token, return_length);
	return stats;
}

function processTopTxData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromPshort, toPshort, fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	for (i = 0; i < txLen; i++) {
		fromPshort = shortenString(canisterTxs[i].from_principal);
		toPshort = shortenString(canisterTxs[i].to_principal);
		fromAshort = shortenString(canisterTxs[i].from_account);
		toAshort = shortenString(canisterTxs[i].to_account);
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);
		const fmtOptionsValue = {
			style: 'decimal',
			maximumFractionDigits: 8,
			minimumFractionDigits: 0
		};
		res.push({
			token: token,
			fromPshort,
			toPshort,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Transfer',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromPrincipal: canisterTxs[i].from_principal,
			toPrincipal: canisterTxs[i].to_principal,
			fromAccount: canisterTxs[i].from_account,
			toAccount: canisterTxs[i].to_account
		});
	}
	return res;
}

function processTopMintData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromPshort, toPshort, fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (i = 0; i < txLen; i++) {
		fromPshort = 'Minting Account';
		toPshort = shortenString(canisterTxs[i].to_principal);
		fromAshort = '';
		toAshort = shortenString(canisterTxs[i].to_account);
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);
		res.push({
			token: token,
			fromPshort,
			toPshort,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Mint',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromPrincipal: 'Minting Account',
			toPrincipal: canisterTxs[i].to_principal,
			fromAccount: '',
			toAccount: canisterTxs[i].to_account
		});
	}
	return res;
}

function processTopBurnData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromPshort, toPshort, fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (i = 0; i < txLen; i++) {
		fromPshort = shortenString(canisterTxs[i].from_principal);
		toPshort = 'Burning Account';
		fromAshort = shortenString(canisterTxs[i].from_account);
		toAshort = '';
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);
		res.push({
			token: token,
			fromPshort,
			toPshort,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Burn',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromPrincipal: canisterTxs[i].from_principal,
			toPrincipal: 'Burning Account',
			fromAccount: canisterTxs[i].from_account,
			toAccount: ''
		});
	}
	return res;
}

// ICP Stats
async function getICP_Stats(timescale) {
	const ID = getIdentity();
	if (timescale == 'hourly') {
		let actor = icActor(ICP_dataCanister, icpDataIDL, ID);
		let stats = await actor.get_hourly_stats();
		return stats;
	}

	if (timescale == 'daily') {
		let actor = icActor(ICP_dataCanister, icpDataIDL, ID);
		let stats = await actor.get_daily_stats();
		return stats;
	}
}

async function getICP_TotalHolders() {
	const ID = getIdentity();
	let actor = icActor(ICP_dataCanister, icpDataIDL, ID);
	let stats = await actor.get_total_holders();
	return stats;
}

async function getICP_TopHolders(numberOfReturns) {
	if (numberOfReturns == 0 || !numberOfReturns) numberOfReturns = 10;
	let stats;
	let vPower;
	const ID = getIdentity();
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	let actor = icActor(ICP_dataCanister, icpDataIDL, ID);
	stats = await actor.get_top_holders(numberOfReturns);
	vPower = 1 / Math.pow(10, ICP_decimals);
	let processed = [];
	let i;
	let statsLen;
	let targID;
	let balance;
	let topACS = [];
	let count;

	statsLen = stats.top_accounts.length ?? 0;
	count = 1;
	for (i = 0; i < statsLen; i++) {
		targID = stats.top_accounts[i].holder;
		balance = (Number(stats.top_accounts[i].balance) * vPower).toLocaleString(
			'en-US',
			fmtOptionsValue
		);
		topACS[i] = {
			holderID: count,
			hasSubAccount: false,
			balance: balance,
			account: targID
		};
		count++;
	} //i

	processed = { topAccounts: topACS };
	return processed;
}

async function getICP_TotalSupply() {
	let ret = 0;
	let stats, vPower;
	let actor = icActor(ICP_ledgerCanister, icpLedgerIDL);
	stats = await actor.icrc1_total_supply();
	vPower = 1 / Math.pow(10, ICP_decimals);
	ret = Number(stats) * vPower;
	return ret;
}

function processICPTopTxData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (i = 0; i < txLen; i++) {
		fromAshort = shortenString(canisterTxs[i].from_account);
		toAshort = shortenString(canisterTxs[i].to_account);
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);

		res.push({
			token: token,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Transfer',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromAccount: canisterTxs[i].from_account,
			toAccount: canisterTxs[i].to_account
		});
	}
	return res;
}

function processICPTopMintData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromPshort, toPshort, fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (i = 0; i < txLen; i++) {
		fromAshort = 'Minting Account';
		toAshort = shortenString(canisterTxs[i].to_account);
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);
		res.push({
			token: token,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Mint',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromAccount: 'Minting Account',
			toAccount: canisterTxs[i].to_account
		});
	}
	return res;
}

function processICPTopBurnData(canisterTxs, decimals, token) {
	let i;
	let txLen = canisterTxs.length ?? 0;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	let fromPshort, toPshort, fromAshort, toAshort;
	let vPower = 1 / Math.pow(10, decimals);
	let txValue;
	let t1, tDate, tTime;
	let res = [];
	let timeMilli;
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
	for (i = 0; i < txLen; i++) {
		fromAshort = shortenString(canisterTxs[i].from_account);
		toAshort = 'Burning Account';
		txValue = Number(canisterTxs[i].tx_value) * vPower;
		timeMilli = BigInt(canisterTxs[i].tx_time) / BigInt(1000000);
		t1 = new Date(Number(timeMilli)); // nano to milli
		tDate = t1.toLocaleString('en-GB', options);
		tTime = t1.toLocaleString('en-GB', options2);
		res.push({
			token: token,
			fromAshort,
			toAshort,
			date: tDate,
			time: tTime,
			value: txValue.toLocaleString('en-US', fmtOptionsValue),
			type: 'Burn',
			block: canisterTxs[i].block,
			hash: canisterTxs[i].hash,
			fromAccount: canisterTxs[i].from_account,
			toAccount: 'Burning Account'
		});
	}
	return res;
}

// USES ICDex
async function getPriceData(tokenPair) {
	if (tokenPair != 'ICP/USD') {
		let scLen = token_swap_canisters.length ?? 0;
		let i;
		let targetCanister = '';
		let foundCanister = false;
		for (i = 0; i < scLen; i++) {
			if (tokenPair == token_swap_canisters[i].tokenPair) {
				targetCanister = token_swap_canisters[i].canister;
				foundCanister = true;
			}
		}
		if (foundCanister) {
			let actor = icActor(targetCanister, swapsIDL);
			let data = await actor.stats();
			return { price: data.price, change: data.change24h, dollar: 0 };
		} else {
			return { price: 0, change: 0, dollar: 0 };
		}
	} else if (tokenPair == 'ICP/USD') {
		try {
		    let API_KEY =  import.meta.env.VITE_CMC_KEY;
		    let ID = "8916";
		    const url = `https://pro-api.coinmarketcap.com/v1/cryptocurrency/quotes/latest?id=${ID}&CMC_PRO_API_KEY=${API_KEY}`;
		    const response = await fetch(url);
		    const resJS = await response.json();
		    let res = {
		        price: resJS.data['8916'].quote.USD.price.toFixed(2),
		        change: resJS.data['8916'].quote.USD.percent_change_24h.toFixed(2),
		        dollar: 0,
		    }
		   return res;
		} catch (error) {
		    console.log(error);
		    return {price: 0, change: 0, dollar: 0};
		}
	 }
}

async function getTokenTableData(){
	let loopLen = canister_ids.length ?? 0;
	let promAR = [];
	let compAR;
	let resOBJ;
	let resAR = [];
	let resTKN = [];
	let pos = 0;
	for(let i=0; i<loopLen; i++){
		if(canister_ids[i].token == "ICP"){
			// ICP STATS
			promAR[pos] = getICP_TotalHolders();
			promAR[pos+1] = getICP_Stats('hourly');
			resTKN.push(canister_ids[i].token);
		} else {
			// ICRC STATS
			promAR[pos] = getICRC_TotalHolders(canister_ids[i].token);
			promAR[pos+1] = getICRC_Stats(canister_ids[i].token, 'hourly');
		}
		pos+=2;
	}

	compAR = await processPromises(promAR);
	let cnt = 0;
	let cARLen = compAR.length ?? 0;
	for(let i=0; i<cARLen; i+=2){
			let lc = canister_ids[cnt].token.toLowerCase();
			resOBJ = {
				link: lc,
				token: canister_ids[cnt].token,
				holders: Number(compAR[i].accounts),
				transactions: Number(compAR[i+1].total_transaction_count),
				active: Number(compAR[i+1].total_unique_accounts),
			};
			cnt++;
			resAR.push(resOBJ);
	}
	return resAR;
}

async function getExchangeBalances(num_to_return){
	if (!num_to_return) num_to_return = 5;

	const ID = getIdentity();
	let stats;
	let actor = icActor(ICP_SnapshotCanister, icpSnapshotIDL, ID);
	stats = await actor.get_exchange_snapshots(num_to_return);
	
	return stats;
}

export {
	getICRC_Stats,
	getPriceData,
	getICRC_TopHolders,
	processTopTxData,
	getICRC_TotalHolders,
	getICRC_TotalSupply,
	processTopMintData,
	processTopBurnData,
	getICP_Stats,
	getICP_TotalHolders,
	getICP_TopHolders,
	getICP_TotalSupply,
	processICPTopTxData,
	processICPTopMintData,
	processICPTopBurnData,
	getICRC_SnapshotQuickStats,
	getICP_SnapshotQuickStats,
	getTokenTableData,
	getExchangeBalances
};
