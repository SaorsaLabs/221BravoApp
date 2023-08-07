<script>
  import { fade, fly } from "svelte/transition";
  import { quintOut } from "svelte/easing";
  import Button from "./button.svelte";

  export let open = false;
  export let showBackdrop = true;
  export let onClosed;
  export let title = 'Modal title';
  export let size = 'extraLarge';
  export let position = 'centre';

  let modalSize; 
  if(size == "extraLarge") modalSize = "modal-xl";
  if(size == "large") modalSize = "modal-lg";
  if(size == "medium") modalSize = "";
  if(size == "small") modalSize = "modal-sm";

  let modalPosition;
  if(position == 'centre') modalPosition = 'modal-dialog-centered';
  if(position == 'top') modalPosition = '';

  const modalClose = (data) => {
    open = false;
    if (onClosed) {
      onClosed(data);
    }
  }

</script>

{#if open}
  <div class="modal" id="popupModal" tabindex="-1" role="dialog" aria-labelledby="popupModal" aria-hidden={false}>
    <div class="modal-dialog {modalPosition} {modalSize}" role="document" in:fly={{ y: -50, duration: 300 }} out:fly={{ y: -50, duration: 300, easing: quintOut }}>
      
      <div class="modal-content">
          <div>
            <table class="headTable">
              <tr>
                <td class="titleDiv">
                  {title}
                </td>
                <td class="btnDiv">
                  <Button type={"blueTP"} on:click={() => modalClose('close')}> 
                    X</Button>
                </td>
              </tr>
            </table>
          </div>

          <div class="slotDiv">
            <slot></slot>
          </div>
          
      </div>
    </div>
  </div>
  {#if showBackdrop}
    <div class="modal-backdrop show" transition:fade={{ duration: 150 }} />
  {/if}
{/if}

<style>
  .box{
    border-color: white;
    border-style: dashed;
    border-width: 2px;
  }
  .headTable{
    width:100%;
    border: 0;
    border-bottom: 2px;
    border-color: rgba(50, 50, 50, 0.5);
    border-style: solid;
  }
  .slotDiv{
    padding: 5px;
  }
  .btnDiv{
    width: 100%;
    text-align: right;
    padding: 5px;
  }
  .titleDiv{
    text-align: left;
    padding: 5px;
    padding-left: 10px;
    width: 95%;
    font-weight: 700;
    font-size: 1.25rem;
  }
  .modal {
    display: block;
  }
  .modal-backdrop {
  --bs-backdrop-zindex: 1050;
  --bs-backdrop-bg: #000000;
  --bs-backdrop-opacity: 0.45;
  position: fixed;
  top: 0;
  left: 0;
  z-index: var(--bs-backdrop-zindex);
  width: 100vw;
  height: 100vh;
  background-color: var(--bs-backdrop-bg);
}
.modal-backdrop.fade {
  opacity: 0;
}
.modal-backdrop.show {
  opacity: var(--bs-backdrop-opacity);
}
.modal-content {
  /* position: relative;
  display: flex;
  flex-direction: column; */
  width: 100%;
  color: rgb(255, 255, 255);
  pointer-events: auto;
  background: #06beb6;  
	background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  
	background: linear-gradient(to right, #89318f, #09cbf1); 
  /* background-color: rgba(200, 200, 200, 1); */
  /* background-clip: content-box; */
  border: 0;
  border-radius: 5px;
  outline: 0;
}
.modal-body {
  padding: 0px;
}
</style>