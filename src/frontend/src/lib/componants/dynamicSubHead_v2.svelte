<script>
	import Button from "../shared/button.svelte";
    import SubHead from "../shared/subHead.svelte";
    import { getAllTokenData } from '$lib/code/utils.js';
    import { createEventDispatcher } from 'svelte';
    export let selected = "";
    export let mode = "search"; // search, stats

    let promise = loadStuff();
    let tokenData, tokenData2;
    let numButtons;
    let opAR = [];
    let dynURL; 

    if (mode == "search") dynURL = "/search/token/";
    if (mode == "stats") dynURL = "/explore/stats/token/";
    if (mode == "visual") dynURL = "/explore/visualblocks/token/";

    async function loadStuff(){
        tokenData2 = await getAllTokenData();

        // underline selected
        numButtons = tokenData2.length ?? 0;
        for(let i = 0; i<numButtons; i++){
            if(selected == tokenData2[i].ticker){
                opAR[i] = "underline";
            }else{
                opAR[i] = "noUnderline";
            }
        }
    }

   

    let reset = false;
    const dispatch = createEventDispatcher();
    function resetCheck(btnClicked){
        numButtons = tokenData2.length ?? 0;
        for(let i = 0; i<numButtons; i++){
            if(btnClicked == tokenData2[i].ticker){
                opAR[i] = "underline";
            }else{
                opAR[i] = "noUnderline";
            }
        }
        dispatch('click', btnClicked);
    }
    
    $: tokenData = tokenData2;
</script>
<SubHead> 
    <div class="shPad">
    {#await promise}
    {:then} 
        <span class="pad">|</span>
          {#each tokenData as TKN, i}
                <a href="{dynURL}{TKN.ticker}" class={opAR[i]}>
                    <Button 
                    slim={true} 
                    flat={true} 
                    noBG={true}
                    on:click={() => resetCheck(TKN.ticker)}>
                        {TKN.shortName}
                    </Button>
                </a>
                <span class="pad">|</span>	
        {/each} 
               
    {/await}
  
    </div>
</SubHead>

<style>
    .shPad{
		height: 100%;
		padding-top: 2px;
	}
    .btnMargin{
        margin-left: 20px;
        margin-right: 2px;
    }
    .pad{
        padding: 4px;
    }
    .underline{
        padding: 2px;
        border: 0;
        border-bottom: 2px;
        border-style: solid;
        border-color: #09cbf1;
    }
    .noUnderline{
        padding: 0px;
    }
</style>