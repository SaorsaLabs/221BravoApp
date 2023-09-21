<script>
import sonicLogo from '$lib/images/projectLogos/SonicLogo.png';
import dscvrLogo from '$lib/images/projectLogos/DSCVR.png';
import nuanceLogo from '$lib/images/projectLogos/nuanceLogo.png';
import distriktLogo from '$lib/images/projectLogos/DistriktLogo.png';
import twitterLogo from '$lib/images/projectLogos/twitter.png';
import ocLogo from '$lib/images/projectLogos/Openchat_logo.png';
import icLight from '$lib/images/projectLogos/IC_light2.png';
import icpSwap from '$lib/images/projectLogos/icp_swap.png';
import catLogo from '$lib/images/projectLogos/catalyzeLogo.png';


export let SocialAR;
export let whitepaperURL;
export let githubURL;
export let teamWebsiteURL;
export let marketplaceURL;
export let marketplaceURL2;
export let marketplaceURL3;
export let canisterDashboardURL;



let twitterURL, dscvrURL, nuanceURL, distriktURL, openchatURL, catalyzeURL;


if (SocialAR != null){
    let arLen = SocialAR.length ?? 0;
    let i;
    for(i=0; i<arLen; i++){
        if(SocialAR[i]?.platform == "twitter") twitterURL = SocialAR[i].url;
        if(SocialAR[i]?.platform == "dscvr") dscvrURL = SocialAR[i].url;
        if(SocialAR[i]?.platform == "nuance") nuanceURL = SocialAR[i].url;
        if(SocialAR[i]?.platform == "distrikt") distriktURL = SocialAR[i].url;
        if(SocialAR[i]?.platform == "openchat") openchatURL = SocialAR[i].url;
        if(SocialAR[i]?.platform == "catalyze") catalyzeURL = SocialAR[i].url;
    }
}

</script>

<div class="mainContent">
    <h4>Links:</h4>
    <table style="width: 100%;">
        {#if teamWebsiteURL != null}
            <tr>
                <td>Team Website: </td>
                <td><a class="textAdj" href={teamWebsiteURL} target="_blank">{teamWebsiteURL}</a></td>
            </tr>
        {/if}
        {#if whitepaperURL != null}
            <tr>
                <td>Whitepaper: </td>
                <td><a class="textAdj" href={whitepaperURL} target="_blank">{whitepaperURL}</a></td>
            </tr>
        {/if}
        {#if githubURL != null}
            <tr>
                <td>Github Repo: </td>
                <td><a class="textAdj" href={githubURL} target="_blank">{githubURL}</a></td>
            </tr>
        {/if}
        {#if marketplaceURL != null}
            <tr>
                <td>Marketplaces: </td>
                <td>  
                    <div class="card-container">
                        {#if marketplaceURL}
                            <a href={marketplaceURL} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={sonicLogo} alt="Sonic" width="31px"/>
                                    Sonic Dex
                                </div>
                            </a>
                        {/if}
                        {#if marketplaceURL2}
                            <a href={marketplaceURL2} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={icLight} alt="IC Lighthouse" width="35px"/>
                                    IC Light Dex
                                </div>
                            </a>
                        {/if}  
                        {#if marketplaceURL3}
                            <a href={marketplaceURL3} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={icpSwap} alt="ICP Swap" width="41px"/>
                                    ICP Swap Dex
                                </div>
                            </a>
                        {/if}                
                    </div>
                </td>
            </tr>
        {/if}
        {#if canisterDashboardURL != null}
            <tr>
                <td>SNS Canisters and Status: </td>
                <td><a class="textAdj" href={canisterDashboardURL} target="_blank">{canisterDashboardURL}</a></td>
            </tr>
        {/if}
        <!-- {#if socials != null} -->
        <tr>
            <td>Socials: </td>
            <td> 
                <div class="card-container">
                    {#if twitterURL != null}
                        <a href={twitterURL} target="_blank">
                            <div class="card">
                                <img class="headAlign" src={twitterLogo} alt="twitter" width="50px"/>
                                Twitter
                            </div>
                        </a>
                    {/if}
                    {#if dscvrURL != null}
                        <a href={dscvrURL} target="_blank">
                            <div class="card">
                                <img class="headAlign" src={dscvrLogo} alt="DSCVR" width="29px"/>
                                Dscvr
                            </div>
                        </a>
                    {/if}
                    {#if nuanceURL != null}
                        <a href={nuanceURL} target="_blank">
                            <div class="card">
                                <img class="headAlign" src={nuanceLogo} alt="Nuance" width="27px"/>
                                Nuance
                            </div>
                        </a>
                    {/if}
                    {#if distriktURL != null}
                        <a href={distriktURL} target="_blank">
                            <div class="card">
                                <img class="headAlign" src={distriktLogo} alt="Distrikt" width="30px"/>
                                Distrikt
                            </div>
                        </a>
                    {/if}
                    {#if openchatURL != null}
                    <a href={openchatURL} target="_blank">
                        <div class="card">
                            <img class="headAlign" src={ocLogo} alt="OpenChat" width="29px"/>
                            Openchat
                        </div>
                    </a>
                    {/if}
                    {#if catalyzeURL != null}
                    <a href={catalyzeURL} target="_blank">
                        <div class="card">
                            <img class="headAlign" src={catLogo} alt="Catalyze" width="30px"/>
                            Catalyze
                        </div>
                    </a>
                {/if}
                </div>
            </td>
        </tr>
    <!-- {/if} -->
    </table>
</div>
<style>
    .mainContent{
        padding: 15px;
    }
    .card-container {
    display: flex;
    flex-wrap: wrap;
    /* justify-content: space-around; */
    }
    .card {
        background-color: #d2f5f9f1;
        margin: 5px;
        padding: 5px;
        box-shadow: 0px 0px 7px 0 rgba(249, 20, 207, 0.968);
        border-radius: 5px;
        width: 80px;
        text-align: center;
        align-items: center;
    }
    a {
    color: rgb(180, 180, 180);
    }
    table td, table th {
        border-top: 1px solid #5c5c5c;
        border-bottom: 1px solid #5c5c5c;
        padding: 10px; 
        word-wrap: break-word;      /* For IE */
        overflow-wrap: break-word;  /* Modern browsers */
    }
    table tr:first-child td, table tr:first-child th {
        border-top: none;
    }

    table tr:last-child td, table tr:last-child th {
        border-bottom: none;
    }
    .textAdj{
        font-size: 1rem;
    }
    @media screen and (max-width:800px){
        .textAdj{
            font-size: 0.80rem;
        }   
    }
    @media screen and (max-width:992px){
        .textAdj{
            font-size: 0.85rem;
        }   
    }
</style>