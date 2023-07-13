<script>
  import saveIcon from "$lib/images/saveIcon.png";
  import Modal from "./modal.svelte";
  export let text = '';
  export let text2 = '';
  export let icrcAccount;

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
    <img class="save" src={saveIcon} alt="SaveIcon" width="16px" style="margin-left:5px"/>
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