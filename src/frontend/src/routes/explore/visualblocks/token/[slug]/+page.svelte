<script>
	import {_slugData} from './+page';
	import LayoutCombine from "../../../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../../../lib/componants/head.svelte";
	import Footer from "../../../../../lib/componants/footer.svelte";
    import DynamicSubHeadV2 from '../../../../../lib/componants/dynamicSubHead_v2.svelte';
    import ContentBox from "../../../../../lib/shared/contentBox.svelte";
	import TxArtistSubDiv from "../../../../../lib/componants/txArtistSubDiv.svelte";
	import TxArtistGeneric from "../../../../../lib/componants/txArtistGeneric.svelte";
	import {getVisualBlockData} from '../../../../../lib/code/searchRequest.js';
	import {visualBlockSubTable} from '../../../../../lib/code/searchRequest.js';
	import {combinePrincipalSubAccount} from '../../../../../lib/code/utils.js';
	import HiddenContent from "../../../../../lib/componants/hiddenContent.svelte";
	import { authStore } from "../../../../../lib/stores/authStore";
	import Loading from "../../../../../lib/shared/loading.svelte";

	// SETTINGS FOR TX ARTIST
	let data;
	let loadedTXS;
	let token;
	let LS;
	async function loadStuff(ticker){
		token = ticker;
		let aS = authStore.read();
        LS = aS.data.loggedIn;
		if (LS == "true" || LS == true){
			let x = await getVisualBlockData(ticker, 0, 0, 0, 0);
			loadedTXS = x.blocks;
			//console.log(loadedTXS);
			data = {
			settings: [{
				token: ticker,
				lineColour: [255,255,255,0.25],
				dotColour: [50,230,255,0.75],
				size: 1.5, 
			}],
			transactions: loadedTXS,
			globalData: {
				canvasWidth: 0,
				canvasHeight: 0,
				canvasBGColour: [0,0,0,0.666],
				globalZoom: 1,
				inX: 1,
				inY: 1,
				globalMoveX: 0,
				globalMoveY: 0,
				text: `${ticker}: Latest Transactions`
			}
			};
			return x;
		}
	}
	let promise = loadStuff(_slugData.token);
	let clickedID = "X";
	let combinedID;
	let returnedData = [];
	let processedData = [];
	let outputData = [];
	
	function handleNodeClick(event){
		if(event.detail.txClicked != null) {
			processedData = [];
			returnedData = [];
			clickedID = event.detail.txClicked;
			returnedData = event.detail.data;
			combinedID = combinePrincipalSubAccount(clickedID.principal, clickedID.account);
			if (returnedData[0].token != "ICP") {
				processedData = visualBlockSubTable(combinedID,returnedData, true);
			} else {
				processedData = visualBlockSubTable(clickedID.account,returnedData, false);
			}
			
		}
		else {
			clickedID = "X";
		}
	}

	// MODAL STUFF
	let showPopup = false;
	const onShowPopup = (ev) => {
		showPopup = true;
	}
	const onPopupClose = (data) => {
		showPopup = false;
	}

	function handleSubHeadClick(event){
		processedData = [];
		returnedData = [];
		processedData = [];
		outputData = [];
		promise = loadStuff(event.detail);
	}

	$: promise;
	$: data;
	$: token;
	$: outputData = processedData;
</script>
<svelte:head>
	<title>Latest Blocks: ckBTC Token</title>
	<meta name="description" content="visual block explorer" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		<DynamicSubHeadV2 selected={token} mode={"visual"} on:click={handleSubHeadClick} />
	</span>

	<span slot="body">
		{#if LS == "false" || LS == false}
			<ContentBox type="standard-shaddow-dark-padding">	
				<HiddenContent>Become a member to access the Visual Block Explorer</HiddenContent>
			</ContentBox>
		{:else}
			<div>
				{#await promise}
					<p class="cntr">Loading Visual Map for {token} transactions... </p>
					<Loading/>
				{:then}
					<div><TxArtistGeneric data={data} on:click={handleNodeClick}/></div>
				{/await}
			</div>
			<div>
				<ContentBox>
					{#if clickedID != "X"}
						{#if token != "ICP"}
							<TxArtistSubDiv tx={outputData} is_icrc={true} popupType={"visualMapIcrc"}/>
						{:else}
							<TxArtistSubDiv tx={outputData} is_icrc={false} popupType={"visualMap"}/>
						{/if}
					{/if}
				</ContentBox>
			</div>
		{/if}
	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	.content {
		min-height: 90vh;
		text-align: center;
		align-content: center;
		min-width: 700px;
	}
	.cntr {
		text-align: center;
	}
	.box{
		border: 2px;
		border-style: dashed;
		border-color: aliceblue;
	}
</style>
