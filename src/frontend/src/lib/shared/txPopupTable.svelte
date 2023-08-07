<script>
import arrow from "$lib/images/Arrow_Down_White.png";
import search from "$lib/images/search.png";
import {DEFAULT_SUBACCOUNT} from "../code/constants.js";
import CopyButton from './copyButton.svelte';
import SaveButton from './saveButton.svelte';

export let data; 
// standard noPrincipal noPrincipalBlock principalBlock visualMap visualMapIcrc icrc icrcBlock
export let type = 'standard'; 
let directionText = '';
let directionColor = ''; 
if (data.direction == 'in') {
    directionText = 'Deposit: ';
    directionColor = 'inText';
}
if (data.direction == 'out') {
    directionText = 'Payment: ';
    directionColor = 'outText';
}
</script>

{#if type == 'standard'}
<div class="row">
    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col ">
                Date: {data.date} 
            </div>
            <div class="col">
                Time: {data.time}
            </div>
            <div class="col">
                Block: {data.block}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col textAdj">
                Transaction Hash: 
                {data.hash}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col textAdj">
                {#if data.direction == 'in'}
                    From: {data.longID}
                    {#if data.longID != null && data.longID != "null"}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.longID} icrcAccount={false}/>
                        <SaveButton text={data.longID} icrcAccount={false}/>
                    {/if}
                    {#if data.shortID != undefined && !data?.shortID?.includes("...")}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.longSubID}
                    {#if data.longSubID != null && data.longSubID != "null"}
                        <a href="/search/ID/{data.token}?id={data.longSubID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.longSubID} icrcAccount={false}/>
                        <SaveButton text={data.longSubID} icrcAccount={false}/>
                    {/if}
                    {#if data.subName != undefined && !data?.subName?.includes("...")}
                        {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.subName}</p>
                    {/if}
                {:else}
                    From: {data.target}
                    {#if data.target != null && data.target != "null"}
                        <a href="/search/ID/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.target} icrcAccount={false}/>
                        <SaveButton text={data.target} icrcAccount={false}/>
                    {/if}
                    {#if data.targetName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.targetSub}
                    {#if data.targetSub != null && data.targetSub != "null"}
                        <a href="/search/ID/{data.token}?id={data.targetSub}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.targetSub} icrcAccount={false}/>
                        <SaveButton text={data.targetSub} icrcAccount={false}/>
                    {/if}
                    {#if data.targetSubName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetSubName}</p>
                    {/if}
                {/if}
            </div>
        </div>
        <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
        
        <div class="row main">
            <div class="col textAdj bottomRule">
                {#if data.direction == 'in'}
                    To: {data.target}
                    {#if data.target != null && data.target != "null"}
                        <a href="/search/ID/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.target} icrcAccount={false}/>
                        <SaveButton text={data.target} icrcAccount={false}/>
                    {/if}
                    {#if data.targetName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.targetSub}
                    {#if data.targetSub != null && data.targetSub != "null"}
                        <a href="/search/ID/{data.token}?id={data.targetSub}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.targetSub} icrcAccount={false}/>
                        <SaveButton text={data.targetSub} icrcAccount={false}/>
                    {/if}
                    {#if data.targetSubName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetSubName}</p>
                    {/if}
                {:else}
                    To: {data.longID}
                    {#if data.longID != null && data.longID != "null"}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.longID} icrcAccount={false}/>
                        <SaveButton text={data.longID} icrcAccount={false}/>
                    {/if}
                    {#if data.shortID != undefined && !data?.shortID?.includes("...")}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.longSubID}
                    {#if data.longSubID != null && data.longSubID != "null"}
                        <a href="/search/ID/{data.token}?id={data.longSubID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.longSubID} icrcAccount={false}/>
                        <SaveButton text={data.longSubID} icrcAccount={false}/>
                    {/if}
                    {#if data.subName != undefined && !data?.subName?.includes("...")}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.subName}</p>
                    {/if}
                {/if}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col {directionColor}">
                {directionText} {data.value} {data.token}
            </div>
        </div>
    </div>
</div>
{:else if type == 'noPrincipal'}
        <div class="row">
            <div class="col-12">
                <div class="row main bottomRule">
                    <div class="col ">
                        Date: {data.date}
                    </div>
                    <div class="col">
                        Time: {data.time}
                    </div>
                    <div class="col">
                        Block: {data.block}
                        <!-- <a href="/search/BLOCK/{data.token}?block={data.block}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a> -->
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main bottomRule">
                    <div class="col textAdj">
                        Transaction Hash: 
                        {data.hash}
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main">
                    <div class="col textAdj">
                        {#if data.direction == 'in'}
                            From: {data.longID}
                            {#if data.longID != null && data.longID != "null"}
                                <a href="/search/ID/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.longID} icrcAccount={false}/>
                                <SaveButton text={data.longID} icrcAccount={false}/>
                            {/if}
                            {#if data.shortID != undefined && !data?.shortID?.includes("...")}
                                {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                            {/if}
                        {:else}
                            From: {data.target}
                            {#if data.target != null && data.target != "null"}
                                <a href="/search/ID/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.target} icrcAccount={false}/>
                                <SaveButton text={data.target} icrcAccount={false}/>
                            {/if}
                            {#if data.targetName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                            {/if}
                        {/if}
                    </div>
                </div>
                <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
                
                <div class="row main">
                    <div class="col textAdj bottomRule">
                        {#if data.direction == 'in'}
                            To: {data.target}
                            {#if data.target != null && data.target != "null"}
                                <a href="/search/ID/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.target} icrcAccount={false}/>
                                <SaveButton text={data.target} icrcAccount={false}/>
                            {/if}
                            {#if data.targetName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                            {/if}
                        {:else}
                            To: {data.longID}
                            {#if data.longID != null && data.longID != "null"}
                                <a href="/search/ID/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.longID} icrcAccount={false}/>
                                <SaveButton text={data.longID} icrcAccount={false}/>
                            {/if}
                            {#if data.shortID != undefined && !data?.shortID?.includes("...")}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                            {/if}
                        {/if}
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main">
                    <div class="col {directionColor}">
                        {directionText} {data.value} {data.token}
                    </div>
                </div>
            </div>
        </div>

{:else if type == 'noPrincipalBlock'}
        <div class="row">
            <div class="col-12">
                <div class="row main bottomRule">
                    <div class="col ">
                        Date: {data.date}
                    </div>
                    <div class="col">
                        Time: {data.time}
                    </div>
                    <div class="col">
                        Block: {data.block}
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main bottomRule">
                    <div class="col textAdj">
                        Transaction Hash: 
                        {data.hash}
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main">
                    <div class="col textAdj">
                            From: {data.fromAccount}
                            {#if data.fromAccount != null && data.fromAccount != "null"}
                                <a href="/search/ID/{data.token}?id={data.fromAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.fromAccount} icrcAccount={false}/>
                                <SaveButton text={data.fromAccount} icrcAccount={false}/>
                            {/if}
                            {#if data.fromAccountName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.fromAccountName}</p>
                            {/if}
                    </div>
                </div>
                <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
                
                <div class="row main">
                    <div class="col textAdj bottomRule">
                            To: {data.toAccount}
                            {#if data.toAccount != null && data.toAccount != "null"}
                                <a href="/search/ID/{data.token}?id={data.toAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.toAccount} icrcAccount={false}/>
                                <SaveButton text={data.toAccount} icrcAccount={false}/>
                            {/if}
                            {#if data.toAccountName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.toAccountName}</p>
                            {/if}
                    </div>
                </div>
            </div>

            <div class="col-12">
                <div class="row main">
                    <div class="col">
                       {data.type} : {data.value} {data.token}
                    </div>
                </div>
            </div>
        </div>

{:else if type == 'principalBlock'}
<div class="row">
    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col ">
                Date: {data.date}
            </div>
            <div class="col">
                Time: {data.time}
            </div>
            <div class="col">
                Block: {data.block}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col textAdj">
                Transaction Hash: 
                {data.hash}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col textAdj">
                    {#if data.fromPrincipal != "N/A"}
                        From: {data.fromPrincipal}
                    {:else}
                        From: Mint Transaction
                    {/if}
                    {#if data.fromPrincipal != null && data.fromPrincipal != "null" && data.fromPrincipal != "N/A"}
                        <a href="/search/ID/{data.token}?id={data.fromPrincipal}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.fromPrincipal} icrcAccount={false}/>
                        <SaveButton text={data.fromPrincipal} icrcAccount={false}/>
                    {/if}
                    {#if data.fromPrincipalName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.fromPrincipalName}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.fromAccount}
                    {#if data.fromAccount != null && data.fromAccount != "null" && data.fromAccount != "N/A"}
                        <a href="/search/ID/{data.token}?id={data.fromAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.fromAccount} icrcAccount={false}/>
                        <SaveButton text={data.fromAccount} icrcAccount={false}/>
                    {/if}
                    {#if data.fromAccountName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.fromAccountName}</p>
                    {/if}
            </div>
        </div>
        <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
        
        <div class="row main">
            <div class="col textAdj bottomRule">
                    {#if data.toPrincipal != "N/A"}
                        To: {data.toPrincipal}
                    {:else}
                        To : Burn Transaction
                    {/if}
                    {#if data.toPrincipal != null && data.toPrincipal != "null" && data.toPrincipal != "N/A"}
                        <a href="/search/ID/{data.token}?id={data.toPrincipal}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.toPrincipal} icrcAccount={false}/>
                        <SaveButton text={data.toPrincipal} icrcAccount={false}/>
                    {/if}
                    {#if data.toPrincipalName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.toPrincipalName}</p>
                    {/if}
                    {@html "<br>"}
                    A/C: {data.toAccount}
                    {#if data.toAccount != null && data.toAccount != "null" && data.toAccount != "N/A"}
                        <a href="/search/ID/{data.token}?id={data.toAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton text={data.toAccount} icrcAccount={false}/>
                        <SaveButton text={data.toAccount} icrcAccount={false}/>
                    {/if}
                    {#if data.toAccountName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.toAccountName}</p>
                    {/if}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col">
                {data.type} : {data.value} {data.token}
            </div>
        </div>
    </div>
</div>
{:else if type == 'visualMap'}
    <div class="row">
        <div class="col-12">
            <div class="row main bottomRule">
                <div class="col ">
                    Date: {data.date}
                </div>
                <div class="col">
                    Time: {data.time}
                </div>
                <div class="col">
                    Block: {data.block}
                    <!-- <a href="/search/BLOCK/{data.token}?block={data.block}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a> -->
                </div>
            </div>
        </div>

        <div class="col-12">
            <div class="row main bottomRule">
                <div class="col textAdj">
                    Transaction Hash: 
                    {data.hash}
                </div>
            </div>
        </div>

        <div class="col-12">
            <div class="row main">
                <div class="col textAdj">
                    {#if data.direction == 'in'}
                        From: {data?.longAC ?? "Mint Account"}
                        {#if data.longAC != null && data.longAC != "null"}
                            <a href="/search/ID/{data.token}?id={data.longAC}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.longAC} icrcAccount={false}/>
                            <SaveButton text={data.longAC} icrcAccount={false}/>
                        {/if}
                        {#if data.shortAC != "Unknown" && !data?.shortAC?.includes("...")}
                            {@html "<br>"}
                        <span style="color:aqua">{data.shortAC}</span>
                        {/if}
                        {#if data.longPR != null && data.longPR != "null" && data.longPR != "Unknown"}
                            {@html "<br>"}
                            {data.longPR}
                            <a href="/search/ID/{data.token}?id={data.longPR}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.longPR} icrcAccount={false}/>
                            <SaveButton text={data.longPR} icrcAccount={false}/>
                        {/if}
                        {#if data.shortPR != "Unknown" && !data?.shortPR?.includes("...")}
                            {@html "<br>"}
                            <span style="color:aqua">{data.shortPR}</span>
                        {/if}
                    {:else}
                        From: {data?.targetAC ?? "Mint Account"}
                        {#if data.targetAC != null && data.targetAC != "null"}
                            <a href="/search/ID/{data.token}?id={data.targetAC}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.targetAC} icrcAccount={false}/>
                            <SaveButton text={data.targetAC} icrcAccount={false}/>
                        {/if}
                        {#if data.targetACName != "Unknown"}
                            {@html "<br>"}
                            <span style="color:aqua">{data.targetACName}</span>
                        {/if}
                        {#if data.targetPR != null && data.targetPR != "null" && data.targetPR != "Unknown"}
                            {@html "<br>"}
                            {data.targetPR}
                            <a href="/search/ID/{data.token}?id={data.targetPR}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.targetPR} icrcAccount={false}/>
                            <SaveButton text={data.targetPR} icrcAccount={false}/>
                        {/if}
                        {#if data.targetPRName != "Unknown"}
                            {@html "<br>"}
                            <span style="color:aqua">{data.targetPRName}</span>
                        {/if}
                    {/if}
                </div>
            </div>

            <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
            
            <div class="row main">
                <div class="col textAdj bottomRule">
                    {#if data.direction == 'in'}
                        To: {data?.targetAC ?? "Burn Account"}
                        {#if data.targetAC != null && data.targetAC != "null"}
                            <a href="/search/ID/{data.token}?id={data.targetAC}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.targetAC} icrcAccount={false}/>
                            <SaveButton text={data.targetAC} icrcAccount={false}/>
                        {/if}
                        {#if data.targetACName != "Unknown"}
                            {@html "<br>"}
                            <span style="color:aqua">{data.targetACName}</span>
                        {/if}
                        {#if data.targetPR != null && data.targetPR != "null" && data.targetPR != "Unknown"}
                            {@html "<br>"}
                            {data.targetPR}
                            <a href="/search/ID/{data.token}?id={data.targetPR}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.targetPR} icrcAccount={false}/>
                            <SaveButton text={data.targetPR} icrcAccount={false}/>
                        {/if}
                        {#if data.targetPRName != "Unknown"}
                            {@html "<br>"}
                            <span style="color:aqua">{data.targetPRName}</span>
                        {/if}
                    {:else}
                        To: {data?.longAC ??  "Burn Account"}
                        {#if data.longAC != null && data.longAC != "null"}
                            <a href="/search/ID/{data.token}?id={data.longAC}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.longAC} icrcAccount={false}/>
                            <SaveButton text={data.longAC} icrcAccount={false}/>
                        {/if}
                        {#if data.shortAC != "Unknown" && !data?.shortAC?.includes("...")}
                            {@html "<br>"}
                            <span style="color:aqua">{data.shortAC}</span>
                        {/if}
                        {#if data.longPR != null && data.longPR != "null" && data.longPR != "Unknown"}
                            {@html "<br>"}
                            {data.longPR}
                            <a href="/search/ID/{data.token}?id={data.longPR}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.longPR} icrcAccount={false}/>
                            <SaveButton text={data.longPR} icrcAccount={false}/>
                        {/if}
                        {#if data.shortPR != "Unknown" && !data?.shortPR?.includes("...")}
                            {@html "<br>"}
                            <span style="color:aqua">{data.shortPR}</span>
                        {/if}
                    {/if}
                </div>
            </div>
        </div>

        <div class="col-12">
            <div class="row main">
                <div class="col {directionColor}">
                    {directionText} {data.value} {data.token}
                </div>
            </div>
        </div>
    </div>

{:else if type == 'icrc'}
<div class="row">
    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col ">
                Date: {data.date}
            </div>
            <div class="col">
                Time: {data.time}
            </div>
            <div class="col">
                Block: {data.block}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col textAdj">
                Transaction Hash: 
                {data.hash}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col textAdj">
                {#if data.direction == 'in'}  
                    <!-- principal -->
                    From: {data.longID}
                    <!-- search icon -->
                    {#if data.longID != null && data.longID != "null" && data.longSubID == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longID} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.longID} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.shortID != undefined && !data?.shortID?.includes("...")} 
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.longSubID != DEFAULT_SUBACCOUNT && data.longSubID != null && data.longSubID != "null"}
                    sub-ac: {data.longSubID}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub={data.longSubID}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longID} text2 ={data.longSubID}/>
                        <SaveButton icrcAccount={true} text={data.longID} text2={data.longSubID} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.subName != undefined && !data?.subName?.includes("...")}
                        {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.subName}</p>
                    {/if}
                {:else}
                    <!-- principal -->
                    From: {data.target}
                    {#if data.target != null && data.target != "null" && data.targetSub == DEFAULT_SUBACCOUNT}   
                        <a href="/search/ID/{data.token}?id={data.target}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.target} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.target} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.targetName != undefined && !data?.shortID?.includes("...")}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.targetSub != null && data.targetSub != "null" && data.targetSub != DEFAULT_SUBACCOUNT}
                    sub-ac: {data.targetSub}
                        <a href="/search/ID/{data.token}?id={data.target}&sub={data.targetSub}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.target} text2 ={data.targetSub}/>
                        <SaveButton icrcAccount={true} text={data.target} text2={data.targetSub} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.targetSubName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetSubName}</p>
                    {/if}
                {/if}
            </div>
        </div>
        <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
        
        <div class="row main">
            <div class="col textAdj bottomRule">
                {#if data.direction == 'in'}
                    <!-- principal -->
                    To: {data.target}
                    {#if data.target != null && data.target != "null" && data.targetSub == DEFAULT_SUBACCOUNT}   
                        <a href="/search/ID/{data.token}?id={data.target}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.target} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.target} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.targetName != undefined && !data?.shortID?.includes("...")}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.targetSub != null && data.targetSub != "null" &&
                        data.targetSub != DEFAULT_SUBACCOUNT
                    }
                    sub-ac: {data.targetSub}
                        <a href="/search/ID/{data.token}?id={data.target}&sub={data.targetSub}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.target} text2 ={data.targetSub}/>
                        <SaveButton icrcAccount={true} text={data.target} text2={data.targetSub} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.targetSubName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetSubName}</p>
                    {/if}
                {:else}
                    <!-- principal -->
                    To: {data.longID}
                    <!-- search icon -->
                    {#if data.longID != null && data.longID != "null" && data.longSubID == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                        <CopyButton icrcAccount={true} text={data.longID} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.longID} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.shortID != undefined && !data?.shortID?.includes("...")} 
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.longSubID != DEFAULT_SUBACCOUNT && data.longSubID != null && data.longSubID != "null"}
                    sub-ac: {data.longSubID}
                        <a href="/search/ID/{data.token}?id={data.longID}&sub={data.longSubID}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                        <CopyButton icrcAccount={true} text={data.longID} text2 ={data.longSubID}/>
                        <SaveButton icrcAccount={true} text={data.longID} text2={data.longSubID} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.subName != undefined && !data?.subName?.includes("...")}
                        {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.subName}</p>
                    {/if}
                {/if}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col {directionColor}">
                {directionText} {data.value} {data.token}
            </div>
        </div>
    </div>
</div>
{:else if type == 'icrcBlock'}
<div class="row">
    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col ">
                Date: {data.date}
            </div>
            <div class="col">
                Time: {data.time}
            </div>
            <div class="col">
                Block: {data.block}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col textAdj">
                Transaction Hash: 
                {data.hash}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col textAdj">
                    <!-- Mint TX -->
                    {#if data.fromPrincipal != "N/A"}
                        From: {data.fromPrincipal}
                    {:else}
                        From: Mint Transaction
                    {/if}
                    {#if data.fromPrincipal != null && data.fromPrincipal != "null" && data.fromPrincipal != "N/A" && data.fromAccount == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.fromPrincipal}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.fromPrincipal} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.fromPrincipal} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    {#if data.fromPrincipalName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.fromPrincipalName}</p>
                    {/if}
                    {@html "<br>"}
                    {#if data.fromAccount != DEFAULT_SUBACCOUNT}
                    A/C: {data.fromAccount}
                        {#if data.fromAccount != null && data.fromAccount != "null" && data.fromAccount != "N/A"}
                            <a href="/search/ID/{data.token}?id={data.fromPrincipal}&sub={data.fromAccount}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                            <CopyButton icrcAccount={true} text={data.fromPrincipal} text2 ={data.fromAccount}/>
                            <SaveButton icrcAccount={true} text={data.fromPrincipal} text2={data.fromAccount} />
                        {/if}
                        {#if data.fromAccountName != undefined}
                        {@html "<br>"}
                            <p style="margin-left:22px; margin-bottom:0px;">{data.fromAccountName}</p>
                        {/if}
                    {/if}
            </div>
        </div>
        <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
        
        <div class="row main">
            <div class="col textAdj bottomRule">
                    {#if data.toPrincipal != "N/A"}
                        To: {data.toPrincipal}
                    {:else}
                        To : Burn Transaction
                    {/if}
                    {#if data.toPrincipal != null && data.toPrincipal != "null" && data.toPrincipal != "N/A" && data.toAccount == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.toPrincipal}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.toPrincipal} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.toPrincipal} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    {#if data.toPrincipalName != undefined}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.toPrincipalName}</p>
                    {/if}
                    {@html "<br>"}
                    {#if data.toAccount != DEFAULT_SUBACCOUNT}
                        A/C: {data.toAccount}
                        {#if data.toAccount != null && data.toAccount != "null" && data.toAccount != "N/A"}
                            <a href="/search/ID/{data.token}?id={data.toPrincipal}&sub={data.toAccount}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton icrcAccount={true} text={data.toPrincipal} text2 ={data.toAccount}/>
                            <SaveButton icrcAccount={true} text={data.toPrincipal} text2={data.toAccount} />
                        {/if}
                        {#if data.toAccountName != undefined}
                        {@html "<br>"}
                            <p style="margin-left:22px; margin-bottom:0px;">{data.toAccountName}</p>
                        {/if}
                    {/if}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col">
                {data.type} : {data.value} {data.token}
            </div>
        </div>
    </div>
</div>
{:else if type == 'visualMapIcrc'}
<div class="row">
    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col ">
                Date: {data.date}
            </div>
            <div class="col">
                Time: {data.time}
            </div>
            <div class="col">
                Block: {data.block}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main bottomRule">
            <div class="col textAdj">
                Transaction Hash: 
                {data.hash}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col textAdj">

                {#if data.direction == 'in'}  
                    <!-- principal -->
                    From: {data?.longPR ?? "Mint Account"}
                    <!-- search icon -->
                    {#if data.longPR != null && data.longPR != "null" && data.longAC == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.longPR}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longPR} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.longPR} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.shortPR != undefined && !data?.shortPR?.includes("...") && data.shortPR !="Unknown"} 
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortPR}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.longAC != DEFAULT_SUBACCOUNT && data.longAC != null && data.longAC != "Mint/ Burning Account"}
                    sub-ac: {data?.longAC ?? "Mint Account"}
                        <a href="/search/ID/{data.token}?id={data.longPR}&sub={data.longAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longPR} text2 ={data.longAC}/>
                        <SaveButton icrcAccount={true} text={data.longPR} text2={data.longAC} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.shortAC != undefined && !data?.shortAC?.includes("...") && data.shortAC !="Unknown"}
                        {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortAC}</p>
                    {/if}
                {:else}
                    <!-- principal -->
                    From: {data?.targetPR ?? "Mint Account"}
                    {#if data.targetPR != null && data.targetPR != "null" && data.targetAC == DEFAULT_SUBACCOUNT}   
                        <a href="/search/ID/{data.token}?id={data.targetPR}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.targetPR} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.targetPR} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.targetPRName != undefined && !data?.targetPRName?.includes("...") && data.targetPRName != "Unknown"}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetPRName}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.targetAC != null && data.targetAC != "Mint/ Burning Account" && data.targetAC != DEFAULT_SUBACCOUNT}
                    sub-ac: {data?.targetAC ?? "Mint Account"}
                        <a href="/search/ID/{data.token}?id={data.targetPR}&sub={data.targetAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.targetPR} text2 ={data.targetAC}/>
                        <SaveButton icrcAccount={true} text={data.targetPR} text2={data.targetAC} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.targetACName != undefined && data.targetACName != "Unknown"}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetACName}</p>
                    {/if}
                {/if}
            </div>
        </div>
        <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
        
        <div class="row main">
            <div class="col textAdj bottomRule">
                {#if data.direction == 'in'}
                    <!-- principal -->
                    To: {data?.targetPR ?? "Burn Account"}
                    {#if data.targetPR != null && data.targetPR != "null" && data.targetAC == DEFAULT_SUBACCOUNT}   
                        <a href="/search/ID/{data.token}?id={data.targetPR}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.targetPR} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.targetPR} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.targetPRName != undefined && !data?.targetPRName?.includes("...") && data.targetPRName !="Unknown"}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetPRName}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.targetAC != null && data.targetAC != "Mint/ Burning Account" &&
                        data.targetAC != DEFAULT_SUBACCOUNT
                    }
                    sub-ac: {data?.targetAC ?? "Burn Account"}
                        <a href="/search/ID/{data.token}?id={data.targetPR}&sub={data.targetAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.targetPR} text2 ={data.targetAC}/>
                        <SaveButton icrcAccount={true} text={data.targetPR} text2={data.targetAC} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.targetACName != undefined && data.targetACName != "Unknown"}
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.targetACName}</p>
                    {/if}
                {:else}
                    <!-- principal -->
                    To: {data?.longPR ?? "Burn Account"}
                    <!-- search icon -->
                    {#if data.longPR != null && data.longPR != "null" && data.longAC == DEFAULT_SUBACCOUNT}
                        <a href="/search/ID/{data.token}?id={data.longPR}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longPR} text2 ={DEFAULT_SUBACCOUNT}/>
                        <SaveButton icrcAccount={true} text={data.longPR} text2={DEFAULT_SUBACCOUNT} />
                    {/if}
                    <!-- principal has name? -->
                    {#if data.shortPR != undefined && !data?.shortPR?.includes("...") && data.shortPR !="Unknown"} 
                    {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortPR}</p>
                    {/if}
                    {@html "<br>"}
                    <!-- sub account -->
                    {#if data.longAC != DEFAULT_SUBACCOUNT && data.longAC != null && data.longAC != "Mint/ Burning Account"}
                    sub-ac: {data?.longAC ?? "Burn Account"}
                        <a href="/search/ID/{data.token}?id={data.longPR}&sub={data.longAC}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                        <CopyButton icrcAccount={true} text={data.longPR} text2 ={data.longAC}/>
                        <SaveButton icrcAccount={true} text={data.longPR} text2={data.longAC} />
                    {/if}
                    <!-- sub account has name? -->
                    {#if data.shortAC != undefined && !data?.shortAC?.includes("...") && data.shortAC !="Unknown"}
                        {@html "<br>"}
                        <p style="margin-left:22px; margin-bottom:0px;">{data.shortAC}</p>
                    {/if}
                {/if}
            </div>
        </div>
    </div>

    <div class="col-12">
        <div class="row main">
            <div class="col {directionColor}">
                {directionText} {data.value} {data.token}
            </div>
        </div>
    </div>
</div>
{/if}


<style>
    @media screen and (max-width:800px){
        .textAdj{
            font-size: 0.80rem;
        }   
    }
    @media screen and (max-width:992px){
        .textAdj{
            font-size: 0.85rem;
        }   
    }
    .arrowImg{
        padding-left: 10px;
    }
    .main{
        margin: 0;
        margin-top: 5px;
        margin-bottom: 5px;
    }
    .box{
        border-color: white;
        border-style: dashed;
        border-width: 2px;
    }
    .inText{
        color: rgb(0, 195, 0);;
    }
    .outText{
        color: red;
    }
    .bottomRule{
        border: 0;
        border-bottom: 1px;
        border-style: solid;
        border-color: rgba(0, 0, 0, 0.25);
        margin: 1px;
    }
    .nameText{
        border:0px;
        padding:0px;
        border-left: 22px;
        border-bottom:0px;
        padding-bottom: 0px;
    }
</style>