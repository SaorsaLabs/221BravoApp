import { Actor, HttpAgent } from '@dfinity/agent';
import fetch from 'isomorphic-fetch';
import { Secp256k1KeyIdentity } from '@dfinity/identity-secp256k1';

function getIdentity(){
   let key = import.meta.env.VITE_FRONTEND_ID;
   let ar = key.split(' ');
    let ID  = Secp256k1KeyIdentity.fromSecretKey(ar);
    return ID; 
}

function icActor (canister, idlFactory, identity) {
    let ID;
    if (identity) ID = identity 
    else ID = getIdentity();

    const canisterId = canister; 
    const host = 'https://ic0.app';
    const agent = new HttpAgent({identity: ID, fetch, host: host}); 
    
    return Actor.createActor(
            idlFactory, 
            {
            agent: agent,
            canisterId: canisterId
            });
}


export {
    icActor,
    getIdentity
}