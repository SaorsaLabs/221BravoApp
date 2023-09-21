<script>
	import Button from "../../lib/shared/button.svelte";
    import { createEventDispatcher } from 'svelte';
    import { DEFAULT_SUBACCOUNT } from "../code/constants";
    import SaveButton from "../shared/saveButton.svelte";

    export let inputType = 'both'; // 'principalOnly' or 'both' or 'accountOnly'
    export let subAcEnabled = false; 
    export let pushID = '';
    export let pushSUB = '';

    const dispatch = createEventDispatcher();
	let inptAC = "";
    let inptSA = "";
	let minValue = 0.0;
	let maxValue = 0.0;
	let startDate = 0;
	let endDate = 0;
	let disabled = false;
	let advanced = false;
    let outputText = '';
    let error = false;
    let searchActive = false;
    let btnClicked = '';
    let labelText, inputText;
    let disableUserSearch = false;
    let saveAccount = "";

    if(inputType == 'both'){
        labelText = 'Account or Principal :';
        inputText = 'Account or Principal ID';
    }
    if(inputType == 'principalOnly'){
        labelText = 'Principal ID :';
        inputText = 'Principal ID';
    }
    if(inputType == 'accountOnly'){
        labelText = 'Account ID :';
        inputText = 'Account ID';
    }
    if(inputType == 'pushSearch'){
        labelText = 'Searched ID :';
        inptAC = pushID;
        disabled = true;
        searchActive = true;
        if( pushSUB != '' 
            && pushSUB != null 
            && pushSUB != "0000000000000000000000000000000000000000000000000000000000000000"
            ){
            subAcEnabled = true;
            inptSA = pushSUB;
        } else {
            subAcEnabled = false;
        }
        saveAccount = formatSave(inptAC, inptSA);
    }

    function search() {
        error = false;
        outputText = '';
        let t1 = new Date(startDate);
        let t2 = new Date(endDate);

        if(inptAC == '' && inputType == "both") {
            outputText = 'Please enter a Principal or Account ID';
            error = true;
        }

        if(inptAC == '' && inputType == "principalOnly") {
            outputText = 'Please enter a Principal ID';
            error = true;
        }
        if(inptAC == '' && inputType == "accountOnly") {
            outputText = 'Please enter an Account ID';
            error = true;
        }

        if(inptAC.includes('-', 0) && inputType == 'accountOnly'){
            outputText = 'Please enter an valid Account ID';
            error = true;
        }
        if(!inptAC.includes('-', 0) && inputType == 'principalOnly'){
            outputText = 'Please enter a valid Principal ID';
            error = true;
        }
        if(minValue >= maxValue && minValue > 0 && maxValue > 0) {
            outputText = 'Minimum Value cannot be greater than OR equal to Maximum Value';
            error = true;
        }
        if(t1 != 0 && t2 != 0 && t1 > t2){
            outputText = 'Start date cannot be after End Date';
            error = true;
        }
        if (inptAC?.length < 20 && !inptAC.includes("-") && inputType == "principalOnly") {
            outputText = 'Please enter a valid Principal ID';
            error = true;
        }

        inptAC = inptAC.replace(/\s/g, ''); // remove whitespace;
        
        if(!error){
            saveAccount = formatSave(inptAC, inptSA);
            searchActive = true;
            btnClicked = 'search';
            dispatch('click', {
                btnClicked,
                searchID: inptAC,
                minValue,
                maxValue,
                startDate,
                endDate,
                subAC: inptSA,
            });
        }
    }
    function reset() {
        searchActive = false;
        inptAC = "";
        inptSA = "";
        minValue = 0.0;
        maxValue = 0.0;
        startDate = 0;
        endDate = 0;
        disabled = false;
        advanced = false;
        outputText = '';
        error = false;
        btnClicked = 'reset';
        dispatch('click', {
            btnClicked
        });
    }

    function formatSave(input, subAC){
        let res = "";
        
        if(input.includes("-") && subAC != ""){
            res = input+"."+subAC;
        }
        else if(input.includes("-") && input.includes(".") && subAC == ""){
           res = input;
        }
        else if(input.includes("-") && subAC == ""){
           res = input+"."+DEFAULT_SUBACCOUNT;
        }
        else {
            res = input;
        }
        return res;
    }
</script>
<form class="mainAlign">
    <div class="form-group row">
        <label for="acSearch" class="col-sm-3 col-form-label-sm">{labelText}</label>
        <div class="col-sm-9">
          <input 
              id="acSearch" 
            class="form-control form-control-sm" 
            alt="Account Search" 
            placeholder={inputText} 
            type="text" 
            bind:value={inptAC} {disabled}
          >
        </div>
    </div>
    {#if subAcEnabled}
        <div class="form-group row" style = "margin-top: 7px;">
            <label for="acSearch" class="col-sm-3 col-form-label-sm">Sub-Account:</label>
            <div class="col-sm-9">
            <input 
                id="acSearch" 
                class="form-control form-control-sm" 
                alt="Account Search" 
                placeholder="Sub-Account (Optional)" 
                type="text" 
                bind:value={inptSA} {disabled}
            >
            </div>
        </div>
    {/if}
    <div class="form-group row">
        <div class="col-sm-3"></div>
        <div class="col-sm-9">
            {#if disabled == false}
            <!-- TODO! -->
            <!-- <label>
                <input class="form-check-input" type=checkbox bind:checked={advanced}>
                Advanced Search
            </label> -->
            {/if}
        </div>
    </div>
    <div class="form-group row">
        {#if advanced && disabled == false}
        <div class="col-sm-3"></div>
        <div class="col-sm-9">
            <!-- {#if subAcEnabled == true}
                <table class="width100">
                    <tr class="smlPadTB">
                        <td style="width: 18%;">
                            <label for="subSearch" class="col-form-label-sm">Sub Account:</label>
                        </td>
                        <td>
                            <input 
                            id="subSearch" 
                            class="form-control form-control-sm" 
                            alt="Sub Account" 
                            placeholder={"Sub Account ID (text)"} 
                            type="text" 
                            bind:value={inptSA} {disabled}
                            >
                        </td>
                    </tr>
                </table>
            {/if} -->
            <table class="width100">
                <tr class="smlPadTB">
                    <td>
                        <label for="minValue" class="col-form-label-sm">Minimum Value:</label>
                    </td>
                    <td>
                        <input 
                            id="minValue" 
                            class="form-control form-control-sm" 
                            alt="Minimum Value" 
                            placeholder="Min Value" 
                            type="number"
                            step="0.00000001"
                            min="0"
                            bind:value={minValue} {disabled}
                        >
                    </td>
                    <td class="cText">
                        <label for="maxValue" class="col-form-label-sm">Maximum Value:</label>
                    </td>
                    <td>
                        <input 
                        id="maxValue" 
                        class="form-control form-control-sm" 
                        alt="Max Value" 
                        placeholder="Max Value" 
                        type="number" 
                        step="0.00000001"
                        min="0"
                        bind:value={maxValue} {disabled}
                    >
                    </td>
                </tr>
                <tr class="smlPadTB">
                    <td>
                        <label for="utcStart" class="col-form-label-sm">UTC Start:</label>
                    </td>
                    <td>
                        <!-- <input type="datetime-local" id="startDate"/>  -->
                        <input 
                            id="utcStart" 
                            class="form-control form-control-sm" 
                            alt="Start Date" 
                            type="datetime-local" 
                            bind:value={startDate} {disabled}
                        >
                    </td>
                    <td class="cText">
                        <label for="utcEnd" class="col-form-label-sm">UTC End:</label>
                    </td>
                    <td>
                        <input 
                        id="utcEnd" 
                        class="form-control form-control-sm" 
                        alt="End Time" 
                        type="datetime-local" 
                        bind:value={endDate} {disabled}
                    >
                    </td>
                </tr>
            </table>
        </div>
        {:else}
            <p></p>
        {/if}
    </div>
    {#if disabled == false}
        <div class="form-group row smlPadTB">
            <div class="col-sm-3"></div>
            <div class="col-sm-9 ">
                <Button type="grey" on:click={() => search()}>Search</Button>
                {#if searchActive == true}
                    <Button type="orange" on:click={() => reset()}>Reset</Button>
                    <SaveButton icrcAccount={false} text={saveAccount} modeLight={true}/>
                {/if}
            </div>
            <div class="warnText">{outputText}</div>
        </div>
    {:else}
    <div class="form-group row smlPadTB">
        <div class="col-sm-3"></div>
        <div class="col-sm-9 ">
            {#if searchActive == true}
                <SaveButton icrcAccount={false} text={saveAccount} modeLight={true}/>
            {/if}
        </div>
        <div class="warnText">{outputText}</div>
    </div>
    {/if}
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
    .warnText{
        color:white;
        background-color: red; 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
</style>