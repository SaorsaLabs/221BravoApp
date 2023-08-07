<script>
    import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
    import chatLogo from '$lib/images/Openchat_logo.png';
    import kinicLogo from '$lib/images/kinic.png';
    import hotLogo from '$lib/images/hotOrNot.png';
    import ghostLogo from '$lib/images/ghost_logo.png';
    import icpLogo from '$lib/images/icpLogo.png';
    import snsLogo from '$lib/images/SNS1_Logo.png';
    
    export let titleText = "";
    export let linkTypes = "stats";

    let linkURL = [];

    if(linkTypes == "stats"){
      linkURL[0] = "/explore/stats/ckbtc";
      linkURL[1] = "/explore/stats/chat";
      linkURL[2] = "/explore/stats/kinic";
      linkURL[3] = "/explore/stats/hot";
      linkURL[4] = "/explore/stats/ghost";
      linkURL[5] = "/explore/stats/icp";
      linkURL[6] = "/explore/stats/sns1";
    }
    if(linkTypes == "search"){
      linkURL[0] = "/search/ckbtc";
      linkURL[1] = "/search/chat";
      linkURL[2] = "/search/kinic";
      linkURL[3] = "/search/hot";
      linkURL[4] = "/search/ghost";
      linkURL[5] = "/search/icp";
      linkURL[6] = "/search/sns1";
    }

    let startIndex = 0;
    let showArray = [0,1,2,3,4,5];
    let showNumber = 6;
    let totalLinks = 7;
    let i;

    const next = () => {
      let CN;
      startIndex += showNumber;
      showArray = [];
      for(i=startIndex; i<startIndex+showNumber; i++){
        CN = i % totalLinks;
        showArray.push(CN);
      }
    };
  
    const prev = () => {
      let CN;
      startIndex = Math.abs(startIndex - showNumber);
      showArray = [];
      for(i=startIndex; i<startIndex+showNumber; i++){
        CN = i % totalLinks;
        showArray.push(CN);
      }
    };
  </script>
  
  <div style="padding:15px;">
    <h5>{titleText}</h5>
    <div class="card-container">
      <button class="prev" on:click={prev}>{@html "&#10094;"}</button>

            {#if showArray.includes(0)}
              <a href={linkURL[0]}>
                  <div class="card">
                      <img class="headAlign" src={ckBTCLogo} alt="" width="100%"/>
                      ckBTC 
                  </div>
              </a>
            {/if}

            {#if showArray.includes(1)}
              <a href={linkURL[1]}>
                  <div class="card">
                      <img class="headAlign" src={chatLogo} alt="" width="100%"/>
                      CHAT
                  </div>
              </a>
            {/if}
            
            {#if showArray.includes(2)}
              <a href={linkURL[2]}>
                  <div class="card">
                      <img class="headAlign" src={kinicLogo} alt="" width="100%"/>
                      KINIC
                  </div>
              </a>
            {/if}
            
            {#if showArray.includes(3)}
              <a href={linkURL[3]}>
                  <div class="card">
                      <img class="headAlign" src={hotLogo} alt="" width="100%"/>
                      HOT
                  </div>
              </a>
            {/if}
            
            {#if showArray.includes(4)}
              <a href={linkURL[4]}>
                  <div class="card">
                      <img class="headAlign" src={ghostLogo} alt="" width="100%"/>
                      GHOST
                  </div>
              </a>
            {/if}
            
            {#if showArray.includes(5)}
              <a href={linkURL[5]}>
                  <div class="card" style="padding-top:27px;">
                      <img class="headAlign" src={icpLogo} alt="" width="100%"/>
                      <span style="padding-top: 14px;">ICP</span>
                  </div>
              </a>
            {/if}
            
            {#if showArray.includes(6)}
              <a href={linkURL[6]}>
                  <div class="card">
                      <img class="headAlign" src={snsLogo} alt="" width="100%"/>
                      Dragginz
                  </div>
              </a>
            {/if}
            
            <button class="next" on:click={next}>{@html "&#10095;"}</button>
    </div>
  </div>


  <!-- <div class="carousel">
    {#each displayedItems() as item, i}
      <div class="carousel-item">
        <img src={item.image} alt={item.text}>
        <p>{item.text}</p>
      </div>
    {/each}
    <button class="prev" on:click={prev}>&#10094;</button>
    <button class="next" on:click={next}>&#10095;</button>
  </div> -->
  
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

  .carousel {
    position: relative;
    display: flex;
    justify-content: space-between;
    max-width: 100%;
    margin: auto;
  }
  
  .carousel-item {
    flex: 1 0 auto;
    margin: 0 10px;
    text-align: center;
    padding: 10px;
    box-sizing: border-box;
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