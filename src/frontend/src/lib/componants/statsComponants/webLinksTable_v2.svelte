<script>
import sonicLogo from '$lib/images/projectLogos/SonicLogo.png';
import dscvrLogo from '$lib/images/projectLogos/DSCVR.png';
import nuanceLogo from '$lib/images/projectLogos/nuanceLogo.png';
import distriktLogo from '$lib/images/projectLogos/DistriktLogo.png';
import twitterLogo from '$lib/images/projectLogos/twitter.png';
import ocLogo from '$lib/images/projectLogos/Openchat_logo.png';
import icLightLogo from '$lib/images/projectLogos/IC_light2.png';
import icpSwapLogo from '$lib/images/projectLogos/icp_swap.png';
import catLogo from '$lib/images/projectLogos/catalyzeLogo.png';


export let socialAR;
export let linksAR;
export let marketsAR;



let twitterURL, dscvrURL, nuanceURL, distriktURL, openchatURL, catalyzeURL;
let whitepaperURL, githubURL, teamWebsiteURL, canisterDashboardURL;
let sonicURL, icpSwapURL, icLightURL;

if (socialAR != null){
    let arLen = socialAR.length ?? 0;
    let i;
    for(i=0; i<arLen; i++){
        if(socialAR[i]?.platform == "twitter") twitterURL = socialAR[i].url;
        if(socialAR[i]?.platform == "dscvr") dscvrURL = socialAR[i].url;
        if(socialAR[i]?.platform == "nuance") nuanceURL = socialAR[i].url;
        if(socialAR[i]?.platform == "distrikt") distriktURL = socialAR[i].url;
        if(socialAR[i]?.platform == "openchat") openchatURL = socialAR[i].url;
        if(socialAR[i]?.platform == "catalyze") catalyzeURL = socialAR[i].url;
    }
}

if (linksAR != null){
    let arLen = linksAR.length ?? 0;
    let i;
    for(i=0; i<arLen; i++){
        if(linksAR[i]?.type == "whitepaper") whitepaperURL = linksAR[i].url;
        if(linksAR[i]?.type == "github") githubURL = linksAR[i].url;
        if(linksAR[i]?.type == "teamWebsite") teamWebsiteURL = linksAR[i].url;
        if(linksAR[i]?.type == "snsDashboard") canisterDashboardURL = linksAR[i].url;
    }
}

if (marketsAR != null){
    let arLen = marketsAR.length ?? 0;
    let i;
    for(i=0; i<arLen; i++){
        if(marketsAR[i]?.platform == "Sonic") sonicURL = marketsAR[i].url;
        if(marketsAR[i]?.platform == "IcpSwap") icpSwapURL = marketsAR[i].url;
        if(marketsAR[i]?.platform == "IcLight") icLightURL = marketsAR[i].url;
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
        {#if marketsAR != null}
            <tr>
                <td>Marketplaces: </td>
                <td>  
                    <div class="card-container">
                        {#if sonicURL}
                            <a href={sonicURL} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={sonicLogo} alt="Sonic" width="31px"/>
                                    Sonic Dex
                                </div>
                            </a>
                        {/if}
                        {#if icLightURL}
                            <a href={icLightURL} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={icLightLogo} alt="IC Lighthouse" width="35px"/>
                                    IC Light Dex
                                </div>
                            </a>
                        {/if}  
                        {#if icpSwapURL}
                            <a href={icpSwapURL} target="_blank">
                                <div class="card">
                                    <img class="headAlign" src={icpSwapLogo} alt="ICP Swap" width="41px"/>
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