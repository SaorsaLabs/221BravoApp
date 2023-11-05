import { canister_ids, backendCanisterID, DEFAULT_SUBACCOUNT, MAX_BLOCK_REQUEST, flagCanister } from './constants.js';
import { getIdentity, icActor } from './icAgent.js';
import { icrcIndexIDL } from './IDL/icrcIndex.js';
import { backendCanisterIDL } from './IDL/backend.js';
import { flagCanisterIDL } from './IDL/flagsCanister.js';
import { icpIndexIDL } from './IDL/icpIndex.js';
import { 
    combinePrincipalSubAccount, 
    processPromises, 
    parsePrincipalSubAccountString, 
    getUniqueValues, 
    getTokenData,
    shortenString, 
    getAllTokenData
    } from './utils.js';
import { authStore } from '../stores/authStore.js';

// get account data
async function getData(token, ID, subAccount) { 
    // destructure if needed
    if (ID.includes('.', 0)){
        let ds = parsePrincipalSubAccountString(ID);
        ID = ds.principal;
        subAccount = ds.subaccount;
    } 
    // default sub account if needed
    else if (subAccount == "" || subAccount == null) subAccount = DEFAULT_SUBACCOUNT;

    const Frontend_ID = getIdentity();
    let indexCanister;
    let i,k;
    let vPower; 
    let tokenData = getTokenData(token);
    
    if (tokenData == "Could not find a matching token") {
		console.log(`Error - Cannot find ${token} in known token list`);
		return {}; 
	}
	indexCanister = tokenData.index221B;
    vPower = 1/Math.pow(10, tokenData.decimals); 
    let standard = tokenData.standard;
    
    // ICRC Search
    if (standard.includes("icrc")){
        let jobArray = [];
        let jobArray2 = [];

        // Fetch Data
        let actor = icActor(indexCanister, icrcIndexIDL, Frontend_ID);
        let combinedAC = combinePrincipalSubAccount(ID, subAccount);
        let NFTS;
        // calls
        jobArray[0] = actor.get_full_from_id(combinedAC);
        jobArray[1] = getLinkedTokens(token, combinedAC);
        jobArray[2] = checkFlags(ID, false);
        NFTS = checkNFTs(ID); 
        let jobArrayDone = await processPromises(jobArray);
       
        // catch nothing found
        if(jobArrayDone[0]?.length == 0){
            return "nothing-found";
        }

        // process BigInts
        let blockArray = [];
        let linksArray = [];
        let flags = {
            knownAccount: "",
            userKnownAccount: "",
        };
        let overview;

        // Process Blocks
        let fm, to, milliTime;
        let blockLen = jobArrayDone[0][0]?.blocks?.length ?? 0;
        let maxResults = false;
        if (blockLen == 10000) maxResults = true;
        let mintCount = 0;
        let mintValue = 0;
        for(i=0; i<blockLen; i++){
            fm = parsePrincipalSubAccountString(jobArrayDone[0][0].blocks[i].from_account);
            to = parsePrincipalSubAccountString(jobArrayDone[0][0].blocks[i].to_account);
            milliTime = BigInt(jobArrayDone[0][0].blocks[i].tx_time)/ BigInt(1000000);
            blockArray.push({
            token : token,
            block : Number(jobArrayDone[0][0].blocks[i].block),
            hash : "no-hash",
            type : jobArrayDone[0][0].blocks[i].tx_type,  
            from : fm.principal, 
            to : to.principal, 
            fromSub : fm.subaccount, 
            toSub : to.subaccount, 
            timeMilli : Number(milliTime), 
            value : Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower,
            standard,
            });
            // mint totals
            if (jobArrayDone[0][0].blocks[i].tx_type == "Mint"){
                mintCount += 1;
                mintValue += Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower;
            }
        }

        // Process Links
        let linkLen = jobArrayDone[0][0]?.links?.length ?? 0;
        for(i=0; i<linkLen; i++){
            milliTime = BigInt(jobArrayDone[0][0].links[i].linked_from)/ BigInt(1000000);
            linksArray.push({
                ID: jobArrayDone[0][0].links[i].linked_id, 
                transactions: jobArrayDone[0][0].links[i].number_txs, 
                grossValue: Number(jobArrayDone[0][0].links[i].gross)*vPower, 
                netValue: Number(jobArrayDone[0][0].links[i].net)*vPower,
                linkedFrom: Number(milliTime),
                standard: standard
            })
        }

        // Overview 
        let milliFrom = BigInt(jobArrayDone[0][0].overview.first_active)/ BigInt(1000000);
        let milliLast = BigInt(jobArrayDone[0][0].overview.last_active)/ BigInt(1000000);;
        overview = {
            activeFrom: Number(milliFrom),
            lastActive: Number(milliLast),
            tokenBalance: Number(jobArrayDone[0][0].overview.balance)*vPower,
            tokenSent: Number(jobArrayDone[0][0].overview.sent[1])*vPower,
            tokenReceived: Number(jobArrayDone[0][0].overview.received[1])*vPower,
            numSent: jobArrayDone[0][0].overview.sent[0],
            numReceived: jobArrayDone[0][0].overview.received[0],
            receivedMint: mintCount, // maybe depricate? 
            tokensMinted: mintValue, // maybe depricate?           
        };

        // Flags
        flags.linkedTokens = jobArrayDone[1].linkedTokens;
        flags.nfts = {ID: ID, number: NFTS?.summary?.ownedNfts};
        flags.searched = combinedAC;
        if ( mintCount > 0 ) {
            flags.mintRewards = {text: token+" Minted :", value: mintValue}
        }
        flags.reports = jobArrayDone[2];

        let ls = authStore.read();
        if (ls.data.loggedIn == true) {
            // get unique accounts
            let uniqueACS = [];
            for(let i = 0; i<linkLen; i++){
                uniqueACS.push(jobArrayDone[0][0].links[i].linked_id);
            }
            uniqueACS.push(combinedAC); // add searched account; 
            // Fetch name data for accounts
            jobArray2[0] = getUserNamedAccounts(ls.data.user, uniqueACS);
            jobArray2[1] = getPublicNamedAccounts(uniqueACS);
            let jobArrayDone2 = await processPromises(jobArray2);

            // check if search ac is known to user.
            let userResLen = jobArrayDone2[0][0]?.length ?? 0;
            for(i=0; i<userResLen; i++){
                if (combinedAC == jobArrayDone2[0][0][i][0]) {
                    flags.userKnownAccount = jobArrayDone2[0][0][i][1];
                }
            }

            // check if search ac is known to the global list
            let globalResLen = jobArrayDone2[1][0]?.length ?? 0;
            for(i=0; i<globalResLen; i++){
                if (combinedAC == jobArrayDone2[1][0][i][0]) {
                    flags.knownAccount = jobArrayDone2[1][0][i][1];
                }
            }

            // update links with names 
            let combinedSavedNames = [];
            let usrLen = jobArrayDone2[0][0]?.length ?? 0;
            let gblLen = jobArrayDone2[1][0]?.length ?? 0;
            if (usrLen != 0 && gblLen != 0) {
                combinedSavedNames = [...jobArrayDone2[0][0], ...jobArrayDone2[1][0]];
            } else if (usrLen != 0) {
                combinedSavedNames = jobArrayDone2[0][0];
            } else if (gblLen != 0) {
                combinedSavedNames = jobArrayDone2[1][0];
            }
            let csnLen = combinedSavedNames?.length ?? 0;

            // Add names to Blocks + Links
            if (csnLen != 0){
                // sort names decending
                combinedSavedNames.sort(function(a,b){ return a[0] > b[0] ? 1 : -1; }) 
                // links
                let lk;
                for(i=0; i<linkLen; i++){
                    lk = linksArray[i].ID;
                    for(k=0; k<csnLen; k++){
                        if(combinedSavedNames[k][0] > lk) { break; } // passed it;
                        if(combinedSavedNames[k][0] == lk) {
                            linksArray[i].name = combinedSavedNames[k][1];
                        }
                    }
                }
                // Blocks
                let fm, to;
                for(i=0; i<blockLen; i++){
                    fm = jobArrayDone[0][0].blocks[i].from_account;
                    to = jobArrayDone[0][0].blocks[i].to_account;
                    for(k=0; k<csnLen; k++){
                        // from
                        if(combinedSavedNames[k][0] == fm) {
                            blockArray[i].fromName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            blockArray[i].toName = combinedSavedNames[k][1];
                        }
                    }
                }
            }
        } // end logged in

        blockArray.reverse();
        // Return data 
        let ret = {
            overview: overview,
            tokenTXS: blockArray,
            linkedIdStats: linksArray,
            flags,
            primaryAccount: combinedAC,
            maxResults,
        }
        return ret;
    }
    else if (standard == "icp-og") {
        // Follows ICP Account Model
        let AC;
        if(ID.includes("-")){AC = await getDefaultAccountFromPrincipal(ID)} 
        else {AC = ID}
        let jobArray = [];
        let jobArray2 = [];

        // Fetch Data
        let actor = icActor(indexCanister, icpIndexIDL, Frontend_ID);
        let NFTS;
        // calls
        jobArray[0] = actor.get_full_from_id(AC);
        jobArray[1] = getLinkedTokens(token, AC);
        jobArray[2] = checkFlags(ID, false);
        NFTS = checkNFTs(AC);
        let jobArrayDone = await processPromises(jobArray);

        // catch no-account
        if(jobArrayDone[0]?.length == 0){
            return "nothing-found";
        }

        // process BigInts
        let blockArray = [];
        let linksArray = [];
        let flags = {
            knownAccount: "",
            userKnownAccount: "",
        };
        let overview;

        // Process Blocks
        let milliTime;
        let blockLen = jobArrayDone[0][0]?.blocks?.length ?? 0;
        let maxResults = false;
        if (blockLen == 10000) maxResults = true;
        let mintCount = 0;
        let mintValue = 0;
        for(i=0; i<blockLen; i++){
            milliTime = BigInt(jobArrayDone[0][0].blocks[i].tx_time)/ BigInt(1000000);
            blockArray.push({
            token : token,
            block : Number(jobArrayDone[0][0].blocks[i].block),
            hash : "no-hash",
            type : jobArrayDone[0][0].blocks[i].tx_type,  
            from : jobArrayDone[0][0].blocks[i].from_account, 
            to : jobArrayDone[0][0].blocks[i].to_account, 
            fromSub : undefined, 
            toSub : undefined, 
            timeMilli : Number(milliTime), 
            value : Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower,
            standard
            });
            // total mints
            if (jobArrayDone[0][0].blocks[i].tx_type == "Mint"){
                mintCount += 1;
                mintValue += Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower;
            }
        }

        // Process Links
        let linkLen = jobArrayDone[0][0]?.links?.length ?? 0;
        for(i=0; i<linkLen; i++){
            milliTime = BigInt(jobArrayDone[0][0].links[i].linked_from)/ BigInt(1000000);
            linksArray.push({
                ID: jobArrayDone[0][0].links[i].linked_id, 
                transactions: jobArrayDone[0][0].links[i].number_txs, 
                grossValue: Number(jobArrayDone[0][0].links[i].gross)*vPower, 
                netValue: Number(jobArrayDone[0][0].links[i].net)*vPower,
                linkedFrom: Number(milliTime),
                standard: standard
            })
        }

        // Overview 
        let milliFrom = BigInt(jobArrayDone[0][0].overview.first_active)/ BigInt(1000000);
        let milliLast = BigInt(jobArrayDone[0][0].overview.last_active)/ BigInt(1000000);;
        overview = {
            activeFrom: Number(milliFrom),
            lastActive: Number(milliLast),
            tokenBalance: Number(jobArrayDone[0][0].overview.balance)*vPower,
            tokenSent: Number(jobArrayDone[0][0].overview.sent[1])*vPower,
            tokenReceived: Number(jobArrayDone[0][0].overview.received[1])*vPower,
            numSent: jobArrayDone[0][0].overview.sent[0],
            numReceived: jobArrayDone[0][0].overview.received[0],
            receivedMint: mintCount, 
            tokensMinted: mintValue,         
        };

        // Flags
        flags.linkedTokens = jobArrayDone[1].linkedTokens;
        flags.nfts = {ID: AC, number: NFTS?.summary?.ownedNfts};
        flags.searched = AC;
        if ( mintCount > 0 ) {
            flags.mintRewards = {text: token+" Minted :", value: mintValue}
        }
        flags.reports = jobArrayDone[2];

        let ls = authStore.read();
        if (ls.data.loggedIn == true) {
            // get unique accounts
            let uniqueACS = [];
            for(let i = 0; i<linkLen; i++){
                uniqueACS.push(jobArrayDone[0][0].links[i].linked_id);
            }
            uniqueACS.push(AC); // add searched account; 
            // Fetch name data for accounts
            jobArray2[0] = getUserNamedAccounts(ls.data.user, uniqueACS);
            jobArray2[1] = getPublicNamedAccounts(uniqueACS);
            let jobArrayDone2 = await processPromises(jobArray2);

            // check if search ac is known to user.
            let userResLen = jobArrayDone2[0][0]?.length ?? 0;

            for(i=0; i<userResLen; i++){
                if (AC == jobArrayDone2[0][0][i][0]) {
                    flags.userKnownAccount = jobArrayDone2[0][0][i][1];
                }
            }

            // check if search ac is known to the global list
            let globalResLen = jobArrayDone2[1][0]?.length ?? 0;
            for(i=0; i<globalResLen; i++){
                if (AC == jobArrayDone2[1][0][i][0]) {
                    flags.knownAccount = jobArrayDone2[1][0][i][1];
                }
            }

            // update links with names 
            let combinedSavedNames = [];
            let usrLen = jobArrayDone2[0][0]?.length ?? 0;
            let gblLen = jobArrayDone2[1][0]?.length ?? 0;
            if (usrLen != 0 && gblLen != 0) {
                combinedSavedNames = [...jobArrayDone2[0][0], ...jobArrayDone2[1][0]];
            } else if (usrLen != 0) {
                combinedSavedNames = jobArrayDone2[0][0];
            } else if (gblLen != 0) {
                combinedSavedNames = jobArrayDone2[1][0];
            }
            let csnLen = combinedSavedNames?.length ?? 0;

            // Add names to Blocks + Links
            if (csnLen != 0){
                // sort names decending
                combinedSavedNames.sort(function(a,b){ return a[0] > b[0] ? 1 : -1; }) 
                // links
                let lk;
                for(i=0; i<linkLen; i++){
                    lk = linksArray[i].ID;
                    for(k=0; k<csnLen; k++){
                        if(combinedSavedNames[k][0] > lk) { break; } // passed it;
                        if(combinedSavedNames[k][0] == lk) {
                            linksArray[i].name = combinedSavedNames[k][1];
                        }
                    }
                }
                // Blocks
                let fm, to;
                for(i=0; i<blockLen; i++){
                    fm = jobArrayDone[0][0].blocks[i].from_account;
                    to = jobArrayDone[0][0].blocks[i].to_account;
                    for(k=0; k<csnLen; k++){
                        // from
                        if(combinedSavedNames[k][0] == fm) {
                            blockArray[i].fromName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            blockArray[i].toName = combinedSavedNames[k][1];
                        }
                    }
                }
            }
        } // end logged in
        
        blockArray.reverse();
        // Return data 
        let ret = {
            overview: overview,
            tokenTXS: blockArray,
            linkedIdStats: linksArray,
            flags,
            primaryAccount: AC,
            maxResults,
        }
        return ret;
    }
}

// process account data (single token)
function processAccountTXS(target, data, token) {
	let dataLen = data?.length ?? 0;
	let i;
	let shortID, shortSA;
	let longID;
	let longSubID;
	let targetSub, targetName;
	let tgt;
	let direction;
	let OP = [];
	let counter = 1;
	let t1, tDate, tTime;
	let is_icrc = target.includes('.');
	let principal = undefined;
	let subaccount = undefined;
	if (is_icrc) {
		let parse = parsePrincipalSubAccountString(target);
		principal = parse.principal;
		subaccount = parse.subaccount;
	}

	for (i = 0; i < dataLen; i++) {

        // Note Target is the searched account (in == payment to target, out == payment from target)
        // Note shortID, longID and longSubID is the other side of the transaction.

        // set direction
        if (data[i].standard == "icp-og"){
			if (data[i].to == target) direction = 'in';
			else direction = 'out';
        }
        else if (data[i].standard.includes("icrc")){
			if (data[i].to == principal && data[i].toSub == subaccount) direction = 'in';
			else direction = 'out';
        }

        // set target and short/ long account data
        if (data[i].standard == "icp-og"){
            if (data[i].type == 'burn' || data[i].type == 'Burn') {
                shortID = 'Burn';
                longID = `${data[i].token} Burn Account`;
                longSubID = undefined;
                tgt = data[i].from;
                targetSub = undefined;
                targetName = data[i]?.fromName ?? undefined;

            } else if (data[i].type == 'mint' || data[i].type == 'Mint') {
                shortID = 'Mint';
                longID = `${data[i].token} Minting Account`;
                longSubID = undefined;
                tgt = data[i].to;
                targetSub = undefined;
                targetName = data[i]?.toName ?? undefined;
            }
            else if (data[i].type == 'approve' || data[i].type == 'Approve') {
                shortID = 'Approve (no tx)';
                longID = 'Approve (no tx)';
                longSubID = undefined;
                tgt = data[i].from; // this IS the from account
                targetSub = undefined;
                targetName = data[i]?.fromName ?? undefined;
            } else {
                // TX is a transfer
                if (direction == 'in') {
                    shortID = data[i]?.fromName ?? shortenString(data[i].from);
                    shortSA = undefined;
                    longID = data[i].from;
                    longSubID = undefined;
                    tgt = data[i].to;
                    targetSub = data[i].toSub;
                    targetName = data[i]?.toName ?? undefined;
                }
                if (direction == 'out') {
                    shortID = data[i]?.toName ?? shortenString(data[i].to);
                    shortSA = undefined;
                    longID = data[i].to;
                    longSubID = undefined;
                    tgt = data[i].from;
                    targetSub = undefined;
                    targetName = data[i]?.fromName ?? undefined;
                }
            }
        }
        else if (data[i].standard.includes("icrc")){
           	// shortAC 
            if (data[i].type == 'burn' || data[i].type == 'Burn') {
                shortID = 'Burn';
                longID = `${data[i].token} Burn Account`;
                longSubID = undefined;
                tgt = data[i].from;
                targetSub = data[i].fromSub;
                targetName = data[i]?.fromName ?? undefined;
            } else if (data[i].type == 'mint' || data[i].type == 'Mint') {
                shortID = 'Mint';
                longID = `${data[i].token} Minting Account`;
                longSubID = 'Minting Account';
                tgt = data[i].to;
                targetSub = data[i].toSub;
                targetName = data[i]?.toName ?? undefined;
            }
            else if (data[i].type == 'approve' || data[i].type == 'Approve') {
                shortID = 'Approve';
                longID = `${data[i].token} Ledger`;
                longSubID = undefined;
                tgt = data[i].from;
                targetSub = data[i].fromSub;
                targetName = data[i]?.fromName ?? undefined;
            } else {
                if (direction == 'in') {
                    shortID = data[i]?.fromName ?? shortenString(data[i].from);
                    shortSA = (data[i]?.fromName) ? undefined : shortenString(data[i].fromSub);
                    longID = data[i].from;
                    longSubID = data[i].fromSub;
                    tgt = data[i].to;
                    targetSub = data[i].toSub;
                    targetName = data[i]?.toName ?? undefined;
                }
                if (direction == 'out') {
                    shortID = data[i]?.toName ?? shortenString(data[i].to);
                    shortSA = (data[i]?.toName) ? undefined : shortenString(data[i].toSub);
                    longID = data[i].to;
                    longSubID = data[i].toSub;
                    tgt = data[i].from;
                    targetSub = data[i].fromSub;
                    targetName = data[i]?.fromName ?? undefined;
                }
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
			target: tgt,
			targetSub: targetSub,
			targetName: targetName,
			token: token,
            standard: data[i].standard,
            type: data[i].type
		};
		counter++;
	}
	OP.reverse();
	return OP;
}

// for combined search (multi token)
function basicAccountTableCombinedTX(target, data, target2) {
	// NOTE target2 is used for Combined search where TXS are coming from ICRC + ICP ledgers. 
	// target2 will be the ICP account if one is found.

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
	let target_icrc = target.includes('.');
	let principal = '';
	let subaccount = '';
	let is_icrc = false;
	if (target_icrc) {
		let parse = parsePrincipalSubAccountString(target);
		principal = parse.principal;
		subaccount = parse.subaccount;
	}

	// SORT DATA BY TIME
	data.sort((a, b) => a.timeMilli - b.timeMilli);

	for (i = 0; i < dataLen; i++) {

		// is ICRC
		is_icrc = data[i].to.includes('-');

		//direction
		if (data[i].to == target || data[i].to == target2 || (data[i].to == principal && data[i].toSub == subaccount) ) direction = 'in';
		else direction = 'out';
		

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
		}
		else if (data[i].type == 'approve' || data[i].type == 'Approve') {
			shortID = 'Approve';
			longID = 'ICRC Ledger';
			longSubID = '';
			subName = 'undefined';
			tgt = subHit ? data[i].from : target;
			targetSub = data[i].fromSub;
			targetName = data[i]?.fromName ?? undefined;
			targetSubName = data[i]?.fromSubName ?? undefined;
		} else {
			if (direction == 'in' && is_icrc == true) {
				shortID = data[i]?.fromName ?? shortenString(data[i].from);
				longID = data[i].from;
				longSubID = data[i].fromSub;
				subName = undefined;
				shortSA = (data[i].fromSub != DEFAULT_SUBACCOUNT) ? shortenString(data[i].fromSub) : undefined;
				tgt = data[i].to;
				targetSub = data[i].toSub;
				targetName = data[i]?.toName ?? undefined;
				targetSubName = data[i]?.toSubName ?? undefined;
			}
			if (direction == 'in' && is_icrc == false) {
				shortID = data[i]?.fromName ?? shortenString(data[i].from);
				longID = data[i].from;
				longSubID = undefined;
				subName = undefined;
				shortSA = undefined;
				tgt = data[i].to;
				targetSub = undefined;
				targetName = data[i]?.toName ?? undefined;
				targetSubName = undefined;
			}
			if (direction == 'out' && is_icrc == true) {
				shortID = data[i]?.toName ?? shortenString(data[i].to);
				longID = data[i].to;
				longSubID = data[i].toSub;
				subName = data[i]?.toSubName ?? undefined;
				shortSA = subName ?? shortenString(data[i].toSub);
				tgt = data[i].from;
				targetSub = data[i].fromSub;
				targetName = data[i]?.fromName ?? undefined;
				targetSubName = data[i]?.fromSubName ?? undefined;
			}
			if (direction == 'out' && is_icrc == false) {
				shortID = data[i]?.toName ?? shortenString(data[i].to);
				longID = data[i].to;
				longSubID = undefined;
				subName = undefined;
				shortSA = undefined;
				tgt = data[i].from;
				targetSub = undefined;
				targetName = data[i]?.fromName ?? undefined;
				targetSubName = undefined;
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
			token: data[i].token
		};
		counter++;
	}

	OP.reverse();
	return OP;
}

// get specific blocks
async function getBlockData(token, start, end){
    const Frontend_ID = getIdentity();
    let known = false;
    let indexCanister;
    let i;
    let vPower;
    let txRequired = end - start;
    if (txRequired > MAX_BLOCK_REQUEST) {return {}};

    let tokenData = getTokenData(token);
    if (tokenData == "Could not find a matching token") {
		console.log(`Error - Cannot find ${token} in known token list`);
		return {}; 
	}

    indexCanister = tokenData.index221B;
    vPower = 1/Math.pow(10, tokenData.decimals); 
    let standard = tokenData.standard;

    // Create Request Arg
    let req = [];
    let c = start;
    for(i=0; i<=txRequired; i++){
        req.push(c);
        c += 1;
    }

    const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
    
    // ICRC
    if(standard.includes("icrc")){
        // Fetch Data
        let actor = icActor(indexCanister, icrcIndexIDL, Frontend_ID);
        let data = await actor.get_multiple_tx(req);
        let dataLen = data.length ?? 0;
        let res = [];
        let fm, to;
        let v;
        let milliTime;
        for(i=0; i<dataLen; i++){
            fm = parsePrincipalSubAccountString(data[i].from_account);
            to = parsePrincipalSubAccountString(data[i].to_account);
            v = Number(data[i].tx_value)*vPower;
            v = v.toLocaleString('en-US', fmtOptionsValue);
            milliTime = BigInt(data[i].tx_time)/ BigInt(1000000);
            res.push({
                token : token,
                block: Number(data[i].block),
                fromPrincipal: fm.principal,
                fromAccount: fm.subaccount,
                toPrincipal: to.principal,
                toAccount: to.subaccount,
                fromName: undefined,
                toName: undefined,
                hash: "No-Hash",
                value: v,
                type: data[i].tx_type,
                timeMilli: Number(milliTime),
                standard
            });
        }

        // logged in 
        let ls = authStore.read();
        if (ls.data.loggedIn == true) {
            // get unique accounts
            let uniqueACS = [];
            let jobArray = []
            for(let i = 0; i<dataLen; i++){
                uniqueACS.push(data[i].from_account);
                uniqueACS.push(data[i].to_account);
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
                for(i=0; i<dataLen; i++){
                    fm = data[i].from_account;
                    to = data[i].to_account;
                    for(k=0; k<csnLen; k++){
                        // from
                        if(combinedSavedNames[k][0] == fm) {
                            res[i].fromName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            res[i].toName = combinedSavedNames[k][1];
                        }
                    }
                }
            }
        }

        let returnData = {
            overview: "",
            blocks: res
        };
        return returnData;
    } 
    
    else if (standard == "icp-og") {
        // Fetch Data
        let actor = icActor(indexCanister, icpIndexIDL, Frontend_ID);
        let data = await actor.get_multiple_tx(req);
        let dataLen = data.length ?? 0;
        let res = [];
        let milliTime;
        let v;
        for(i=0; i<dataLen; i++){
            milliTime = BigInt(data[i].tx_time)/ BigInt(1000000);
            v = Number(data[i].tx_value)*vPower;
            v = v.toLocaleString('en-US', fmtOptionsValue);
            res.push({
                token : token,
                block: Number(data[i].block),
                fromPrincipal: undefined,
                fromAccount: data[i].from_account,
                toPrincipal: undefined,
                toAccount: data[i].to_account,
                fromName: undefined,
                toName: undefined,
                hash: "No-Hash",
                value: v,
                type: data[i].tx_type,
                timeMilli: Number(milliTime),
                standard
            });
        }

          // logged in 
          let ls = authStore.read();
          if (ls.data.loggedIn == true) {
              // get unique accounts
              let uniqueACS = [];
              let jobArray = []
              for(let i = 0; i<dataLen; i++){
                  uniqueACS.push(data[i].from_account);
                  uniqueACS.push(data[i].to_account);
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
                  for(i=0; i<dataLen; i++){
                      fm = data[i].from_account;
                      to = data[i].to_account;
                      for(k=0; k<csnLen; k++){
                          // from
                          if(combinedSavedNames[k][0] == fm) {
                              res[i].fromName = combinedSavedNames[k][1];
                          }
                          // to
                          if(combinedSavedNames[k][0] == to) {
                              res[i].toName = combinedSavedNames[k][1];
                          }
                      }
                  }
              }
          }

        let returnData = {
            overview: "",
            blocks: res
        };
        return returnData;
    }
}

// get latest blocks
async function getLatestBlockData(length, token){
    const Frontend_ID = getIdentity();
    let indexCanister;
    let i;
    let vPower;
    let tokenData = getTokenData(token);
    let standard;

    if (tokenData == "Could not find a matching token") {
		console.log(`Error - Cannot find ${token} in known token list`);
		return {}; 
	}

    indexCanister = tokenData.index221B;
    vPower = 1/Math.pow(10, tokenData.decimals);
    standard = tokenData.standard;

    const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };

    // ICRC
    if (standard.includes("icrc")) {
        let actor = icActor(indexCanister, icrcIndexIDL, Frontend_ID);
        let data = await actor.get_latest_transactions(length);
        let dataLen = data.length ?? 0;
        let res = [];
        let fm, to;
        let milliTime;
        let v;
        for(i=0; i<dataLen; i++){
            fm = parsePrincipalSubAccountString(data[i].from_account);
            to = parsePrincipalSubAccountString(data[i].to_account);
            milliTime = BigInt(data[i].tx_time)/ BigInt(1000000);
            v = Number(data[i].tx_value)*vPower;
            v = v.toLocaleString('en-US', fmtOptionsValue);
            res.push({
                token : token,
                block: Number(data[i].block),
                fromPrincipal: fm.principal,
                fromAccount: fm.subaccount,
                toPrincipal: to.principal,
                toAccount: to.subaccount,
                hash: "No-Hash",
                value: v,
                type: data[i].tx_type,
                timeMilli: Number(milliTime),
                standard
            });
        }
        
        // logged in 
        let ls = authStore.read();
        if (ls.data.loggedIn == true) {
            // get unique accounts
            let uniqueACS = [];
            let jobArray = []
            for(let i = 0; i<dataLen; i++){
                uniqueACS.push(data[i].from_account);
                uniqueACS.push(data[i].to_account);
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
                for(i=0; i<dataLen; i++){
                    fm = data[i].from_account;
                    to = data[i].to_account;
                    for(k=0; k<csnLen; k++){
                        // from
                        if(combinedSavedNames[k][0] == fm) {
                            res[i].fromName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            res[i].toName = combinedSavedNames[k][1];
                        }
                    }
                }
            }
        }
        
        let returnData = {
            overview: "",
            blocks: res.reverse()
        };
        return returnData;
    } 
    else if ( standard = "icp-og" ){
           // ICP Account Standard
           let actor = icActor(indexCanister, icpIndexIDL, Frontend_ID);
           let data = await actor.get_latest_transactions(length);
           let dataLen = data.length ?? 0;
           let res = [];
           let milliTime;
           let v;
           for(i=0; i<dataLen; i++){
               milliTime = BigInt(data[i].tx_time)/ BigInt(1000000);
               v = Number(data[i].tx_value)*vPower;
               v = v.toLocaleString('en-US', fmtOptionsValue);
               res.push({
                   token : token,
                   block: Number(data[i].block),
                   fromPrincipal: undefined,
                   fromAccount: data[i].from_account,
                   toPrincipal: undefined,
                   toAccount: data[i].to_account,
                   hash: "No-Hash",
                   value: v,
                   type: data[i].tx_type,
                   timeMilli: Number(milliTime),
                   standard
               });
           }

            // logged in 
            let ls = authStore.read();
            if (ls.data.loggedIn == true) {
                // get unique accounts
                let uniqueACS = [];
                let jobArray = []
                for(let i = 0; i<dataLen; i++){
                    uniqueACS.push(data[i].from_account);
                    uniqueACS.push(data[i].to_account);
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
                    for(i=0; i<dataLen; i++){
                        fm = data[i].from_account;
                        to = data[i].to_account;
                        for(k=0; k<csnLen; k++){
                            // from
                            if(combinedSavedNames[k][0] == fm) {
                                res[i].fromName = combinedSavedNames[k][1];
                            }
                            // to
                            if(combinedSavedNames[k][0] == to) {
                                res[i].toName = combinedSavedNames[k][1];
                            }
                        }
                    }
                }
            }
        
        let returnData = {
            overview: "",
            blocks: res.reverse()
        };
        return returnData;
    }
}

function processBlockTXS(data) {
	let dataLen = data?.length ?? 0;
	let i;
	let fromShortID, toShortID;
	let fromShortSA, toShortSA;
	let usePrincipal = true;
	let OP = [];

    for (i = 0; i < dataLen; i++) {

        // ICP-OG Standard
        if(data[i].standard == "icp-og"){
            // shortAC
			if (data[i].type == 'Burn') {
				toShortID = `${data[i].token} Burn Account`;
				fromShortID = shortenString(data[i].fromAccount);
			} else if (data[i].type == 'Mint') {
				fromShortID = `${data[i].token} Mint Account`;
                toShortID = shortenString(data[i].toAccount);
			} 
			else if (data[i].type == 'Approve') {
				toShortID = 'Approve Only';
				fromShortID = shortenString(data[i].fromAccount);
			} else {
                fromShortID = data[i]?.fromName
                    ? data[i].fromName
                    : shortenString(data[i].fromAccount);
                toShortID = data[i]?.toName
                    ? data[i].toName
                    : shortenString(data[i].toAccount);
			}

			// Time
			let t1, tDate, tTime;
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
				fromShortSA: undefined,
				toShortSA: undefined,
				toPrincipal: undefined,
				toName: data[i]?.toName ? data[i].toName : undefined,
				toAccount: data[i].toAccount,
				fromPrincipal: data[i].fromPrincipal,
				fromName: data[i]?.fromName ? data[i].fromName : undefined,
				fromAccount: data[i].fromAccount,
				value: data[i].value,
				hash: data[i].hash,
				token: data[i].token,
				type: data[i].type,
                standard: data[i].standard
			};
        }

        // ICRC STANDARD
        else if(data[i].standard.includes("icrc")){
            // shortAC
            if (data[i].type == 'Burn') {
                toShortID = `${data[i].token} Burn Account`;
                toShortSA = undefined;
                fromShortID = shortenString(data[i].fromPrincipal);
                fromShortSA = undefined;
            } else if (data[i].type == 'Mint') {
                fromShortID = 'Mint';
                fromShortSA = undefined;
                toShortID = shortenString(data[i].toPrincipal);
                toShortSA = undefined;
            } 
            else if (data[i].type == 'Approve') {
                toShortID = 'Approve Only';
                toShortSA = undefined;
                fromShortID = data[i]?.fromName
                ? data[i].fromName : shortenString(data[i].fromPrincipal);
                fromShortSA = (data[i].fromAccount == DEFAULT_SUBACCOUNT || data[i]?.fromName) 
                ? undefined : shortenString(data[i].fromAccount);
            } else {
                // Regular Transaction
                fromShortID = data[i]?.fromName
                    ? data[i].fromName : shortenString(data[i].fromPrincipal);
                fromShortSA = (data[i].fromAccount == DEFAULT_SUBACCOUNT || data[i]?.fromName) 
                    ? undefined : shortenString(data[i].fromAccount);
                toShortID = data[i]?.toName
                    ? data[i].toName : shortenString(data[i].toPrincipal);
                toShortSA = (data[i].toAccount == DEFAULT_SUBACCOUNT || data[i]?.toName) 
                    ? undefined : shortenString(data[i].toAccount);
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
                toName: data[i]?.toName,
                toAccount: data[i].toAccount,
                fromPrincipal: data[i].fromPrincipal,
                fromName: data[i]?.fromName,
                fromAccount: data[i].fromAccount,
                value: data[i].value,
                hash: data[i].hash,
                token: data[i].token,
                type: data[i].type,
                standard: data[i].standard
            };
        }
    }

	OP.reverse();
	let returnData = {
		blocks: OP
	};
	return returnData;
}

async function getDefaultAccountFromPrincipal(principal){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.get_single_account(principal, 0);
    return res; 
}

// for getData search ()
async function getLinkedTokens(searchedToken, ID, icpLinkedPrincipal){
    if (searchedToken == "ICP") { searchedToken = "~"} // overide not self searching for ICP
    let TKNS = getAllTokenData();
    let tLen = TKNS.length ?? 0;
    const Frontend_ID = getIdentity();
    let i;
    let icrcObj = {};
    let icpLinkedAccounts = [];
    // loop over tokens
    for(i=0; i<tLen; i++){
        if(TKNS[i].ticker != searchedToken){
            if(TKNS[i].ticker == "ICP") {
                // check if principal or ac
                if(ID.includes("-") && ID.includes(".")){
                    let PS = parsePrincipalSubAccountString(ID);
                    let P = PS.principal;
                    let S = PS.subaccount;
                    if(S == DEFAULT_SUBACCOUNT ){
                        // only get ICP Account if search account is the default principal id (Not a subaccount)
                        let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
                        let res = await actor.get_single_account(P, 0);
                        let icpActor = icActor(TKNS[i].index221B, icpIndexIDL, Frontend_ID);
                        let res2 = await icpActor.get_overview_by_id(res);
                        let resLen = res2?.length ?? 0;
                        if (resLen != 0 ){
                            icrcObj[TKNS[i].ticker] = true;
                        }
                    }
                }
            } else {
                // ICRC Search
                let actor = icActor(TKNS[i].index221B, icrcIndexIDL, Frontend_ID);
                let res = await actor.get_overview_by_id(ID); 
                let resLen = res?.length ?? 0;
                if (resLen != 0 ){
                    icrcObj[TKNS[i].ticker] = true;
                }
            }
        }
    }

    let combined = {
        searched: ID,
        linkedPrincipal: "",
        ICRC: icrcObj,
        ICP: icpLinkedAccounts,
    };
    return {linkedTokens: combined};
} 

// For linkTokenSearch.svelte
async function getAllLinkedTokens(ID, sub, returnData){
    const Frontend_ID = getIdentity();
    let allTKNS = getAllTokenData();
    let tLen = allTKNS.length ?? 0;
    let i;
    let icrcObj = {};
    let retData = [];
    let account; 
    if (sub == undefined || sub == "" || sub == null) sub = DEFAULT_SUBACCOUNT;
    let icrcID = combinePrincipalSubAccount(ID, sub);
    if(ID.includes("-")){
        // search by principal
        if(sub == DEFAULT_SUBACCOUNT){
            // only do ICP account if sub account is default!
            account = await getDefaultAccountFromPrincipal(ID);
            let actor = icActor(allTKNS[0].index221B, icpIndexIDL, Frontend_ID);
            let res = await actor.get_overview_by_id(account); 
            let resLen = res?.length ?? 0;
            if (resLen != 0 ){
                if(returnData == true) retData.push({token: "ICP" , data: res});
                icrcObj["ICP"] = true;
            }
        }
        for(i=0; i<tLen; i++){
            // ICP Done search for ICRC tokens
            if(allTKNS[i].standard.includes("icrc")){
                let actor = icActor(allTKNS[i].index221B, icrcIndexIDL, Frontend_ID);
                let res = await actor.get_overview_by_id(icrcID); 
                let resLen = res?.length ?? 0;
                if (resLen != 0 ){
                    if(returnData == true) retData.push({token: allTKNS[i].ticker , data: res});
                    icrcObj[allTKNS[i].ticker] = true;
                }
            }
        }
    } else {
        // account only search
        account = ID;
        let actor = icActor(allTKNS[0].index221B, icpIndexIDL, Frontend_ID);
        let res = await actor.get_overview_by_id(ID); 
        let resLen = res?.length ?? 0;
        if (resLen != 0 ){
            if(returnData == true) retData.push({token: "ICP" , data: res});
            icrcObj["ICP"] = true;
        }
        // check for known principal (TODO!)
        // search against principal
    }
    // same format as getLinkedTokens to keep compatability with A/C Search componant.
    let combined = {
        searched: account,// any linked ICP account
        linkedPrincipal: icrcID, // ICRC account (combined format)
        ICRC: icrcObj, // all results here !!!
        ICP: "", // ignore this ***
    };

    if(returnData == true){
        return retData; // for api/screener
    } else {
        return {linkedTokens: combined}; // for linked token search (index.html)
    }
}

// For Combined Search (Members)
async function searchAllLinkedTokens(ID, sub){
    let tLen = canister_ids.length ?? 0;
    let i;
    let combinedCalls = [];
    let combinedCallsRes = [];
    let icrcObj = {};
    let retData = [];
    let account; 


    for(i=0; i<tLen; i++){ // index 0 already done.
        combinedCalls[i] = getData(canister_ids[i].token, ID, sub);
    }
    combinedCallsRes = await processPromises(combinedCalls);

    let ccResLen = combinedCallsRes.length ?? 0;
    let returnAR = [];
    for(i=0; i<ccResLen; i++){
        if (combinedCallsRes[i] != "nothing-found"){
            returnAR.push(
                {
                    token: canister_ids[i].token,
                    tokenData: combinedCallsRes[i]
                }
            );
        }
    }

    return returnAR;
}  

async function checkNFTs(ID){
    let nftData;
    let R;
    if(ID.includes('-', 0)){
        let url = "https://api.nftgeek.app/api/1/principal/"+ID+"/summary";
        let settings = { method: "Get" };
        try {
            nftData = await fetch(url, settings);
        } catch (error) {
            console.log("error fetching NFT data : ", error)
        }
        try {
            R = await nftData.json();
        } catch (error) {
            console.log("Error parsing NFT JSON : ", error);
        }
        return R;
    }
    else{
        let url = "https://api.nftgeek.app/api/1/accountIdentifier/"+ID+"/summary";
        let settings = { method: "Get" };
        try {
            nftData = await fetch(url, settings);
        } catch (error) {
            console.log("error fetching NFT data : ", error)
        }
        try {
            R = await nftData.json();
        } catch (error) {
            console.log("Error parsing NFT JSON : ", error);
        }
        return R;
    }
}  

async function getUserNamedAccounts(owner, checkAR){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = [];
    res = await actor.get_user_named_accounts(owner, checkAR); 
    return res;
}

async function getPublicNamedAccounts(checkAR){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = [];
    res = await actor.get_public_named_accounts(checkAR); 
    return res;
}

async function checkFlags(ID, fullResponse){
    const Frontend_ID = getIdentity();
    let actor = icActor(flagCanister, flagCanisterIDL, Frontend_ID);
    let flags = await actor.get_flags(ID); 

    let res = [];
    let flagLen = flags[0]?.length ?? 0;
	let k;
	for (let i = 0; i<flagLen; i++) {
        k = Object.keys(flags[0][i]);

        if(fullResponse == true){
            if(k[0] == "FraudFlag"){ 
                res.push({ FraudFlag: {
                    id: flags[0][i].FraudFlag.id,
                    flag_from: Number(flags[0][i].FraudFlag.flag_from),
                    link: flags[0][i].FraudFlag.link,
                    time_added: Number(flags[0][i].FraudFlag.time_added),
                    flagged_by: flags[0][i].FraudFlag.flagged_by
                }});
            }
            if(k[0] == "MixerFlag"){ 
                res.push({ MixerFlag: {
                    id: flags[0][i].MixerFlag.id,
                    flag_from: Number(flags[0][i].MixerFlag.flag_from),
                    text: flags[0][i].MixerFlag.text,
                    level: Number(flags[0][i].MixerFlag.level),
                    time_added: Number(flags[0][i].MixerFlag.time_added)
                }});
            }
            if(k[0] == "GenesisFlag"){ 
                res.push({ GenesisFlag: {
                    id: flags[0][i].GenesisFlag.id,
                    flag_from: Number(flags[0][i].GenesisFlag.flag_from),
                    text: flags[0][i].GenesisFlag.text,
                    time_added: Number(flags[0][i].GenesisFlag.time_added)
                }});
            }
            if(k[0] == "SARFlag"){ 
                res.push({ SARFlag: {
                    id: flags[0][i].SARFlag.id,
                    flag_from: Number(flags[0][i].SARFlag.flag_from),
                    text: flags[0][i].SARFlag.text,
                    time_added: Number(flags[0][i].SARFlag.time_added),
                    link: flags[0][i].SARFlag.link, 
                    flagged_by: flags[0][i].SARFlag.flagged_by 
                }});
            }
            if(k[0] == "CommunityFlag"){ 
                res.push({ CommunityFlag: {
                    id: flags[0][i].CommunityFlag.id,
                    flag_from: Number(flags[0][i].CommunityFlag.flag_from),
                    text: flags[0][i].CommunityFlag.text,
                    time_added: Number(flags[0][i].CommunityFlag.time_added),
                    link: flags[0][i].CommunityFlag.link, 
                    number_of_flags: Number(flags[0][i].CommunityFlag.number_of_flags)
                }});
            }
        } else {
            res.push(k[0]);
        }
	}
    return res;
}

async function submitFraudReport(t1, t2, t3, t4){
    const Frontend_ID = getIdentity();
    let actor = icActor(flagCanister, flagCanisterIDL, Frontend_ID);
    let res = await actor.add_fraud_report(t1,t2,t3,t4); 
    return res;
}

function linkedIDTable(data, token) {

	let dataLen = data?.length ?? 0;
	let i;
	let parseGross;
	let parseNet;
	let counter = 1;
	let split, shortID;

	for (i = 0; i < dataLen; i++) {
		if (data[i].standard.includes('icrc')) {
			split = parsePrincipalSubAccountString(data[i].ID);
			data[i].shortID = shortenString(split.principal);
			data[i].shortSA =
				split.subaccount == DEFAULT_SUBACCOUNT
					? undefined
					: shortenString(split.subaccount);
			data[i].splitID = split.principal;
			data[i].splitSA = split.subaccount;
			data[i].grossValue = parseFloat(data[i].grossValue);
			data[i].netValue = parseFloat(data[i].netValue);
		} 
        else if (data[i].standard == "icp-og") {
			shortID = shortenString(data[i].ID);
			parseGross = parseFloat(data[i].grossValue);
			parseNet = parseFloat(data[i].netValue);
			data[i].shortID = shortID;
			data[i].shortAC = undefined;
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

function toFixedValue(num, precision) {
	return Number.parseFloat(num.toFixed(precision));
}


export {
	getData,
    getLinkedTokens,
    getAllLinkedTokens,
    checkNFTs,
    getBlockData,
    getLatestBlockData,
    getUserNamedAccounts,
    getPublicNamedAccounts,
    checkFlags,
    submitFraudReport,
    searchAllLinkedTokens,
    processAccountTXS,
    processBlockTXS,
    linkedIDTable
};

