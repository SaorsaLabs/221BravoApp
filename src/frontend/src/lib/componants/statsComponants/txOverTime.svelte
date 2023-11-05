<script>
import Button from "../../shared/button.svelte";
import BarChart from "../charts/barChart.svelte";

export let hourlyData = {};
export let dailyData = {};
export let token = "";
export let hours = 0;
export let days = 0;
export let is_icrc;

const fmtOptions = { style: 'decimal', maximumFractionDigits: 2, minimumFractionDigits: 0 };
const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 2, minimumFractionDigits: 0 };

let revData;
let revLabel;
let revDataWeek;
let revLabelWeek; 

revData =  hourlyData.hourlyOTdata;
revLabel = hourlyData.hourlyOTlabels;
revDataWeek = dailyData.dailyOTdata;
revLabelWeek = dailyData.dailyOTlabels;

let hourlyBtnColour = "blueTP";
let dailyBtnColour = "orange";
let showChart = "daily";

// init (daily)
let numTransactions = dailyData.dailyTotalCount;
let numMints = dailyData.dailyTotalMint;
let numBurns = dailyData.dailyTotalBurn;
let numActiveAcs = dailyData.dailyActiveAccounts;
let numActivePrs = dailyData.dailyActivePrincipals;
let totValue = dailyData.dailyTotalValue ?? 0;
let mintValue = dailyData.dailyMintValue ?? 0;
let burnValue = dailyData.dailyBurnValue ?? 0;
numTransactions = numTransactions.toLocaleString('en-US', fmtOptions);
numMints = numMints.toLocaleString('en-US', fmtOptions);
numBurns = numBurns.toLocaleString('en-US', fmtOptions);
numActiveAcs = numActiveAcs.toLocaleString('en-US', fmtOptions);
numActivePrs = numActivePrs?.toLocaleString('en-US', fmtOptions);
totValue = totValue.toLocaleString('en-US', fmtOptionsValue);
mintValue = mintValue.toLocaleString('en-US', fmtOptionsValue);
burnValue = burnValue.toLocaleString('en-US', fmtOptionsValue);

function dtoggleHourly(){
    hourlyBtnColour = "orange";
    dailyBtnColour = "blueTP";
    showChart = "hourly";
    numTransactions = hourlyData.hourlyTotalCount;
    numMints = hourlyData.hourlyTotalMint;
    numBurns = hourlyData.hourlyTotalBurn;
    numActiveAcs = hourlyData.hourlyActiveAccounts;
    numActivePrs = hourlyData.hourlyActivePrincipals;
    totValue = hourlyData.hourlyTotalValue ?? 0;
    mintValue = hourlyData.hourlyMintValue ?? 0;
    burnValue = hourlyData.hourlyBurnValue ?? 0;
    numTransactions = numTransactions.toLocaleString('en-US', fmtOptions);
    numMints = numMints.toLocaleString('en-US', fmtOptions);
    numBurns = numBurns.toLocaleString('en-US', fmtOptions);
    numActiveAcs = numActiveAcs.toLocaleString('en-US', fmtOptions);
    numActivePrs = numActivePrs.toLocaleString('en-US', fmtOptions);
    totValue = totValue.toLocaleString('en-US', fmtOptionsValue);
    mintValue = mintValue.toLocaleString('en-US', fmtOptionsValue);
    burnValue = burnValue.toLocaleString('en-US', fmtOptionsValue);
}
function toggleDaily(){
    hourlyBtnColour = "blueTP";
    dailyBtnColour = "orange";
    showChart = "daily";
    numTransactions = dailyData.dailyTotalCount;
    numMints = dailyData.dailyTotalMint;
    numBurns = dailyData.dailyTotalBurn;
    numActiveAcs = dailyData.dailyActiveAccounts;
    numActivePrs = dailyData.dailyActivePrincipals;
    totValue = dailyData.dailyTotalValue ?? 0;
    mintValue = dailyData.dailyMintValue ?? 0;
    burnValue = dailyData.dailyBurnValue ?? 0;
    numTransactions = numTransactions.toLocaleString('en-US', fmtOptions);
    numMints = numMints.toLocaleString('en-US', fmtOptions);
    numBurns = numBurns.toLocaleString('en-US', fmtOptions);
    numActiveAcs = numActiveAcs.toLocaleString('en-US', fmtOptions);
    numActivePrs = numActivePrs.toLocaleString('en-US', fmtOptions);
    totValue = totValue.toLocaleString('en-US', fmtOptionsValue);
    mintValue = mintValue.toLocaleString('en-US', fmtOptionsValue);
    burnValue = burnValue.toLocaleString('en-US', fmtOptionsValue);
}

$: token;
</script>

<div class="mainAlign ">
    {#if showChart == "hourly"}
        <table style="width: 100%; height: 100%;">
            <tr>
                <td class="contentLeft">
                        <table>
                            <tr>
                                <td>
                                    <Button slim={true} flat={true} type={hourlyBtnColour} on:click={()=>{dtoggleHourly()}}>{hours} Hours</Button>
                                    <Button slim={true} flat={true} type={dailyBtnColour} on:click={()=>{toggleDaily()}}>{days} Days</Button>
                                </td>
                                <td>
                                    <div class="topNote">
                                       * total transactions (UTC)
                                    </div>
                                </td>
                            </tr>
                        </table>
                        <BarChart dataArray={revData} labelsArray={revLabel} datasetTitle={"Transactions"} />
                <td>
                    <div class="contentRight">
                        <p class="fontSpanTitle" style="text-align: center;">{token} Stats ({hours}hr)</p>
                            <div class="tableContainer">
                                <table>
                                    <tr>
                                        <span class="fontSpan"> Total Transactions: 
                                            <span style="color: aqua; padding-left:5px;">{numTransactions}</span>
                                        </span> 
        
                                    </tr>
                                    <tr style="padding-top: 5px;">
                                        <span class="fontSpan"> Total Value: 
                                            <span style="color: aqua; padding-left:5px;">{totValue} {token}</span>
                                        </span>
                                    </tr> 
                                    <tr style="padding-top: 5px;">
                                        <span class="fontSpan"> Total Mints: 
                                            <span style="color: chartreuse; padding-left:5px;">{numMints}</span>
                                        </span>
                                        {@html "<br>"}
                                        <span class="fontSpan"> Mint Value: 
                                            <span style="color: chartreuse; padding-left:5px;">{mintValue} {token}</span>
                                        </span>
                                    </tr>
                                    <tr style="padding-top: 5px;">
                                        <span class="fontSpan"> Total Burn: 
                                            <span style="color: orangered; padding-left:5px;">{numBurns}</span>
                                        </span>
                                        {@html "<br>"}
                                        <span class="fontSpan"> Burn Value: 
                                            <span style="color: orangered; padding-left:5px;">{burnValue} {token}</span>
                                        </span>
                                    </tr>
                                    <tr style="padding-top: 5px;">
                                        {#if is_icrc == true}
                                            <span class="fontSpan"> Active Accounts: 
                                                <span style="color: aqua; padding-left:5px;">{numActiveAcs}</span>
                                            </span>
                                            {@html "<br>"}
                                            <span class="fontSpan"> Active Principals: 
                                                <span style="color: aqua; padding-left:5px;">{numActivePrs}</span>
                                            </span>
                                        {:else}
                                            <span class="fontSpan"> Active Accounts: 
                                                <span style="color: aqua; padding-left:5px;">{numActiveAcs}</span>
                                            </span>
                                        {/if}
                                    </tr>
                                </table>
                            </div>
                    </div>
                </td>
            </tr>
        </table>
    {:else}
        <table style="width: 100%; height: 100%;">
            <tr>
                <td class="contentLeft">
                    <table>
                        <tr>
                            <td>
                                <Button slim={true} flat={true} type={hourlyBtnColour} on:click={()=>{dtoggleHourly()}}>{hours} Hours</Button>
                                <Button slim={true} flat={true} type={dailyBtnColour} on:click={()=>{toggleDaily()}}>{days} Days</Button>
                            </td>
                            <td>
                                <div class="topNote">
                                   * total transactions (UTC)
                                </div>
                            </td>
                        </tr>
                    </table>
                    <BarChart dataArray={revDataWeek} labelsArray={revLabelWeek} datasetTitle={"Transactions"} />
                <td>
                    <div class="contentRight">
                        <p class="fontSpanTitle" style="text-align: center;">{token} Stats ({days} days)</p>
                        <div class="tableContainer">
                            <table>
                                <tr>
                                    <span class="fontSpan"> Total Transactions: 
                                        <span style="color: aqua; padding-left:5px;">{numTransactions}</span>
                                    </span> 
    
                                </tr>
                                <tr style="padding-top: 5px;">
                                    <span class="fontSpan"> Total Value: 
                                        <span style="color: aqua; padding-left:5px;">{totValue} {token}</span>
                                    </span>
                                </tr> 
                                <tr style="padding-top: 5px;">
                                    <span class="fontSpan"> Total Mints: 
                                        <span style="color: chartreuse; padding-left:5px;">{numMints}</span>
                                    </span>
                                    {@html "<br>"}
                                    <span class="fontSpan"> Mint Value: 
                                        <span style="color: chartreuse; padding-left:5px;">{mintValue} {token}</span>
                                    </span>
                                </tr>
                                <tr style="padding-top: 5px;">
                                    <span class="fontSpan"> Total Burn: 
                                        <span style="color: orangered; padding-left:5px;">{numBurns}</span>
                                    </span>
                                    {@html "<br>"}
                                    <span class="fontSpan"> Burn Value: 
                                        <span style="color: orangered; padding-left:5px;">{burnValue} {token}</span>
                                    </span>
                                </tr>
                                <tr style="padding-top: 5px;">
                                    {#if is_icrc == true}
                                        <span class="fontSpan"> Active Accounts: 
                                            <span style="color: aqua; padding-left:5px;">{numActiveAcs}</span>
                                        </span>
                                        {@html "<br>"}
                                        <span class="fontSpan"> Active Principals: 
                                            <span style="color: aqua; padding-left:5px;">{numActivePrs}</span>
                                        </span>
                                    {:else}
                                        <span class="fontSpan"> Active Accounts: 
                                            <span style="color: aqua; padding-left:5px;">{numActiveAcs}</span>
                                        </span>
                                    {/if}
                                </tr>
                            </table>
                        </div>
                    </div>
                </td>
            </tr>
        </table>
    {/if}

</div>

<style>
	.mainAlign{
		padding: 10px;
        margin-left: 0px; 
	}
    .tableAlign{
        width:100%;
    }
    .box{
        border: 1px;
        border-left: 0px;
        border-right: 0px;
        border-color: rgb(28, 28, 28);
        border-style: dotted;
    }
    .contentLeft{
        width: 66%;
        text-align: left;
        padding-right: 0px;
    }
    .contentRight{
        width: 100%;
        text-align: left;
        padding-left: 25px;
        height: 100%;
        vertical-align: top;
        /* border: 1px;
        border-style: dashed;
        border-color: chartreuse; */
    }
    .tableContainer {
      display: flex;
      align-items: center;
      justify-content: center;
      height: 70%;
    }
    .topNote{
      display: flex;
      align-items: center;
      justify-content: center;
      font-size: 12px;
      padding-left: 10px;
    }
    .fontSpan{
        font-size: 15px;
        }
    @media (min-width: 992px) 
    {
        .fontSpan{
        font-size: 17px;
        }
    }

    .fontSpanTitle{
        font-size: 20px;
        }
    @media (min-width: 992px) 
    {
        .fontSpanTitle{
        font-size: 25px;
        }
    }

    .fontSpanSubTitle{
        font-size: 17px;
        }
    @media (min-width: 992px) 
    {
        .fontSubSpanTitle{
        font-size: 22px;
        }
    }
</style>