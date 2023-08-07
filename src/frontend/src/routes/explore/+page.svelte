<script>
	import LayoutCombine from "../../lib/componants/layoutCombine.svelte";
	import Head from "../../lib/componants/head.svelte";
	import Footer from "../../lib/componants/footer.svelte";
	import ContentBox from "../../lib/shared/contentBox.svelte";
	import Button from "../../lib/shared/button.svelte";
	import { createActor } from '../../../../declarations/backend/index.js';
	import { getIdentity } from '../../lib/code/icAgent.js';

	const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
	const host = import.meta.env.VITE_HOST;

	async function handlePotato(){
		  const ID = getIdentity();
		  const backendActor = createActor(canisterId, { agentOptions: { identity: ID, host: host } }); //,identity
		  let testCall = await backendActor.whoami();
		  console.log("WHO? : ",testCall);
	}

</script>
<svelte:head>
	<title>TEST - Explore</title>
	<meta name="description" content="Explore the Internet Computer" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<div>
			<h3>TEST - Explore</h3>
			<ContentBox>
				<p> WORK IN PROGRESS... <span style="color: aqua;">Transaction Search:</span></p>
				<p style="padding-bottom: 10px;">
				<a href="/search/icp">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					ICP Transactions
					</Button>
					</a>
				<a href="/search/ckbtc">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					ckBTC Transactions
					</Button>
				</a>
				<a href="/search/sns1">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					SNS-Dragginz Transactions
					</Button>
				</a>
				<a href="/search/chat">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					CHAT Transactions
					</Button>
				</a>
				<a href="/search/kinic">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					Kinic Transactions
					</Button>
				</a>
				<a href="/search/hot">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					Hot Token Transactions
					</Button>
				</a>
				<a href="/search/ghost">
					<Button type="blueTP" 
					flat={false} 
					noBG={true}>
					Ghost Token Transactions
					</Button>
				</a>
				</p>
			</ContentBox>

			<ContentBox>
				<p> WORK IN PROGRESS... <span style="color: aqua;">Visual Explorer:</span></p>
				<p style="padding-bottom: 10px;">
				<a href="/explore/visualblocks/icp">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest ICP
					</Button>
					</a>
				<a href="/explore/visualblocks/ckbtc">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest CKBTC
					</Button>
				</a>
				<a href="/explore/visualblocks/sns1">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest SNS-Dragginz
					</Button>
				</a>
				<a href="/explore/visualblocks/chat">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest CHAT
					</Button>
				</a>
				<a href="/explore/visualblocks/kinic">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest Kinic
					</Button>
				</a>
				<a href="/explore/visualblocks/hot">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest Hot Token
					</Button>
				</a>
				<a href="/explore/visualblocks/ghost">
					<Button type="orange" 
					flat={false} 
					noBG={true}>
					Latest Ghost Token
					</Button>
				</a>
				</p>
			</ContentBox>

			<ContentBox>
				<p> WORK IN PROGRESS... <span style="color: aqua;">Token Stats/ Metrics</span></p>
				<p style="padding-bottom: 10px;">
						<a href="/explore/stats/ckbtc">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							ckBTC Stats
							</Button>
						</a>
						<a href="/explore/stats/icp">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							ICP Stats
							</Button>
						</a>
						<a href="/explore/stats/chat">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							CHAT Stats
							</Button>
						</a>
						<a href="/explore/stats/sns1">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							Draggonz Stats
							</Button>
						</a>
						<a href="/explore/stats/kinic">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							Kinic Stats
							</Button>
						</a>
						<a href="/explore/stats/hot">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							Hot Stats
							</Button>
						</a>
						<a href="/explore/stats/ghost">
							<Button type="blueTP" 
							flat={false} 
							noBG={true}>
							Ghost Stats
							</Button>
						</a>
				</p>



			</ContentBox>

			<ContentBox>
				<p> WORK IN PROGRESS... <span style="color: aqua;">Random Tests</span></p>
				<!-- <p style="padding-bottom: 10px;">
						<Button type="orange" 
						flat={false} 
						noBG={true}
						on:click={handlePotato}
						>
						POTATO!
						</Button>
				</p> -->
			</ContentBox>


		</div>
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
</style>
