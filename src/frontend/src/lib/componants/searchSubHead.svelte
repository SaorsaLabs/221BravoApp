

<script>
	import Button from "../shared/button.svelte";
    import SubHead from "../shared/subHead.svelte";
    import { createEventDispatcher } from 'svelte';
    export let selected = 0;

    // underline selected
    let numButtons = 6;
    let opAR = [];
    for(let i = 0; i<numButtons; i++){
        if(selected == i+1){
            opAR[i] = "underline";
        }else{
            opAR[i] = "noUnderline";
        }
    }

    let reset = false;
    const dispatch = createEventDispatcher();
    function resetCheck(btnClicked){
        if(btnClicked == selected){
            reset = true;
            dispatch('click', {reset});
        }
        reset = false;
    }
    

</script>
<SubHead> 
    <div class="shPad">
                        <a href="/search/icp" class={opAR[0]}>
                            <Button 
                            slim={true} 
                            flat={true} 
                            noBG={true}
                            on:click={() => resetCheck(1)}>
                                ICP
                            </Button>
                        </a>
                    <span class="pad"></span>
                    <a href="/search/ckbtc" class={opAR[1]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}
                        on:click={() => resetCheck(2)}>
                            ckBTC
                        </Button>
                    </a>
                    <span class="pad"></span>
                    <a href="/search/sns1" class={opAR[2]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}
                        on:click={() => resetCheck(3)}>
                            SNS-Dragginz
                        </Button>
                    </a>
                    <span class="pad"></span>
                    <a href="/search/chat" class={opAR[3]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}
                        on:click={() => resetCheck(4)}>
                            SNS-Chat
                        </Button>
                    </a>
                    <span class="pad"></span>
                    <a href="/search/kinic" class={opAR[4]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}
                        on:click={() => resetCheck(5)}>
                            SNS-Kinic
                        </Button>
                    </a>
                                        <!--<span class="pad"></span>
                    <a href="/search/cketh" class={opAR[2]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            ckETH
                        </Button>
                    </a> -->
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