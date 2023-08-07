<script>
  import saveIcon from "$lib/images/saveIcon.png";
  import saveIcon2 from "$lib/images/saveIcon_white.png";
  import Modal from "./modal.svelte";
  export let text = '';
  export let text2 = '';
  export let icrcAccount;
  export let modeLight = false;

  function saveAccount(text, text2, isICRC){
    showPopup = true;
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
  
  <button on:click={() => saveAccount(text, text2, icrcAccount)}>
    {#if modeLight == false} 
      <img class="save" src={saveIcon} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {:else}
      <img class="save" src={saveIcon2} alt="SaveIcon" width="16px" style="margin-left:5px"/>
    {/if}
    
  </button>

  <Modal open={showPopup} title={"Save Account"} size={"large"} onClosed={() => onPopupClose()}> 
    SAVE ACCOUNT HERE -- Work in Progress
  </Modal>
  
  <style>
    button{
      cursor: pointer;
      background: none;
      padding: 0px;
      margin: 0px;
      border: 0px;
    }
  </style>