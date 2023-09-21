<script>
  import { browser } from '$app/environment';
  import { onMount } from "svelte";
  // tools images
  import mChat from '$lib/images/members/MemberChat.png';
  import visual from '$lib/images/members/VisualExplorer.png';

  // Stats images 
  import genesis from '$lib/images/members/icpGenesis.png';
  import exchange from '$lib/images/members/exchangeBalance.png';

  export let mode = "tools"; // "tools", "stats", "research",  
  let pos;
  let projectArray = [];
  let startIndex = 0;
  let showNumber = 0;
  let totalLinks;
  let showArray = [];
  let showButtons = false;
  let screenWidth;



    onMount(()=>{
    if(mode == "tools"){
      projectArray = [
        {id: 0, title: "Members Chat", linkUrl: "/members/chat", image: mChat},
        {id: 1, title: "Visual Explorer", linkUrl: "/explore/visualblocks/icp", image: visual},
      ]
      totalLinks = projectArray?.length ?? 0;
    }

    if(mode == "stats"){
      projectArray = [
        {id: 0, title: "Genesis A/Cs", linkUrl: "/members/genesisData", image: genesis},
        {id: 1, title: "Exchange Balances", linkUrl: "/members/exchangeBalances", image: exchange},
      ]
      totalLinks = projectArray?.length ?? 0;
    }

    // first run
    if(screenWidth < 935){
      if (totalLinks < 3){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
      showNumber = 3;
      showButtons = true;
      }
    }
    if(screenWidth < 1400 && screenWidth >= 935){
      if (totalLinks < 4){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
      showNumber = 4;
      showButtons = true;
      }
    }
    if(screenWidth >= 1400){
      if (totalLinks < 5){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
      showNumber = 5;
      showButtons = true;
      }
    }
    // init show array
    showArray = [];
    pos = startIndex;
    for(let i = 0; i<showNumber; i++){
      if (pos >= totalLinks) pos = 0;
      showArray.push(projectArray[pos]);
      pos++;
    }
  });
 

  if (browser) {
    screenWidth = window.innerWidth;
    const handleResize = () => {
      screenWidth = window.innerWidth;
      // resize event
      if(screenWidth < 935){
        if (totalLinks < 3){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
          showNumber = 3;
          showButtons = true;
         }
      }
      if(screenWidth < 1400 && screenWidth >= 935){
        if (totalLinks < 4){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
          showNumber = 4;
          showButtons = true;
         }
      }
      if(screenWidth >= 1400){
        if (totalLinks < 5){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
          showNumber = 5;
          showButtons = true;
         }
      }
      showArray = [];
      let pos = startIndex;
      for(let i = 0; i<showNumber; i++){
        if (pos >= totalLinks) pos = 0;
        showArray.push(projectArray[pos]);
        pos++;
      }
    };
    window.addEventListener('resize', handleResize);
  }
  
  const next = () => {
    let diff = Math.abs(totalLinks - showNumber);
    let add;
    if (diff > showNumber) {add = showNumber} else {add = diff};
    startIndex = (startIndex + add) % totalLinks
    showArray = [];
    pos = startIndex;
    for(let i = 0; i<showNumber; i++){
      if (pos >= totalLinks) pos = 0;
      showArray.push(projectArray[pos]);
      pos++;
    }
  };

  const prev = () => {
    let diff = Math.abs(totalLinks - showNumber);
    let add;
    if (diff > showNumber) {add = showNumber} else {add = diff};
    startIndex = (((startIndex - add) % totalLinks) + totalLinks) % totalLinks;
    showArray = [];
    pos = startIndex;
    for(let i = 0; i<showNumber; i++){
      if (pos >= totalLinks) pos = 0;
      showArray.push(projectArray[pos]);
      pos++;
    }
  };

</script>
<div style="padding:15px;">
  <div class="card-container">
    {#if showButtons == true}
      <button class="prev" on:click={prev}>{@html "&#10094;"}</button>
    {/if}
      {#each showArray as PR}
        <a href={PR.linkUrl}>
            <div class="card">
              <img class="headAlign" src={PR.image} alt="" width="100%"/>  
              <span class="title">{PR.title}</span>
            </div>
        </a>
      {/each}
    {#if showButtons == true}
    <button class="next" on:click={next}>{@html "&#10095;"}</button>
  {/if}
  </div>
</div>

<style>
  .card-container {
      display: flex;
      flex-wrap: wrap;
      justify-content: space-around;
  }
  .card {
      background-color: #d2f5f9f1;
      margin: 5px;
      padding: 5px;
      box-shadow: 0px 0px 7px 0 rgba(249, 20, 207, 0.968);
      border-radius: 5px;
      width: 140px;
      height: 180px;
      text-align: center;
      align-items: center;
      overflow: hidden;
  }

  .title {
    position: absolute;
    top: 65%;  
    left: 0;
    right: 0;
    background-color: rgba(255, 255, 255, 0.8); /* Semi-transparent background for better readability */
    padding: 10px;
    box-sizing: border-box;
    text-align: center;
    overflow: hidden;
    text-overflow: ellipsis;
    /* white-space: nowrap; Ensures title stays on one line and doesn't wrap */
    max-width: 100%; /* Ensures title doesn't overflow card width */
    font-weight: bold;
  }
  a {
      color: rgb(180, 180, 180);
  }


.prev, .next {

  background: #f1f1f129;
  border: #8f8f8f;
  border-width: 1px;
  border-radius: 3px;
  border-style: solid;
  color: white;
  cursor: pointer;
  width: auto;
  padding: 16px;
  font-weight: bold;
  font-size: 18px;
  user-select: none;
}


</style>