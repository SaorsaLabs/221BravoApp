<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
    import ContentBox from "../../../lib/shared/contentBox.svelte";
	import projectLogo from '$lib/images/Avatar_221B.jpg'
	import SearchForm from "../../../lib/componants/searchForm.svelte";
    import SearchOverview from "../../../lib/componants/searchOverview.svelte";
    import TxAcTableCombined from "../../../lib/componants/members/txAcTableCombined.svelte";
	import Loading from "../../../lib/shared/loading.svelte";
	import HiddenContent from "../../../lib/componants/hiddenContent.svelte";
	import FlagsTable from "../../../lib/componants/flagsTable.svelte";
	import LinksTable from "../../../lib/componants/linksTable.svelte";
	import {basicAccountTableCombinedTX, linkedIDTable } from '../../../lib/code/searchRequest.js';
	import { searchAllLinkedTokens } from '../../../lib/code/searchRequest_v2.js'
	import {authStore, authTrigger} from "../../../lib/stores/authStore.js";
	import {parsePrincipalSubAccountString} from '../../../lib/code/utils.js';
	import { browser } from '$app/environment';
  import SubHead from "../../../lib/shared/subHead.svelte";
    
    let LS = false;
    // Logged In? 
    authTrigger.subscribe(value =>{
        if(browser){
            if(value>=1){
                let x = authStore.read();
                LS = x.data.loggedIn;
            }
        }
    });

	let searchResults = [];
	let txTableProcessed = [];
	let linkedTableProcessed = [];
	let outcome = '';
	let hideResultDivs = true;
	let hideAccountSearch = false;
	let hideBlockSearch = false;
	let searchMode = 'account';
	let ovData = {};
	let contentLoading = false;
	let btnClicked = '';
	let inptAC, inptSA;
	let allResults; 

	async function handleAccountClick(event) {
		outcome = '';
		contentLoading = true;
		hideResultDivs = true;
		txTableProcessed = [];
		searchResults = [];
		btnClicked = event.detail.btnClicked;

		if(btnClicked == 'search'){

			inptAC = event.detail.searchID;
			inptSA = event.detail.subAC;

			 // catch copy paste search using combined ICRC format
			 if(inptAC.includes(".") && inptAC.includes("-") ){
				let parsed = parsePrincipalSubAccountString(inptAC);
				inptAC = parsed.principal;
				inptSA = parsed.subaccount;
        	}

			inptAC = inptAC.replace(/\s/g, ''); // remove whitespace;
			inptSA = inptSA.replace(/\s/g, ''); // remove whitespace;

			// search tokens
			allResults = await searchAllLinkedTokens(inptAC, inptSA);
			let allResLen = allResults.length ?? 0;
			//console.log(allResults);
			// searchResults

			if( allResLen > 0 ){ 
				// if(searchResults.maxResults == true || searchResults.maxResults == 'true') {
				// alert(`Too Many Results - Only first 1,000 transactions will be shown.`); 
				// }
				// overview
				// ovData = {
				// 	sentValue: searchResults.overview.tokenSent,
				// 	receivedValue: searchResults.overview.tokenReceived,
				// 	balance: searchResults.overview.tokenBalance,
				// 	sentTX: searchResults.overview.numSent,
				// 	receivedTX: searchResults.overview.numReceived,
				// 	activeFrom: searchResults.overview.activeFrom,
				// 	lastActive: searchResults.overview.lastActive
				// };

				// TX Table 
				let allTXS = [];
				let target1 = "unknown-undefined";
				let target2 = "unknown-undefined";
				for (let i=0; i<allResLen; i++){
					if (allResults[i].token == "ICP") target2 = allResults[i].tokenData.primaryAccount
					else target1 = allResults[i].tokenData.primaryAccount;
					allTXS = [...allTXS, ...allResults[i].tokenData.tokenTXS];
				}

				txTableProcessed = basicAccountTableCombinedTX(
					target1,
					allTXS, 
					target2
					);
				console.log("ALL PROCESSED ", txTableProcessed);

				// combine transactions
				// 	let returnARLen = returnAR.length ?? 0;
				// 	let combinedTXS = [];
				// 	for(i=0; i<returnARLen; i++){
				// 		combinedTXS = [...combinedTXS, ... returnAR[i].tokenData.tokenTXS];
				// }

				//linkedTableProcessed = linkedIDTable(searchResults.linkedIdStats,token);
				// Linked ID Table 

				searchMode = 'account';
				hideResultDivs = false;
				hideBlockSearch = true;
				contentLoading = false;
			} else {
				// nothing found
				outcome = "Search returned 0 results";
				contentLoading = false;
			}
		}// end btnClicked search

		if(btnClicked == 'reset'){
			contentLoading = false;
			hideAccountSearch = false;
			hideBlockSearch = false;
		}
	}
</script>
<svelte:head>
	<title>Combined Search</title>
	<meta name="description" content="Search All Tokens" />
</svelte:head>

<LayoutCombine>
	<span slot="head">
		<Head/>
		<SubHead>
			<span class="headText"> Members Only - Account Combined Search </span>
		</SubHead>
	</span>

	<span slot="body">
		<!-- Account Search -->
		{#if hideAccountSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">Combined Search</h4></td>
						<td class="rText"><img class="headAlign" src={projectLogo} alt="Project Logo" width="50px"/> </td>
					</tr>
				</table>
				{#key resetKey}
					<SearchForm inputType={'both'} subAcEnabled={true} on:click={handleAccountClick}/>
				{/key}
				<div class="cText smlPadTB">{outcome}</div>
			</ContentBox>
		{/if}

		<!-- LOADING -->
		{#if contentLoading == true}
			<div class="cText loaderMargins"> 
				<Loading style={'loader'} align={'centre'}/>
				<p>Loading...</p>
			</div>
		{/if}

		<!-- RESULTS -->
		{#if hideResultDivs == false}
		<!-- ACCOUNT SEARCH -->
			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Transactions</h4>
				<TxAcTableCombined  txData={txTableProcessed} is_icrc={true} popupType={'icrc'}/>
			</ContentBox>

		{/if}
		
	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	.mainAlign{
		padding: 10px;
	}
	.smlPadTB{
		padding-top: 3px;
		padding-bottom: 3px;
		font-size: larger;
	}
	.box{
		border: 2px;
		border-style: dashed;
		border-color: aliceblue;
	}
	.width100{
		width: 100%;
	}
	.cText{
		text-align: center;
	}
	.rText{
		text-align: right;
		vertical-align: text-top;
	}
	.headAlign{
		margin-top: 5px;
		margin-left: 5px;
	}
	.loaderMargins{
		margin: 20px;
	}
	.headText {
		color: aliceblue;
	}
</style>
