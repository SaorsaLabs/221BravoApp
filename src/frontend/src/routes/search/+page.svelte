<script>
	import LayoutCombine from "../../lib/componants/layoutCombine.svelte";
	import Head from "../../lib/componants/head.svelte";
	import Footer from "../../lib/componants/footer.svelte";

	

</script>
<svelte:head>
	<title>Search</title>
	<meta name="description" content="Explore the Internet Computer" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<div>
			<h1>Search</h1>
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
