<script>
	import LayoutCombine from "../../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../../lib/componants/head.svelte";
	import Footer from "../../../../lib/componants/footer.svelte";
	import ContentBox from "../../../../lib/shared/contentBox.svelte";
	import TxOverTime from "../../../../lib/componants/statsComponsnts/txOverTime.svelte";
	import { 
		getICP_Stats, 
		getICP_TopHolders, 
		processICPTopTxData,
		processICPTopMintData,
		processICPTopBurnData,
		getICP_TotalSupply } from '../../../../lib/code/fetchStats.js';
	import Loading from "../../../../lib/shared/loading.svelte";
	import icpLogo from '$lib/images/icpLogo.png';
    import TopHolderTable from "../../../../lib/componants/statsComponsnts/topHolderTable.svelte";
    import HeadCards from "../../../../lib/componants/statsComponsnts/headCards.svelte";
	import { ICP_decimals } from "../../../../lib/code/constants";
	import Button from "../../../../lib/shared/button.svelte";
	import TopTxTable from "../../../../lib/componants/statsComponsnts/topTxTable.svelte";
	import MintBurnDonutChart from "../../../../lib/componants/charts/mintBurnDonutChart.svelte";
	import MintBurnSideBar from "../../../../lib/componants/statsComponsnts/mintBurnSideBar.svelte";
	import HiddenContent from "../../../../lib/componants/hiddenContent.svelte";
	import { authStore, authTrigger } from "../../../../lib/stores/authStore";
	import { browser } from '$app/environment';
	import DailyActive from "../../../../lib/componants/statsComponsnts/dailyActive.svelte";
	import WebLinksTable from "../../../../lib/componants/statsComponsnts/webLinksTable.svelte";
	import SnsCarousel from "../../../../lib/componants/SNSCarousel.svelte";
	import StatsSubHead from "../../../../lib/componants/statsComponsnts/statsSubHead.svelte";
	
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

	let decimals  = ICP_decimals;
    let vPower = 1/Math.pow(10, decimals);
	let token = "ICP";

	let socials = [
		{platform: "twitter", url: "https://twitter.com/dfinity"},
	];
	let canisterDashboard = "https://dashboard.internetcomputer.org/";
	let whitepaperURL = "https://internetcomputer.org/whitepaper.pdf";
	let githubURL = "https://github.com/dfinity";
	let teamWebsite = "https://dfinity.org/";
	let marketplace = "";
	let marketplace2 = "";
	let marketplace3 = "";

	let hourlyData;
	let dailyData;	
	let promise = getStats();
	let dataLoaded = false;
	const fmtOptions = { style: 'decimal', maximumFractionDigits: 2, minimumFractionDigits: 0 };
	const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };

	// Stats
	let hourlyOTdata = [];
	let hourlyOTlabels = [];
	let dailyOTdata = [];
	let dailyOTlabels = [];
	let hourLen, dayLen;
	let hourlyExport = {};
	let dailyExport = {};
	let topAll = {};
	let topAccounts = [];
	let topTXhourly = [];
	let topTXdaily = [];
	let mBData = [];
	let topBurnHourly = [];
	let topMintHourly = [];
	let topBurnDaily = [];
	let topMintDaily = [];
	let mintBurnExport = {};
	let valMintH, valBurnH, numMintH, numBurnH;
	let valMintD, valBurnD, numMintD, numBurnD;
	let totSupply;
	let supChangeH, supChangeD;
	let supChangePctH, supChangePctD;

	// async function getStats(){}
	async function getStats(){
		let i;
		// HOURLY STATS
		hourlyData = await getICP_Stats("hourly");
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
		topTXhourly = processICPTopTxData(hourlyData.top_transactions,decimals, token);
		topMintHourly = processICPTopMintData(hourlyData.top_mints, decimals, token);
		topBurnHourly = processICPTopBurnData(hourlyData.top_burns, decimals, token);

		// WEEKLY STATS
		dailyData = await getICP_Stats("daily");
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
		topTXdaily = processICPTopTxData(dailyData.top_transactions,decimals, token);
		topMintDaily = processICPTopMintData(dailyData.top_mints, decimals, token);
		topBurnDaily =processICPTopBurnData(dailyData.top_burns, decimals, token);

		// top holders
		topAll = await getICP_TopHolders(100);
		topAccounts = topAll.topAccounts;

		hourlyExport = {
			hourlyOTdata: hourlyOTdata.reverse(),
			hourlyOTlabels: hourlyOTlabels.reverse(),
			hourlyTotalCount: Number(hourlyData.total_transaction_count),
			hourlyTotalValue: Number(hourlyData.total_transaction_value)*vPower,
			hourlyTotalMint: Number(hourlyData.mint_stats.count),
			hourlyMintValue: Number(hourlyData.mint_stats.total_value)*vPower,
			hourlyTotalBurn: Number(hourlyData.burn_stats.count),
			hourlyBurnValue: Number(hourlyData.burn_stats.total_value)*vPower,
			hourlyActiveAccounts: hourlyData.total_unique_accounts,
			hourlyActivePrincipals: 0,
		};

		dailyExport = {
			dailyOTdata: dailyOTdata.reverse(),
			dailyOTlabels: dailyOTlabels.reverse(),
			dailyTotalCount: Number(dailyData.total_transaction_count),
			dailyTotalValue: Number(dailyData.total_transaction_value)*vPower,
			dailyTotalMint: Number(dailyData.mint_stats.count),
			dailyMintValue: Number(dailyData.mint_stats.total_value)*vPower,
			dailyTotalBurn: Number(dailyData.burn_stats.count),
			dailyBurnValue: Number(dailyData.burn_stats.total_value)*vPower,
			dailyActiveAccounts: dailyData.total_unique_accounts,
			dailyActivePrincipals: 0,
		};

		// mint/ burn init (Daily)
		mBData[0] =Number(dailyData.mint_stats.total_value)*vPower;
		mBData[1] = Number(dailyData.burn_stats.total_value)*vPower;
		mBData[2] = Number(dailyData.transaction_stats.total_value)*vPower;
		valMintH = hourlyExport.hourlyMintValue.toLocaleString('en-US', fmtOptionsValue);
		valBurnH = hourlyExport.hourlyBurnValue.toLocaleString('en-US', fmtOptionsValue);
		numMintH = hourlyExport.hourlyTotalMint.toLocaleString('en-US', fmtOptions);
		numBurnH = hourlyExport.hourlyTotalBurn.toLocaleString('en-US', fmtOptions);
		valMintD = dailyExport.dailyMintValue.toLocaleString('en-US', fmtOptionsValue);
		valBurnD = dailyExport.dailyBurnValue.toLocaleString('en-US', fmtOptionsValue);
		numMintD = dailyExport.dailyTotalMint.toLocaleString('en-US', fmtOptions);
		numBurnD = dailyExport.dailyTotalBurn.toLocaleString('en-US', fmtOptions);
		totSupply = await getICP_TotalSupply();
		supChangeD = dailyExport.dailyMintValue - dailyExport.dailyBurnValue;
		supChangeH = hourlyExport.hourlyMintValue - hourlyExport.hourlyBurnValue;
		supChangePctD = (supChangeD/(totSupply-supChangeD))*100;
		supChangePctH = (supChangeH/(totSupply-supChangeH))*100;
		supChangePctD = supChangePctD.toLocaleString('en-US', fmtOptionsValue);
		supChangePctH = supChangePctH.toLocaleString('en-US', fmtOptionsValue);
		supChangeD = supChangeD.toLocaleString('en-US', fmtOptionsValue);
		supChangeH = supChangeH.toLocaleString('en-US', fmtOptionsValue);
		totSupply = totSupply.toLocaleString('en-US', fmtOptions);
		mintBurnExport = {
			mint: topMintDaily,
			burn: topBurnDaily,
		};

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

	// TopTX
	let ttHBtnColour = "blueTP";
	let ttDBtnColour = "orange";
	let showTTX = "daily";
	function toggleTopTxHr(){
		ttHBtnColour = "orange";
		ttDBtnColour = "blueTP";
		showTTX = "hourly";
	}
	function toggleTopTxDy(){
		ttHBtnColour = "blueTP";
		ttDBtnColour = "orange";
		showTTX = "daily";
	}

	// Mint/ Burn
	let mBLabels = ["Mint", "Burn", "Transfer"];
	let mbHBtnColour = "blueTP";
	let mbDBtnColour = "orange";
	let showMB = "daily";
	function toggleMbHr(){
		mbHBtnColour = "orange";
		mbDBtnColour = "blueTP";
		showMB = "hourly";
		mBData[0] = Number(hourlyData.mint_stats.total_value)*vPower;
		mBData[1] = Number(hourlyData.burn_stats.total_value)*vPower;
		mBData[2] = Number(hourlyData.transaction_stats.total_value)*vPower;
		mintBurnExport = {
			mint: topMintHourly,
			burn: topBurnHourly,
		};
	}
	function toggleMbDy(){
		mbHBtnColour = "blueTP";
		mbDBtnColour = "orange";
		showMB = "daily";
		mBData[0] = Number(dailyData.mint_stats.total_value)*vPower;
		mBData[1] = Number(dailyData.burn_stats.total_value)*vPower;
		mBData[2] = Number(dailyData.transaction_stats.total_value)*vPower;
		mintBurnExport = {
			mint: topMintDaily,
			burn: topBurnDaily,
		};
	}

</script>
<svelte:head>
	<title>Stats : {token}</title>
	<meta name="description" content="Internet Computer Statistics" />
</svelte:head>

<LayoutCombine>
	<span slot="head">
		<Head/>
		<StatsSubHead selected="1" />
	</span>

	<span slot="body">
		<ContentBox type="standard-shaddow-black">
			<div style="padding: 10px;">
				<table style="width: 100%">
					<tr>
						<td style="width: 70px;">
							<img class="headAlign" src={icpLogo} alt="ckBTC Logo" width="50px"/>
						</td>
						<td >
							<p class="tokenTextHead">{token}</p>
						</td>
						<td class="box" style="width: 100%">
							<HeadCards 
								is_icrc={false} 
								token={token} 
								tradePair={"ICP/USD"} 
								searchURL={"/search/icp"} 
								tradeURL={"https://iclight.io/ICDex/ckBTC/ICP"}
								quoteCurrency = {"usd"}
							/>
						</td>
					</tr>
				</table>
			</div>
		</ContentBox>

		<!-- tx over time -->
		<ContentBox type={"standard-shaddow-black"}>
			{#await promise}
				<Loading/>
			{:then}
				<TxOverTime 
					is_icrc={false} 
					hourlyData={hourlyExport} 
					dailyData={dailyExport} 
					hours={hourLen} 
					days={dayLen} 
					token={token}
				/>
			{/await}
		</ContentBox>

		<!-- Whale Moves -->
		<ContentBox type={"standard-shaddow-black"}>
			{#await promise}
				<Loading/>
			{:then}
				<table>
					<tr>
						<td>
							<div>
								<h4 style="padding-left:5px; padding-top:5px;">{token} Largest Transactions: </h4>
							</div>
						</td>
						<td>
							<div style="padding-top: 5px; padding-left: 15px;">		
								<Button slim={true} type={ttHBtnColour} on:click={()=>{toggleTopTxHr()}}>{hourLen} hours</Button>
								<Button slim={true} type={ttDBtnColour} on:click={()=>{toggleTopTxDy()}}>{dayLen} days</Button>
							</div>
						</td>
					</tr>
				</table>


				{#if showTTX == "hourly"}
					<TopTxTable txData={topTXhourly} is_icrc={false}/>	
				{:else}
					<TopTxTable txData={topTXdaily} is_icrc={false}/>	
				{/if}
			{/await}
		</ContentBox>

		<!-- Mint/ Burn Ratio -->
		<ContentBox type={"standard-shaddow-black"}>
			<div style="padding-bottom:25px">
				{#await promise}
				<Loading/>
				{:then}
					<table>
						<tr>
							<td>
								<div>
									<h4 style="padding-left:5px; padding-top:5px;">{token} Minting/ Burning Stats: </h4>
								</div>
							</td>
							<td>
								<div style="padding-top: 5px; padding-left: 15px;">		
									<Button slim={true} type={mbHBtnColour} on:click={()=>{toggleMbHr()}}>{hourLen} hours</Button>
									<Button slim={true} type={mbDBtnColour} on:click={()=>{toggleMbDy()}}>{dayLen} days</Button>
								</div>
							</td>
						</tr>
					</table>
					{#if showMB == "hourly"}
						<table style="width: 100%; min-width:300px; padding-bottom:10px;">
							<tr>
								<td style="width:33.3%;" class="mBContentCentre">
									<h5 style="padding-top:5px; color:chartreuse">Total Mints</h5>
									{numMintH} Transactions
									{@html "<br>"}
									{valMintH} {token}

									<h5 style="padding-top:15px; color:orangered">Total Burns</h5>
									{numBurnH} Transactions
									{@html "<br>"}
									{valBurnH} {token}

									<h5 style="padding-top:15px; color:aqua">Change in Supply</h5>
									{supChangeH} {token}
									{@html "<br>"}
									{supChangePctH} %
								</td>

								<td style="width:33.3%" class="mBContentCentre">
									<MintBurnDonutChart dataArray={mBData} labelsArray={mBLabels} datasetTitle={"Total Volume by Type"} token={token}/>
								</td>
								<td style="width:33.3%" class="mBContentRight">
									<MintBurnSideBar data={mintBurnExport} token={token} is_icrc={false}/>
								</td>
							</tr>
						</table>
					{:else}
					<table style="width: 100%;">
						<tr>
							<td style="width:33.3%" class="mBContentCentre">
									<h5 style="padding-top:5px; color:chartreuse">Total Mints</h5>
									{numMintD} Transactions
									{@html "<br>"}
									{valMintD} {token}

									<h5 style="padding-top:15px; color:orangered">Total Burns</h5>
									{numBurnD} Transactions
									{@html "<br>"}
									{valBurnD} {token}

									<h5 style="padding-top:15px; color:aqua">Change in Supply</h5>
									{supChangeD} {token}
									{@html "<br>"}
									{supChangePctD} %
							</td>
							<td style="width:33.3%" class="mBContentCentre">
								<MintBurnDonutChart dataArray={mBData} labelsArray={mBLabels} datasetTitle={"Total Volume by Type"} token={token}/>
							</td>
							<td style="width:33.3%" class="mBContentRight">
								<MintBurnSideBar data={mintBurnExport} token={token} is_icrc={false}/>
							</td>
						</tr>
					</table>
					{/if}
				{/await}
			</div>
		</ContentBox>
		
		{#if LS == false || LS == "false"} 
			<ContentBox type={"standard-shaddow-black"}>
				<div style="padding:5px;"> 
					<HiddenContent>Become a member to see top {token} holders</HiddenContent>
				</div>
			</ContentBox>
			{:else}
			<!-- largest holders -->
			<ContentBox type={"standard-shaddow-black"}>
				{#await promise}
					<Loading/>
				{:then}
					<h4 style="padding-left:5px; padding-top:5px;">{token} Top Holders: </h4>
					<TopHolderTable data={topAccounts} isIcrc={false} showSubAccounts={true} token={token}/>
				{/await}
			</ContentBox>
		{/if}

			<!-- Daily Active Accounts (Snapshot Canister) -->
			{#if LS == true || LS == "true"} 
			<ContentBox type={"standard-shaddow-black"} style="">
				{#await promise}
					<Loading/>
				{:then}
					<DailyActive token={token} is_icrc={false}/>
				{/await}
			</ContentBox>
		{/if}
		
		<ContentBox type={"standard-shaddow-black"} style="">
			<WebLinksTable 
				SocialAR={socials}
				 whitepaperURL={whitepaperURL}
 				githubURL={githubURL}
 				teamWebsiteURL={teamWebsite}
 				marketplaceURL={marketplace}
				marketplaceURL2={marketplace2}
				marketplaceURL3={marketplace3}
 				canisterDashboardURL={canisterDashboard}
			/>
		</ContentBox>

		<ContentBox type={"standard-shaddow-black"} style="">
			<SnsCarousel/>
		</ContentBox>


	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	.box{
		border-color: rgb(25, 89, 89);
		border-style: solid;
		border-radius: 5px;
		border-width: 1px;
		padding: 5px;
	}
	.mBContentLeft{
		text-align: left;
		padding-left: 10px;
	}
	.mBContentCentre{
		text-align: center;
	}
	.mBContentRight{
		text-align: center;
		padding-right: 10px;
	}
	.tokenTextHead{
		font-size: 30px;
		padding-top:18px;
		padding-left: 10px;
		padding-right: 15px;
	}
</style>
