<script>
	import LayoutCombine from "../lib/componants/layoutCombine.svelte";
	import Head from "../lib/componants/head.svelte";
	import Footer from "../lib/componants/footer.svelte";
	import TxArtistHome from "../lib/componants/txArtistHome.svelte";
	import ContentBox from '../lib/shared/contentBox.svelte';
</script>
<svelte:head>
	<title>TEST - Home</title>
	<meta name="description" content="Home for the fam" />
</svelte:head>

<LayoutCombine>

	<span slot="head">
		<Head/>
	</span>

	<span slot="body">
		<div>
			<TxArtistHome/>
		</div>
		<div>
			<ContentBox>
				Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua. Sit amet facilisis magna etiam tempor. Eget aliquet nibh praesent tristique magna sit amet purus gravida. Porta non pulvinar neque laoreet suspendisse interdum consectetur libero. Id venenatis a condimentum vitae sapien. Nunc non blandit massa enim nec dui nunc mattis enim. In fermentum posuere urna nec tincidunt praesent. Egestas diam in arcu cursus euismod quis viverra. Velit scelerisque in dictum non consectetur a erat. Senectus et netus et malesuada. Mattis pellentesque id nibh tortor. In metus vulputate eu scelerisque. Fermentum posuere urna nec tincidunt praesent semper feugiat nibh.

At varius vel pharetra vel turpis nunc eget. Ut porttitor leo a diam sollicitudin tempor. Aliquet risus feugiat in ante. Facilisi nullam vehicula ipsum a arcu cursus vitae. Posuere sollicitudin aliquam ultrices sagittis orci a scelerisque. Integer enim neque volutpat ac tincidunt vitae semper quis lectus. Leo vel fringilla est ullamcorper eget nulla facilisi etiam. Orci sagittis eu volutpat odio facilisis mauris. Vel orci porta non pulvinar neque laoreet. Quis viverra nibh cras pulvinar mattis nunc sed blandit. Montes nascetur ridiculus mus mauris vitae ultricies leo integer malesuada.

			</ContentBox>
		</div>
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
</style>
