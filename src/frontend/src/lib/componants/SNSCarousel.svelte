<script>
    import TokenLogos from "../shared/tokenLogos.svelte";
    import { getAllTokenData } from "../code/utils";
    import { createEventDispatcher } from 'svelte';
    
    export let titleText = "";
    export let linkTypes = "stats";

    let linkURL = [];
    let allLinks = [];
    let showArray = [];
    let link; 
    let totalLinks; 
    let startIndex;
    let showNumber;
    let pos;

    let tokenData;
    let promise = getData();

    function getData(){
      tokenData = getAllTokenData();
      let tLen = tokenData.length ?? 0;

      if (linkTypes == "stats") {
        link = "/explore/stats/token/"
      }
      if (linkTypes == "search") {
        link ="/search/token/"
      }

      let index = 0;
      for(let i = 0; i<tLen; i++){
        allLinks.push({idx: index, ticker: tokenData[i].ticker, link: `${link}${tokenData[i].ticker}`, text: tokenData[i].shortName});
          index++;
      } 

      totalLinks = allLinks?.length ?? 0; 
      startIndex = 0;
      showNumber = 6;

      // init show array
      showArray = [];
      pos = startIndex;
      for(let i = 0; i<showNumber; i++){
        if (pos >= totalLinks) pos = 0;
        showArray.push(allLinks[pos]);
        pos++;
      }
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
          showArray.push(allLinks[pos]);
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
          showArray.push(allLinks[pos]);
          pos++;
        }
      };

    const dispatch = createEventDispatcher();
    function resetCheck(btnClicked){
       dispatch('click', btnClicked);
    }
  </script>
  {#await promise}
  {:then}
    <div style="padding:15px;">
      <h5>{titleText}</h5>
      <div class="card-container">
        <button class="prev" on:click={prev}>{@html "&#10094;"}</button>

      {#each showArray as TX}
        <a href={TX.link}  on:click={() => resetCheck(TX.ticker)}>
          <div class="card">
              {#if TX.ticker == "ICP"}
                <span style="padding-top: 20px;">
                  <TokenLogos token={TX.ticker} width="100%"/>
                </span>
                <span style="padding-top: 13px;">{TX.text}</span>
              {:else}
                <TokenLogos token={TX.ticker} width="100%"/>
                {TX.text}
              {/if}
              
          </div>
        </a>
      {/each}  
              
        <button class="next" on:click={next}>{@html "&#10095;"}</button>
      </div>
    </div>
  {/await}

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
        width: 80px;
        height: 100px;
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