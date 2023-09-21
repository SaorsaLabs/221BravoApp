<script>
  import Button from "./button.svelte";
  import saveIcon from "$lib/images/saveIcon.png";
  import saveIcon2 from "$lib/images/saveIcon_white.png";
  import Modal from "./modal.svelte";
  import Loading from "./loading.svelte";
  import { saveAccount } from '../code/addressBook.js';
  export let text = '';
  export let text2 = '';
  export let icrcAccount;
  export let modeLight = false;

  //
  let InputAcName = "";
  let accountID = "";
  let loading = false;
  if (icrcAccount == 'true' || icrcAccount == true)  {
      accountID = text+"."+text2;
  } else {
      accountID = text;
  }
  let infoBoxMode = 0;
  let saveResult;

  function saveAccountIconClick(){
    showPopup = true;
    InputAcName = "";
    infoBoxMode = 0;
  }

  async function saveClicked(){
    loading = true;
    let res = await saveAccount(accountID, InputAcName);
    if (res == "Not Logged In!") infoBoxMode = -2;
    if (res == "Address book updated with new entry") infoBoxMode = 1;
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
      <img class="save" src={saveIcon} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {:else}
      <img class="save" src={saveIcon2} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {/if}
    
  </button>

  <Modal open={showPopup} title={"Save Account"} size={"large"} onClosed={() => onPopupClose()}> 

    <div style="padding: 10px;">
      <div class="form-group row">
          <p> Add to address book : </p>
          {#if icrcAccount == true || icrcAccount == "true"}
              <p> <span class="text-info">Principal: </span> {text} {@html "<br>"} 
                  <span class="text-info">Sub-Ac:</span> {text2}
              </p>
          {:else}
              <p><span class="text-info">Account: </span> {text}</p>
          {/if}
          <label for="acSave" class="col-sm-3 col-form-label-sm">Name (max 40 characters) : </label>
          <div class="col-sm-9">
            <input 
                id="acSave" 
              class="form-control form-control-sm" 
              alt="Name for Saved Account" 
              placeholder="Account Name"
              type="text"
              maxlength=40
              pattern="[a-zA-Z0-9]+"
              bind:value={InputAcName} 
            >
          </div>
          {#if loading == false}
            {#if infoBoxMode == -1}
                <div class="warnText">Save Error</div>
            {:else if infoBoxMode == -2}
              <div class="warnText">Save Error - Not logged in!</div>
            {:else if infoBoxMode == 1}
                <div class="okText">Address book updated with new entry</div>
            {/if}
          {:else}
              <div style="padding-top: 20px;"><Loading style={'loaderBlue'} align={'centre'}/></div>
          {/if}
          <table style="width: 100%; margin-top:20px;">
            <tr>
              <td style="width: 80%;"></td>
              <td style="text-align: right;"> <Button type="green" on:click={() => saveClicked()}>Save</Button> </td>
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