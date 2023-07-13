import {DOMAIN} from './constants.js';
import {StoicIdentity} from './stoicAuth.js';
import { browser } from '$app/environment';
import {authStore, authTrigger} from "../stores/authStore.js";
import { createActor } from '../../../../declarations/backend/index.js';
import { getIdentity } from './icAgent.js';

const canisterId = import.meta.env.VITE_BACKEND_CANISTER_ID;
const host = import.meta.env.VITE_HOST;

// STOIC LOGIN
async function stoicLogin(){
    StoicIdentity.load()
    .then(async identity => {
      identity = await StoicIdentity.connect();
      const P = identity.getPrincipal().toText();
      if(P.length > 0) {
      //Verify Holder 
      let url =  `${DOMAIN}/Verify?Holder=${P}`; 
      let settings = { method: 'Get', mode: 'cors', headers: {'Content-Type': 'application/json',}};
      // WIP - Verify on IC
      // const ID = getIdentity();
      // const backendActor = createActor(canisterId, { agentOptions: { host: host, identity: ID} });
		  // let isHolder = await backendActor.is_genesis_holder(P);
      // console.log("IC Verify: Is Holder : ", isHolder);
      // TEMP - Verify by server
      const checkHODLER = await fetch(url, settings);
      const data = await checkHODLER.json();
        if(data[0] == true || data[0] == 'true'){
          authTrigger.update(n => n + 1);
          let d = new Date();
          let time = (d.getTime()/1000); // current in secs.
          authStore.set(true,data[1],time, data[2]);   
          StoicIdentity.disconnect();
          return true;
        } else {
          authTrigger.update(n => -1);
          StoicIdentity.disconnect();
          return false;
        }
      }// if 
    });// stoic ID

}

async function plugLogin(){
  if(browser){
    try {
      const publicKey = await window.ic.plug.requestConnect();
      const account = window.ic.plug.accountId
      console.log(`The connected user's account ID is:`, account);
      if(account.length > 0) {
        //Verify Holder 
        let url =  `${DOMAIN}/Verify?Holder=${account}`; 
        let settings = { method: 'Get', mode: 'cors', headers: {'Content-Type': 'application/json'}};
        const checkHODLER = await fetch(url, settings)
        .then((res) => res.json())
        .then((data) => {
            if(data[0] == true || data[0] == "true"){
              authTrigger.update(n => n + 1);
              let d = new Date();
              let time = (d.getTime()/1000); // current in secs.
              authStore.set(true,data[1],time, data[2]);
              return true;
            }
            else {
              // Verified = false
              authTrigger.update(n => -1);
              return false;
            }
        });
        }// if 
    } catch (e) {
      console.log(e);
    }
  }
}

async function bitfinityLogin(){
  if(browser){
  try {
        // const publicKey = await window.ic.infinityWallet.requestConnect();
        const p1 = await window.ic.infinityWallet.getPrincipal();
        const principal = p1.toText();
        if(principal.length > 0) {
          //Verify Holder 
          let url =  `${DOMAIN}/Verify?Holder=${principal}`; 
          let settings = { method: 'Get', mode: 'cors', headers: {'Content-Type': 'application/json'}};
          const checkHODLER = await fetch(url, settings)
          .then((res) => res.json())
          .then((data) => {
              if(data[0] == true || data[0] == "true"){
                authTrigger.update(n => n + 1);
                let d = new Date();
                let time = (d.getTime()/1000); // current in secs.
                authStore.set(true,data[1],time, data[2]);
                return true;
              }
              else {
                authTrigger.update(n => -1);
                return false;
              }
          });
          }// if 
      } catch (e) {
        console.log(e);
      }
  }
}

async function getSync(isLoggedIn, user){
  try {
    let url =  `${DOMAIN}/v2/sync?auth=${isLoggedIn}&user=${user}`; 
    let settings = { method: 'Get', mode: 'cors', headers: {'Content-Type': 'application/json',}};
    const syncCall = await fetch(url, settings);
    const data = await syncCall.json();
    return data;
  } catch (error) {
    console.log(error);
  }
}

export {stoicLogin, plugLogin, bitfinityLogin, getSync};