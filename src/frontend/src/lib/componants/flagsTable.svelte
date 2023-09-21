<script>
import Button from "../shared/button.svelte";
import Modal from "../shared/modal.svelte";
import search from "$lib/images/search.png";
import SaveButton from "../shared/saveButton.svelte";
import CopyButton from "../shared/copyButton.svelte";
import LinkedTokenCarousel from "../shared/linkedTokenCarousel.svelte";
import { parsePrincipalSubAccountString } from '../code/utils.js';

export let flagData = {};
console.log("Flag DATA :: ", flagData);

let showPrincipal = false;
let showSubAccount = false;
let showNFT = false;
let showMinting = false;
let showKnownName = false;
let showUserKnownName = false; 
let showLinkedTokens = false;
let showReports = false;

// check if linked ICRC tokens
let keys;
keys = Object.keys(flagData.linkedTokens.ICRC);
let keyLen = keys?.length ?? 0;
for(let i=0; i<keyLen; i++){
    if(flagData.linkedTokens.ICRC[keys[i]] == true){
        showLinkedTokens = true;
        break;
    }
}
let rep = flagData.reports.length ?? 0;
if( rep > 0 ) showReports = true;
if( flagData.linkedTokens?.principal ) showPrincipal = true;
if( flagData.nfts?.number > 0 ) showNFT = true;
let subACLen = 0;
let uniqueSA = [];
let icpLinksLen = flagData.linkedTokens.ICP?.length ?? 0;

// if(flagData.linkedTokens?.ICP.length > 0) {
//     subACLen = flagData.linkedTokens.ICP.length;
//     for(let i = 0; i<subACLen; i++){
//         if(flagData.linkedTokens.ICP[i].account != flagData.linkedTokens.searched &&
//            flagData.linkedTokens.ICP[i].active == true) uniqueSA.push(flagData.linkedTokens.ICP[i]);
//     }
//     subACLen = uniqueSA.length;
//     if(subACLen == 0) showSubAccount = false;
//     else showSubAccount = true;
// }
if(flagData.knownAccount != "") showKnownName = true;
if(flagData.userKnownAccount != "") showUserKnownName = true;
if(flagData.mintRewards?.value > 0) showMinting = true;

let sP, sSA;
let isPrincipal = flagData?.searched?.includes("-");
if (isPrincipal == true){
    let parsed = parsePrincipalSubAccountString(flagData.searched);
    sP = parsed.principal;
    sSA = parsed.subaccount;
}else{
    sP = flagData.searched;
    sSA = "";
}

// MODAL STUFF
let showPopup = false;
const onShowPopup = (ev) => {
    showPopup = true;
}
const onPopupClose = (data) => {
    showPopup = false;
    //console.log(data); <-- any return data from modal
}

</script>

<div class="mainAlign ">
    Account Flags: 
    <table class="box tableAlign">
        {#if showKnownName == true}
            <tr class="subBox">
                <td class="contentLeft" style="color:aqua;">
                    Known Account : 
                <td class="contentRight" style="color:aqua;">
                    {flagData.knownAccount}
                </td>
            </tr>
        {/if}
        {#if showUserKnownName == true}
        <tr class="subBox">
            <td class="contentLeft" style="color:aqua;">
                User Saved Account : 
            <td class="contentRight" style="color:aqua;">
                {flagData.userKnownAccount}
            </td>
        </tr>
        {/if}
        {#if showReports == true}
            <tr class="subBox">
                <td class="contentLeft" style="color:aqua;">
                    Reports : 
                <td class="contentRight" style="color:aqua;">
                    {flagData.reports}
                </td>
            </tr>
        {/if}
        {#if showPrincipal == true}
            <tr class="subBox">
                <td class="contentLeft">
                    Principal : 
                <td class="contentRight">
                    {flagData.linkedTokens.principal}
                </td>
            </tr>
        {/if}
        {#if showSubAccount == true}
            <tr class="subBox">
                <td class="contentLeft">
                    ICP Sub-Accounts : 
                <td class="contentRight">
                    {subACLen}
                    <span class="buttonAlign">
                        <Button
                        slim={true}
                        type="grey"
                        on:click={onShowPopup}
                        >
                        View ICP Sub-Accounts
                        </Button>
                    </span>
                </td>
            </tr>
        {/if}
        {#if showNFT == true}
        <tr class="subBox">
                <td class="contentLeft">
                    Linked NFTs : 
                <td class="contentRight">
                    {flagData.nfts.number} 
                    <a class="buttonAlign" href="https://t5t44-naaaa-aaaah-qcutq-cai.raw.ic0.app/holder/{flagData.nfts.ID}/summary.html" target="_blank">
                        <Button
                            slim={true}
                            type="grey"
                        >
                         View NFTs
                        </Button>
                    </a>
                </td>
            </tr>
        {/if}
        {#if showMinting == true}
        <tr class="subBox">
            <td class="contentLeft">
                {flagData.mintRewards.text} 
            <td class="contentRight">
                {flagData.mintRewards.value}
            </td>
        </tr>
        {/if}
        {#if showLinkedTokens == true}
        <tr class="subBox">
            <td class="contentLeft">
                Linked Tokens : 
            <td class="contentRight">
                <LinkedTokenCarousel 
                    linkedTokenData={flagData.linkedTokens.ICRC}
                    mode={"clickThrough"}
                    searchedPrincipal={sP}
                    searchedSubAccount={sSA}
                />
            </td>
        </tr>
        {/if}
    </table>

    <!-- NOTHING TO SHOW -->
    {#if showPrincipal==false 
        && showSubAccount==false
        && showNFT==false
        && showMinting==false
        && showKnownName==false
        && showUserKnownName==false
        && showLinkedTokens==false
        && showReports==false
    }
    [][]-- This account has no flags --[][]
    {/if}


</div>

<Modal open={showPopup} title={"ICP Sub-Accounts :"} size={"large"} onClosed={() => onPopupClose()}> 
    
    <table style="width: 100%">
        {#each uniqueSA as ac, i}
        <tr>
            <td style="padding-right: 30px;"> <span class="fontSpan">{i+1} : </span></td>
            <td>
                <span class="fontSpan">{ac.account}</span>
                <a href="/search/ID/ICP?id={ac.account}" target="_blank"> <img class="search" src={search} alt="search" width="18px" style="margin-left:5px"/> </a>
                <CopyButton icrcAccount={false} text={ac.account}/>
                <SaveButton icrcAccount={false} text={ac.account}/>
                {#if ac.name != undefined}
                {@html "<br>"}
                <p style="margin-left:0px; margin-bottom:0px; color:black;">{ac.name}</p>
                {/if}
            </td>
        </tr>
        {/each}
    </table>
</Modal>

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
    .subBox{
        border: 1px;
        border-top: 0px;
        border-left: 0px;
        border-right: 0px;
        border-color: rgb(28, 28, 28);
        border-style: dotted;
    }
    .buttonAlign{
        margin-left: 30px;
    }
    .contentLeft{
        width: 33%;
        text-align: center;
        padding-right: 0px;
    }
    .contentRight{
        width: 100%;
        text-align: left;
        padding-left: 0px;
    }
    .contentTokens{
        width: 100%;
        text-align: center;
    }
    .logoAlign{
        margin-left: 15px;
    }
    .fontSpan{
        font-size: 12px;
        }
    @media (min-width: 992px) 
    {
        .fontSpan{
        font-size: 16px;
        }
    }
</style>