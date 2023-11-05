<script>
import { 
    getPriceData, 
    getICRC_TotalHolders, 
    getICRC_TotalSupply,
    getICP_TotalHolders, 
    getICP_TotalSupply 
} from '../../code/fetchStats.js';

import Button from '../../shared/button.svelte';
import search from "$lib/images/search.png";
import trade from "$lib/images/trade.png";

export let token = "";
export let tradePair = "";
export let tradeURL = "";
export let tokenStandard = "";
export let quoteCurrency = "icp";

let prevToken = ""; 
let priceData;
let totalAll;
let searchURL = `/search/token/${token}`;
let promise = loadStuff();
let changeColour = "white";
let change;
let totalAccounts;
let totalPrincipals;
let totSupply;
const fmtOptions = { style: 'decimal', maximumFractionDigits: 2, minimumFractionDigits: 0 };

async function loadStuff(){
    prevToken = token;
    searchURL  = `/search/token/${token}`;
    if(tokenStandard.includes("icrc")){
        priceData = await getPriceData(tradePair);
        if (priceData.change > 0) changeColour = "chartreuse";
        if (priceData.change < 0) changeColour = "orangered";
        change = priceData.change ?? 0;

        totalAll = await getICRC_TotalHolders(token);
        totalAccounts = totalAll?.accounts ?? 0;
        totalPrincipals = totalAll?.principals ?? 0;
        totalAccounts = totalAccounts.toLocaleString('en-US', fmtOptions);
        totalPrincipals = totalPrincipals.toLocaleString('en-US', fmtOptions);

        // total supply
        totSupply = await getICRC_TotalSupply(token);
        totSupply = totSupply.toLocaleString('en-US', fmtOptions);
    } 
    else if (tokenStandard == "icp-og"){
        priceData = await getPriceData(tradePair);
        if (priceData.change > 0) changeColour = "chartreuse";
        if (priceData.change < 0) changeColour = "orangered";
        change = priceData.change;

        totalAll = await getICP_TotalHolders(token);
        totalAccounts = totalAll?.accounts ?? 0;
        totalPrincipals = totalAll?.principals ?? 0;
        totalAccounts = totalAccounts.toLocaleString('en-US', fmtOptions);
        totalPrincipals = totalPrincipals.toLocaleString('en-US', fmtOptions);

        // total supply
        totSupply = await getICP_TotalSupply(token);
        totSupply = totSupply.toLocaleString('en-US', fmtOptions);
    }
}

// bit of a hack.. forces data to be reloaded when token changes.
$: if (token != prevToken) {
    promise = loadStuff();
    prevToken = token;
}
</script>

<div style="width:100%;">
    {#await promise}
    <p class="cntr">Loading Data... </p>
    {:then}
        <table style="width:100%; text-align:center">
            <tr>
                <td>
                    <span class="price"> Price: {priceData.price} {quoteCurrency} </span>
                    {@html "<br>"}
                    <span class="change" style="color: {changeColour};">24hr Change : {change} % </span>
                </td>
                <td >
                    <div class="container">
                        <div class="box3">
                            {#if totalAccounts != 0}
                                <span style="font-size:large">Total Holders</span>
                                {@html "<br>"} 
                                <span style="color: aqua;">{totalAccounts}</span>
                            {/if}
                        </div>
                        <div class="box3">
                            {#if totalPrincipals != 0}
                                <span style="font-size:large">Total Principals</span>
                                {@html "<br>"} 
                                <span style="color: aqua;">{totalPrincipals}</span>
                            {/if}   
                        </div>
                        <div class="box3">
                            <span style="font-size:large">Total Supply</span>
                            {@html "<br>"} 
                            <span style="color: aqua;">{totSupply} {token}</span>
                        </div>
                    </div>
                </td>
                <td> 
                    <div class="container">
                        <div class="box">
                            <a href={searchURL}>
                                <Button slim={true} flat={true} type={"blueTP"} >
                                    <table>
                                        <tr>
                                            <td><img src={search} alt="Search" width="25px"/></td>
                                            <td>
                                                Search Blocks 
                                                {@html "<br>"} 
                                                + Accounts
                                            </td>
                                        </tr>
                                    </table>
                                </Button>
                            </a> 
                        </div>
                        <div class="box">
                            <a href={tradeURL} target="_blank">
                                <Button slim={true} flat={true} type={"blueTP"} >
                                    <table>
                                        <tr>
                                            <td style="padding-right: 5px;">
                                                <img src={trade} alt="Trade" width="21px"/>
                                            </td>
                                            <td>    
                                                Trade 
                                                {@html "<br>"} 
                                                {token}
                                            </td>
                                        </tr>
                                    </table>
                                    

                                </Button>
                        </div>
                    </div>      
                </td>
            </tr>
        </table>
    {/await}
</div>

<style>
    .price{
        font-size: x-large;
    }
    .change{
        font-size: small;
    }

    .container {
      display: flex;
      flex-wrap: wrap;
    }
    .box {
      flex: 1 0 50%; /* Two boxes will take up 50% of the container width */
      /* min-width: 200px; Set a minimum width for the boxes */
      padding: 5px;
      box-sizing: border-box;
      align-items: center;
      align-content: center;
    }
    .box3 {
      flex: 1 0 33.33%; /* Two boxes will take up 50% of the container width */
      /* min-width: 200px; Set a minimum width for the boxes */
      padding: 5px;
      box-sizing: border-box;
      align-items: center;
      align-content: center;
    }

    /* Media query for smaller screens */
    @media (max-width: 900px) {
      .box {
        flex-basis: 100%; /* Boxes will take up 100% of the container width */
      }
      .box3 {
        flex-basis: 100%; /* Boxes will take up 100% of the container width */
      }
    }
</style>