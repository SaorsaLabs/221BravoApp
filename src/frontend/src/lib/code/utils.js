import tokenJSON from '../data/tokens.json';

function millisToDate(epochMillis) {
	let t1, tDate;
	const options = { dateStyle: 'long', timeZone: 'UTC' };
	t1 = new Date(epochMillis);
	tDate = t1.toLocaleString('en-GB', options); /// number.toLocaleString('en-US')
	return tDate;
}

function millisToTime(epochMillis) {
	let t1, tTime;
	const options2 = { timeStyle: 'long', timeZone: 'UTC' };
	t1 = new Date(epochMillis);
	tTime = t1.toLocaleString('en-GB', options2); //
	return tTime;
}

function datetimeToMillis(datetime, timezone) {
	let stLen;
	let lastChar;
	let dTime = datetime;
	let epochTime = 0;
	if (timezone == 'UTC') {
		stLen = datetime?.length ?? 0;
		lastChar = dTime[stLen - 1];
		if (lastChar != null) {
			if (lastChar != 'Z') dTime = dTime + 'Z';
			epochTime = Date.parse(dTime);
			return epochTime;
		} else {
			console.log("Error - Can't determine last charater");
		}
	}
}

function combinePrincipalSubAccount(principal, subaccount) {
	let ret = `${principal}.${subaccount}`;
	return ret;
}

function parsePrincipalSubAccountString(str) {
	const separatorIndex = str.indexOf('.');

	if (separatorIndex === -1) {
		// If no separator found, return an object with empty strings
		return { principal: '', subaccount: '' };
	}

	const principal = str.slice(0, separatorIndex);
	const subaccount = str.slice(separatorIndex + 1);

	return { principal, subaccount };
}

function shortenString(str) {
	if (str == undefined) return undefined;

	if (str.length < 15) {
		return str;
	} else {
		const firstchunk = str.slice(0, 10);
		const lastchunk = str.slice(-6);
		return `${firstchunk}....${lastchunk}`;
	}
}

function getUniqueValues(array){
    array.sort();
    let keepers = [];
    let i;
    keepers[0] = array[0]; // 1st is always a keeper
    let LL = array.length;
    for(i = 1; i<LL; i++) {
        if(array[i] != array[i-1]) keepers.push(array[i]);
    }
    return keepers;
}

async function processPromises(arrayOfPromises) {
    let responses = await Promise.all(arrayOfPromises);
    return responses;
}

function getTokenData(TICKER){
	let data = tokenJSON;
	let len = data.length ?? 0;
	let retData; 
	let foundOne = false;
	for(let i = 0; i< len; i++){
		if(data[i].ticker == TICKER) {
			retData = data[i];
			foundOne = true;
			break;
		}
	}
	if (foundOne == true) return retData;
	else return "Could not find a matching token";
}

function getAllTokenData(){
	let data = tokenJSON;
	return data;
}

export {
	millisToDate,
	millisToTime,
	datetimeToMillis,
	parsePrincipalSubAccountString,
	combinePrincipalSubAccount,
	shortenString,
	processPromises,
	getUniqueValues,
	getTokenData,
	getAllTokenData
};
