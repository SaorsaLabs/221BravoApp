<script>
import Button from "../shared/button.svelte";
import Modal from "../shared/modal.svelte";
import search from "$lib/images/search.png";
import icpLogo from '$lib/images/icpLogo.png';
import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
import chatLogo from '$lib/images/Openchat_logo.png';
import sns1Logo from '$lib/images/SNS1_Logo.png';
import SaveButton from "../shared/saveButton.svelte";
import CopyButton from "../shared/copyButton.svelte";

export let flagData = {};

let showPrincipal = false;
let showSubAccount = false;
let showNFT = false;
let showMinting = false;
let showKnownName = false;
let showUserKnownName = false; 
let showLinkedTokens = false;

if(flagData.linkedTokens?.principal) showPrincipal = true;
if(flagData.nfts?.number > 0) showNFT = true;
let subACLen = 0;
let uniqueSA = [];
if(flagData.linkedTokens?.ICP.length > 0) {
    subACLen = flagData.linkedTokens.ICP.length;
    for(let i = 0; i<subACLen; i++){
        if(flagData.linkedTokens.ICP[i].account != flagData.linkedTokens.searched &&
           flagData.linkedTokens.ICP[i].active == true) uniqueSA.push(flagData.linkedTokens.ICP[i]);
    }
    subACLen = uniqueSA.length;
    if(subACLen == 0) showSubAccount = false;
    else showSubAccount = true;
}
if(flagData.knownAccount != "") showKnownName = true;
if(flagData.userKnownAccount != "") showUserKnownName = true;
if(flagData.mintRewards.value > 0) showMinting = true;
if(flagData.linkedTokens != "") showLinkedTokens = true;

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
                <div class="row">
                    {#if flagData.linkedTokens?.ICP?.length > 0}
                    <div class="col">
                        <img class="logoAlign" style="padding-top:11px" src={icpLogo} alt="ICP Logo" width="40px"/>
                        <div style="padding-left:15px">
                        ICP
                        <a href="/search/ID/ICP?id={flagData.linkedTokens.searched}" target="_blank"> <img class="search" src={search} alt="search" width="20px" /> </a>
                        </div>
                    </div>
                    {/if}
                    {#if flagData.linkedTokens.ckBTC == true}
                    <div class="col">
                        <img class="logoAlign" src={ckBTCLogo} alt="CKBTC Logo" width="35px"/>
                        <div style="padding-left:5px">
                        ckBTC
                        <a href="/search/ID/CKBTC?id={flagData.linkedTokens.searched}" target="_blank"> <img class="search" src={search} alt="search" width="20px" /> </a>
                        </div>
                    </div>
                    {/if}
                    {#if flagData.linkedTokens.CHAT == true}
                    <div class="col">
                        <img class="logoAlign" src={chatLogo} alt="Chat Logo" width="35px"/>
                        <div style="padding-left:8px">
                        Chat
                        <a href="/search/ID/CHAT?id={flagData.linkedTokens.searched}" target="_blank"> <img class="search" src={search} alt="search" width="20px" /> </a>
                        </div>
                    </div>
                    {/if}
                    {#if flagData.linkedTokens.SNS1 == true}
                    <div class="col">
                        <img class="logoAlign" src={sns1Logo} alt="SNS1 Logo" width="35px"/>
                        <div style="padding-left:8px">
                        SNS1
                        <a href="/search/ID/SNS1?id={flagData.linkedTokens.searched}" target="_blank"> <img class="search" src={search} alt="search" width="20px" /> </a>
                        </div>
                    </div>
                    {/if}
                </div>
            </td>
        </tr>
        {/if}
    </table>
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