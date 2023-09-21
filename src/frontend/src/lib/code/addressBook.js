import { backendCanisterID } from './constants.js';
import { getIdentity, icActor } from './icAgent.js';
import { backendCanisterIDL } from './IDL/backend.js';
import { authStore, authTrigger } from '../stores/authStore.js';

async function saveAccount(account, name) {
    let res;
    // get user 
    let usr = authStore.read();
    // save details
    if (usr.data.loggedIn == true){
        const Frontend_ID = getIdentity();
		const backendActor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
		let save_res = await backendActor.add_user_named_accounts(usr.data.user, account, name);
        res = save_res;
    } else {
        res = "Not Logged In!"
    }
    return res;
}

async function getAllAccounts() {
    const Frontend_ID = getIdentity();
    let usr = authStore.read();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.get_all_user_named_accounts(usr.data.user); 
    return res;
}

async function deleteAccount(account){
    const Frontend_ID = getIdentity();
    let usr = authStore.read();
    let actor = icActor(backendCanisterID, backendCanisterIDL, Frontend_ID);
    let res = await actor.remove_user_named_account(usr.data.user, account); 
    console.log("Delete RES :: ", res);
    if (res == "Account removed from directory") {
        authTrigger.update((n) => n + 1);
    }
    return res;
}

export {
    saveAccount,
    getAllAccounts,
    deleteAccount
};