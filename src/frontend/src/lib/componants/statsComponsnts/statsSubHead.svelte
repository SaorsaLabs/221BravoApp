

<script>
	import Button from "../../shared/button.svelte";
    import SubHead from "../../shared/subHead.svelte";
    export let selected = 0;

    // underline selected
    let numButtons = 7;
    let opAR = [];
    for(let i = 0; i<numButtons; i++){
        if(selected == i+1){
            opAR[i] = "underline";
        }else{
            opAR[i] = "noUnderline";
        }
    }

</script>
<SubHead> 
    <div class="shPad">
                        <a href="/explore/stats/icp" class={opAR[0]}>
                            <Button 
                            slim={true} 
                            flat={true} 
                            noBG={true}>
                                ICP
                            </Button>
                        </a>
                    <span class="pad"></span>
                    <a href="/explore/stats/ckbtc" class={opAR[1]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            ckBTC
                        </Button>
                    </a>
                    <span class="pad"></span>
                    <a href="/explore/stats/sns1" class={opAR[2]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Dragginz
                        </Button>
                    </a>
                    <span class="pad"></span>
                    <a href="/explore/stats/chat" class={opAR[3]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Chat
                        </Button>
                    </a>
                    <a href="/explore/stats/kinic" class={opAR[4]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Kinic
                        </Button>
                    </a>
                    <a href="/explore/stats/hot" class={opAR[5]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            HotOrNot
                        </Button>
                    </a>
                    <a href="/explore/stats/ghost" class={opAR[6]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Ghost
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