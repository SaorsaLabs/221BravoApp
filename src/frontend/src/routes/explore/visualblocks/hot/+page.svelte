<script>
	import LayoutCombine from "../../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../../lib/componants/head.svelte";
	import Footer from "../../../../lib/componants/footer.svelte";
    import VisualBlockSubHead from "../../../../lib/componants/visualBlockSubHead.svelte";
    import ContentBox from "../../../../lib/shared/contentBox.svelte";
	import TxArtistSubDiv from "../../../../lib/componants/txArtistSubDiv.svelte";
	import TxArtistGeneric from "../../../../lib/componants/txArtistGeneric.svelte";
	import {getVisualBlockData} from '../../../../lib/code/searchRequest.js';
	import {visualBlockSubTable} from '../../../../lib/code/searchRequest.js';
	import {combinePrincipalSubAccount} from '../../../../lib/code/utils.js';

	// SETTINGS FOR TX ARTIST
	let data;
	let loadedTXS;
	async function loadStuff(){
		let x = await getVisualBlockData("HOT", 0, 0, 0, 0);
		loadedTXS = x.blocks;
		//console.log(loadedTXS);
		data = {
        settings: [{
            token: "HOT",
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
			text: "Hot Token: Latest Transactions"
        }
        };
		return x;
	}
	let promise = loadStuff();
	let clickedID = "X";
	let combinedID;
	let returnedData = [];
	let processedData = [];
	let outputData = [];
	$: outputData = processedData;
	function handleNodeClick(event){
		if(event.detail.txClicked != null) {
			processedData = [];
			returnedData = [];
			clickedID = event.detail.txClicked;
			returnedData = event.detail.data;
			combinedID = combinePrincipalSubAccount(clickedID.principal, clickedID.account);
			processedData = visualBlockSubTable(combinedID,returnedData, true);
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

</script>
<svelte:head>
	<title>Latest Blocks: Hot Token</title>
	<meta name="description" content="Home for the fam" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		<VisualBlockSubHead selected="6"/>
	</span>

	<span slot="body">
		<div>
			{#await promise}
				<p class="cntr">Loading Visual Map for Hot Token transactions... </p>
			{:then}
				<div><TxArtistGeneric data={data} on:click={handleNodeClick}/></div>
			{/await}
		</div>
		<div>
			<ContentBox>
				{#if clickedID != "X"}
					<TxArtistSubDiv tx={outputData} is_icrc={true} popupType={"visualMapIcrc"}/>
				{/if}
			</ContentBox>
		</div>

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
