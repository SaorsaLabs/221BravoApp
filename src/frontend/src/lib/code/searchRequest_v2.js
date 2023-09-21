import { canister_ids, backendCanisterID, DEFAULT_SUBACCOUNT, MAX_BLOCK_REQUEST, flagCanister } from './constants.js';
import { getIdentity, icActor } from './icAgent.js';
import { icrcIndexIDL } from './IDL/icrcIndex.js';
import { backendCanisterIDL } from './IDL/backend.js';
import { flagCanisterIDL } from './IDL/flagsCanister.js';
import { icpIndexIDL } from './IDL/icpIndex.js';
import { combinePrincipalSubAccount, processPromises, parsePrincipalSubAccountString, getUniqueValues } from './utils.js';
import { authStore } from '../stores/authStore.js';

async function getData(token, ID, subAccount) { // , min, max, start, end
    // destructure if needed
    if (ID.includes('.', 0)){
        let ds = parsePrincipalSubAccountString(ID);
        ID = ds.principal;
        subAccount = ds.subaccount;
    } 
    // default sub account if needed
    else if (subAccount == "" || subAccount == null) subAccount = DEFAULT_SUBACCOUNT;

    const Frontend_ID = getIdentity();
    let known = false;
    let indexCanister;
    let i,k;
    let vPower; 
    // get canister ID
    for (i = 0; i < canister_ids.length; i++) {
		if (token == canister_ids[i].token) {
			indexCanister = canister_ids[i].index;
            vPower = 1/Math.pow(10, canister_ids[i].decimals); 
			known = true;
		}
	}

	if (known == false) {
		console.log(`Error - Cannot find ${token} in known token Canisters list`);
		return {}; 
	}

    // ICRC Search
    if (token != "ICP"){
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
            value : Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower
            });
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
                linkedFrom: Number(milliTime)
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
            primaryAccount: combinedAC,
            maxResults,
        }
        return ret;
    }else{
        // ICP Search
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
        let fm, to, milliTime;
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
            fromSub : "", 
            toSub : "", 
            timeMilli : Number(milliTime), 
            value : Number(jobArrayDone[0][0].blocks[i].tx_value)*vPower
            });
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
                linkedFrom: Number(milliTime)
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

// get specific blocks
async function getBlockData(token, start, end){
    const Frontend_ID = getIdentity();
    let known = false;
    let indexCanister;
    let i;
    let vPower;
    let decimals;
    let txRequired = end - start;
    if (txRequired > MAX_BLOCK_REQUEST) {return {}};
    // get canister ID
    for (i = 0; i < canister_ids.length; i++) {
		if (token == canister_ids[i].token) {
			indexCanister = canister_ids[i].index;
            decimals = canister_ids[i].decimals;
            vPower = 1/Math.pow(10, decimals); 
			known = true;
		}
	} 
    if (known == false) {
		console.log(`Error - Cannot find ${token} in known token Canisters list`);
		return {}; 
	}

    // Create Request Arg
    let req = [];
    let c = start;
    for(i=0; i<=txRequired; i++){
        req.push(c);
        c += 1;
    }

    const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };
    
    // ICRC
    if(token != "ICP"){
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
                fromPrincipalName: null,
                toPrincipalName: null,
                hash: "No-Hash",
                value: v,
                type: data[i].tx_type,
                timeMilli: Number(milliTime)
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
                            res[i].fromPrincipalName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            res[i].toPrincipalName = combinedSavedNames[k][1];
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
    } else {
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
                fromPrincipal: "",
                fromAccount: data[i].from_account,
                toPrincipal: "",
                toAccount: data[i].to_account,
                fromAccountName: undefined,
                toAccountName: undefined,
                hash: "No-Hash",
                value: v,
                type: data[i].tx_type,
                timeMilli: Number(milliTime)
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
                              res[i].fromAccountName = combinedSavedNames[k][1];
                          }
                          // to
                          if(combinedSavedNames[k][0] == to) {
                              res[i].toAccountName = combinedSavedNames[k][1];
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
    let known = false;
    let indexCanister;
    let i;
    let vPower;
    let decimals;
    // get canister ID
    for (i = 0; i < canister_ids.length; i++) {
		if (token == canister_ids[i].token) {
			indexCanister = canister_ids[i].index;
            decimals = canister_ids[i].decimals;
            vPower = 1/Math.pow(10, decimals); 
			known = true;
		}
	} 
    if (known == false) {
		console.log(`Error - Cannot find ${token} in known token Canisters list`);
		return {}; 
	}

    const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };

    if (token != "ICP") {
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
                timeMilli: Number(milliTime)
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
                            res[i].fromPrincipalName = combinedSavedNames[k][1];
                        }
                        // to
                        if(combinedSavedNames[k][0] == to) {
                            res[i].toPrincipalName = combinedSavedNames[k][1];
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
    } else {
           // ICP TOKEN
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
                   fromPrincipal: "",
                   fromAccount: data[i].from_account,
                   toPrincipal: "",
                   toAccount: data[i].to_account,
                   hash: "No-Hash",
                   value: v,
                   type: data[i].tx_type,
                   timeMilli: Number(milliTime)
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
                                res[i].fromAccountName = combinedSavedNames[k][1];
                            }
                            // to
                            if(combinedSavedNames[k][0] == to) {
                                res[i].toAccountName = combinedSavedNames[k][1];
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

async function getDefaultAccountFromPrincipal(principal){
    const Frontend_ID = getIdentity();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.get_single_account(principal, 0);
    return res; 
}

// for getData search ()
async function getLinkedTokens(searchedToken, ID, icpLinkedPrincipal){
    if (searchedToken == "ICP") { searchedToken = "~"} // overide not self searching for ICP
    let tLen = canister_ids.length ?? 0;
    const Frontend_ID = getIdentity();
    let i;
    let icrcObj = {};
    let icpLinkedAccounts = [];
    // loop over tokens
    for(i=0; i<tLen; i++){
        if(canister_ids[i].token != searchedToken){
            if(canister_ids[i].token == "ICP") {
                // ICP Search
                // Check if ID is linked to Principal.. then search based on that. 
                // Also iterate over the ICP accounts to find active ones. 
                // TODO!
            } else {
                // ICRC Search
                let actor = icActor(canister_ids[i].index, icrcIndexIDL, Frontend_ID);
                let res = await actor.get_overview_by_id(ID); 
                let resLen = res?.length ?? 0;
                if (resLen != 0 ){
                    icrcObj[canister_ids[i].token] = true;
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
    let tLen = canister_ids.length ?? 0;
    let i;
    let icrcObj = {};
    let retData = [];
    let account; 
    if (sub == undefined || sub == "" || sub == null) sub = DEFAULT_SUBACCOUNT;
    let icrcID = combinePrincipalSubAccount(ID, sub);
    if(ID.includes("-")){
        // search by principal
        account = await getDefaultAccountFromPrincipal(ID);
        let actor = icActor(canister_ids[0].index, icpIndexIDL, Frontend_ID);
        let res = await actor.get_overview_by_id(account); 
        let resLen = res?.length ?? 0;
        if (resLen != 0 ){
            if(returnData == true) retData.push({token: "ICP" , data: res});
            icrcObj["ICP"] = true;
        }
        for(i=1; i<tLen; i++){ // index 0 already done.
            let actor = icActor(canister_ids[i].index, icrcIndexIDL, Frontend_ID);
            let res = await actor.get_overview_by_id(icrcID); 
            let resLen = res?.length ?? 0;
            if (resLen != 0 ){
                if(returnData == true) retData.push({token: canister_ids[i].token , data: res});
                icrcObj[canister_ids[i].token] = true;
            }
        }
    } else {
        // account search
        account = ID;
        let actor = icActor(canister_ids[0].index, icpIndexIDL, Frontend_ID);
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
    submitFraudReport
};

