<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
  	import ContentBox from "../../../lib/shared/contentBox.svelte";
	import {authStore, authTrigger} from "../../../lib/stores/authStore.js";
	import { browser } from '$app/environment';
	import SubHead from "../../../lib/shared/subHead.svelte";
	import Loading from "../../../lib/shared/loading.svelte";
	import { getExchangeBalances, getLiveExchangeBalances } from '../../../lib/code/fetchStats.js'
  	import { ICP_decimals } from "../../../lib/code/constants";
	import MembersCharts from "../../../lib/componants/members/membersCharts.svelte";

	let LS = false;
	let screenSize; 
    // Logged In? 
	let usr;
    authTrigger.subscribe(value =>{
        if(browser){
            if(value>=1){
                usr = authStore.read();
                LS = usr.data.loggedIn;
            }
        }
    });

	let decimals  = ICP_decimals;
    let vPower = 1/Math.pow(10, decimals);

	let data = null;
	let promise = loadStuff();
	let binanceChange24, binanceTrans24, binanceChange14, binanceTrans14;
	let coinbaseChange24, coinbaseTrans24, coinbaseChange14, coinbaseTrans14;
	let kucoinChange24, kucoinTrans24, kucoinChange14, kucoinTrans14;
	let krakenChange24, krakenTrans24, krakenChange14, krakenTrans14;
	let gateChange24, gateTrans24, gateChange14, gateTrans14; 
	let bitfinexChange24, bitfinexTrans24, bitfinexChange14, bitfinexTrans14;
	let huobiChange24, huobiTrans24, huobiChange14, huobiTrans14;

	let totalChange24, totalChange14; 
	let tc24, tc24_2, tc24_14, tc24_2_14;
	let chartChange = [];
	let chartTotal = [];
	let labels = [];

	let chart1Colours = ['rgb(255, 217, 25)'];
	let chart2Colours = ['rgb(75, 192, 192)'];
	let labels2 = [];

	async function loadStuff(){
		let data = await getExchangeBalances(30);
		let now = await getLiveExchangeBalances();

		// binance
		binanceChange24 = changeCalculation(
			now.binance.total_balance, 
			data[0][1].binance.total_balance,
			vPower );
		binanceTrans24 = changeCalculation(
			now.binance.num_transactions, 
			data[0][1].binance.num_transactions,
			1 );

		// coinbase
		coinbaseChange24 = changeCalculation(
			now.coinbase.total_balance, 
			data[0][1].coinbase.total_balance,
			vPower );
		coinbaseTrans24 = changeCalculation(
			now.coinbase.num_transactions, 
			data[0][1].coinbase.num_transactions,
			1 );
		
		// kucoin
		kucoinChange24 = changeCalculation(
			now.kucoin.total_balance, 
			data[0][1].kucoin.total_balance,
			vPower );
		kucoinTrans24 = changeCalculation(
			now.kucoin.num_transactions, 
			data[0][1].kucoin.num_transactions,
			1 );		

		// kraken
		krakenChange24 = changeCalculation(
			now.kraken.total_balance, 
			data[0][1].kraken.total_balance,
			vPower );
		krakenTrans24 = changeCalculation(
			now.kraken.num_transactions, 
			data[0][1].kraken.num_transactions,
			1 );
			
		// gate
		gateChange24 = changeCalculation(
			now.gate.total_balance, 
			data[0][1].gate.total_balance,
			vPower );
		gateTrans24 = changeCalculation(
			now.gate.num_transactions, 
			data[0][1].gate.num_transactions,
			1 );

		// bitfinex
		bitfinexChange24 = changeCalculation(
			now.bitfinex.total_balance, 
			data[0][1].bitfinex.total_balance,
			vPower );
		bitfinexTrans24 = changeCalculation(
			now.bitfinex.num_transactions, 
			data[0][1].bitfinex.num_transactions,
			1 );

		// huobi
		huobiChange24 = changeCalculation(
			now.huobi.total_balance, 
			data[0][1].huobi.total_balance,
			vPower );
		huobiTrans24 = changeCalculation(
			now.huobi.num_transactions, 
			data[0][1].huobi.num_transactions,
			1 );

		tc24 = (
			now.binance.total_balance +
			now.coinbase.total_balance +
			now.kucoin.total_balance + 
			now.kraken.total_balance + 
			now.gate.total_balance + 
			now.bitfinex.total_balance + 
			now.huobi.total_balance
		); 

		tc24_2 = (
			data[0][1].binance.total_balance +
			data[0][1].coinbase.total_balance +
			data[0][1].kucoin.total_balance + 
			data[0][1].kraken.total_balance + 
			data[0][1].gate.total_balance + 
			data[0][1].bitfinex.total_balance + 
			data[0][1].huobi.total_balance
		); 
		totalChange24 =changeCalculation( tc24, tc24_2, vPower);

		// 14 DAY STATS
		// binance
		binanceChange14 = changeCalculation(
			now.binance.total_balance, 
			data[0][7].binance.total_balance,
			vPower );
		binanceTrans14 = changeCalculation(
			now.binance.num_transactions, 
			data[0][7].binance.num_transactions,
			1 );

		// coinbase
		coinbaseChange14 = changeCalculation(
			now.coinbase.total_balance, 
			data[0][7].coinbase.total_balance,
			vPower );
		coinbaseTrans14 = changeCalculation(
			now.coinbase.num_transactions, 
			data[0][7].coinbase.num_transactions,
			1 );
		
		// kucoin
		kucoinChange14 = changeCalculation(
			now.kucoin.total_balance, 
			data[0][7].kucoin.total_balance,
			vPower );
		kucoinTrans14 = changeCalculation(
			now.kucoin.num_transactions, 
			data[0][7].kucoin.num_transactions,
			1 );		

		// kraken
		krakenChange14 = changeCalculation(
			now.kraken.total_balance, 
			data[0][7].kraken.total_balance,
			vPower );
		krakenTrans14 = changeCalculation(
			now.kraken.num_transactions, 
			data[0][7].kraken.num_transactions,
			1 );
			
		// gate
		gateChange14 = changeCalculation(
			now.gate.total_balance, 
			data[0][7].gate.total_balance,
			vPower );
		gateTrans14 = changeCalculation(
			now.gate.num_transactions, 
			data[0][7].gate.num_transactions,
			1 );

		// bitfinex
		bitfinexChange14 = changeCalculation(
			now.bitfinex.total_balance, 
			data[0][7].bitfinex.total_balance,
			vPower );
		bitfinexTrans14 = changeCalculation(
			now.bitfinex.num_transactions, 
			data[0][7].bitfinex.num_transactions,
			1 );

		// huobi
		huobiChange14 = changeCalculation(
			now.huobi.total_balance, 
			data[0][7].huobi.total_balance,
			vPower );
		huobiTrans14 = changeCalculation(
			now.huobi.num_transactions, 
			data[0][7].huobi.num_transactions,
			1 );

		tc24_14 = (
			now.binance.total_balance +
			now.coinbase.total_balance +
			now.kucoin.total_balance + 
			now.kraken.total_balance + 
			now.gate.total_balance + 
			now.bitfinex.total_balance + 
			now.huobi.total_balance
		); 

		tc24_2_14 = (
			data[0][7].binance.total_balance +
			data[0][7].coinbase.total_balance +
			data[0][7].kucoin.total_balance + 
			data[0][7].kraken.total_balance + 
			data[0][7].gate.total_balance + 
			data[0][7].bitfinex.total_balance + 
			data[0][7].huobi.total_balance
		); 
		totalChange14 =changeCalculation( tc24_14, tc24_2_14, vPower);


		/// CHART DATA 

		let dataLen = data[0].length ?? 0;
		let cng, cng2;
		let adj = 0;
		let date, shortDate;
		let currentDate = new Date();

		for(let i=0; i<dataLen-1; i++){
			labels.push(currentDate.toISOString().slice(0, 10));
    		currentDate.setTime(currentDate.getTime() - 86400000); 
            
			adj += 86400000;
			cng = (
				data[0][i].binance.total_balance +
				data[0][i].coinbase.total_balance +
				data[0][i].kucoin.total_balance + 
				data[0][i].kraken.total_balance + 
				data[0][i].gate.total_balance + 
				data[0][i].bitfinex.total_balance + 
				data[0][i].huobi.total_balance
			); 
			cng2 = (
				data[0][i+1].binance.total_balance +
				data[0][i+1].coinbase.total_balance +
				data[0][i+1].kucoin.total_balance + 
				data[0][i+1].kraken.total_balance + 
				data[0][i+1].gate.total_balance + 
				data[0][i+1].bitfinex.total_balance + 
				data[0][i+1].huobi.total_balance
			);
			chartChange.push(changeCalculation( cng, cng2, vPower).percent);
			if (i == 0){
				chartTotal.push(Number(tc24)*vPower);
			} else {
				chartTotal.push(Number(cng)*vPower);
			}
		}
		labels.reverse();
	}

	function changeCalculation(t0, t1, vPow){
		let v = (Number(t0) - Number(t1))*vPow;
		let p = ((Number(t0) - Number(t1)) / Number(t1)) *100;

		v = v.toLocaleString('en-US');
		p = p.toFixed(4);
		let ret = {
			value: v, 
			percent: p
		}
		return ret;
	}

</script>
<!-- SCREEN SIZE FOR VID SWAP -->
<svelte:window bind:innerWidth={screenSize} />

<svelte:head>
	<title>Exchange Balances</title>
	<meta name="description" content="Exchange Stats - Internet Computer Protocol" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		{#if LS == true || LS == "true"}
			<SubHead>
				<span class="headText"> Members Only - Exchange Balances </span>
			</SubHead>
		{:else}
			<SubHead/>
		{/if}

	</span>

	<span slot="body">

		{#if LS == true || LS == 'true'}
			{#await promise}
						<Loading/>
					{:then}
					<ContentBox type={"standard-shaddow-black"} style="text">
						
						<div style="padding: 5px; padding-bottom: 10px">
							<h4>Exchange Balances (still building here!)</h4>
							
							{@html "<br>"}
							
							<table class="tbl" style="width: 100%;">
								<thead>
									<tr>
										<th>24hr Balance Change (ICP)</th>
										<th>24hr % Change</th>
										<th>7d Balance Change (ICP) </th>
										<th>7d % Change</th>
									</tr>
								</thead>
								<tbody>
									<tr>
										<td> <h3> {totalChange24.value} </h3> </td> 
										<td> <h3> {totalChange24.percent} %</h3> </td>
										<td> <h3> {totalChange14.value} </h3> </td> 
										<td> <h3> {totalChange14.percent} %</h3> </td> 
									</tr>
								</tbody>
							</table>

							{@html "<br>"}

							<table class="tbl" style="width: 100%;">
								<!-- Headers -->
								<thead>
									<tr>
										<th>Exchange</th>
										<th>24hr Balance Change</th>
										<th>24hr % Change</th>
										<th>7d Balance Change</th>
										<th>7d % Change</th>
									</tr>
								</thead>
							
								<!-- Rows -->
								<tbody>
									<tr>
										<td>Binance</td>  
										<td>{binanceChange24.value} {@html "<br>"} {binanceTrans24.value} Txs </td> 
										<td>{binanceChange24.percent} % {@html "<br>"} {binanceTrans24.percent} Tx %</td>
										<td>{binanceChange14.value} {@html "<br>"} {binanceTrans14.value} Txs </td> 
										<td>{binanceChange14.percent} % {@html "<br>"} {binanceTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Coinbase</td>  
										<td>{coinbaseChange24.value} {@html "<br>"} {coinbaseTrans24.value} Txs </td> 
										<td>{coinbaseChange24.percent} % {@html "<br>"} {coinbaseTrans24.percent} Tx %</td>
										<td>{coinbaseChange14.value} {@html "<br>"} {coinbaseTrans14.value} Txs </td> 
										<td>{coinbaseChange14.percent} % {@html "<br>"} {coinbaseTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Kucoin</td> 
										<td>{kucoinChange24.value} {@html "<br>"} {kucoinTrans24.value} Txs </td> 
										<td>{kucoinChange24.percent} % {@html "<br>"} {kucoinTrans24.percent} Tx %</td>
										<td>{kucoinChange14.value} {@html "<br>"} {kucoinTrans14.value} Txs </td> 
										<td>{kucoinChange14.percent} % {@html "<br>"} {kucoinTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Kraken</td> 
										<td>{krakenChange24.value} {@html "<br>"} {krakenTrans24.value} Txs </td> 
										<td>{krakenChange24.percent} % {@html "<br>"} {krakenTrans24.percent} Tx %</td>
										<td>{krakenChange14.value} {@html "<br>"} {krakenTrans14.value} Txs </td> 
										<td>{krakenChange14.percent} % {@html "<br>"} {krakenTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Gate.io</td> 
										<td>{gateChange24.value} {@html "<br>"} {gateTrans24.value} Txs </td> 
										<td>{gateChange24.percent} % {@html "<br>"} {gateTrans24.percent} Tx %</td>
										<td>{gateChange14.value} {@html "<br>"} {gateTrans14.value} Txs </td> 
										<td>{gateChange14.percent} % {@html "<br>"} {gateTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Bitfinex</td> 
										<td>{bitfinexChange24.value} {@html "<br>"} {bitfinexTrans24.value} Txs </td> 
										<td>{bitfinexChange24.percent} % {@html "<br>"} {bitfinexTrans24.percent} Tx %</td>
										<td>{bitfinexChange14.value} {@html "<br>"} {bitfinexTrans14.value} Txs </td> 
										<td>{bitfinexChange14.percent} % {@html "<br>"} {bitfinexTrans14.percent} Tx %</td>
									</tr>
									<tr>
										<td>Huobi</td> 
										<td>{huobiChange24.value} {@html "<br>"} {huobiTrans24.value} Txs </td> 
										<td>{huobiChange24.percent} % {@html "<br>"} {huobiTrans24.percent} Tx %</td>
										<td>{huobiChange14.value} {@html "<br>"} {huobiTrans14.value} Txs </td> 
										<td>{huobiChange14.percent} % {@html "<br>"} {huobiTrans14.percent} Tx %</td>
									</tr>
								</tbody>
							</table>
						</div>
					</ContentBox>

					<ContentBox type={"standard-shaddow-black"} style="text">
						<MembersCharts
							dataArray1={chartChange}
							labelsArray={labels}
							title={"Daily Balance Change %"}
							chartLines={1}
							colours={chart1Colours}
							reverse={true}
						/>
					</ContentBox>
					
					<ContentBox type={"standard-shaddow-black"} style="text">
						<MembersCharts
							dataArray1={chartTotal}
							labelsArray={labels}
							title={"Daily Balance (ICP)"}
							chartLines={1}
							colours={chart2Colours}
							reverse={true}
						/>
					</ContentBox>
			{/await}
		{:else}
		<!-- NOT A MEMBER -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text"> <b> Become a Member to access this page </b></h3>
			</div>
		</ContentBox>
		{/if}
	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	p {
    	color: azure;
	}
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
	.gradient-text {
		/* font-size: 3em; */
		/* background: linear-gradient(to right, #a83caf, #09cbf1); */
		background: linear-gradient(to top, #a49257, #e3e37b);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
	}
	a {
		color: aliceblue;
		text-decoration: none;
	}
	table, thead, tbody, td, tr, th {
		border-width: 1px;
		border-radius: 3px;
		padding-left: 5px;
	}
	.pad{
        padding: 4px;
    }
	.headText {
		color: aliceblue;
	}
</style>
