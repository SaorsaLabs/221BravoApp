<script>
  import copy from "$lib/images/copyIcon_sml.png";
  export let text = '';
  export let text2 = '';
  export let icrcAccount;

  let bgChange = false;

    function setClipboard(text, text2, icrcAccount) {
    const type = "text/plain";
    let writeText = '';
    if (icrcAccount == false || !icrcAccount) {
      writeText = text;
    } else {
      writeText = text+"."+text2;
    }

    const blob = new Blob([writeText], { type });
    const data = [new ClipboardItem({ [type]: blob })];

      navigator.clipboard.write(data).then(
        () => {
          bgChange = true;
          setTimeout(resetColour, 500)
        },
        () => {
          bgChange = false;
        }
      );
    }
    function resetColour() {
      bgChange = false;
    }
  </script>
  
  <button on:click={() => setClipboard(text, text2, icrcAccount)}>
    <img class="copy" src={copy} class:BG={bgChange} alt="copyText" width="20px" style="margin-left:5px"/>
  </button>

  
  <style>
    button{
      cursor: pointer;
      background: none;
      padding: 0px;
      margin: 0px;
      border: 0px;
    }
    .BG{
      background-color: rgb(40, 202, 40);
      border-radius: 4px;
    }
  </style>