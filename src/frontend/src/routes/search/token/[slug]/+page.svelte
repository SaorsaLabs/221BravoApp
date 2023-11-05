<script>
	import LayoutCombine from "$lib/componants/layoutCombine.svelte";
	import Head from "$lib/componants/head.svelte";
	import Footer from "$lib/componants/footer.svelte";
    import DynamicSubHead from "$lib/componants/dynamicSubHead_v2.svelte";
    import ContentBox from "$lib/shared/contentBox.svelte";
	import TokenLogos from "../../../../lib/shared/tokenLogos.svelte";
	import SearchForm from "$lib/componants/searchForm.svelte";
	import BlockSearchForm from "$lib/componants/blockSearchForm.svelte";
    import SearchOverview from "$lib/componants/searchOverview.svelte";
    import TxAcTable from "$lib/componants/txAcTable_v2.svelte";
	import TxBlockTable from "$lib/componants/txBlockTable_v2.svelte";
	import Loading from "$lib/shared/loading.svelte";
	import HiddenContent from "$lib/componants/hiddenContent.svelte";
	import FlagsTable from "$lib/componants/flagsTable.svelte";
	import LinksTable from "$lib/componants/linksTable_v2.svelte";
	import { getData, getBlockData, getLatestBlockData, processAccountTXS, processBlockTXS, linkedIDTable } from '$lib/code/searchRequest_v3.js'
	import { datetimeToMillis, getTokenData } from '$lib/code/utils.js';
	import {authStore, authTrigger} from "$lib/stores/authStore.js";
	import { browser } from '$app/environment';
	import {_slugData} from './+page';
	import { onMount } from "svelte";
    
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

	let tokenData, tokenData2;
	let token = _slugData.token;
	let hasUrlParams = false;
	let searchResults = [];
	let blockSearchResults = [];
	let txTableProcessed = [];
	let linkedTableProcessed = [];
	let blockTableProcessed = [];
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
	let subInput = false;

	// check if has Url Params
	if(_slugData.id != null && _slugData.sub != null){
		hasUrlParams = true;
		hideBlockSearch = true;
	}

	let promise = loadStuff()
	
	async function loadStuff(){
		tokenData2 = getTokenData(_slugData.token);
		if(tokenData2.standard.includes("icrc")) subInput = true;
	}

	async function handleAccountClick(event, isUrlSearch) {
		outcome = '';
		contentLoading = true;
		hideResultDivs = true;
		txTableProcessed = [];
		searchResults = [];
		btnClicked = event?.detail?.btnClicked;

		if(btnClicked == 'search' || isUrlSearch == true){
			//let target = event.detail.searchID;
			let subAcInput = event?.detail?.subAC; 

			// user input search
			if(hasUrlParams == false){
				searchResults = await getData(
					token,
					event.detail.searchID,
					subAcInput
				);
			} else {
				// search by URL params
				searchResults = await getData(
					token,
					_slugData.id,
					_slugData.sub
				);
			}


			if(searchResults != "nothing-found"){ 
				if(searchResults.maxResults == true || searchResults.maxResults == 'true') {
				alert(`Too Many Results - Only first 1,000 transactions will be shown.`); 
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

				// TX Table 
				txTableProcessed = processAccountTXS(searchResults.primaryAccount,searchResults.tokenTXS,token);
				linkedTableProcessed = linkedIDTable(searchResults.linkedIdStats,token);
				
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

	async function handleBlockClick(event){
		contentLoading = true;
		hideResultDivs = true;
		blockTableProcessed = [];
		blockSearchResults = [];
		btnClicked = event.detail.btnClicked;
		
		if(btnClicked == 'search'){
			if(event.detail.startBlockDate != 0){
				epochStart = datetimeToMillis(event.detail.startBlockDate, 'UTC');
			}
			else epochStart  = 0;
			if(event.detail.endBlockDate != 0){
				epochEnd = datetimeToMillis(event.detail.endBlockDate, 'UTC');
			}
			else epochEnd = 0;

			blockSearchResults = await getBlockData(
				token,
				event.detail.startBlock,
				event.detail.endBlock,
				// epochStart,
				// epochEnd
			);

			// Blocks Table
			blockTableProcessed = processBlockTXS(blockSearchResults.blocks);
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
			blockTableProcessed = processBlockTXS(blockSearchResults.blocks);
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
		//let resetPage = event.detail.reset; 
		contentLoading = false;
		hideAccountSearch = false;
		hideBlockSearch = false;
		btnClicked = '';
		searchMode = '';
		outcome = '';
		_slugData.id = null;
		_slugData.sub = null;
		resetKey = {};
		tokenData2 = getTokenData(event.detail);
		if(tokenData2.standard.includes("icrc")) subInput = true;
		else subInput = false;
		token = event.detail;
	}

	onMount(() => {
		if(hasUrlParams == true) handleAccountClick(null, true)
  	});

	$: tokenData = tokenData2;
</script>
<svelte:head>
	<title>{token} Search</title>
	<meta name="description" content="Search {token} Transactions" />
</svelte:head>

<LayoutCombine>
	<span slot="head">
		<Head/>
		<DynamicSubHead selected={token} on:click={handleSubHeadClick}/>
	</span>

	<span slot="body">
		{#await promise}
		{:then}
		<!-- Account Search -->
		{#if hideAccountSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">{tokenData.shortName} Account Search</h4></td>
						<td class="rText">
							<TokenLogos token={token} width={"50px"}/>
						</td>
					</tr>
				</table>
				{#key resetKey}
					{#if hasUrlParams == false}
						<SearchForm inputType={'both'} subAcEnabled={subInput} on:click={handleAccountClick}/>
					{:else}
						<SearchForm inputType={'pushSearch'} pushID={_slugData.id} pushSUB={_slugData.sub} subAcEnabled={subInput}/>
					{/if}
				{/key}
				<div class="cText smlPadTB">{outcome}</div>
			</ContentBox>
		{/if}

		<!-- Block Search -->
		{#if hideBlockSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">{tokenData.shortName} Block Search</h4></td>
						<td class="rText"><TokenLogos token={token} width={"50px"}/></td>
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
				<LinksTable is_icrc={true} data={linkedTableProcessed}/>
			</ContentBox>
			{/if}

			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Transactions</h4>
				<TxAcTable  txData={txTableProcessed} is_icrc={true} popupType={'icrc'}/>
			</ContentBox>
			
		{:else if hideResultDivs == false && searchMode =='blocks'}
			<!-- BLOCK SEARCH -->
			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<TxBlockTable txData={blockTableProcessed.blocks} is_icrc={true} popupType={'icrcBlock'}/>
			</ContentBox>
		{/if}
		{/await}
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
		padding: 10px;
	}
	.headAlign{
		margin-top: 5px;
		margin-left: 5px;
	}
	.loaderMargins{
		margin: 20px;
	}
</style>
