<div>
	<header class="min">
		<slot name="head"/>
	</header>
	<main class="min gradBG">
		<div class="container-lg main">
			<slot name="body"/>
		</div>
	</main>
    <footer class="min">
        <slot name="foot"/>
    </footer>
</div>

<style>
	.min{
		min-width: 700px;
	}
    .main{
        padding-top: 10px;
		padding-bottom: 10px;        
    }
	.gradBG{
		min-height: calc(100vh - 85px); 
		background: #06beb6;  
		background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  
		background: linear-gradient(to right, #89318f, #09cbf1); 
		min-width: 700px;
	}
    /* //NEW - #09cbf1 OLD - #06beb6*/
</style>
