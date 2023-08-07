<script>
	import LayoutCombine from "../lib/componants/layoutCombine.svelte";
	import Head from "../lib/componants/head.svelte";
	import Footer from "../lib/componants/footer.svelte";
	import TxArtistHome from "../lib/componants/txArtistHome.svelte";
	import ContentBox from '../lib/shared/contentBox.svelte';
	import DoubleContentBox from "../lib/shared/doubleContentBox.svelte";
	import nftMedia from '$lib/media/Genesis_NFT_Wide.gif';
	import nftMedia2 from '$lib/media/Genesis_NFT_Slim.gif';
	import nftBanner from '$lib/images/NFT_banner.png';
 	import Button from "../lib/shared/button.svelte";
	import SnsCarousel from "../lib/componants/SNSCarousel.svelte";

	let screenSize; 
</script>
<!-- SCREEN SIZE FOR VID SWAP -->
<svelte:window bind:innerWidth={screenSize} />

<svelte:head>
	<title>TEST - Home</title>
	<meta name="description" content="Home for the fam" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<div style="height: 100px;">
			<TxArtistHome/>
		</div>
		<ContentBox>
			<h4>Latest News:</h4>
			Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sit amet facilisis magna etiam tempor. Eget aliquet nibh praesent tristique magna 
			sit amet purus gravida. Porta non pulvinar neque laoreet suspendisse interdum consectetur libero. Id venenatis a condimentum vitae sapien. Nunc non blandit massa enim nec dui nunc mattis enim. In fermentum posuere urna nec tincidunt praesent. Egestas diam in arcu cursus euismod quis viverra. Velit scelerisque in dictum non consectetur a erat. Senectus et netus et malesuada. Mattis pellentesque id nibh tortor. In metus vulputate eu scelerisque. Fermentum posuere urna nec tincidunt praesent semper feugiat nibh.
			At varius vel pharetra vel turpis nunc eget. Ut porttitor leo a diam sollicitudin tempor. Aliquet risus feugiat in ante. Facilisi nullam vehicula ipsum a arcu cursus vitae. Posuere sollicitudin aliquam ultrices sagittis orci a scelerisque. Integer enim neque volutpat ac tincidunt vitae semper quis lectus. Leo vel fringilla est ullamcorper eget nulla facilisi etiam. Orci sagittis eu volutpat odio facilisis mauris. Vel orci porta non pulvinar neque laoreet. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Montes nascetur ridiculus mus mauris vitae ultricies leo integer malesuada.
		</ContentBox>

		<!-- Coins/ Tokens Section -->
		<ContentBox type="standard-shaddow-light" addedTopMargin=true>
			<div style="text-align: center; padding: 5px">
				<h2 class="gradient-text"> <b> Search Accounts and Blocks</b></h2>
			</div>
		</ContentBox>

		<ContentBox>
			<SnsCarousel titleText="" linkTypes="search"/>
		</ContentBox>

		<!-- Coins/ Tokens Section -->
		<ContentBox type="standard-shaddow-light" addedTopMargin=true>
			<div style="text-align: center; padding: 5px">
				<h2 class="gradient-text"> <b> Explore ICP Coins and Tokens  </b></h2>
			</div>
		</ContentBox>

		<DoubleContentBox>
			<span slot="leftContent"> 
				DIP 20, EXT, 
			</span>
			<span slot="rightContent"> 
				SNS / ICRC-1 Tokens
			</span>
		</DoubleContentBox>

		<!-- Membership Section -->
		<ContentBox type="standard-shaddow-light" addedTopMargin=true>
			<div style="text-align: center; padding: 5px">
				<h2 class="gradient-text"> <b> Get More from 221Bravo.App! </b></h2>
			</div>
		</ContentBox>

		<DoubleContentBox>
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
					<li>Access premium stats, visual block explorer and other tools</li>
					<li>Access to exclusive members chat</li>
					<li>Ability to name accounts and store them in your address book</li>
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

		<!-- Membership Section -->
		<ContentBox type="standard-shaddow-light" addedTopMargin=true>
			<div style="text-align: center; padding: 5px">
				<h2 class="gradient-text"> <b>Report a Scam/ Fraud</b></h2>
			</div>
		</ContentBox>
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
	}
</style>
