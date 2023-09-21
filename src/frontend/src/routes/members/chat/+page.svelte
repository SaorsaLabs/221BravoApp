<script>
	import LayoutCombine from "../../../lib/componants/layoutCombine.svelte";
	import Head from "../../../lib/componants/head.svelte";
	import Footer from "../../../lib/componants/footer.svelte";
	import MembersChat from "../../../lib/componants/membersChat.svelte";
	import SubHead from "../../../lib/shared/subHead.svelte";

</script>
<svelte:head>
	<title>Members Chat</title>
	<meta name="description" content="Members Chat - 221Bravo App" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
		<SubHead><span class="headText">| Members Chat |</span></SubHead>
	</span>

	<span slot="body">
	<MembersChat/>
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
	.headText{
		color: azure;
	}
	.box{
		border: 2px;
		border-style: dashed;
		border-color: aliceblue;
	}
</style>
