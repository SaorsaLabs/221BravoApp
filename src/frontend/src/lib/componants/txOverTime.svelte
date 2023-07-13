<script>
import Button from "../shared/button.svelte";
import BarChart from "./charts/barChart.svelte";


export let hourlyData = {};
export let dailyData = {};
export let token = "";
export let hours = 0;
export let days = 0;

let revData = hourlyData.hourlyOTdata.reverse();
let revLabel = hourlyData.hourlyOTlabels.reverse();

let revDataWeek = dailyData.dailyOTdata.reverse();
let revLabelWeek = dailyData.dailyOTlabels.reverse();

let dailyBtnColour = "orange";
let weeklyBtnColour = "grey";
let showChart = "daily";

let totValue = hourlyData.hourlyTotalValue.toFixed(2) ?? 0;
let mintValue = hourlyData.hourlyMintValue.toFixed(2) ?? 0;
let burnValue = hourlyData.hourlyBurnValue.toFixed(2) ?? 0;

let totValueDay = dailyData.dailyTotalValue.toFixed(2) ?? 0;
let mintValueDay = dailyData.dailyMintValue.toFixed(2) ?? 0;
let burnValueDay = dailyData.dailyBurnValue.toFixed(2) ?? 0;

function toggleDaily(){
    dailyBtnColour = "orange";
    weeklyBtnColour = "grey";
    showChart = "daily";
}
function toggleWeekly(){
    dailyBtnColour = "grey";
    weeklyBtnColour = "orange";
    showChart = "weekly";
}

</script>

<div class="mainAlign ">
    {#if showChart == "daily"}
        <table style="width: 100%; height: 100%;">
            <tr>
                <td class="contentLeft">
                        <Button slim={true} type={dailyBtnColour} on:click={()=>{toggleDaily()}}>{hours} Hours</Button>
                        <Button slim={true} type={weeklyBtnColour} on:click={()=>{toggleWeekly()}}>{days} Days</Button>
                        <BarChart dataArray={revData} labelsArray={revLabel} datasetTitle={"Transactions"} />
                <td>
                    <div class="contentRight">
                        <p class="fontSpanTitle" style="text-align: center;">Daily Stats ({hours}hr)</p>
                        <table>
                            <tr>
                                <span class="fontSpan"> Total Transactions: 
                                    <span style="color: aqua; padding-left:5px;">{hourlyData.hourlyTotalCount}</span>
                                </span> 

                            </tr>
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Value: 
                                    <span style="color: aqua; padding-left:5px;">{totValue} {token}</span>
                                </span>
                            </tr> 
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Mints: 
                                    <span style="color: chartreuse; padding-left:5px;">{hourlyData.hourlyTotalMint}</span>
                                </span>
                                {@html "<br>"}
                                <span class="fontSpan"> Mint Value: 
                                    <span style="color: chartreuse; padding-left:5px;">{mintValue} {token}</span>
                                </span>
                            </tr>
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Burn: 
                                    <span style="color: orangered; padding-left:5px;">{hourlyData.hourlyTotalBurn}</span>
                                </span>
                                {@html "<br>"}
                                <span class="fontSpan"> Burn Value: 
                                    <span style="color: orangered; padding-left:5px;">{burnValue} {token}</span>
                                </span>
                            </tr>
                        </table>
                    </div>
                </td>
            </tr>
        </table>
    {:else}
        <table style="width: 100%; height: 100%;">
            <tr>
                <td class="contentLeft">
                        <Button slim={true} type={dailyBtnColour} on:click={()=>{toggleDaily()}}>{hours} Hours</Button>
                        <Button slim={true} type={weeklyBtnColour} on:click={()=>{toggleWeekly()}}>{days} Days</Button>
                        <BarChart dataArray={revDataWeek} labelsArray={revLabelWeek} datasetTitle={"Transactions"} />
                <td>
                    <div class="contentRight">
                        <p class="fontSpanTitle" style="text-align: center;">Weekly Stats ({days} days)</p>
                        <table>
                            <tr>
                                <span class="fontSpan"> Total Transactions: 
                                    <span style="color: aqua; padding-left:5px;">{dailyData.dailyTotalCount}</span>
                                </span> 

                            </tr>
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Value: 
                                    <span style="color: aqua; padding-left:5px;">{totValueDay} {token}</span>
                                </span>
                            </tr> 
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Mints: 
                                    <span style="color: chartreuse; padding-left:5px;">{dailyData.dailyTotalMint}</span>
                                </span>
                                {@html "<br>"}
                                <span class="fontSpan"> Mint Value: 
                                    <span style="color: chartreuse; padding-left:5px;">{mintValueDay} {token}</span>
                                </span>
                            </tr>
                            <tr style="padding-top: 5px;">
                                <span class="fontSpan"> Total Burn: 
                                    <span style="color: orangered; padding-left:5px;">{dailyData.dailyTotalBurn}</span>
                                </span>
                                {@html "<br>"}
                                <span class="fontSpan"> Burn Value: 
                                    <span style="color: orangered; padding-left:5px;">{burnValueDay} {token}</span>
                                </span>
                            </tr>
                        </table>
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
</style>