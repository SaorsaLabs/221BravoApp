import { getIdentity, icActor } from './icAgent.js';
import { dynamicContentIDL } from './IDL/dynamicContent.js';
import { dynamicContentCanister } from './constants.js';

async function get_all_news(){
    const ID = getIdentity();
    let actor = icActor(dynamicContentCanister, dynamicContentIDL, ID);
    let news = await actor.read_news_items();
    let ret = news.reverse();
    let retLen = ret.length ?? 0;
    for(let i=0; i<retLen; i++){
        ret[i].idx = i;
    }
    return ret;
}

async function get_all_projects(){
    const ID = getIdentity();
    let actor = icActor(dynamicContentCanister, dynamicContentIDL, ID);
    let ret = await actor.read_all_project_buckets();
    return ret;
}

async function get_single_project(bucket){
    const ID = getIdentity();
    let actor = icActor(dynamicContentCanister, dynamicContentIDL, ID);
    let ret = await actor.read_single_project_bucket(bucket);
    return ret;
}

export {
    get_all_news,
    get_all_projects,
    get_single_project
}