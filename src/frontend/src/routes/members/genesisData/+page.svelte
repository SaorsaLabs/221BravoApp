<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
  	import ContentBox from "../../../lib/shared/contentBox.svelte";
	import {authStore, authTrigger} from "../../../lib/stores/authStore.js";
	import { browser } from '$app/environment';
	import SubHead from "../../../lib/shared/subHead.svelte";
	import GenesisTable from "../../../lib/componants/members/genesisTable.svelte";
	import Loading from "../../../lib/shared/loading.svelte";
	import jsonData from '../../../lib/data/GA_Balances.json';

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

	let data = null;
	let promise = loadStuff();

	async function loadStuff(){
		data = jsonData;
	}

</script>
<!-- SCREEN SIZE FOR VID SWAP -->
<svelte:window bind:innerWidth={screenSize} />

<svelte:head>
	<title>Genesis Accounts</title>
	<meta name="description" content="Genesis Accounts on the Internet Computer Blockchain." />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		{#if LS == true || LS == "true"}
			<SubHead>
				<span class="headText"> Members Only - ICP Genesis Accounts </span>
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
					<ContentBox>
						<h4>ICP Genesis Accounts: </h4>
						<GenesisTable data={data}/>
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
	.headText {
		color: aliceblue;
	}
</style>
