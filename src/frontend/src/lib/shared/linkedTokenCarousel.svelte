<script>
    import { browser } from '$app/environment';
    import { parsePrincipalSubAccountString } from '../code/utils';
    import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
    import chatLogo from '$lib/images/Openchat_logo.png';
    import kinicLogo from '$lib/images/kinic.png';
    import hotLogo from '$lib/images/hotOrNot.png';
    import ghostLogo from '$lib/images/ghost_logo.png';
    import icpLogo from '$lib/images/icpLogo.png';
    import snsLogo from '$lib/images/SNS1_Logo.png';
    import modLogo from '$lib/images/projectLogos/ModClub.png';
    import catLogo from '$lib/images/projectLogos/catalyzeLogo.png';
    import boomLogo from '$lib/images/projectLogos/BoomLogo.png';
    import icxLogo from '$lib/images/projectLogos/IcxLogo.png';
    
    export let linkedTokenData;
    export let searchedPrincipal = "";
    export let searchedSubAccount = "";
    export let mode = "standard"; // standard, clickThrough, homePage

    let allLinks = [];
    allLinks = [
    {idx: 0, image: ckBTCLogo, text: "ckBTC", ticker: "CKBTC", link: "/search/ckbtc"},
    {idx: 1, image: chatLogo, text: "CHAT", ticker: "CHAT", link: "/search/chat"},
    {idx: 2, image: kinicLogo, text: "KINIC", ticker: "KINIC", link: "/search/kinic"},
    {idx: 3, image: hotLogo, text: "HOT", ticker: "HOT", link: "/search/hot"},
    {idx: 4, image: ghostLogo, text: "GHOST", ticker: "GHOST", link: "/search/ghost"},
    {idx: 5, image: icpLogo, text: "ICP", ticker: "ICP", link: "/search/icp"},
    {idx: 6, image: snsLogo, text: "Dragginz", ticker: "SNS1", link: "/search/sns1"},
    {idx: 7, image: modLogo, text: "ModClub", ticker: "MODCLUB", link: "/search/modclub"},
    {idx: 8, image: catLogo, text: "Catalyze", ticker: "CAT", link: "/search/cat"},
    {idx: 9, image: boomLogo, text: "BoomDAO", ticker: "BOOM", link: "/search/boom"},
    {idx: 10, image: icxLogo, text: "IC-X", ticker: "ICX", link: "/search/icx"},
    ];

    let icpAccount = "";
    if (mode == "homePage"){
        icpAccount = searchedSubAccount;
        if(searchedPrincipal.includes(".")){
          let parse = parsePrincipalSubAccountString(searchedPrincipal);
          searchedPrincipal = parse.principal;
          searchedSubAccount = parse.subaccount;
        }
    }
    let activeLinks = [];
    let keys; 
    if (linkedTokenData){
        keys = Object.keys(linkedTokenData);
        let keyLen = keys?.length ?? 0;
        let allLen = allLinks?.length ?? 0;
        for(let i=0; i<keyLen; i++){
            for(let k=0; k<allLen; k++){
                if(keys[i] == allLinks[k].ticker){
                    activeLinks.push(allLinks[k])
                }
            }
        }
    }
    let totalLinks = activeLinks?.length ?? 0; 
    let startIndex = 0;
    let showNumber = 5;
    let showArray = [];
    let showButtons = false;
    
    // adjust number visible by screen width
    let screenWidth;
    if (browser) {
      screenWidth = window.innerWidth;
      const handleResize = () => {
        screenWidth = window.innerWidth;
        // resize event
        if(screenWidth < 740 ){
          if (totalLinks < 3){ 
            showNumber = totalLinks;
            showButtons = false;
           } else { 
            showNumber = 3;
            showButtons = true;
           }
        }
        if(screenWidth < 935 && screenWidth >= 740){
          if (totalLinks < 4){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
            showNumber = 4;
            showButtons = true; 
          }
        }
        if(screenWidth < 1200 && screenWidth >= 935){
          if (totalLinks < 5){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
            showNumber = 5;
            showButtons = true; 
          }
        }
        if(screenWidth < 1400 && screenWidth >= 1200){
          if (totalLinks < 6){ 
            showNumber = totalLinks;
            showButtons = false; 
          } else { 
            showNumber = 6;
            showButtons = true;
          }
        }
        if(screenWidth >= 1400){
          if (totalLinks < 7){ 
            showNumber = totalLinks;
            showButtons = false;
          } else { 
            showNumber = 7;
            showButtons = true;
           }
        }
        showArray = [];
        let pos = startIndex;
        for(let i = 0; i<showNumber; i++){
          if (pos >= totalLinks) pos = 0;
          showArray.push(activeLinks[pos]);
          pos++;
        }
      };
      window.addEventListener('resize', handleResize);
    }
    
    // first run
    if(screenWidth < 700 ){
      if (totalLinks < 3){ 
        showNumber = totalLinks;
        showButtons = false;
       } else { 
            showNumber = 3;
            showButtons = true;
      }
    }
    if(screenWidth < 935 && screenWidth >= 700){
      if (totalLinks < 4){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
            showNumber = 4;
            showButtons = true; 
          }
    }
    if(screenWidth < 1200 && screenWidth >= 935){
      if (totalLinks < 5){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
            showNumber = 5;
            showButtons = true; 
          }
    }
    if(screenWidth < 1400 && screenWidth >= 1200){
      if (totalLinks < 6){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
            showNumber = 6;
            showButtons = true;
          }
    }
    if(screenWidth >= 1400){
      if (totalLinks < 7){ 
        showNumber = totalLinks;
        showButtons = false;
      } else { 
            showNumber = 7;
            showButtons = true;
           }
    }

    // First run
    let pos = startIndex;
    for(let i = 0; i<showNumber; i++){
      if (pos > totalLinks) pos = 0;
      showArray.push(activeLinks[pos]);
      pos++;
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
        showArray.push(activeLinks[pos]);
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
        showArray.push(activeLinks[pos]);
        pos++;
      }
    };

  </script>
  <div style="padding:15px;">
    <div class="card-container">
      {#if showButtons == true}
        <button class="prev" on:click={prev}>{@html "&#10094;"}</button>
      {/if}
      {#each showArray as TX}
        <!-- Standard - link to search page -->
        {#if mode == "standard"}
            <a href={TX.link}>
              <div class="card">
                  {#if TX.image == icpLogo}
                    <img class="headAlign" style="padding-top: 10px" src={TX.image} alt="" width="100%"/>
                    <span style="padding-top: 5px;">{TX.text}</span>
                  {:else}
                    <img class="headAlign" src={TX.image} alt="" width="70%"/>
                    <span>
                    {TX.text}
                  </span>
                  {/if}
              </div>
            </a>

          <!-- Click Through - Performs search on specified account -->
          {:else if mode == "clickThrough"}
            <a href={`/search/ID/${TX.ticker}?id=${searchedPrincipal}&sub=${searchedSubAccount}`} target="_blank">
              <div class="card">
                  {#if TX.image == icpLogo}
                    <img class="headAlign" style="padding-top: 10px" src={TX.image} alt="" width="100%"/>
                    <span style="padding-top: 5px;">{TX.text}</span>
                  {:else}
                    <img class="headAlign" src={TX.image} alt="" width="70%"/>
                    <span>
                    {TX.text}
                  </span>
                  {/if}
              </div>
            </a>

          {:else if mode == "homePage"}
            {#if TX.ticker == "ICP"}
              <a href={`/search/ID/${TX.ticker}?id=${icpAccount}&sub=""`} target="_blank">
                <div class="card">
                  <img class="headAlign" style="padding-top: 10px" src={TX.image} alt="" width="100%"/>
                  <span style="padding-top: 5px;">{TX.text}</span>
                </div>
              </a>
            {:else}
              <a href={`/search/ID/${TX.ticker}?id=${searchedPrincipal}&sub=${searchedSubAccount}`} target="_blank">
                <div class="card">        
                    <img class="headAlign" src={TX.image} alt="" width="70%"/>
                    {TX.text}
                </div>
              </a>
            {/if}
        {/if}
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
        width: 70px;
        height: 80px;
        text-align: center;
        align-items: center;
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