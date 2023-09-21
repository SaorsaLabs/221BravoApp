<script>
	import LayoutCombine from "../../lib/componants/layoutCombine.svelte";
	import Head from "../../lib/componants/head.svelte";
	import Footer from "../../lib/componants/footer.svelte";
	import AddressBook from "../../lib/componants/addressBook.svelte";
  	import ContentBox from "../../lib/shared/contentBox.svelte";
	import {authStore, authTrigger} from "../../lib/stores/authStore.js";
	import { browser } from '$app/environment';
	import DoubleContentBox from "../../lib/shared/doubleContentBox.svelte";
	import SubHead from "../../lib/shared/subHead.svelte";
	import nftMedia from '$lib/media/Genesis_NFT_Wide.gif';
	import nftMedia2 from '$lib/media/Genesis_NFT_Slim.gif';
	import nftBanner from '$lib/images/NFT_banner.png';
	import Button from "../../lib/shared/button.svelte";
	import MembersLinksCarousel from "../../lib/componants/members/membersLinksCarousel.svelte";
	import FraudReport from "../../lib/componants/fraudReport.svelte";

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


</script>
<!-- SCREEN SIZE FOR VID SWAP -->
<svelte:window bind:innerWidth={screenSize} />

<svelte:head>
	<title>Members</title>
	<meta name="description" content="Members area - 221Bravo App" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		{#if LS == true || LS == "true"}
			<SubHead>
				<span class="pad">|</span>
				<a href="#AB">Address Book</a>
				<span class="pad">|</span>
				<a href="#Tools">Member Tools</a>
				<span class="pad">|</span>
				<a href="#Stats">Stats</a>
				<span class="pad">|</span>
				<a href="#Research">Research</a>
				<span class="pad">|</span>
				<a href="#Report">Report Scam</a>
			</SubHead>
		{:else}
			<SubHead/>
		{/if}

	</span>

	<span slot="body">

		{#if LS == true || LS == 'true'}
		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 1px">
				<h3 class="gradient-text"> <b> Welcome Home Data-Detective</b></h3>
			</div>
		</ContentBox>
			<ContentBox>
				<h4 id="AB">Your Address Book</h4>
				<div style="padding: 10px;">
					<AddressBook></AddressBook>
				</div>
			</ContentBox>

		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 1px">
				<h3 class="gradient-text" id="Tools"> <b> Member Only Tools</b></h3>
			</div>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark">
			<MembersLinksCarousel mode={"tools"}/>
		</ContentBox>

		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 1px">
				<h3 class="gradient-text" id="Stats"> <b> Exclusive Stats</b></h3>
			</div>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark">
			<MembersLinksCarousel mode={"stats"}/>
		</ContentBox>

		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 1px">
				<h3 class="gradient-text" id="Research"> <b> Exclusive Research/ News</b></h3>
			</div>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark">
			Work in Progress...
		</ContentBox>

		{:else}
		<!-- NOT A MEMBER -->
		<ContentBox type="standard-shaddow-black" addedTopMargin=true>
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text"> <b> Become a Member to access this page </b></h3>
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
		{/if}

		<ContentBox type="standard-shaddow-black" addedTopMargin=true >
			<div style="text-align: center; padding: 4px">
				<h3 class="gradient-text" id="Report"> <b>Report a Scam/ Fraud</b></h3>
			</div>
		</ContentBox>

		<ContentBox type="standard-shaddow-dark">
			<FraudReport/>
		</ContentBox>			
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
	.pad{
        padding: 4px;
    }
</style>
