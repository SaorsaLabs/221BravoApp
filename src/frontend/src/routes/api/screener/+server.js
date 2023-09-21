import { json } from '@sveltejs/kit';
import { getAllLinkedTokens, checkFlags } from '../../../lib/code/searchRequest_v2.js';
import { parsePrincipalSubAccountString } from '../../../lib/code/utils.js';

async function GET(request) {
	let id = request.url.searchParams.get('id');
	let key = request.url.searchParams.get('key');

	// Handle Bad Inputs 
	if (!key || !id) {
	  return {
		status: 400, // Bad Request
		body: {
		  error: 'An id and key are required as query parameters.'
		}
	  };
	}
	// CHECK KEY IS AUTHORISED
	// TODO!! 

	// IS ICRC
	let pr, sub;
	let isICRC = false;
	if (id.includes("-") && id.includes(".")){
		let parse = parsePrincipalSubAccountString(id);
		pr = parse.principal;
		sub = parse.subaccount;
		isICRC = true;
	} 

	// Search on ID
	let overviews, flags;
	if (isICRC == true){ overviews = await getAllLinkedTokens(pr,sub,true); }
	else { overviews = await getAllLinkedTokens(id,"",true); }
	flags = await checkFlags(id, true);

	let firstActive = Number.MAX_SAFE_INTEGER; 
	let lastActive = Number.MIN_SAFE_INTEGER;
	let tokens = [];
	let faNum, laNum;

	let ovLen = overviews.length ?? 0; 
	for(let i=0; i<ovLen; i++){
		faNum = Number(overviews[i].data[0].first_active);
		laNum = Number(overviews[i].data[0].last_active);
		if (faNum < firstActive ) firstActive = faNum;
		if (laNum > lastActive ) lastActive = laNum; 
		tokens.push(overviews[i].token);
	}

	if(ovLen>0){
		let retOBJ = {
			tokens,
			first_active: firstActive,
			last_active: lastActive,
			flags,
		};
		console.log(retOBJ);
		 return json(JSON.stringify({result: retOBJ}));
	} else {
		return json(JSON.stringify({ result: "no-trace" }));
	}
	
}
export {GET};
