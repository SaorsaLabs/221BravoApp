<script>
import stoicLogo from '$lib/images/Stoic.png';
import bitfinity from '$lib/images/Bitfinity.png';
import plug from '$lib/images/Plug.png';
import Button from '../shared/button.svelte';
import { stoicLogin, plugLogin, bitfinityLogin } from '../code/auth.js';
import { browser } from '$app/environment';
import Loading from '../shared/loading.svelte';
import {authStore, authTrigger} from "../stores/authStore.js";
import {getSync} from '../code/auth.js';

export let loggedIn = false;
export let userData = {
    user : "Anonymous User",
    tokens : 0,
    rank : 0
};
let loading = false;
let outcome = '';
let res;

authTrigger.subscribe(value =>{
        if(browser){
            if(value>=1){
                let x = authStore.read();
                loggedIn = x.data.loggedIn;
            }
            if(value == -1){
                loading = false;
                res = false;
                outcome = 'Not Authorised';
                authTrigger.update(n => n + 2);
            }
        }
});

async function handleStoicClick(){
    if(browser){
        loading = true;
        outcome = '';
        await stoicLogin();
    }
}
async function handlePlugClick(){
    if(browser){
        loading = true;
        outcome = '';
        await plugLogin();
    }
}
async function handleBitfinityClick(){
    if(browser){
        loading = true;
        outcome = '';
        await bitfinityLogin();
    }
}
async function handleLogout(){
    let x = await getSync(false,0);
    let d = new Date();
    let time = (d.getTime()/1000); // current in secs.
    authStore.set(false,x,time,"abc123");
    authTrigger.update(n => n + 1);
    outcome = '';
    loggedIn = false;
    res = false;
    loading = false;
}
</script>

{#if loggedIn == false || loggedIn == 'false' || loggedIn == null}
<div>
    <p class="textAlign">Login Using NFT : </p>
    <div>
        <!-- login table -->
        {#if loading == false}
            <table class="loginTable">
                <tr>
                    <td class="tableAlign">
                        <img  src={stoicLogo} alt="Stoic Wallet Logo" width="54px"/>
                    </td>
                    <td class="buttonAlign">
                        <Button type="grey" 
                        flat={false} 
                        noBG={false}
                        on:click={handleStoicClick}
                        >
                        Login with Stoic Wallet
                        </Button> 
                    </td>
                </tr>
                <tr>
                    <td class="tableAlign">
                        <img  src={bitfinity} alt="Bitfinity Wallet Logo" width="50px"/>
                    </td>
                    <td class="buttonAlign">
                        <Button type="grey" 
                        flat={false} 
                        noBG={false}
                        on:click={handleBitfinityClick}
                        >
                        Login with Bitfinity Wallet
                        </Button> 
                    </td>
                </tr>
                <tr>
                    <td class="tableAlign">
                        <img  src={plug} alt="Bitfinity Wallet Logo" width="50px"/>
                    </td>
                    <td class="buttonAlign">
                        <Button type="grey" 
                        flat={false} 
                        noBG={false}
                        on:click={handlePlugClick}
                        >
                        Login with Plug Wallet
                        </Button> 
                    </td>
                </tr>
            </table>
        {:else}
            <Loading/>
        {/if}
        <!-- not authorised -->
        {#if res == false || res == 'false'}
        <p class = "warnText">{outcome}</p>
        {/if}
    </div>
    <hr>
    <div class="textAlign">
        <h5> Dont have a Membership NFT yet? </h5>
        <p> Pick up a 'Genesis II' membership NFT from  
            <a href="https://toniq.io/marketplace/genesis-ii"
                target="_blank">
                <Button type="white" 
                flat={true} 
                noBG={true}
                slim={true}
                >
                Toniq Marketplace
                </Button>
                </a> 
            to get access to all the 221Bravo premium content. 
        </p>
    </div>
</div>
{:else}
    <!-- authorised -->
    <div class="textAlign">
        <p> Welcome Data-Detective!</p> 
        <p> User : {userData.user}</p>
        <p> Tokens : {userData.tokens}</p>
        <p> Rank : {userData.rank}</p>
        <td class="buttonAlign">
            <Button type="grey" 
            flat={false} 
            noBG={false}
            on:click={() => handleLogout()}
            >
            Logout
            </Button> 
    </div>
{/if}

<style>
    .loginTable{
        width:100%;
    }
    .tableAlign{
        padding-left: 100px;
        width:35%;
    }
    .buttonAlign{
        padding-top: 4px;
    }
    .textAlign{
        margin-left: 7px;
    }
    p{
        margin-bottom: 3px;
    }
    .warnText{
        color:white;
        background-color: red; 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }

</style>