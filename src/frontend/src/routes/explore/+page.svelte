<script>
	import LayoutCombine from "../../lib/componants/layoutCombine.svelte";
	import Head from "../../lib/componants/head.svelte";
	import SubHead from "../../lib/shared/subHead.svelte";
	import Footer from "../../lib/componants/footer.svelte";
	import ContentBox from "../../lib/shared/contentBox.svelte";
  	import TokensTable from "../../lib/componants/tokensTable.svelte";
	import ProjectCarousel from "../../lib/componants/projectCarousel.svelte";

</script>
<svelte:head>
	<title>Explore Internet Computer</title>
	<meta name="description" content="Explore the Internet Computer Ecosystem" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		<SubHead>
			<span class="pad">|</span>
			<a href="#TopTokens">Top Tokens</a>
			<span class="pad">|</span>
			<a href="#Social">Social Apps</a>
			<span class="pad">|</span>
			<a href="#BNM">News/ Music</a>
			<span class="pad">|</span>
			<a href="#DEFI">DEFI Apps</a>
			<span class="pad">|</span>
			<a href="#Gaming">Games</a>
			<span class="pad">|</span>
			<a href="#NFT">NFT Apps</a>
			<span class="pad">|</span>
			<a href="#Wallets">Wallets</a>
			<span class="pad">|</span>
			<a href="#Devs">Devs</a>
			<span class="pad">|</span>
		</SubHead>
	</span>

	<span slot="body">
		<div>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="TopTokens"> <b> Top Internet Computer Tokens</b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<TokensTable/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="Social"> <b> ICP Social Apps </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={1}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="BNM"> <b> ICP Blogging/ News/ Music Apps </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={2}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="DEFI"> <b> ICP Defi Apps </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={3}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="Gaming"> <b> ICP Gaming Apps </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={4}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="NFT"> <b> ICP NFT Apps </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={6}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="Wallets"> <b> ICP Wallets </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={5}/>
			</ContentBox>

			<ContentBox type="standard-shaddow-black" addedTopMargin=true >
				<div style="text-align: center; padding: 4px">
					<h3 class="gradient-text" id="Devs"> <b> ICP Developer/ Tools </b></h3>
				</div>
			</ContentBox>

			<ContentBox>
				<ProjectCarousel bucket={7}/>
			</ContentBox>
			
		</div>
	</span>

	<span slot="foot">
		<Footer/>
	</span>
</LayoutCombine>

<style>
	.gradient-text {
		/* font-size: 3em; */
		/* background: linear-gradient(to right, #a83caf, #09cbf1); */
		background: linear-gradient(to top, #a49257, #e3e37b);
		-webkit-background-clip: text;
		-webkit-text-fill-color: transparent;
		background-clip: text;
		text-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
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
	a {
		color: aliceblue;
		text-decoration: none;
	}
	.pad{
        padding: 4px;
    }
</style>
