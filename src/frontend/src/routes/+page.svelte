<script>
	import LayoutCombine from "../lib/componants/layoutCombine.svelte";
	import Head from "../lib/componants/head.svelte";
	import SubHead from "../lib/shared/subHead.svelte";
	import Footer from "../lib/componants/footer.svelte";
	import ContentBox from '../lib/shared/contentBox.svelte';
	import DoubleContentBox from "../lib/shared/doubleContentBox.svelte";
	import nftMedia from '$lib/media/Genesis_NFT_Wide.gif';
	import nftMedia2 from '$lib/media/Genesis_NFT_Slim.gif';
	import nftBanner from '$lib/images/NFT_banner.png';
 	import Button from "../lib/shared/button.svelte";
	import SnsCarousel from "../lib/componants/SNSCarousel.svelte";
	import NewsCarousel from "../lib/componants/newsCarousel.svelte";
	import LinkTokenSearch from "../lib/componants/linkTokenSearch.svelte";
	import { basicBlockTableTX } from '../lib/code/searchRequest';
	import { getLatestBlockData } from '../lib/code/searchRequest_v2.js';
	import TxBlockTable from "../lib/componants/txBlockTable.svelte";
	import { canister_ids } from '../lib/code/constants.js';
  	import Loading from "../lib/shared/loading.svelte";

	let screenSize; 
	let tokenSelected = "ICP";
	let blockTableProcessed;
	let blockTableLoading = true;
	let promise = loadStuff();
	async function loadStuff(){
		await latestBlocksClick(tokenSelected);
		blockTableLoading = false;
	}

	async function latestBlocksClick(token){
		blockTableLoading = true;
		let blockSearchResults = await getLatestBlockData(500,token);
		blockTableProcessed = basicBlockTableTX(blockSearchResults.blocks,token, true);
		tokenSelected = token;
		blockTableLoading = false;
	}

</script>
<!-- SCREEN SIZE FOR VID SWAP -->
<svelte:window bind:innerWidth={screenSize} />

<svelte:head>
	<title>Home - 221Bravo App</title>
	<meta name="description" content="Home for ICP Data-Detectives" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		<SubHead>
			<span class="pad">|</span>
			<a href="#LatestNews">Latest News</a>
			<span class="pad">|</span>
			<a href="#SearchTokens">Search Tokens</a>
			<span class="pad">|</span>
			<a href="./explore">Explore Ecosystem</a>
			<span class="pad">|</span>
			<a href="#GetMembership">Get Membership</a>
			<span class="pad">|</span>
			<a href="./members">Report Fraud</a>
			<span class="pad">|</span>
		</SubHead>
	</span>

	<span slot="body">
		<!-- Latest News -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true id="LatestNews">
			<div style="text-align: center; padding: 4px">
				<h3 id="LatestNews" class="gradient-text"> <b> Latest News</b></h3>
			</div>
		</ContentBox>
		<ContentBox type="light-shaddow-dark">
			<NewsCarousel/>
		</ContentBox>

		<!-- Coins/ Tokens Section -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text" id="SearchTokens"> <b> Search Accounts and Blocks</b></h3>
			</div>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark" >
			<LinkTokenSearch/>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark">
			{#await promise}
			{:then}
			<div style="padding:5px; padding-top:10px">
				<table style="width: 100%;">
					<tr>
						<td>
							{#each canister_ids as BTN}
								<span style="padding: 5px;">
									{#if BTN.token == "SNS1"}
										{#if BTN.token == tokenSelected}
											<Button slim={true} type={"orange"} on:click={()=>{latestBlocksClick(BTN.token)}}>{"DRAGGINZ"}</Button>
										{:else}
											<Button slim={true} type={"blueTP"} on:click={()=>{latestBlocksClick(BTN.token)}}>{"DRAGGINZ"}</Button>
										{/if}
									{:else if BTN.token == "CAT"}
										{#if BTN.token == tokenSelected}
											<Button slim={true} type={"orange"} on:click={()=>{latestBlocksClick(BTN.token)}}>{"CATALYZE"}</Button>
										{:else}
											<Button slim={true} type={"blueTP"} on:click={()=>{latestBlocksClick(BTN.token)}}>{"CATALYZE"}</Button>
										{/if}
									{:else}
										{#if BTN.token == tokenSelected}
											<Button slim={true} type={"orange"} on:click={()=>{latestBlocksClick(BTN.token)}}>{BTN.token}</Button>
										{:else}
											<Button slim={true} type={"blueTP"} on:click={()=>{latestBlocksClick(BTN.token)}}>{BTN.token}</Button>
										{/if}
									{/if}
								</span>
							{/each}
						</td>
					</tr>
					<tr>
						<td>
							{#if blockTableLoading == true}
								<div style="padding:40px"><Loading/></div>
							{:else}
								{#if tokenSelected != "ICP"}
								<TxBlockTable txData={blockTableProcessed.blocks} perPage={5} is_icrc={true} popupType={'icrcBlock'}/>
								{:else}
								<!-- not icrc but needs is_icrc true to show a/c -->
								<TxBlockTable 
									txData={blockTableProcessed.blocks} 
									popupType={'noPrincipalBlock'}
									usePrincipal={false}
									is_icrc={false} 
									perPage={5}
								/>	
								{/if}
							{/if}
						</td>
					</tr>
				</table>
			</div>
			{/await}
		</ContentBox>

		<!-- Coins/ Tokens Section -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true >
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text"> <b> Explore Top ICP Coins and Tokens  </b></h3>
			</div>
		</ContentBox>

		<!-- Search Accounts Carousel -->
		<SnsCarousel titleText="" linkTypes="stats"/>

		<!-- Membership Section -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true >
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text" id="GetMembership"> <b> Get More from 221Bravo.App! </b></h3>
			</div>
		</ContentBox>

		<DoubleContentBox type="standard-shaddow-dark">
			<span slot="leftContent"> 
				<div style="padding: 5px; margin-top:5px;" class="video-container">
					<h5 style="text-align: center;">Genesis II NFTs feature real-time ICP transactions! </h5>
					{#if screenSize >= 1200}
						<img class="headAlign" src={nftMedia} alt="" width="100%"/>
					{:else}
						<img class="headAlign" src={nftMedia2} alt="" width="100%"/>
					{/if}
					<p style="text-align: center;">
						<a href="https://s6ruu-aaaaa-aaaal-qbjuq-cai.raw.ic0.app/934.html" target="_blank">
							View Full Size Example
						 </a>
					</p>		
				</div>
			</span>
			<span slot="rightContent"> 
				<div style="padding: 8px;">
					<h4 style="text-align: center;">Become a member of 221Bravo.App and get access to exclusive content and tools.</h4>
				{@html "<br>"}
				<p>
					We use Genesis II NFTs as your membership card giving you ultimate control over your membership and the ability to transfer or sell it at any time! 
				</p>	
					{@html "<br>"}
					<li>Exchanges and other interesting accounts are named</li>
					<li>Access premium stats, visual block explorer and other tools</li>
					<li>Access to exclusive members chat</li>
					<li>Name accounts and store them in your address book</li>
					<li>Access to exclusive research and reports</li>
					<li>First in line benefits for future events and sales</li>
					<li>No monthly costs!</li>
					{@html "<br>"}
					<img class="headAlign" src={nftBanner} alt="" width="100%"/>
				<h5 style="padding-top: 10px; text-align:center;">Get your membership today from Toniq NFT Marketplace.</h5> 
				
				<div style="width: 100%; text-align:center; padding-top: 5px;"> 
					<a href="https://toniq.io/marketplace/genesis-ii" target="_blank">
						<Button type="orange">Get Membership NFT</Button>
					</a>
				</div>
				</div>
			</span>
		</DoubleContentBox>

	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	.content {
		min-height: 80vh;
		text-align: center;
		align-content: center;
		min-width: 700px;
	}
	.cntr {
		text-align: center;
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
	.video-container {
		position: relative;
		width: 100%;     /* Set to desired width */
		height: 100%;    /* Set to desired height */
		overflow: hidden;
	}
	video {
		width: 100%;
		height: 600px;
	}
	a {
		color: aliceblue;
		text-decoration: none;
	}
	.pad{
        padding: 4px;
    }
</style>
