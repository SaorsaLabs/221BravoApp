<script>
	import LayoutCombine from "../../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../../lib/componants/head.svelte";
	import Footer from "../../../../lib/componants/footer.svelte";
	import ContentBox from "../../../../lib/shared/contentBox.svelte";
	import TxOverTime from "../../../../lib/componants/statsComponsnts/txOverTime.svelte";
	import { 
		getICRC_Stats, 
		getICRC_TopHolders, 
		processTopTxData,
		processTopMintData,
		processTopBurnData,
		getICRC_TotalSupply } from '../../../../lib/code/fetchStats.js';
	import Loading from "../../../../lib/shared/loading.svelte";
	import projectLogo from '$lib/images/projectLogos/IcxLogo.png';
    import TopHolderTable from "../../../../lib/componants/statsComponsnts/topHolderTable.svelte";
    import HeadCards from "../../../../lib/componants/statsComponsnts/headCards.svelte";
	import { ICX_decimals } from "../../../../lib/code/constants";
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

	let decimals  = ICX_decimals;
    let vPower = 1/Math.pow(10, decimals);
	let token = "ICX";

	let socials = [
		{platform: "twitter", url: "https://twitter.com/boomdaosns"},
		{platform: "openchat", url: "https://oc.app/community/z6b5f-6iaaa-aaaar-asvvq-cai/?ref=xp567-tqaaa-aaaar-as7hq-cai"},
	];
	let canisterDashboard = "https://dashboard.internetcomputer.org/sns/u67kc-jyaaa-aaaaq-aabpq-cai";
	let whitepaperURL = "";
	let githubURL = "";
	let teamWebsite = "https://icx.one/";
	let marketplace = "";
	let marketplace2 = "";
	let marketplace3 = "";

	let promise = getStats();
	let hourlyData;
	let dailyData;	
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
	let topPrincipals = [];
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

	async function getStats(){
		let i;
		// HOURLY STATS
		hourlyData = await getICRC_Stats(token, "hourly");

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
		topTXhourly = processTopTxData(hourlyData.top_transactions,decimals, token);
		topMintHourly = processTopMintData(hourlyData.top_mints, decimals, token);
		topBurnHourly = processTopBurnData(hourlyData.top_burns, decimals, token);

		// WEEKLY STATS
		dailyData = await getICRC_Stats(token, "daily");
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
		topTXdaily = processTopTxData(dailyData.top_transactions,decimals, token);
		topMintDaily = processTopMintData(dailyData.top_mints, decimals, token);
		topBurnDaily =processTopBurnData(dailyData.top_burns, decimals, token);

		// top holders
		topAll = await getICRC_TopHolders(token, 100);
		topAccounts = topAll.topAccounts;
		topPrincipals = topAll.topPrincipals;

		// outputs
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
			hourlyActivePrincipals: hourlyData.total_unique_principals,
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
			dailyActivePrincipals: dailyData.total_unique_principals,
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
		totSupply = await getICRC_TotalSupply(token);
		supChangeD = dailyExport.dailyMintValue - dailyExport.dailyBurnValue;
		supChangeH = hourlyExport.hourlyMintValue - hourlyExport.hourlyBurnValue;
		supChangePctD = (supChangeD/(totSupply-supChangeD))*100;
		supChangePctH = (supChangeH/(totSupply-supChangeH))*100;
		supChangeD = supChangeD.toLocaleString('en-US', fmtOptionsValue);
		supChangePctH = supChangePctH.toLocaleString('en-US', fmtOptionsValue);
		supChangeH = supChangeH.toLocaleString('en-US', fmtOptionsValue);
		supChangePctD = supChangePctD.toLocaleString('en-US', fmtOptionsValue);
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

	// Top Holders
	let acBtnColour = "orange";
	let prBtnColour = "blueTP";
	let showTopHolder = "accounts";
	function toggleAccount(){
		acBtnColour = "orange";
		prBtnColour = "blueTP";
		showTopHolder = "accounts";
	}
	function togglePrincipal(){
		acBtnColour = "blueTP";
		prBtnColour = "orange";
		showTopHolder = "principals";
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
		<StatsSubHead selected="11" />
	</span>

	<span slot="body">
		<ContentBox type="standard-shaddow-black">
			<div style="padding: 10px;">
				<table style="width: 100%">
					<tr>
						<td style="width: 70px;">
							<img class="headAlign" src={projectLogo} alt="Chat Logo" width="50px"/>
						</td>
						<td >
							<p class="tokenTextHead">{token}</p>
						</td>
						<td class="box" style="width: 100%">
							<HeadCards 
								token={token} 
								tradePair={"MOD/ICP"} 
								searchURL={"/search/icx"} 
								tradeURL={"https://iclight.io/ICDex/MOD/ICP"}
								is_icrc={true}
								quoteCurrency={"icp"}
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
					is_icrc={true} 
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
					<TopTxTable txData={topTXhourly} is_icrc={true}/>	
				{:else}
					<TopTxTable txData={topTXdaily} is_icrc={true}/>	
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
									<MintBurnSideBar data={mintBurnExport} token={token}/>
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
								<MintBurnSideBar data={mintBurnExport} token={token}/>
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
					<table>
						<tr>
							<td>
								<div>
									<h4 style="padding-left:5px; padding-top:5px;">{token} Top Holders: </h4>
								</div>
							</td>
							<td>
								<div style="padding-top: 5px; padding-left: 15px;">		
									<Button slim={true} type={acBtnColour} on:click={()=>{toggleAccount()}}>Single Accounts</Button>
									<Button slim={true} type={prBtnColour} on:click={()=>{togglePrincipal()}}>Principals (All Sub-Accounts)</Button>
								</div>
							</td>
						</tr>
					</table>
					{#if showTopHolder == "accounts"}
						<TopHolderTable data={topAccounts} isIcrc={true} showSubAccounts={true} token={token}/>
					{:else}
						<TopHolderTable data={topPrincipals} isIcrc={true} showSubAccounts={false} token={token}/>
					{/if}
				{/await}
			</ContentBox>
		{/if}


		<!-- Daily Active Accounts (Snapshot Canister) -->
		{#if LS == true || LS == "true"} 
			<ContentBox type={"standard-shaddow-black"} style="">
				{#await promise}
					<Loading/>
				{:then}
					<!-- Note name on snapshot canister is not in CAPS -->
					<DailyActive token={"ICX"} is_icrc={true}/>
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
