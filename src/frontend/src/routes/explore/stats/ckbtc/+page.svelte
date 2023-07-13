<script>
	import LayoutCombine from "../../../../lib/componants/layoutCombine.svelte";
	import { onMount } from 'svelte';
	import Head from "../../../../lib/componants/head.svelte";
	import Footer from "../../../../lib/componants/footer.svelte";
	import ContentBox from "../../../../lib/shared/contentBox.svelte";
	import TxOverTime from "../../../../lib/componants/txOverTime.svelte";
	import { getICRC_Stats } from '../../../../lib/code/fetchStats.js';
	import Loading from "../../../../lib/shared/loading.svelte";
	import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
    import TopHolderTable from "../../../../lib/componants/topHolderTable.svelte";
	
	let decimals  = 8;
    let vPower = 1/Math.pow(10, decimals); 

	onMount(() => {getStats()});
	let hourlyData;
	let dailyData;	
	let dataLoaded = false;
	// Stats
	let hourlyOTdata = [];
	let hourlyOTlabels = [];
	let dailyOTdata = [];
	let dailyOTlabels = [];
	let hourLen, dayLen;
	let hourlyExport = {};
	let dailyExport = {};
	let thDataDaily = {};
	let thDataHourly = {};

	async function getStats(){
		let i;

		// HOURLY STATS
		hourlyData = await getICRC_Stats("CKBTC", "hourly");

		// tx over time
		let otHourLen = hourlyData.count_over_time.length ?? 0;
		hourLen = otHourLen;
		let opTime;
		let startTime;  
		for(i = 0; i<otHourLen; i++){
			startTime = Number(hourlyData.count_over_time[i].start_time);
			opTime = nanoToDate(startTime);
			hourlyOTdata.push(Number(hourlyData.count_over_time[i].total_count));
			hourlyOTlabels.push(opTime.shortTime);
		}

		// // top holders
		// let hourlyAC = [];
		// let hourlyPR = [];
		// let acLen = 0;
		// let counter = 1;
		// for(i = 0; i<acLen; i++){
		// 	hourlyAC.push();
		// 	counter++;
		// }
		// let prLen = 0;
		// counter = 0;
		// for(i = 0; i<acLen; i++){
		// 	hourlyPR.push();
		// 	counter++;
		// }


		// WEEKLY STATS
		dailyData = await getICRC_Stats("CKBTC", "daily");
		// tx over time
		let otLen = dailyData.count_over_time.length ?? 0;
		dayLen = otLen;
		let opTimeDaily;
		let startTimeDaily;  
		for(i = 0; i<otLen; i++){
			startTimeDaily = Number(dailyData.count_over_time[i].start_time);
		 	opTimeDaily = nanoToDate(startTimeDaily);
		 	dailyOTdata.push(Number(dailyData.count_over_time[i].total_count));
		 	dailyOTlabels.push(opTimeDaily.dateOnly);
		}

		// top holders

		// outputs
		hourlyExport = {
			hourlyOTdata,
			hourlyOTlabels,
			hourlyTotalCount: Number(hourlyData.total_transaction_count),
			hourlyTotalValue: Number(hourlyData.total_transaction_value)*vPower,
			hourlyTotalMint: Number(hourlyData.mint_stats.count),
			hourlyMintValue: Number(hourlyData.mint_stats.total_value)*vPower,
			hourlyTotalBurn: Number(hourlyData.burn_stats.count),
			hourlyBurnValue: Number(hourlyData.burn_stats.total_value)*vPower,
		};

		dailyExport = {
			dailyOTdata,
			dailyOTlabels,
			dailyTotalCount: Number(dailyData.total_transaction_count),
			dailyTotalValue: Number(dailyData.total_transaction_value)*vPower,
			dailyTotalMint: Number(dailyData.mint_stats.count),
			dailyMintValue: Number(dailyData.mint_stats.total_value)*vPower,
			dailyTotalBurn: Number(dailyData.burn_stats.count),
			dailyBurnValue: Number(dailyData.burn_stats.total_value)*vPower,
		};

		//topHolderData = 

		// type HolderBalance = record {holder: text; balance: nat};
		// type TopHolderResponse = record {top_accounts: vec HolderBalance; top_principals: vec HolderBalance}; 

		dataLoaded = true;
	}

	function nanoToDate(epochNanoseconds){
		const milliseconds = epochNanoseconds / 1000000; // Convert nanoseconds to milliseconds
		const date = new Date(milliseconds);
		let isoDate = date.toISOString();
		let opDate = isoDate;
		let ind = isoDate.indexOf("T");
		let short = isoDate.substring(ind + 1, ind + 6);
		let dateOnly = isoDate.substring(0,10);
		let ret = {
			fullDateTime: opDate,
			shortTime: short,
			dateOnly: dateOnly, 
		}
		return ret;
	}

</script>
<svelte:head>
	<title>Stats : ckBTC</title>
	<meta name="description" content="Home for the fam" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<ContentBox type="standard-shaddow-black">
			<div style="padding: 10px;">
				<table style="width: 100%">
					<tr>
						<td style="width: 70px;">
							<img class="headAlign" src={ckBTCLogo} alt="ckBTC Logo" width="50px"/>
						</td>
						<td>
							<p style="font-size: 30px; padding-top:10px;">ckBTC</p>
						</td>
					</tr>
				</table>
			</div>
		</ContentBox>

		<!-- tx over time -->
		<ContentBox type={"standard-shaddow-black"}>
			{#if dataLoaded == false}
				<Loading/>
			{:else}
				<TxOverTime hourlyData={hourlyExport} dailyData={dailyExport} hours={hourLen} days={dayLen} token={"ckBTC"}/>
			{/if}
		</ContentBox>

		<!-- largest holders -->
		<!-- <ContentBox type={"standard-shaddow-black"}>
			{#if dataLoaded == false}
				<Loading/>
			{:else}
				<TopHolderTable dataHourly={thDataHourly} dataDaily={thDataDaily} is_icrc={true}/>
			{/if}
		</ContentBox> -->
	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	
</style>
