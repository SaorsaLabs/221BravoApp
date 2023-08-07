<script>
import Button from "../../shared/button.svelte";
import TopTxTable from "./topTxTable.svelte";
import ModalDark from "../../shared/modalDark.svelte";

export let token = "Tokens";
export let data = [];
export let is_icrc = true;

let icrc = is_icrc;

// MODAL STUFF
let showPopup = false;
let showPopup2 = false;

const onShowPopup = (ev) => {
    showPopup = true;
}
const onPopupClose = (data) => {
    showPopup = false;
    //console.log(data); <-- any return data from modal
}

const onShowPopup2 = (ev) => {
    showPopup2 = true;
}
const onPopupClose2 = (data) => {
    showPopup2 = false;
    //console.log(data); <-- any return data from modal
}

</script>

<div style="width:100%;">
    <div style="padding:10px">
        <Button slim={false} type={"blueTP"} on:click={()=>{onShowPopup()}}>View Top Mint Transactions</Button>
    </div>
    <div style="padding:10px">
        <Button slim={false} type={"blueTP"} on:click={()=>{onShowPopup2()}}>View Top Burn Transactions</Button>
    </div>
</div>

<ModalDark open={showPopup} title={"Mint Transactions:"} size={"extraLarge"} onClosed={() => onPopupClose()}> 
    <TopTxTable txData={data.mint} is_icrc={icrc}/>	
</ModalDark> 
<!-- data={topAccounts} isIcrc={true} showSubAccounts={true} token={"CKBTC"} -->

<ModalDark open={showPopup2} title={"Burn Transactions:"} size={"extraLarge"} onClosed={() => onPopupClose2()}> 
    <TopTxTable txData={data.burn} is_icrc={icrc}/>	
</ModalDark>

<style>
   
</style>