<script>
import arrow from "$lib/images/Arrow_Down_White.png";
import search from "$lib/images/search.png";
import {DEFAULT_SUBACCOUNT} from '../code/constants.js';
import CopyButton from './copyButton.svelte';
import SaveButton from './saveButton.svelte';

export let data; 
// account, block, visualMap?

export let type = 'account'; 
let directionText = '';
let directionColor = ''; 
if (data.direction == 'in') {
    directionText = 'Deposit: ';
    directionColor = 'inText';
}
else if (data.direction == 'out') {
    if (data.type != 'Approve'){
        directionText = 'Payment: ';
        directionColor = 'outText';
    } else {
        directionText = 'Approve : ';
        directionColor = 'approveText';
    }
}
</script>

{#if type == "account"}
    <!-- ICP-OG Standard - ACCOUNT  -->
    {#if data.standard == "icp-og"}
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
                            {#if data.longID != undefined && data.type != "Mint"}
                                <a href="/search/token/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.longID} icrcAccount={false}/>
                                <SaveButton text={data.longID} icrcAccount={false}/>
                            {/if}
                            {#if data.shortID != undefined && !data?.shortID?.includes("...") && data.type != "Mint"}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                            {/if}
                        {:else}
                            From: {data.target}
                            {#if data.target != undefined}
                                <a href="/search/token/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
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
                            {#if data.target != undefined }
                                <a href="/search/token/{data.token}?id={data.target}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.target} icrcAccount={false}/>
                                <SaveButton text={data.target} icrcAccount={false}/>
                            {/if}
                            {#if data.targetName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.targetName}</p>
                            {/if}
                        {:else}
                            To: {data.longID}
                            {#if data.longID != undefined && data.type != "Burn" && data.type != "Approve"}
                                <a href="/search/token/{data.token}?id={data.longID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.longID} icrcAccount={false}/>
                                <SaveButton text={data.longID} icrcAccount={false}/>
                            {/if}
                            {#if data.shortID != undefined && !data?.shortID?.includes("...") && data.type != "Burn" && data.type != "Approve"}
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
    {/if}

    <!-- ICRC Standard - ACCOUNT -->
    {#if data.standard == "icrc-1" || data.standard == "icrc-2" }
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
                        {#if data.longID != undefined && data.longSubID == DEFAULT_SUBACCOUNT && data.type != "Mint"}
                            <a href="/search/token/{data.token}?id={data.longID}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton icrcAccount={true} text={data.longID} text2 ={DEFAULT_SUBACCOUNT}/>
                            <SaveButton icrcAccount={true} text={data.longID} text2={DEFAULT_SUBACCOUNT} />
                        {/if}
                        <!-- principal has name? -->
                        {#if data.shortID != undefined && !data?.shortID?.includes("...") && data.type !="Mint"} 
                        {@html "<br>"}
                            <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                        {/if}
                        {@html "<br>"}
                        <!-- sub account -->
                        {#if data.longSubID != DEFAULT_SUBACCOUNT && data.longSubID != undefined && data.type != "Mint"}
                        sub-ac: {data.longSubID}
                            <a href="/search/token/{data.token}?id={data.longID}&sub={data.longSubID}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton icrcAccount={true} text={data.longID} text2 ={data.longSubID}/>
                            <SaveButton icrcAccount={true} text={data.longID} text2={data.longSubID} />
                        {/if}
                    {:else}
                        <!-- principal -->
                        From: {data.target}
                        {#if data.target != undefined && data.targetSub == DEFAULT_SUBACCOUNT}   
                            <a href="/search/token/{data.token}?id={data.target}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
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
                        {#if data.targetSub != undefined && data.targetSub != DEFAULT_SUBACCOUNT}
                        sub-ac: {data.targetSub}
                            <a href="/search/token/{data.token}?id={data.target}&sub={data.targetSub}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton icrcAccount={true} text={data.target} text2 ={data.targetSub}/>
                            <SaveButton icrcAccount={true} text={data.target} text2={data.targetSub} />
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
                        {#if data.target != undefined && data.targetSub == DEFAULT_SUBACCOUNT}   
                            <a href="/search/token/{data.token}?id={data.target}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
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
                        {#if data.targetSub != undefined && data.targetSub != DEFAULT_SUBACCOUNT}
                        sub-ac: {data.targetSub}
                            <a href="/search/token/{data.token}?id={data.target}&sub={data.targetSub}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton icrcAccount={true} text={data.target} text2 ={data.targetSub}/>
                            <SaveButton icrcAccount={true} text={data.target} text2={data.targetSub} />
                        {/if}
                    {:else}
                        <!-- principal -->
                        To: {data.longID}
                        <!-- search icon -->
                        {#if data.longID != undefined && data.longSubID == DEFAULT_SUBACCOUNT && data.type != "Burn" && data.type != "Approve"}
                            <a href="/search/token/{data.token}?id={data.longID}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                            <CopyButton icrcAccount={true} text={data.longID} text2 ={DEFAULT_SUBACCOUNT}/>
                            <SaveButton icrcAccount={true} text={data.longID} text2={DEFAULT_SUBACCOUNT} />
                        {/if}
                        <!-- principal has name? -->
                        {#if data.shortID != undefined && !data?.shortID?.includes("...") && data.type != "Burn" && data.type != "Approve"} 
                        {@html "<br>"}
                            <p style="margin-left:22px; margin-bottom:0px;">{data.shortID}</p>
                        {/if}
                        {@html "<br>"}
                        <!-- sub account -->
                        {#if data.longSubID != DEFAULT_SUBACCOUNT && data.longSubID != undefined}
                        sub-ac: {data.longSubID}
                            <a href="/search/token/{data.token}?id={data.longID}&sub={data.longSubID}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                            <CopyButton icrcAccount={true} text={data.longID} text2 ={data.longSubID}/>
                            <SaveButton icrcAccount={true} text={data.longID} text2={data.longSubID} />
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
{/if}

{#if type == "block"}
    <!-- ICP-OG Standard - BLOCK -->
    {#if data.standard == "icp-og"}
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
                            {#if data.fromAccount != undefined && data.type != "Mint"}
                                <a href="/search/token/{data.token}?id={data.fromAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton text={data.fromAccount} icrcAccount={false}/>
                                <SaveButton text={data.fromAccount} icrcAccount={false}/>
                            {/if}
                            {#if data.fromName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.fromName}</p>
                            {/if}
                    </div>
                </div>

                <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
                
                <div class="row main">
                    <div class="col textAdj bottomRule">
                        {#if data.type != "Burn" && data.type != "Approve"}
                            To: {data.toAccount}
                        {:else if data.type == "Approve"}
                            Note: This tx approves only approves spender. No tokens have moved.
                        {:else if data.type == "Burn"}
                            To : {data.token} Burn Account
                        {/if}
                        {#if data.toAccount != undefined && data.type != "Burn" && data.type != "Approve"}
                            <a href="/search/token/{data.token}?id={data.toAccount}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                            <CopyButton text={data.toAccount} icrcAccount={false}/>
                            <SaveButton text={data.toAccount} icrcAccount={false}/>
                        {/if}
                        {#if data.toName != undefined}
                        {@html "<br>"}
                            <p style="margin-left:22px; margin-bottom:0px;">{data.toName}</p>
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
    {/if}

    <!-- ICRC Standards - BLOCK -->
    {#if data.standard == "icrc-1" || data.standard == "icrc-2" }
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
                            {#if data.type != "Mint"}
                                From: {data.fromPrincipal}
                            {:else}
                                From: {data.token} Mint Account
                            {/if}
                            {#if data.fromPrincipal != undefined && data.fromAccount == DEFAULT_SUBACCOUNT && data.type != "Mint"}
                                <a href="/search/token/{data.token}?id={data.fromPrincipal}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton icrcAccount={true} text={data.fromPrincipal} text2 ={DEFAULT_SUBACCOUNT}/>
                                <SaveButton icrcAccount={true} text={data.fromPrincipal} text2={DEFAULT_SUBACCOUNT} />
                            {/if}
                            {#if data.fromName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.fromName}</p>
                            {/if}
                            {#if data.fromAccount != DEFAULT_SUBACCOUNT && data.type != "Mint"}
                            sub-ac: {data.fromAccount}
                                {#if data.fromAccount != undefined}
                                    <a href="/search/token/{data.token}?id={data.fromPrincipal}&sub={data.fromAccount}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/></a>
                                    <CopyButton icrcAccount={true} text={data.fromPrincipal} text2 ={data.fromAccount}/>
                                    <SaveButton icrcAccount={true} text={data.fromPrincipal} text2={data.fromAccount} />
                                {/if}
                            {/if}
                    </div>
                </div>

                <img class="arrowImg" src={arrow} alt="white arrow" style="height:30px"/>
                
                <div class="row main">
                    <div class="col textAdj bottomRule">
                            {#if data.type != "Burn" && data.type != "Approve"}
                                To: {data.toPrincipal}
                            {:else if data.type == "Approve"}
                                Note: This tx approves only approves spender. No tokens have moved.
                            {:else if data.type == "Burn"}
                                To : {data.token} Burn Account
                            {/if}
                            {#if data.toPrincipal != undefined && data.toAccount == DEFAULT_SUBACCOUNT && data.type != "Approve" && data.type != "Burn"}
                                <a href="/search/token/{data.token}?id={data.toPrincipal}&sub={DEFAULT_SUBACCOUNT}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                <CopyButton icrcAccount={true} text={data.toPrincipal} text2 ={DEFAULT_SUBACCOUNT}/>
                                <SaveButton icrcAccount={true} text={data.toPrincipal} text2={DEFAULT_SUBACCOUNT} />
                            {/if}
                            {#if data.toName != undefined}
                            {@html "<br>"}
                                <p style="margin-left:22px; margin-bottom:0px;">{data.toName}</p>
                            {/if}
                            {#if data.toAccount != DEFAULT_SUBACCOUNT && data.type != "Approve" && data.type != "Burn"}
                                sub-ac: {data.toAccount}
                                {#if data.toAccount != undefined }
                                    <a href="/search/token/{data.token}?id={data.toPrincipal}&sub={data.toAccount}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
                                    <CopyButton icrcAccount={true} text={data.toPrincipal} text2 ={data.toAccount}/>
                                    <SaveButton icrcAccount={true} text={data.toPrincipal} text2={data.toAccount} />
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
    {/if}
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
    .approveText{
        color: rgb(0, 207, 222);
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