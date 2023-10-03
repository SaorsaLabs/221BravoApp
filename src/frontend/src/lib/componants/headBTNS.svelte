<script>
    export let buttonHighlight = 1;
    import Button from "../shared/button.svelte";
    import login from '../images/login.png';
    import logout from '../images/logout.png';
    import Modal from "../shared/modal.svelte";
    import LoginForm from "./loginForm.svelte";
    import { browser } from '$app/environment';
    import {authStore, authTrigger} from "../stores/authStore.js";
    
    let LS = false;
    let showPopup = false;
    let once = true;
    let userData = {
        user : "Anonymous User",
        tokens : 0,
        rank : 0
        };
    let logInClick;
    let oldValue = null;
    // Logged In? 
    authTrigger.subscribe(value =>{
        if(browser){
            // fix pop up stuck issue.
            if (value != oldValue && logInClick == true) {
                oldValue = value;
                logInClick = false;
                showPopup = false;
                location.reload();
            }
            if(value>=1){
                showPopup = false;
                authStore.check();
                let x = authStore.read();
                LS = x.data.loggedIn;
            }
            if(value == 0){
            authStore.init();
            authTrigger.update(n => n + 1);
            logInClick = true;
            }
        }
    });

    const onShowPopup = () => {
        showPopup = true;
        if(LS == true || LS == 'true'){
            // TODO 
            // Retrive user data from user canister
        } else {
            logInClick = true; // is initial login attempt. 
        }
    }
    const onPopupClose = (data) => {
        showPopup = false;
        //console.log(data); <-- return data from modal using data
    }

</script>

<div>
    <table class="alignSelf">
        <tr>
            <td class="alignSelf"> 
                <a href="/">
                <Button type="head" 
                flat={true} 
                noBG={true}>
                Home
                </Button> 
                </a>
            </td>
            <td class="alignSelf"> 
                <a href="/explore">
                <Button type="head" 
                flat={true} 
                noBG={true}>
                Explore
                </Button>
                </a>
            </td>
            <td class="alignSelf"> 
                <a href="/search/icp">
                <Button type="head" 
                flat={true} 
                noBG={true}>
                Search
                </Button> 
                </a>
            </td>
            <td class="alignSelf"> 
                <a href="/members">
                    <Button type="head" 
                    flat={true} 
                    noBG={true}>
                    Members
                    </Button> 
                </a>
            </td>
            <td class="alignSelf"> 
                {#if LS == 'false' || LS == false}
                    <img class="login" src={login} alt="login" on:click={() => onShowPopup()} width="40px" />
                {:else}
                    <img class="login" src={logout} alt="login" on:click={() => onShowPopup()} width="40px" />
                {/if}
            </td>
        </tr>
    </table>
</div>

<Modal open={showPopup} title={""} size={"medium"} position={'top'} onClosed={() => onPopupClose()}> 
    <LoginForm loggedIn={LS} userData={userData} />
</Modal>

<style>
    .alignSelf{
        width: 100%;
        text-align: right;
        height: 100%;
        padding: 10px 5px;
    }
    .login{
        cursor: pointer; 
    }
</style>