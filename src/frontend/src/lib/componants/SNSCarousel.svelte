<script>
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
    import nuanceLogo from '$lib/images/projectLogos/NuanceLogo.png';
    
    export let titleText = "";
    export let linkTypes = "stats";

    let linkURL = [];
    let allLinks = [];
    let showArray = [];

    allLinks = [
      {idx: 0, image: ckBTCLogo, text: "ckBTC"},
      {idx: 1, image: chatLogo, text: "CHAT"},
      {idx: 2, image: kinicLogo, text: "KINIC"},
      {idx: 3, image: hotLogo, text: "HOT"},
      {idx: 4, image: ghostLogo, text: "GHOST"},
      {idx: 5, image: icpLogo, text: "ICP"},
      {idx: 6, image: snsLogo, text: "Dragginz"},
      {idx: 7, image: modLogo, text: "ModClub"},
      {idx: 8, image: catLogo, text: "Catalyze"},
      {idx: 9, image: boomLogo, text: "BoomDAO"},
      {idx: 10, image: icxLogo, text: "IC-X"},
      {idx: 11, image: nuanceLogo, text: "NUANCE"},
    ];

    let totalLinks = allLinks?.length ?? 0; 

    if(linkTypes == "stats"){
      linkURL[0] = "/explore/stats/ckbtc";
      linkURL[1] = "/explore/stats/chat";
      linkURL[2] = "/explore/stats/kinic";
      linkURL[3] = "/explore/stats/hot";
      linkURL[4] = "/explore/stats/ghost";
      linkURL[5] = "/explore/stats/icp";
      linkURL[6] = "/explore/stats/sns1";
      linkURL[7] = "/explore/stats/modclub";
      linkURL[8] = "/explore/stats/cat";
      linkURL[9] = "/explore/stats/boom";
      linkURL[10] = "/explore/stats/icx";
      linkURL[11] = "/explore/stats/nuance";
    }
    if(linkTypes == "search"){
      linkURL[0] = "/search/ckbtc";
      linkURL[1] = "/search/chat";
      linkURL[2] = "/search/kinic";
      linkURL[3] = "/search/hot";
      linkURL[4] = "/search/ghost";
      linkURL[5] = "/search/icp";
      linkURL[6] = "/search/sns1";
      linkURL[7] = "/search/modclub";
      linkURL[8] = "/search/cat";
      linkURL[9] = "/search/boom";
      linkURL[10] = "/search/icx";
      linkURL[11] = "/search/nuance";
    }

    let startIndex = 0;
    let showNumber = 6;

    // init show array
    showArray = [];
    let pos = startIndex;
    for(let i = 0; i<showNumber; i++){
      if (pos >= totalLinks) pos = 0;
      showArray.push(allLinks[pos]);
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

  </script>
  
  <div style="padding:15px;">
    <h5>{titleText}</h5>
    <div class="card-container">
      <button class="prev" on:click={prev}>{@html "&#10094;"}</button>

    {#each showArray as TX}
      <a href={linkURL[TX.idx]}>
        <div class="card">
            {#if TX.image == icpLogo}
              <img class="headAlign" style="padding-top: 20px" src={TX.image} alt="" width="100%"/>
              <span style="padding-top: 13px;">{TX.text}</span>
            {:else}
              <img class="headAlign" src={TX.image} alt="" width="100%"/>
              {TX.text}
            {/if}
            
        </div>
      </a>
    {/each}  
            
      <button class="next" on:click={next}>{@html "&#10095;"}</button>
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