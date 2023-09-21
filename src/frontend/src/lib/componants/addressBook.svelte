<script>
    // import Modal from "./modal.svelte";
    // import Loading from "./loading.svelte";
     import { saveAccount, getAllAccounts } from '../code/addressBook.js';
     import { DEFAULT_SUBACCOUNT } from '../code/constants.js';
     import { combinePrincipalSubAccount, parsePrincipalSubAccountString } from '../code/utils.js';
     import Button from "../shared/button.svelte";
     import Loading from "../shared/loading.svelte";
     import AddressBookTable from './addressBookTable.svelte';
     import Fuse from 'fuse.js';

    let btn1Colour = "orange";
	let btn2Colour = "blueTP";
    let btn3Colour = "blueTP";
    let tab = 0;
    let name;
    let account;
    let subAccount;
    let loading = false;
    let searchComplete = false;
    let infoBoxMode = 0;
    let searchName;
    let searchAccount;
    let allAccountsProcessed;
    let searchRes;
    let searchResProcessed;

    async function toggle(num){
        if (num == 0){
            btn1Colour = "orange";
            btn2Colour = "blueTP";
            btn3Colour = "blueTP";
            tab = 0;
        }
        if (num == 1){
            btn1Colour = "blueTP";
            btn2Colour = "orange";
            btn3Colour = "blueTP";
            tab = 1;
            await getAll();
        }
        if (num == 2){
            btn1Colour = "blueTP";
            btn2Colour = "blueTP";
            btn3Colour = "orange";
            tab = 2;
            await getAll();
        }
    }

    async function saveInput(){
        // check if name or account is blank
        if (name == undefined || account == undefined) {
            infoBoxMode = -3;
            return;
        }

        // check if account or principal 
        if (account.includes("-")){
            if(subAccount == "" || subAccount == null || subAccount == undefined){
                subAccount = DEFAULT_SUBACCOUNT;
            }
            //principal
            let combinedAC = combinePrincipalSubAccount(account, subAccount);
            loading = true;
            let res = await saveAccount(combinedAC, name);
            if (res == "Not Logged In!") infoBoxMode = -2;
            if (res == "Address book updated with new entry") infoBoxMode = 1;
            loading = false;
        } else {
            // account
            loading = true;
            let res = await saveAccount(account, name);
            if (res == "Not Logged In!") infoBoxMode = -2;
            if (res == "Address book updated with new entry") infoBoxMode = 1;
            loading = false;
        }
        
    }

    function clearInput(){
        account = undefined;
        name = undefined;
        subAccount = undefined;
        infoBoxMode = 0;
    }

    function clearSearchInput(){
        searchName = "";
        searchAccount = "";
        searchComplete = false;
    }
    async function searchInput(){
        searchComplete = false;
        // uses allAccountsProcessed
        const fuseOptions = {
            isCaseSensitive: false,
            keys: [
                "account",
                "name"
            ]
        };
        let searchTerm; 
        if(searchName == undefined || searchName == ""){
            searchTerm = searchAccount;
        } else {
            searchTerm = searchName;
        }
        const fuse = new Fuse(allAccountsProcessed, fuseOptions);
        searchRes = fuse.search(searchTerm)

        let searchResLen = searchRes.length ?? 0;
        searchResProcessed = [];
        for(let i = 0; i<searchResLen; i++){

            searchResProcessed.push({
                count: searchRes[i].item.count,
                isICRC: searchRes[i].item.isICRC,
                name: searchRes[i].item.name,
                account: searchRes[i].item.account,
                subaccount: searchRes[i].item.subaccount
            });
        }
        searchComplete = true;
    }


    async function getAll(){
        loading = true;
        let resACS = await getAllAccounts();
        let resACSLen = resACS[0]?.length ?? 0;
        let ac, sa, parse, isICRC; 
        let count = 1;
        let combined;
        allAccountsProcessed = [];
        for(let i=0; i<resACSLen; i++){
            combined = resACS[0][i][0];
            isICRC = false;
            if(combined.includes(".")){
                parse = parsePrincipalSubAccountString(resACS[0][i][0])
                ac = parse.principal;
                sa = parse.subaccount;
                isICRC = true;
            } else {
                ac = resACS[0][i][0];
            }
            allAccountsProcessed.push(
                {   
                    count,
                    isICRC,
                    name: resACS[0][i][1],
                    account: ac,
                    subaccount: sa
                }
            );
            count++;
        }
        loading = false;
    }


</script>

<div>
    <table style="width: 100%;">
        <tr>
            <td>
            <Button slim="true" type={btn1Colour} on:click={()=>{toggle(0)}}>Add Entry</Button>
            <Button slim="true" type={btn2Colour} on:click={()=>{toggle(1)}}>Search</Button>
            <Button slim="true" type={btn3Colour} on:click={()=>{toggle(2)}}>Show All</Button>
        </td>
        </tr>
        <tr>
            <td style="padding-top: 10px; width: 100%">
                {#if tab == 0}

                    <table style="width: 100%;">
                        <tr>
                            <td style="width: 25%;" class="alignRight">
                                Name :
                            </td>
                            <td>
                                <div style="padding-top: 5px">
                                    <input 
                                    id="acSave" 
                                    class="form-control form-control-sm" 
                                    alt="Name for Saved Account" 
                                    placeholder="Account Name (Max 40 Characters)"
                                    type="text"
                                    maxlength=40
                                    pattern="[a-zA-Z0-9]+"
                                    bind:value={name} 
                                    >
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td style="width: 25%;" class="alignRight">
                                Account :
                            </td>
                            <td>
                                <div style="padding-top: 5px">   
                                    <input 
                                    id="inputAccount" 
                                    class="form-control form-control-sm" 
                                    alt="Input Account or Principal" 
                                    placeholder="Account/ Principal" 
                                    type="text" 
                                    bind:value={account}
                                    >
                                </div>
                            </td>
                        </tr>
                        <tr>
                            <td style="width: 25%;" class="alignRight">
                                Sub-Account (Optional) :
                            </td>
                            <td>
                                <div style="padding-top: 5px">   
                                    <input 
                                    id="acSearch" 
                                    class="form-control form-control-sm" 
                                    alt="Input Sub Account - Optional" 
                                    placeholder="Sub Account (For ICRC Standard)" 
                                    type="text" 
                                    bind:value={subAccount}
                                    >
                                </div> 
                            </td>
                        </tr>
                    </table>
                
                    <div>
                        {#if loading == false}
                        {#if infoBoxMode == -1}
                            <div class="warnText">Save Error</div>
                        {:else if infoBoxMode == -2}
                        <div class="warnText">Save Error - Not logged in!</div>
                        {:else if infoBoxMode == -3}
                        <div class="warnText">Name and Account must have a value</div>
                        {:else if infoBoxMode == 1}
                            <div class="okText">Address book updated with new entry</div>
                        {/if}
                    {:else}
                        <div style="padding-top: 20px;"><Loading style={'loaderBlue'} align={'centre'}/></div>
                    {/if}
                    <table style="width: 100%; margin-top:20px;">
                        <tr>
                        <td style="width: 90%;"></td>
                        <td style="text-align: right;"> <Button type="green" on:click={() => {saveInput()}}>Save</Button> </td>
                        <td style="text-align: right; padding-right: 10px"> <Button type="grey" on:click={() => {clearInput()}}>Clear</Button> </td>
                        </tr>
                    </table>
                    </div>
                {:else if tab ==1}
                    {#if loading == true}
                        <div style="padding-top: 20px;"><Loading style={'loaderBlue'} align={'centre'}/></div>
                    {:else}
                        <table style="width: 100%;">
                            <tr>
                                <td style="width: 25%;" class="alignRight">
                                    Search by Name : 
                                </td>
                                <td>
                                    <div style="padding-top: 5px">
                                        <input 
                                        id="acSearch" 
                                        class="form-control form-control-sm" 
                                        alt="Search by name input" 
                                        placeholder=""
                                        type="text"
                                        maxlength=40
                                        pattern="[a-zA-Z0-9]+"
                                        bind:value={searchName} 
                                        >
                                </td>
                            </tr>
                            <tr>
                                <td style="width: 25%;" class="alignRight">
                                    Search by Account :
                                </td>
                                <td>
                                    <div style="padding-top: 5px">
                                        <input 
                                        id="acSearch" 
                                        class="form-control form-control-sm" 
                                        alt="Search by name input" 
                                        placeholder=""
                                        type="text"
                                        bind:value={searchAccount} 
                                    >
                                </td>
                            </tr>
                        </table>
                        <table style="width: 100%; margin-top:20px;">
                            <tr>
                            <td style="width: 90%;"></td>
                            <td style="text-align: right; padding-right: 10px"> <Button type="green" on:click={() => {searchInput()}}>Search</Button> </td>
                            <td style="text-align: right; padding-right: 10px"> <Button type="grey" on:click={() => {clearSearchInput()}}>Clear</Button> </td>
                            </tr>
                        </table>

                        {#if searchComplete == true}
                            <AddressBookTable data={searchResProcessed}/>
                        {/if}
                    {/if}
                {:else if tab ==2}
                    {#if loading == true}
                        <div style="padding-top: 20px;"><Loading style={'loaderBlue'} align={'centre'}/></div>
                    {:else}
                        <AddressBookTable data={allAccountsProcessed}/>
                    {/if}
                {/if}
            </td>
        </tr>
    </table>
</div>

<!-- Delete Modal -->
<!-- <Modal open={showPopup} title={""} size={"medium"} position={'top'} onClosed={() => onPopupClose()}> 
    
</Modal> -->

<style>
    .underline{
        padding: 2px;
        border: 0;
        border-bottom: 2px;
        border-style: solid;
        border-color: #09cbf1;
    }
    .noUnderline{
        padding: 0px;
    }
    .warnText{
        color:white;
        background-color: red; 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
    .okText{
        color:white;
        background-color: rgb(130, 234, 26); 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
    .alignRight{
        text-align: right;
        padding-right: 20px;
    }

</style>