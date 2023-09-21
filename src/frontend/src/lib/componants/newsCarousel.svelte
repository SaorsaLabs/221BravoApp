<script>
    import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
    import icpLogo from '$lib/images/icpLogo.png';
    import { browser } from '$app/environment';
    import { get_all_news } from '../code/fetchContent.js';
    import Loading from '../shared/loading.svelte';

    // export let Mode = ""; 
    let pos;
    let newsArray = [];
    let promise = loadNews();
    async function loadNews(){
      newsArray = await get_all_news();
      totalLinks = newsArray?.length ?? 0;
      // first run
      if(screenWidth < 700 ){
        if (totalLinks < 1){ 
          showNumber = totalLinks;
          showButtons = false;
        } else { 
        showNumber = 1;
        showButtons = true;
        }
      }
      if(screenWidth < 935 && screenWidth >= 700){
        if (totalLinks < 2){ 
          showNumber = totalLinks;
          showButtons = false;
        } else { 
        showNumber = 2;
        showButtons = true;
        }
      }
      if(screenWidth < 1400 && screenWidth >= 935){
        if (totalLinks < 3){ 
          showNumber = totalLinks;
          showButtons = false;
        } else { 
        showNumber = 3;
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
        showArray.push(newsArray[pos]);
        pos++;
      }
    }
   


    let startIndex = 0;
    let showNumber = 0;
    let totalLinks;
    let showArray = [];
    let showButtons = false;

    let screenWidth;
    if (browser) {
      screenWidth = window.innerWidth;
      const handleResize = () => {
        screenWidth = window.innerWidth;
        // resize event
        if(screenWidth < 700 ){
          if (totalLinks < 1){ 
              showNumber = totalLinks;
              showButtons = false;
            } else { 
            showNumber = 1;
            showButtons = true;
           }
        }
        if(screenWidth < 935 && screenWidth >= 700){
          if (totalLinks < 2){ 
              showNumber = totalLinks;
              showButtons = false;
            } else { 
            showNumber = 2;
            showButtons = true;
           }
        }
        if(screenWidth < 1400 && screenWidth >= 935){
          if (totalLinks < 3){ 
              showNumber = totalLinks;
              showButtons = false;
            } else { 
            showNumber = 3;
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
          showArray.push(newsArray[pos]);
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
        showArray.push(newsArray[pos]);
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
        showArray.push(newsArray[pos]);
        pos++;
      }
    };

  </script>
  <div style="padding:15px;">
    <div class="card-container">
      {#if showButtons == true}
        <button class="prev" on:click={prev}>{@html "&#10094;"}</button>
      {/if}

      {#await promise}
      <!-- Loading Content -->
      <Loading/>
      {:then}
        {#each showArray as NI}
          <a href={NI.article_url} target="_blank">
              <div class="card">
                <!-- NI.image_url -->
                <img class="headAlign" src={NI.image_url} alt="" width="100%"/>  
                <span class="title">{NI.title}</span>
                <!-- <span>{NI.sub_title}</span> -->
              </div>
          </a>
        {/each}
      {/await}
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
        width: 180px;
        height: 250px;
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