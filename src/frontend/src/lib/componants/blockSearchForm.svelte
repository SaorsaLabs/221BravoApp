<script>
	import Button from "../../lib/shared/button.svelte";
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();

    let startBlock = 0;
    let endBlock = 0;
    let startBlockDate = '';
    let endBlockDate = '';
    let error = false;
    let outputText = '';
    let btnClicked = '';
    let empty = '';
    let searchActive = false;

    let disabled = false;
    function searchBlock() {
        error = false;
        outputText = '';
        let t1 = new Date(startBlockDate);
        let t2 = new Date(endBlockDate);

        if(startBlock == 0 && endBlock == 0 &&  startBlockDate == '' && endBlockDate == ''){
            error = true;
            outputText = 'Enter Start and End Blocks OR select Start and End Dates';
        }
        if(t1 > t2){
            error = true;
            outputText = 'Start Date cannot be after End Date';
        }
        if(startBlock > endBlock){
            error = true;
            outputText = 'Start Block must be less or equal to the End Block';
        }
        if((endBlock - startBlock) > 10000){
            error = true;
            outputText = 'Max 10,000 Blocks in one search!';
        }

        if(!error){
            searchActive = true;
            btnClicked = 'search';
            dispatch('click', {
                btnClicked,
                startBlock,
                endBlock,
                startBlockDate,
                endBlockDate
            });
        }
    }

    function quickBlock() {
        error = false;
        outputText = '';
        searchActive = true;
        btnClicked = 'latest';
        dispatch('click', {
            btnClicked,
            empty,
            empty,
            empty,
            empty
        });
    }

    function reset(){
        startBlock = 0;
        endBlock = 0;
        startBlockDate = '';
        endBlockDate = '';
        error = false;
        outputText = '';
        searchActive = false;
        btnClicked = 'reset';
            dispatch('click', {
                btnClicked
            });
    }
</script>
<form class="mainAlign">
    <div class="form-group row">
        <label for="bkSearch" class="col-sm-3 col-form-label-sm">Start Block:</label>
        <div class="col-sm-3">
          <input 
              id="bkSearchStart" 
            class="form-control form-control-sm" 
            alt="Block Start" 
            placeholder="Start Block" 
            type="number" 
            bind:value={startBlock} {disabled}
          >
        </div>
        <label for="bkSearchEnd" class="col-sm-3 col-form-label-sm rText">End Block:</label>
        <div class="col-sm-3">
          <input 
              id="bkSearchEnd" 
            class="form-control form-control-sm" 
            alt="Block End" 
            placeholder="End Block" 
            type="number" 
            bind:value={endBlock} {disabled}
          >
        </div>
    </div>

    <!-- TODO! -->
    <!-- <div class="form-group row">
        <label for="blockStTime" class="col-sm-3 col-form-label-sm smlPadTB">Start Time (UTC):</label>
        <div class="col-sm-3 smlPadTB">
            <input 
            id="minValue" 
            class="form-control form-control-sm" 
            alt="Start Block Time" 
            type="datetime-local" 
            bind:value={startBlockDate} {disabled}
        >
        </div>
        <label for="blockStTime" class="col-sm-3 col-form-label-sm smlPadTB rText">End Time (UTC):</label>
        <div class="col-sm-3 smlPadTB">
            <input 
            id="minValue" 
            class="form-control form-control-sm" 
            alt="End Block Time" 
            type="datetime-local" 
            bind:value={endBlockDate} {disabled}
        >
        </div>
    </div> -->

    <div class="form-group row smlPadTB">
        <div class="col-sm-3"></div>
        <div class="col-sm-9" style="margin-top: 15px">
            <Button type="grey" on:click={() => searchBlock()}>Search</Button>
            {#if searchActive == false}
            <span> : OR : </span>
            <Button type="orange" on:click={() => quickBlock()}>Show Latest Blocks</Button>
            {/if}
            {#if searchActive == true}
                <Button type="orange" on:click={() => reset()}>Reset</Button>
            {/if}
        </div>
    </div>
    <div class="warnText">{outputText}</div>
</form>

<style>
	.mainAlign{
		padding: 10px;
	}
	.smlPadTB{
		padding-top: 3px;
		padding-bottom: 3px;
	}
	.box{
		border: 2px;
		border-style: dashed;
		border-color: aliceblue;
	}
	.width100{
		width: 100%;
	}
	.cText{
		text-align: center;
	}
    .rText{
        text-align: right;
    }
    .warnText{
        color:white;
        background-color: red; 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
</style>