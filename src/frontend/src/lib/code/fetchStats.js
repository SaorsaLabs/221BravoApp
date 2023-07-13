import { getIdentity, icActor } from './icAgent.js';
import {ckBTC_dataCanister} from './constants.js';
import {icrcIDL} from './IDL/icrcDataProcessor.js';

async function getICRC_Stats(token, timescale){
    const ID = getIdentity();
    if(token == "CKBTC"){
        if(timescale == "hourly"){
            let actor = icActor(ckBTC_dataCanister,icrcIDL,ID);
            let stats = await actor.get_hourly_stats();
            console.log("HOURLY :: ",stats);
            return stats;
        }
        if(timescale == "daily"){
            let actor = icActor(ckBTC_dataCanister,icrcIDL,ID);
            let stats = await actor.get_daily_stats();
            console.log("DAILY :: ",stats);
            return stats;
        }
    }
}

export {
    getICRC_Stats
};