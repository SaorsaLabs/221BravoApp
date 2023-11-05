

<script>
	import Button from "../../shared/button.svelte";
    import SubHead from "../../shared/subHead.svelte";
    export let selected = 0;

    // underline selected
    let numButtons = 12;
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
                    <span class="pad">|</span>
                    <a href="/explore/stats/ckbtc" class={opAR[1]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            ckBTC
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/sns1" class={opAR[2]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Dragginz
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/chat" class={opAR[3]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Chat
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/kinic" class={opAR[4]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Kinic
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/hot" class={opAR[5]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            HotOrNot
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/ghost" class={opAR[6]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Ghost
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/modclub" class={opAR[7]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            ModClub
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/cat" class={opAR[8]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Catalyze
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/boom" class={opAR[9]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            BoomDAO
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/icx" class={opAR[10]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            IC-X
                        </Button>
                    </a>
                    <span class="pad">|</span>
                    <a href="/explore/stats/nuance" class={opAR[11]}>
                        <Button 
                        slim={true} 
                        flat={true} 
                        noBG={true}>
                            Nuance
                        </Button>
                    </a>
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