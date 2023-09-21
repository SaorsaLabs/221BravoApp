<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
    import SearchSubHead from "../../../lib/componants/searchSubHead.svelte";
    import ContentBox from "../../../lib/shared/contentBox.svelte";
	import icpLogo from '../../../lib/images/icpLogo.png'
	import SearchForm from "../../../lib/componants/searchForm.svelte";
	import BlockSearchForm from "../../../lib/componants/blockSearchForm.svelte";
    import SearchOverview from "../../../lib/componants/searchOverview.svelte";
    import TxAcTable from "../../../lib/componants/txAcTable.svelte";
	import TxBlockTable from "../../../lib/componants/txBlockTable.svelte";
	import Loading from "../../../lib/shared/loading.svelte";
	import HiddenContent from "../../../lib/componants/hiddenContent.svelte";
	import FlagsTable from "../../../lib/componants/flagsTable.svelte";
	import LinksTable from "../../../lib/componants/linksTable.svelte";
	import {basicAccountTableTX, basicBlockTableTX, linkedIDTable } from '../../../lib/code/searchRequest.js';
	import { getData, getLatestBlockData, getBlockData } from '../../../lib/code/searchRequest_v2.js'
	import {authStore, authTrigger} from "../../../lib/stores/authStore.js";
	import { browser } from '$app/environment';
    
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

	let token = 'ICP';
	let searchResults = [];
	let blockSearchResults = [];
	let txTableProcessed = [];
	let linkedTableProcessed = [];
	let blockTableProcessed = [];
	let blockTableUsePrincipal = true;
	let outcome = '';
	let hideResultDivs = true;
	let hideAccountSearch = false;
	let hideBlockSearch = false;
	let searchMode = 'account';
	let ovData = {};
	let epochStart = 0;
	let epochEnd = 0; 
	let contentLoading = false;
	let btnClicked = '';

	async function handleAccountClick(event) {
		outcome = '';
		contentLoading = true;
		hideResultDivs = true;
		txTableProcessed = [];
		searchResults = [];
		btnClicked = event.detail.btnClicked;

		if(btnClicked == 'search'){
			let target = event.detail.searchID;
			searchResults = await getData(
				token,
				event.detail.searchID,
				"0",
			);

			if(searchResults != "nothing-found"){ 
				if(searchResults.maxResults == true || searchResults.maxResults == 'true') {
				alert(`Too Many Results - Only first 10,000 transactions will be shown. 
						Try using a smaller timeframe for the search `); 
				}
				// overview
				ovData = {
					sentValue: searchResults.overview.tokenSent,
					receivedValue: searchResults.overview.tokenReceived,
					balance: searchResults.overview.tokenBalance,
					sentTX: searchResults.overview.numSent,
					receivedTX: searchResults.overview.numReceived,
					activeFrom: searchResults.overview.activeFrom,
					lastActive: searchResults.overview.lastActive
				};

				// TX Table 						//  searchResults.primaryAccount only on non-principal datasets, otherwise use target !! 
				txTableProcessed = basicAccountTableTX(searchResults.primaryAccount,searchResults.tokenTXS,token);
				linkedTableProcessed = linkedIDTable(searchResults.linkedIdStats,token);
				// Linked ID Table 

				searchMode = 'account';
				hideResultDivs = false;
				hideBlockSearch = true;
				contentLoading = false;
			}else {
				// nothing-found
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

	async function handleBlockClick(event){
		contentLoading = true;
		hideResultDivs = true;
		blockTableProcessed = [];
		blockSearchResults = [];
		btnClicked = event.detail.btnClicked;
		
		if(btnClicked == 'search'){
			blockSearchResults = await getBlockData(
				token,
				event.detail.startBlock,
				event.detail.endBlock,
				0,
				0
			);

			// Blocks Table
			blockTableProcessed = basicBlockTableTX(blockSearchResults.blocks,token, false);
			blockTableUsePrincipal = blockTableProcessed.usePrincipal;
			searchMode = 'blocks';
			hideResultDivs = false;
			hideAccountSearch = true;
			contentLoading = false;
		}// end btnClicked search

		if(btnClicked == 'latest'){
			epochStart = 0;
			epochEnd = 0;

			blockSearchResults = await getLatestBlockData(500,token);

			// Blocks Table
			blockTableProcessed = basicBlockTableTX(blockSearchResults.blocks,token, false);
			blockTableUsePrincipal = blockTableProcessed.usePrincipal;
			searchMode = 'blocks';
			hideResultDivs = false;
			hideAccountSearch = true;
			contentLoading = false;
		}// end btnClicked search

		if(btnClicked == 'reset'){
			btnClicked = '';
			contentLoading = false;
			searchMode = '';
			hideAccountSearch = false;
			hideBlockSearch = false;
		}
	}
	let resetKey;
	function handleSubHeadClick(event){
		let resetPage = event.detail.reset; 
		if (resetPage == true) {
			contentLoading = false;
			hideAccountSearch = false;
			hideBlockSearch = false;
			btnClicked = '';
			searchMode = '';
			outcome = '';
			resetKey = {};
		}
	}
</script>
<svelte:head>
	<title>ICP Search</title>
	<meta name="description" content="Search ICP Transactions" />
</svelte:head>

<LayoutCombine>
	<span slot="head">
		<Head/>
		<SearchSubHead selected="1" on:click={handleSubHeadClick}/>
	</span>

	<span slot="body">
		<!-- Account Search -->
		{#if hideAccountSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">ICP Account Search</h4></td>
						<td class="rText"><img class="headAlign" src={icpLogo} alt="ICP Logo" width="50px"/> </td>
					</tr>
				</table>
				{#key resetKey}
					<SearchForm inputType={'both'} on:click={handleAccountClick}/>
				{/key}
				<div class="cText smlPadTB">{outcome}</div>
			</ContentBox>
		{/if}

		<!-- Block Search -->
		{#if hideBlockSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">ICP Block Search</h4></td>
						<td class="rText"><img class="headAlign" src={icpLogo} alt="ICP Logo" width="50px"/> </td>
					</tr>
				</table>
				{#key resetKey}
					<BlockSearchForm on:click={handleBlockClick}/>
				{/key}
				<!-- <div class="cText smlPadTB">{outcome}</div> -->
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
		{#if hideResultDivs == false && searchMode == 'account'}
		<!-- ACCOUNT SEARCH -->
			<!-- Overview -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<SearchOverview token={token} data={ovData} />
			</ContentBox>
			<!-- non Premium Div -->
			{#if LS == false || LS == "false"} 
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>	
					<HiddenContent>Become a member to see linked accounts, account names and flags</HiddenContent>
			</ContentBox>
			{:else}
			<!-- Flags -->
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>
				<FlagsTable flagData={searchResults.flags}/>
			</ContentBox>
			<!-- Linked Accounts -->
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Linked {token} Accounts</h4>
				<LinksTable data={linkedTableProcessed}/>
			</ContentBox>
			{/if}

			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Transactions</h4>
				<TxAcTable  txData={txTableProcessed} popupType={'noPrincipal'}/>
			</ContentBox>
			
		{:else if hideResultDivs == false && searchMode =='blocks'}
			<!-- BLOCK SEARCH -->
			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<TxBlockTable txData={blockTableProcessed.blocks} usePrincipal={true} popupType={'noPrincipalBlock'}/>
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
		padding-bottom: 6px;
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
</style>
