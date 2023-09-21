<script>
  import Button from "./button.svelte";
  import deleteIcon from "$lib/images/deleteIcon.png";
  import deleteIcon2 from "$lib/images/deleteIcon_white.png";
  import Modal from "./modal.svelte";
  import Loading from "./loading.svelte";
  import { deleteAccount } from '../code/addressBook.js';
  import { shortenString } from "../code/utils";
  export let text = '';
  export let text2 = '';
  export let name = '';
  export let icrcAccount;
  export let modeLight = false;

  //
  let InputAcName = "";
  let accountID = "";
  let loading = false;
  if (icrcAccount == 'true' || icrcAccount == true)  {
      accountID = text+"."+text2;
      if (text2 != ''){
        text2 = shortenString(text2);
      }
  } else {
      accountID = text;
      text = shortenString(text);
  }
  let infoBoxMode = 0;

  function saveAccountIconClick(){
    showPopup = true;
  }

  async function deleteClicked(){
    loading = true;
    let res = await deleteAccount(accountID);
    if (res == "Not Logged In!") infoBoxMode = -2;
    if (res == "Account removed from directory") infoBoxMode = 1;
    loading = false;
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
  
  <button on:click={() => saveAccountIconClick(text, text2, icrcAccount)}>
    {#if modeLight == false} 
      <img class="save" src={deleteIcon} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {:else}
      <img class="save" src={deleteIcon2} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {/if}
    
  </button>

  <Modal open={showPopup} title={"Delete Account"} size={"medium"} onClosed={() => onPopupClose()}> 

    <div style="padding: 10px;">
      <div class="form-group row">
          {#if icrcAccount == true || icrcAccount == "true"}
              <p> 
                  <span class="text-info">{name}</span>
                  {@html "<br>"} 
                  Principal: {text} 
                  {@html "<br>"} 
                  Sub-Ac: {text2}
              </p>
          {:else}
            <span class="text-info">{name}</span>
            {@html "<br>"} 
              <p>Account: {text}</p>
          {/if}
  
          {#if loading == false}
            {#if infoBoxMode == -1}
                <div class="warnText">Save Error</div>
            {:else if infoBoxMode == -2}
              <div class="warnText">Save Error - Not logged in!</div>
            {:else if infoBoxMode == 1}
                <div class="okText ">Account removed from Address Book</div>
            {/if}
          {:else}
              <div style="padding-top: 20px;"><Loading style={'loaderBlue'} align={'centre'}/></div>
          {/if}
          <table style="width: 100%; margin-top:20px;">
            <tr>
              <td style="width: 80%;"></td>
              <td style="text-align: right;"> <Button type="green" on:click={() => deleteClicked()}>Delete</Button> </td>
              <td style="text-align: right; padding-right: 20px;"> <Button type="grey" on:click={() => onPopupClose()}>Cancel</Button> </td>
            </tr>
          </table>
      </div>
    </div>
  </Modal>
  
  <style>
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

    button{
      cursor: pointer;
      background: none;
      padding: 0px;
      margin: 0px;
      border: 0px;
    }
  </style>