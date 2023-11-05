<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
	import ContentBox from "../../../lib/shared/contentBox.svelte";

</script>
<svelte:head>
	<title>Disclaimer and Terms</title>
	<meta name="description" content="Explore the Internet Computer" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<div>
			<h2>Disclaimers and Privacy</h2>
		</div>
		<ContentBox type="standard-dotborder">
			<div style="padding: 10px;">
				<b>IMPORTANT DISCLAIMER:</b> 
				<p>
				All content provided herein our website (221Bravo.App), hyperlinked sites, 
				associated applications, forums, blogs, social media accounts and other platforms 
				(“Site”) is for your general information only.
				We make no warranties of any kind in relation to our content, including but not limited 
				to accuracy and updatedness. No part of the content that we provide constitutes 
				financial advice, legal advice or any other form of advice meant for your specific 
				reliance for any purpose. Any use or reliance on our content is solely at your own 
				risk and discretion. 
				</p>
				<p>
				You should conduct your own research, review, analyse and verify 
				our content before relying on them. Trading is a highly risky activity that can lead 
				to major losses, please therefore consult your financial advisor before making any 
				decision. No content on our Site is meant to be a solicitation or offer.
				</p>
			</div>
		</ContentBox>

		<ContentBox type="standard-dotborder">
			<div style="padding: 10px;">
				<h3>Privacy Policy</h3>

				<p>
				Welcome to 221Bravo.App (“we”, “our”, or “us”). We respect your privacy and are committed to protecting it through our compliance with this policy.
				</p>
	
				<p>
				<b>This policy describes:</b>
				</p>
				<p>
				The types of information we may collect or that you may provide when you visit our website, 221Bravo.App.
	
				Our practices for collecting, using, maintaining, protecting, and disclosing that information.
				</p>
				<p>
				<b>Information We Collect and How We Collect It</b>
				</p>
				<p>
				We do not store any personal identifiable data via our Canister Smart Contracts other than information provided voluntarily by you through the ‘Report a Scam/Fraud’ submission form (See member’s section of our website). This information is required to process reports and is store securely with access controls. Once processed the data is removed from the canister smart contract. Use of the scam/ fraud reporting form constitutes acceptance of our storage and processing methodologies for the data you submit. 
				</p>
				<p>
				Members of the website can voluntarily add named accounts to their address book. This information cannot be accessed by other users and is access-gated to the Account ID or Principal ID of the wallet holding the user’s membership NFT. Users wishing to remove named accounts can do so via the members section of the website. 
				</p>
				<p>
				The Members Chat function is a third-party plugin provided by OpenChat (https://oc.app). No data is collected, processed or stored by us in regards the chat functionality. 
				We implement measures designed to secure all data from accidental loss and from unauthorized access, use, alteration, and disclosure.
				</p>
				<p>
				  <b>Publicly Available Information</b>
				</p>
				<p>
					221Bravo.App collects data from numerous public data sources, including public ledgers and third-party aggregators which we do not control. 221Bravo app does not share user data with any third party organizations. 
				</p>
				
				<p>
					<b>Changes to Our Privacy Policy</b>
				</p>
			
				<p>
					It is our policy to post any changes we make to our privacy policy on this page. If we make material changes to how we treat our users' personal information, we will notify you through a notice on the website home page.	
				</p>
				<p>
					Contact Information:
					To ask questions or comment about this privacy policy and our privacy practices, contact us at: hello@saorsalabs.net
				</p>
			</div>
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
</style>
