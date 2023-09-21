<script>
import icpLogo from '$lib/images/icpLogo.png';
import ckBTCLogo from '$lib/images/ckBTC_logo.svg';
import chatLogo from '$lib/images/Openchat_logo.png';
import kinicLogo from '$lib/images/kinic.png';
import hotLogo from '$lib/images/hotOrNot.png';
import ghostLogo from '$lib/images/ghost_logo.png';
import snsLogo from '$lib/images/SNS1_Logo.png';
import modLogo from '$lib/images/projectLogos/ModClub.png';
import catLogo from '$lib/images/projectLogos/catalyzeLogo.png';
import boomLogo from '$lib/images/projectLogos/BoomLogo.png'; 
import icxLogo from '$lib/images/projectLogos/IcxLogo.png';    

export let token;
export let width; // eg "100%" , "50px"

</script>

{#if token == "ICP"}
    <img src={icpLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CKBTC"}
    <img src={ckBTCLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CHAT"}
    <img src={chatLogo} alt='Token Logo' width="{width}"/>
{:else if token == "KINIC"}
    <img src={kinicLogo} alt='Token Logo' width="{width}"/>
{:else if token == "HOT"}
    <img src={hotLogo} alt='Token Logo' width="{width}"/>
{:else if token == "GHOST"}
    <img src={ghostLogo} alt='Token Logo' width="{width}"/>
{:else if token == "SNS1"}
    <img src={snsLogo} alt='Token Logo' width="{width}"/>
{:else if token == "MODCLUB"}
    <img src={modLogo} alt='Token Logo' width="{width}"/>
{:else if token == "CAT"}
    <img src={catLogo} alt='Token Logo' width="{width}"/>
{:else if token == "BOOM"}
    <img src={boomLogo} alt='Token Logo' width="{width}"/>
{:else if token == "ICX"}
    <img src={icxLogo} alt='Token Logo' width="{width}"/>
{/if}
