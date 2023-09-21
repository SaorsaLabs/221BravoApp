mod state_management;
mod custom_types;
mod constants;
mod utils;

use ic_cdk_macros::*;
use state_management::{ state_init, state_pre_upgrade, state_post_upgrade, STABLE_STATE, RUNTIME_STATE };
use custom_types::{ MemoryData, LogEntry, NewsItem, ProjectCard, ProjectCollection };



// [][] ---------------- [][]
// [][] ---  Methods --- [][]
// [][] ---------------- [][]
#[update]
fn add_news_item(title: String, sub_title: String, article_url: String, image_url: String) -> String {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let add = RUNTIME_STATE.with(|s|{
        s.borrow_mut().add_news(title, sub_title, article_url, image_url)
    });
    return add;
}

#[update]
fn remove_news_item(index: usize) -> String {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let rm = RUNTIME_STATE.with(|s|{
        s.borrow_mut().remove_news(index)
    });
    return rm;
}

#[query]
fn read_news_items() -> Vec<NewsItem> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let all: Vec<NewsItem> = RUNTIME_STATE.with(|s|{
        s.borrow().get_all_news()
    });
    return all;
}

#[update]
fn add_project(bucket: u8, title: String, sub_title: String, project_url: String, image_url: String) -> String {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let add = RUNTIME_STATE.with(|s|{
        s.borrow_mut().add_project(bucket, title, sub_title, project_url, image_url)
    });
    return add;
}

#[update]
fn remove_project(index: usize, bucket: u8) -> String {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let rm = RUNTIME_STATE.with(|s|{
        s.borrow_mut().remove_project(bucket, index)
    });
    return rm;
}

#[query]
fn read_single_project_bucket(bucket: u8) -> Vec<ProjectCard> {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let all: Vec<ProjectCard> = RUNTIME_STATE.with(|s|{
        s.borrow().get_all_single_project(bucket)
    });
    return all;
}

#[query]
fn read_all_project_buckets() -> ProjectCollection {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    let all: ProjectCollection = RUNTIME_STATE.with(|s|{
        s.borrow().get_all_projects()
    });
    return all;
}



// [][] --------------------------- [][]
// [][] --- Canister Management --- [][]
// [][] --------------------------- [][]
#[update]
fn add_authorised(principal_id: String) -> String {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .add_authorised(principal_id)
    })
}

#[update]
fn remove_authorised(principal_id: String) -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // add authorised 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .remove_authorised(principal_id)
    })
}

#[update]
fn set_canister_name(name: String) -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // set canister name 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_canister_name(name)
    })
}

#[update]
fn set_stats_public(are_stats_public: bool) -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // set canister name 
    STABLE_STATE.with(|state| {
        state.borrow_mut().as_mut().unwrap().canister_data
        .set_stats_public(are_stats_public)
    })
}


#[query]
fn get_all_authorised() -> Vec<String> {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get all authorised
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_all_authorised();
        return ret;
    })
}

#[query]
fn get_canister_name() -> String {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get canister name
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.get_canister_name();
        return ret;
    })
}

#[query]
fn are_stats_public() -> bool {
   // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // check if stats are public
    STABLE_STATE.with(|state| {
        let ret = state.borrow_mut().as_mut().unwrap()
        .canister_data.are_stats_public();
        return ret;
    })
}

#[query]
fn get_canister_logs() -> Vec<LogEntry> {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    RUNTIME_STATE.with(|state|{
        state.borrow().canister_logs.to_owned()
    })
}

#[query]
fn get_cycles_balance() -> u64 {
    // check authorised
   STABLE_STATE.with(|state| {
    state.borrow().as_ref().unwrap().canister_data
    .check_authorised(ic_cdk::caller().to_text());
    });
    // get cycles balance
    let cycles: u64 = ic_cdk::api::canister_balance();
    return cycles;
}

#[query]
#[cfg(target_arch = "wasm32")]
fn get_memory_stats() -> MemoryData {
    // check authorised
    STABLE_STATE.with(|state| {
        state.borrow().as_ref().unwrap().canister_data
        .check_authorised(ic_cdk::caller().to_text());
    });

    let wasm_page_size: u64 = 65536;
    let m: u64 =
        (ic_cdk::api::stable::stable64_size() as u64) * wasm_page_size +
        (core::arch::wasm32::memory_size(0) as u64) * wasm_page_size;
    let m2: u64 = (core::arch::wasm32::memory_size(0) as u64) * wasm_page_size;
    let ret = MemoryData {
        memory: m,
        heap_memory: m2,
    };
    return ret;
}


// [][] -------------------------------- [][]
// [][] --- Canister Setup/ Upgrades --- [][]
// [][] -------------------------------- [][]
#[init]
fn init() {
    state_init();
}

#[pre_upgrade]
fn pre_upgrade() {
    state_pre_upgrade();
}

#[post_upgrade]
fn post_upgrade() {
    state_post_upgrade();
}




// [][] ------------- [][]
// [][] --- Tests --- [][]
// [][] ------------- [][]

//#[test]
    // fn test_string_to_key(){
    //     let input: String = "2vxsx-fae.0000000000000000000000000000000000000000000000000000000000000004".to_string();
    //     let as_key: IDKey = string_to_key(&input).unwrap();
    //     let output: String = key_to_string(&as_key).unwrap();
    //     assert_eq!(input, output);

    //     let input2: String = "q6osm-57cdv-5zmcc-p7dtq-v2lpi-uuzkr-pzhgf-lncpe-ns2yr-cxqsc-uqe.0000000000000000000000000000000000000000000000000000000000000000".to_string();
    //     let as_key2:IDKey = string_to_key(&input2).unwrap();
    //     let output2: String = key_to_string(&as_key2).unwrap();
    //     assert_eq!(input2, output2);

    //     let input3: String = "q6osm".to_string();
    //     let as_key3:IDKey  = string_to_key(&input3).unwrap();
    //     let output3: String = key_to_string(&as_key3).unwrap();
    //     assert_eq!(input3, output3);
    // }