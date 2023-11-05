<script>
	import Button from "../../lib/shared/button.svelte";
    import { browser } from '$app/environment';
    import { getAllLinkedTokens } from '../code/searchRequest_v3';
    import { parsePrincipalSubAccountString } from "../code/utils";
    import LinkedTokenCarousel from "../shared/linkedTokenCarousel_v2.svelte";
    import Loading from "../shared/loading.svelte";

    export let inputType = 'both';

    let subAcEnabled = false; 
	let inptAC = "";
    let inptSA = "";
    let outputText = '';
    let resText = 'Linked Tokens - Click to explore : ';
    let searchResult;
    let error = false;
    let searchActive = false;
    let searchComplete = false;
    let labelText, inputText;

    if(inputType == 'both'){
        labelText = 'Search by Account or Principal :';
        inputText = 'Account or Principal ID';
    }

    let screenWidth;
    if (browser) {
      screenWidth = window.innerWidth;
      const handleResize = () => {
        // resize event
        screenWidth = window.innerWidth;
      };
      window.addEventListener('resize', handleResize);
    }
    
    async function search() {
        searchComplete = false;
        error = false;
        outputText = '';
        resText = 'Linked Tokens - Click to explore : ';

        if(inptAC == '' && inputType == "both") {
            outputText = 'Please enter a Principal or Account ID';
            error = true;
        }

        // catch copy paste search using combined ICRC format
        if(inptAC.includes(".") && inptAC.includes("-") ){
            let parsed = parsePrincipalSubAccountString(inptAC);
            inptAC = parsed.principal;
            inptSA = parsed.subaccount;
        }

        inptAC = inptAC.replace(/\s/g, ''); // remove whitespace;
        inptSA = inptSA.replace(/\s/g, ''); // remove whitespace;

        if(!error){
            searchActive = true;
            searchResult = await getAllLinkedTokens(inptAC, inptSA, false);
            if (Object.keys(searchResult?.linkedTokens.ICRC).length == 0) { 
                resText = 'Search returned 0 results'; 
            }
            searchActive = false;
            searchComplete = true;
        }
    }
    function reset() {
        searchActive = false;
        searchComplete = false;
        inptAC = "";
        inptSA = "";
        resText = 'Linked Tokens - Click to explore : ';
    }
</script>

<!-- full screen -->
{#if screenWidth > 830}
    <div class="mainAlign">
        <div class="form-group row">
            <label for="acSearch" class="col-sm-3 col-form-label-sm">{labelText}</label>
            <div class="col-sm-9">
                <input 
                    id="acSearch" 
                    class="form-control form-control-sm" 
                    alt="Account Search" 
                    placeholder={inputText} 
                    type="text" 
                    bind:value={inptAC}
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
                    placeholder="Example: 0000000000000000000000000000000000000000000000000000000000000001" 
                    type="text" 
                    bind:value={inptSA} 
                >
                </div>
            </div>
        {/if}
        <div class="form-group row smlPadTB">
            <div class="col-sm-3"></div>
            <div class="col-sm-9 ">
            <label>
                <input class="form-check-input" type=checkbox bind:checked={subAcEnabled}>
                    Include Sub-Account
            </label>
            </div>
        </div>
        <div class="form-group row smlPadTB">
            <div class="col-sm-3"></div>
            <div class="col-sm-9 ">
                <Button type="grey" on:click={() => search()}>Search</Button>
                {#if searchComplete == true}
                    <Button type="orange" on:click={() => reset()}>Reset</Button>
                {/if}
            </div>
            <div class="warnText">{outputText}</div>
        </div>
    </div>
{:else}
<!-- small screens -->
    <div class="mainAlign" style="margin: 10px;">
        <div class="form-group row">
            <input 
                id="acSearch" 
                class="form-control form-control-sm" 
                alt="Account Search" 
                placeholder={inputText} 
                type="text" 
                bind:value={inptAC}
            >
        </div>
        {#if subAcEnabled}
            <div class="form-group row" style = "margin-top: 7px;">
                <input 
                    id="acSearch" 
                    class="form-control form-control-sm" 
                    alt="Account Search" 
                    placeholder="Example: 0000000000000000000000000000000000000000000000000000000000000001" 
                    type="text" 
                    bind:value={inptSA} 
                >
            </div>
        {/if}
        <label>
            <input class="form-check-input" type=checkbox bind:checked={subAcEnabled}>
                Include Sub-Account
        </label>
        <div class="form-group row smlPadTB">
            <div class="col-sm-9 ">
                <Button type="grey" on:click={() => search()}>Search</Button>
                {#if searchComplete == true}
                    <Button type="orange" on:click={() => reset()}>Reset</Button>
                {/if}
            </div>
            <div class="warnText">{outputText}</div>
        </div>
    </div>
{/if}
{#if searchActive == true}
    <div style="padding: 30px;">
        <Loading/>
    </div>
    {:else}
    {#if searchComplete == true}
    <span class="text-warning" style="padding-left: 15px;">{resText}</span>
        <LinkedTokenCarousel 
        linkedTokenData={searchResult.linkedTokens.ICRC}
        searchedPrincipal={searchResult.linkedTokens.linkedPrincipal}
        searchedSubAccount={searchResult.linkedTokens.searched}
        mode={"homePage"}
        />
    {/if}
{/if}


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