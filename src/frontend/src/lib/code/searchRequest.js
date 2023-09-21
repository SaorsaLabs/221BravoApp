import { DOMAIN } from '../code/constants.js';
import { authStore } from '../stores/authStore.js';
import { shortenString, parsePrincipalSubAccountString, getUniqueValues, combinePrincipalSubAccount, processPromises } from '../code/utils.js';
import { getUserNamedAccounts, getPublicNamedAccounts } from './searchRequest_v2.js';
import { calculateKey } from './auth.js';

async function getData(token, ID, subAccount, min, max, start, end) {
	let key = calculateKey();
	let ID2 = ID;
	let sub2 = subAccount;
	let getUser = authStore.read();
	let user = getUser.data.user;
	ID2 = ID2.replace(/\s/g, ''); // remove spaces
	sub2 = sub2.replace(/\s/g, ''); // remove spaces
	if (sub2 == '' || sub2 == null)
		sub2 = '0000000000000000000000000000000000000000000000000000000000000000';
	let params = `token=${token}&ID=${ID2}&sub=${sub2}&start=${start}&end=${end}&min=${min}&max=${max}&user=${user}&key=${key}`;
	let url = DOMAIN + `/v2/BasicSearch?${params}`;
	let settings = { method: 'Get', mode: 'cors', headers: { 'Content-Type': 'application/json' } };
	const txDATA = await fetch(url, settings).then((res) => res.json());
	return txDATA;
}

async function getBlockData(token, minBlock, maxBlock, startTime, endTime) {
	let key = calculateKey();
	let getUser = authStore.read();
	let user = getUser.data.user;
	let params = `token=${token}&startBlock=${minBlock}&endBlock=${maxBlock}&minTime=${startTime}&maxTime=${endTime}&user=${user}&key=${key}`;
	let url = DOMAIN + `/v2/BasicBlockSearch?${params}`;
	let settings = { method: 'Get', mode: 'cors', headers: { 'Content-Type': 'application/json' } };
	const txDATA = await fetch(url, settings).then((res) => res.json());
	return txDATA;
}

async function getLatestBlockData(blocks, token) {
	let key = calculateKey();
	let getUser = authStore.read();
	let user = getUser.data.user;
	let params = `token=${token}&blocks=${blocks}&user=${user}&key=${key}`;
	let url = DOMAIN + `/v2/LatestBlocks?${params}`;
	let settings = { method: 'Get', mode: 'cors', headers: { 'Content-Type': 'application/json' } };
	const txDATA = await fetch(url, settings);
	const resAR = await txDATA.json();
	resAR.blocks.reverse();
	return resAR;
}

async function getVisualBlockData(token, minBlock, maxBlock, startTime, endTime) {
	let key = calculateKey();
	let getUser = authStore.read();
	let user = getUser.data.user;
	let params = `token=${token}&startBlock=${minBlock}&endBlock=${maxBlock}&minTime=${startTime}&maxTime=${endTime}&user=${user}&key=${key}`;
	let url = DOMAIN + `/v2/VisualBlockSearch?${params}`;
	let settings = { method: 'Get', mode: 'cors', headers: { 'Content-Type': 'application/json' } };
	const txDATA = await fetch(url, settings).then((res) => res.json());

	// check names from v2 backend
	if (token != "ICP") {
        // logged in 
        let ls = authStore.read();
        if (ls.data.loggedIn == true) {
            // get unique accounts
			let dataLen = txDATA?.blocks?.length ?? 0;
            let uniqueACS = [];
            let jobArray = []
			let fmCombined, toCombined;
            for(let i = 0; i<dataLen; i++){
				fmCombined = combinePrincipalSubAccount(txDATA.blocks[i].fromPrincipal,txDATA.blocks[i].fromAccount);
				toCombined = combinePrincipalSubAccount(txDATA.blocks[i].toPrincipal,txDATA.blocks[i].toAccount);
                uniqueACS.push(fmCombined);
                uniqueACS.push(toCombined);
            }
            uniqueACS = getUniqueValues(uniqueACS);

            // Fetch name data for accounts
            jobArray[0] = getUserNamedAccounts(ls.data.user, uniqueACS);
            jobArray[1] = getPublicNamedAccounts(uniqueACS);
            let jobArrayDone = await processPromises(jobArray);

            // update links with names 
            let combinedSavedNames = [];
            let usrLen = jobArrayDone[0][0]?.length ?? 0;
            let gblLen = jobArrayDone[1][0]?.length ?? 0;
            if (usrLen != 0 && gblLen != 0) {
                combinedSavedNames = [...jobArrayDone[0][0], ...jobArrayDone[1][0]];
            } else if (usrLen != 0) {
                combinedSavedNames = jobArrayDone[0][0];
            } else if (gblLen != 0) {
                combinedSavedNames = jobArrayDone[1][0];
            }
            let csnLen = combinedSavedNames?.length ?? 0;

            // Add names to Blocks + Links
            if (csnLen != 0){
                // sort names decending
                combinedSavedNames.sort(function(a,b){ return a[0] > b[0] ? 1 : -1; }) 
                // Blocks
                let fm, to, k;
                for(let i=0; i<dataLen; i++){
					fm = txDATA.blocks[i].fromAccount;
					to = txDATA.blocks[i].toAccount;
                    for(k=0; k<csnLen; k++){
                        // from
                        if(combinedSavedNames[k][0] == fm) {
                            txDATA.blocks[i].fromAccountName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            txDATA.blocks[i].toAccountName = combinedSavedNames[k][1];
                        }
                    }
                }
            }
        }
	} else {

		let ls = authStore.read();
		if (ls.data.loggedIn == true) {
			// get unique accounts
			let uniqueACS = [];
			let jobArray = []
			let dataLen = txDATA?.blocks?.length ?? 0;
			for(let i = 0; i<dataLen; i++){
				uniqueACS.push(txDATA.blocks[i].fromAccount);
				uniqueACS.push(txDATA.blocks[i].toAccount);
			}
			
			uniqueACS = getUniqueValues(uniqueACS);

			// Fetch name data for accounts
			jobArray[0] = getUserNamedAccounts(ls.data.user, uniqueACS);
			jobArray[1] = getPublicNamedAccounts(uniqueACS);
			let jobArrayDone = await processPromises(jobArray);
			console.log(jobArrayDone);

			// update links with names 
			let combinedSavedNames = [];
			let usrLen = jobArrayDone[0][0]?.length ?? 0;
			let gblLen = jobArrayDone[1][0]?.length ?? 0;
			if (usrLen != 0 && gblLen != 0) {
				combinedSavedNames = [...jobArrayDone[0][0], ...jobArrayDone[1][0]];
			} else if (usrLen != 0) {
				combinedSavedNames = jobArrayDone[0][0];
			} else if (gblLen != 0) {
				combinedSavedNames = jobArrayDone[1][0];
			}
			let csnLen = combinedSavedNames?.length ?? 0;

			// Add names to Blocks + Links
			if (csnLen != 0){
				// sort names decending
				combinedSavedNames.sort(function(a,b){ return a[0] > b[0] ? 1 : -1; }) 
				// Blocks
				let fm, to, k;
				for(let i=0; i<dataLen; i++){
					fm = txDATA.blocks[i].fromAccount;
					to = txDATA.blocks[i].toAccount;
					for(k=0; k<csnLen; k++){
						// from
						if(combinedSavedNames[k][0] == fm) {
							txDATA.blocks[i].fromAccountName = combinedSavedNames[k][1];
						}
						// to
						if(combinedSavedNames[k][0] == to) {
							txDATA.blocks[i].toAccountName = combinedSavedNames[k][1];
						}
					}
				}
			}
		}

	}

	return txDATA;
}

function basicAccountTableTX(target, data, token) {

	let dataLen = data?.length ?? 0;
	let i;
	let shortID, shortSA;
	let longID;
	let longSubID, subName;
	let targetSub, targetName, targetSubName;
	let tgt;
	let direction;
	let OP = [];
	let counter = 1;
	let t1, tDate, tTime;
	let subHit;
	let is_icrc = target.includes('.');
	let principal = '';
	let subaccount = '';
	if (is_icrc) {
		let parse = parsePrincipalSubAccountString(target);
		principal = parse.principal;
		subaccount = parse.subaccount;
	}

	for (i = 0; i < dataLen; i++) {
		subHit == false;

		//direction
		if (is_icrc == false) {
			if (data[i].to == target) direction = 'in';
			else direction = 'out';
			// subHit?
			if (data[i].fromSub == target || data[i].toSub == target) subHit = true;
		} else {
			if (data[i].to == principal && data[i].toSub == subaccount) direction = 'in';
			else direction = 'out';
			subHit = true; // not part of icrc but needed below.
		}

		// shortAC
		if (data[i].type == 'burn' || data[i].type == 'Burn') {
			shortID = 'Burn';
			longID = 'Burn Account';
			longSubID = 'Burn Account';
			subName = undefined;
			tgt = subHit ? data[i].from : target;
			targetSub = data[i].fromSub;
			targetName = data[i]?.fromName ?? undefined;
			targetSubName = data[i]?.fromSubName ?? undefined;
		} else if (data[i].type == 'mint' || data[i].type == 'Mint') {
			shortID = 'Mint';
			longID = 'Minting Account';
			longSubID = 'Minting Account';
			subName = undefined;
			tgt = subHit ? data[i].to : target;
			targetSub = data[i].toSub;
			targetName = data[i]?.toName ?? undefined;
			targetSubName = data[i]?.toSubName ?? undefined;
		} else {
			if (direction == 'in') {
				shortID = data[i]?.fromName ?? shortenString(data[i].from);
				longID = data[i].from;
				longSubID = data[i].fromSub;
				subName = data[i]?.fromSubName ?? undefined;
				shortSA = subName ?? shortenString(data[i].fromSub);
				tgt = subHit ? data[i].to : target;
				targetSub = data[i].toSub;
				targetName = data[i]?.toName ?? undefined;
				targetSubName = data[i]?.toSubName ?? undefined;
			}
			if (direction == 'out') {
				shortID = data[i]?.toName ?? shortenString(data[i].to);
				longID = data[i].to;
				longSubID = data[i].toSub;
				subName = data[i]?.toSubName ?? undefined;
				shortSA = subName ?? shortenString(data[i].toSub);
				tgt = subHit ? data[i].from : target;
				targetSub = data[i].fromSub;
				targetName = data[i]?.fromName ?? undefined;
				targetSubName = data[i]?.fromSubName ?? undefined;
			}
		}

		// Time
		const options = { dateStyle: 'long', timeZone: 'UTC' };
		const options2 = { timeStyle: 'long', timeZone: 'UTC' };
		t1 = new Date(data[i].timeMilli);
		tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
		tTime = t1.toLocaleString('en-GB', options2); //
		OP[i] = {
			counter: counter,
			date: tDate,
			time: tTime,
			shortID: shortID,
			shortSA: shortSA,
			direction: direction,
			value: data[i].value,
			hash: data[i].hash,
			block: data[i].block,
			longID: longID,
			longSubID: longSubID,
			subName: subName,
			target: tgt,
			targetSub: targetSub,
			targetName: targetName,
			targetSubName: targetSubName,
			token: token
		};
		counter++;
	}
	OP.reverse();
	return OP;
}

function basicBlockTableTX(data, token, is_icrc) {
	let dataLen = data?.length ?? 0;
	let i;
	let fromShortID, toShortID;
	let fromShortSA, toShortSA;
	let usePrincipal = true;
	let OP = [];

	if (is_icrc == false) {
		let fmP = data[0]?.fromPrincipal ?? '';
		let toP = data[0]?.toPrincipal ?? '';
		let fmP2 = data[dataLen - 1]?.fromPrincipal ?? '';
		let toP2 = data[dataLen - 1]?.toPrincipal ?? '';
		// usePrincipal
		if (fmP == '' && toP == '' && fmP2 == '' && toP2 == '') {
			usePrincipal = false;
		}
		for (i = 0; i < dataLen; i++) {
			// shortAC
			if (data[i].type == 'burn' || data[i].type == 'Burn') {
				toShortID = 'Burn';
				if (usePrincipal == true) {
					fromShortID = shortenString(data[i].fromPrincipal);
				} else {
					fromShortID = shortenString(data[i].fromAccount);
				}
			} else if (data[i].type == 'mint' || data[i].type == 'Mint') {
				fromShortID = 'Mint';
				if (usePrincipal == true) {
					toShortID = shortenString(data[i].toPrincipal);
				} else {
					toShortID = shortenString(data[i].toAccount);
				}
			} else {
				if (usePrincipal == true) {
					fromShortID = data[i]?.fromPrincipalName
						? data[i].fromPrincipalName
						: shortenString(data[i].fromPrincipal);
					toShortID = data[i]?.toPrincipalName
						? data[i].toPrincipalName
						: shortenString(data[i].toPrincipal);
				} else {
					fromShortID = data[i]?.fromAccountName
						? data[i].fromAccountName
						: shortenString(data[i].fromAccount);
					toShortID = data[i]?.toAccountName
						? data[i].toAccountName
						: shortenString(data[i].toAccount);
				}
			}

			// Time
			let t1, tDate, tTime, tOP;
			const options = { dateStyle: 'long', timeZone: 'UTC' };
			const options2 = { timeStyle: 'long', timeZone: 'UTC' };
			t1 = new Date(data[i].timeMilli);
			tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
			tTime = t1.toLocaleString('en-GB', options2); //
			OP[i] = {
				block: data[i].block,
				date: tDate,
				time: tTime,
				fromShortID: fromShortID,
				toShortID: toShortID,
				fromShortSA: fromShortID,
				toShortSA: toShortID,
				toPrincipal: data[i].toPrincipal,
				toPrincipalName: data[i]?.toPrincipalName,
				toAccount: data[i].toAccount,
				toAccountName: data[i]?.toAccountName,
				fromPrincipal: data[i].fromPrincipal,
				fromPrincipalName: data[i]?.fromPrincipalName,
				fromAccount: data[i].fromAccount,
				fromAccountName: data[i]?.fromAccountName,
				value: data[i].value,
				hash: data[i].hash,
				token: token,
				type: data[i].type
			};
		}
	} else {
		// ICRC BLOCKS
		for (i = 0; i < dataLen; i++) {
			// shortAC
			if (data[i].type == 'burn' || data[i].type == 'Burn') {
				toShortID = 'Burn';
				toShortSA = '';
				fromShortID = shortenString(data[i].fromPrincipal);
				fromShortSA = shortenString(data[i].fromAccount);
			} else if (data[i].type == 'mint' || data[i].type == 'Mint') {
				fromShortID = 'Mint';
				fromShortSA = '';
				toShortID = shortenString(data[i].toPrincipal);
				toShortSA = shortenString(data[i].toAccount);
			} else {
				fromShortID = data[i]?.fromPrincipalName
					? data[i].fromPrincipalName
					: shortenString(data[i].fromPrincipal);
				fromShortSA = data[i]?.fromAccountName
					? data[i].fromAccountName
					: shortenString(data[i].fromAccount);
				toShortID = data[i]?.toPrincipalName
					? data[i].toPrincipalName
					: shortenString(data[i].toPrincipal);
				toShortSA = data[i]?.toAccountName
					? data[i].toAccountName
					: shortenString(data[i].toAccount);
			}

			// Time
			let t1, tDate, tTime, tOP;
			const options = { dateStyle: 'long', timeZone: 'UTC' };
			const options2 = { timeStyle: 'long', timeZone: 'UTC' };
			t1 = new Date(data[i].timeMilli);
			tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
			tTime = t1.toLocaleString('en-GB', options2); //

			OP[i] = {
				block: data[i].block,
				date: tDate,
				time: tTime,
				fromShortID: fromShortID,
				toShortID: toShortID,
				fromShortSA: fromShortSA,
				toShortSA: toShortSA,
				toPrincipal: data[i].toPrincipal,
				toPrincipalName: data[i]?.toPrincipalName,
				toAccount: data[i].toAccount,
				toAccountName: data[i]?.toAccountName,
				fromPrincipal: data[i].fromPrincipal,
				fromPrincipalName: data[i]?.fromPrincipalName,
				fromAccount: data[i].fromAccount,
				fromAccountName: data[i]?.fromAccountName,
				value: data[i].value,
				hash: data[i].hash,
				token: token,
				type: data[i].type
			};
		}
	}

	OP.reverse();
	let returnData = {
		usePrincipal,
		blocks: OP
	};
	return returnData;
}

function visualBlockSubTable(target, data, is_icrc) {
	// INPUT DATA CONTAINS:
	// block
	// endX
	// endY
	// fromAccount
	// fromPrincipal
	// fromAccountName
	// hash
	// startX
	// startY
	// timeMilli
	// toAccount
	// toAccountName
	// toPrincipal
	// token
	// type
	// value

	let dataLen = data?.length ?? 0;
	let i;
	let shortAC, shortPR;
	let targetACName, targetPRName;
	let targetAC, targetPR, longPR, longAC;
	let direction;
	let OP = [];
	let counter = 1;
	let t1, tDate, tTime;
	let sIDswitchF, sIDswitchT;
	let sACswitchF, sACswitchT;
	let sPRswitchF, sPRswitchT;
	let targACSwitch, targPRSwitch;

	if (is_icrc == false) {
		for (i = 0; i < dataLen; i++) {
			if (data[i].fromAccount) {
				sIDswitchF = data[i]?.fromAccountName ?? shortenString(data[i].fromAccount);
				sPRswitchF = data[i]?.fromPrincipalName ?? 'Unknown';
			} else {
				sIDswitchF = 'Mint';
				sPRswitchF = 'Mint';
			}

			if (data[i].toAccount) {
				sIDswitchT = data[i]?.toAccountName ?? shortenString(data[i].toAccount);
				sPRswitchT = data[i]?.fromPrincipalName ?? 'Unknown';
			} else {
				sIDswitchT = 'Burn';
				sPRswitchT = 'Burn';
			}

			//direction
			if (data[i].toAccount == target) direction = 'in';
			else direction = 'out';

			if (direction == 'in') {
				shortAC = sIDswitchF;
				shortPR = sPRswitchF;
				longPR = data[i]?.fromPrincipal ?? 'N/A';
				longAC = data[i]?.fromAccount ?? 'Mint';
				targetAC = target;
				targetACName = data[i]?.toAccountName ?? 'Unknown';
				targetPR = data[i]?.toPrincipal ?? 'N/A';
				targetPRName = data[i]?.toPrincipalName ?? 'Unknown';
			}
			if (direction == 'out') {
				shortAC = sIDswitchT;
				shortPR = sPRswitchT;
				longAC = data[i]?.toAccount ?? 'Burn';
				longPR = data[i]?.toPrincipal ?? 'N/A';
				targetAC = target;
				targetACName = data[i]?.fromAccountName ?? 'Unknown';
				targetPR = data[i]?.fromPrincipal ?? 'N/A';

				targetPRName = data[i]?.fromPrincipalName ?? 'Unknown';
			}

			// Time
			const options = { dateStyle: 'long', timeZone: 'UTC' };
			const options2 = { timeStyle: 'long', timeZone: 'UTC' };
			t1 = new Date(data[i].timeMilli);
			tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
			tTime = t1.toLocaleString('en-GB', options2); //
			OP[i] = {
				counter: counter,
				date: tDate,
				time: tTime,
				shortAC: shortAC,
				shortPR: shortPR,
				longAC: longAC,
				longPR: longPR,
				direction: direction,
				value: data[i].value,
				hash: data[i].hash,
				block: data[i].block,
				targetAC: targetAC,
				targetACName: targetACName,
				targetPR: targetPR,
				targetPRName: targetPRName,
				token: data[i].token,
				type: data[i].type
			};
			counter++;
		}
	} else {
		// parse target account from {principal}.{account}
		let parseICRC = parsePrincipalSubAccountString(target);
		for (i = 0; i < dataLen; i++) {
			if (data[i].fromAccount != '' && data[i].fromPrincipal != '') {
				sACswitchF = data[i]?.fromAccountName ?? shortenString(data[i].fromAccount);
				sPRswitchF = data[i]?.fromPrincipalName ?? shortenString(data[i].fromPrincipal);
			} else {
				sACswitchF = 'Mint';
				sPRswitchF = 'Mint';
			}

			if (data[i].toAccount != '' && data[i].toPrincipal != '') {
				sACswitchT = data[i]?.toAccountName ?? shortenString(data[i].toAccount);
				sPRswitchT = data[i]?.fromPrincipalName ?? shortenString(data[i].toPrincipal);
			} else {
				sACswitchT = 'Burn';
				sPRswitchT = 'Burn';
			}

			if (parseICRC.principal == '') targPRSwitch = 'Mint/ Burning Account';
			else targPRSwitch = parseICRC.principal;
			if (parseICRC.subaccount == '') targACSwitch = 'Mint/ Burning Account';
			else targACSwitch = parseICRC.subaccount;

			//direction
			if (data[i].toAccount == parseICRC.subaccount && data[i].toPrincipal == parseICRC.principal)
				direction = 'in';
			else direction = 'out';

			if (direction == 'in') {
				shortAC = sACswitchF; //data[i]?.fromAccountName ?? shortenString(data[i].fromAccount);
				shortPR = sPRswitchF; //data[i]?.fromPrincipalName ?? shortenString(data[i].fromPrincipal);
				longPR = data[i]?.fromPrincipal;
				longAC = data[i]?.fromAccount;
				targetPR = targPRSwitch;
				targetAC = targACSwitch;
				targetACName = data[i]?.toAccountName ?? 'Unknown';
				targetPRName = data[i]?.toPrincipalName ?? 'Unknown';
			}
			if (direction == 'out') {
				shortAC = sACswitchT; //data[i]?.toAccountName ?? shortenString(data[i].toAccount);
				shortPR = sPRswitchT; //data[i]?.toPrincipalName ?? shortenString(data[i].toPrincipal);
				longPR = data[i]?.toPrincipal;
				longAC = data[i]?.toAccount;
				targetPR = targPRSwitch;
				targetAC = targACSwitch;
				targetACName = data[i]?.fromAccountName ?? 'Unknown';
				targetPRName = data[i]?.fromPrincipalName ?? 'Unknown';
			}
			//}

			// Time
			const options = { dateStyle: 'long', timeZone: 'UTC' };
			const options2 = { timeStyle: 'long', timeZone: 'UTC' };
			t1 = new Date(data[i].timeMilli);
			tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
			tTime = t1.toLocaleString('en-GB', options2); //
			OP[i] = {
				counter: counter,
				date: tDate,
				time: tTime,
				shortAC: shortAC,
				shortPR: shortPR,
				longAC: longAC,
				longPR: longPR,
				direction: direction,
				value: data[i].value,
				hash: data[i].hash,
				block: data[i].block,
				targetAC: targetAC,
				targetACName: targetACName,
				targetPR: targetPR,
				targetPRName: targetPRName,
				token: data[i].token,
				type: data[i].type
			};
			counter++;
		}
	}
	return OP;
}

function linkedIDTable(data, token) {
	let dataLen = data?.length ?? 0;
	let i;
	let parseGross;
	let parseNet;
	let counter = 1;
	let split, shortID;

	for (i = 0; i < dataLen; i++) {
		if (data[i].ID.includes('.')) {
			split = parsePrincipalSubAccountString(data[i].ID);
			data[i].shortID = shortenString(split.principal);
			data[i].shortSA =
				split.subaccount == '0000000000000000000000000000000000000000000000000000000000000000'
					? 'N/A'
					: shortenString(split.subaccount);
			data[i].splitID = split.principal;
			data[i].splitSA = split.subaccount;
			data[i].grossValue = parseFloat(data[i].grossValue);
			data[i].netValue = parseFloat(data[i].netValue);
		} else {
			shortID = shortenString(data[i].ID);
			parseGross = parseFloat(data[i].grossValue);
			parseNet = parseFloat(data[i].netValue);
			data[i].shortID = shortID;
			data[i].shortAC = '';
			data[i].grossValue = parseGross;
			data[i].netValue = parseNet;
		}
	}
	data.sort(function (a, b) {
		return b.netValue - a.netValue;
	});
	for (i = 0; i < dataLen; i++) {
		data[i].count = counter;
		data[i].token = token;
		counter++;
	}
	return data;
}

export {
	getData,
	basicAccountTableTX,
	basicBlockTableTX,
	getBlockData,
	linkedIDTable,
	getLatestBlockData,
	getVisualBlockData,
	visualBlockSubTable
};
