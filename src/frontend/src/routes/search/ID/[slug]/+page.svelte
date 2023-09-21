<script>
	import LayoutCombine from "../../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../../lib/componants/head.svelte";
	import Footer from "../../../../lib/componants/footer.svelte";
    import SearchSubHead from "../../../../lib/componants/searchSubHead.svelte";
    import ContentBox from "../../../../lib/shared/contentBox.svelte";
	import SearchForm from "../../../../lib/componants/searchForm.svelte";
    import SearchOverview from "../../../../lib/componants/searchOverview.svelte";
    import TxAcTable from "../../../../lib/componants/txAcTable.svelte";
	import Loading from "../../../../lib/shared/loading.svelte";
	import HiddenContent from "../../../../lib/componants/hiddenContent.svelte";
	import FlagsTable from "../../../../lib/componants/flagsTable.svelte";
	import LinksTable from "../../../../lib/componants/linksTable.svelte";
	import { basicAccountTableTX, linkedIDTable} from '../../../../lib/code/searchRequest.js';
	import { getData } from '../../../../lib/code/searchRequest_v2.js'
    import { onMount } from "svelte";
    import {_slugData} from './+page';
	import {authStore, authTrigger} from "../../../../lib/stores/authStore.js";
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

	// let subSwitch = _slugData.sub = encodeURIComponent("''") ? "None" :  _slugData.sub;
    let headSelected;
	let token = _slugData.token;
	let searchResults = [];
	let txTableProcessed = [];
	let linkedTableProcessed = [];
	let outcome = '';
	let hideResultDivs = true;
	let hideAccountSearch = false;
	let ovData = {};
	let contentLoading = false;
	let txTableType = "";
	let icrcToggle = false;
	let showSubACInput = false;

    onMount(async () => {
        contentLoading = true;
		hideResultDivs = true;
		txTableProcessed = [];
		searchResults = [];

        // Head Select 
        if(_slugData.token == "ICP") {
			headSelected = 1;
			txTableType = "noPrincipal";
			icrcToggle = false;
			showSubACInput = false;
		}
        if(_slugData.token == "CKBTC") {
			headSelected = 2;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
        if(_slugData.token == "SNS1") {
			headSelected = 3;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
        if(_slugData.token == "CHAT") {
			headSelected = 4;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
		if(_slugData.token == "KINIC") {
			headSelected = 5;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
		if(_slugData.token == "HOT") {
			headSelected = 6;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
		if(_slugData.token == "GHOST") {
			headSelected = 7;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
		if(_slugData.token == "MODCLUB") {
			headSelected = 8;
			txTableType = "icrc";
			icrcToggle = true;
			showSubACInput = true;
		}
		// if(_slugData.token == "CKETH") {
		// 	headSelected = 3;
		// }

        let target = _slugData.id;
        if(_slugData.id != null && _slugData.id !=''){
            searchResults = await getData(
                _slugData.token,
                _slugData.id,
				_slugData.sub,
                0,
                0,
                0,
                0
            );
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

		if(searchResults.maxResults == true || searchResults.maxResults == 'true') {
				alert("Too Many Results - Only first 10,000 transactions will be shown. Try using a smaller timeframe for the search"); 
			}

        // TX Table
        txTableProcessed = basicAccountTableTX(searchResults.primaryAccount,searchResults.tokenTXS,token);
		linkedTableProcessed = linkedIDTable(searchResults.linkedIdStats,_slugData.token);

        hideResultDivs = false;
        contentLoading = false;
	});

</script>
<svelte:head>
	<title>221B Search</title>
	<meta name="description" content="Search Transactions" />
</svelte:head>

<LayoutCombine>
	<span slot="head">
		<Head/>
		<SearchSubHead selected={headSelected}/>
	</span>

	<span slot="body">
		<!-- Account Search -->
		{#if hideAccountSearch == false}
			<ContentBox type="standard-shaddow-dark">
				<table style="width: 100%;">
					<tr>
						<td><h4 class="headAlign">{_slugData.token} Account Search</h4></td>
					</tr>
				</table>
				<SearchForm inputType={'pushSearch'} pushID={_slugData.id} pushSUB={_slugData.sub} subAcEnabled={showSubACInput}/>
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
			<!-- Overview -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<SearchOverview token={token} data={ovData} />
			</ContentBox>
			<!-- non Premium Div -->
			{#if LS == false || LS == "false"} 
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>	
					<HiddenContent>Become a member to see linked accounts and account flags</HiddenContent>
			</ContentBox>
			{:else}
			<!-- Flags -->
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>
				<FlagsTable flagData={searchResults.flags}/>
			</ContentBox>
			<!-- Linked Accounts -->
			<ContentBox type="standard-shaddow-dark-padding" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Linked {token} Accounts</h4>
				<LinksTable is_icrc={icrcToggle} data={linkedTableProcessed}/>
			</ContentBox>
			{/if}

			<!-- TX Table -->
			<ContentBox type="standard-shaddow-dark" hidden={hideResultDivs}>
				<h4 class="mainAlign" style="padding-bottom: 0;">Transactions</h4>
				<TxAcTable is_icrc={icrcToggle} txData={txTableProcessed} popupType={txTableType}/>
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
