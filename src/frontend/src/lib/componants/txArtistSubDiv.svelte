<script>
import TxBlockVisualTable from "./txBlockVisualTable.svelte";
import search from "$lib/images/search.png";
import CopyButton from "../shared/copyButton.svelte";
import SaveButton from "../shared/saveButton.svelte";
  import { loop_guard } from "svelte/internal";

// Min
export let tx = {};
export let is_icrc = false; 

$: update(tx)
let dataOP = [];
function update(data){
	dataOP = data;
}
export let popupType = "visualMap";
</script>

<div> 
	<!-- ACCOUNT CLICKED -->
	{#if is_icrc == false}
			
			<p class="textPad" style="padding-top:15px;"> Account Clicked : {dataOP[0]?.targetAC ?? "Mint/Burn Account"}
			{#if dataOP[0]?.targetAC != null && dataOP[0]?.targetAC != "null" && dataOP[0]?.targetAC != "Unknown"}
				<a href="/search/ID/{dataOP[0].token}?id={dataOP[0]?.targetAC}&sub=None" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
				<CopyButton icrcAccount={false} text={dataOP[0]?.targetAC}/>
				<SaveButton icrcAccount={false} text={dataOP[0].targetAC} />
				<span class="textPad" style="color:aqua;"> {dataOP[0]?.targetACName}</span>
			{/if} 
			</p>
		
			<!-- <p class="textPad" style="padding-bottom: 10px;"> Principal : {dataOP[0]?.targetPR}
				{#if dataOP[0]?.targetPR != null && dataOP[0]?.targetPR != "null" && dataOP[0]?.targetPR != "Unknown"}
				<a href="/search/ID/{dataOP[0].token}?id={dataOP[0].targetPR}&sub=None" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
				<CopyButton icrcAccount={false} text={dataOP[0].targetPR}/>
				<SaveButton icrcAccount={false} text={dataOP[0].targetPR} />
				<span class="textPad" style="color:aqua; margin-bottom: 10px;"> {dataOP[0]?.targetPRName}</span>
				{/if} 
			</p> -->
	{:else}
		<!-- icrc -->
			<p class="textPad" style="padding-top:15px;"> Principal : {dataOP[0]?.targetPR ?? "Mint/Burn Account"}
				{#if dataOP[0]?.targetPR != null && dataOP[0]?.targetPR != "null" && dataOP[0]?.targetPR != "Unknown"}
					{#if dataOP[0]?.targetAC == "0000000000000000000000000000000000000000000000000000000000000000"}
						<a href="/search/ID/{dataOP[0].token}?id={dataOP[0].targetPR}&sub={dataOP[0].targetAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
						<CopyButton icrcAccount={true} text={dataOP[0].targetPR} text2={dataOP[0].targetAC}/>
						<SaveButton icrcAccount={true} text={dataOP[0].targetPR} text2={dataOP[0].targetAC} />
						<span class="textPad" style="color:aqua; margin-bottom: 10px;"> {dataOP[0]?.targetPRName}</span>
					{/if}
				{/if} 
			</p>
			{#if dataOP[0]?.targetAC != "0000000000000000000000000000000000000000000000000000000000000000"}
			<p class="textPad" style="padding-bottom: 10px;"> Account Clicked : {dataOP[0]?.targetAC ?? "Mint/ Burn Account"}
			{#if dataOP[0]?.targetAC != null && dataOP[0]?.targetAC != "Mint/ Burning Account" && dataOP[0]?.targetAC != "Unknown"}
				<a href="/search/ID/{dataOP[0].token}?id={dataOP[0].targetPR}&sub={dataOP[0].targetAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
				<CopyButton icrcAccount={true} text={dataOP[0].targetPR} text2={dataOP[0].targetAC}/>
				<SaveButton icrcAccount={true} text={dataOP[0].targetPR} text2={dataOP[0].targetAC} />
				<span class="textPad" style="color:aqua;"> {dataOP[0]?.targetACName}</span>
			{/if} 	
			</p>
		{/if}
	{/if}

	<TxBlockVisualTable txData={dataOP} is_icrc={is_icrc} popupType={popupType}/>
</div>


<style>
	.textPad{
		padding: 0px;
		padding-left: 5px;
		margin: 2px;
	}
	p{
        padding-top: 0px;
    }
</style>