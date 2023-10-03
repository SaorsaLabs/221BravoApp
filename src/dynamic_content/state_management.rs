use std::cell::RefCell;
use std::ops::DerefMut;
use candid::{CandidType, encode_one, decode_one};
use serde::{ Deserialize, Serialize };
use ic_stable_memory::{
    retrieve_custom_data, stable_memory_init, stable_memory_post_upgrade,
    stable_memory_pre_upgrade, store_custom_data, SBox,
};
use ic_stable_memory::derive::{AsFixedSizeBytes, StableType};
use crate::custom_types::{Directory, LogEntry, CanisterSettings, NewsItem, IDKey, ProjectCard, ProjectCollection};
use crate::utils::{string_to_idkey, log};

// [][] ---------------------------------------- [][]
// [][] --- Main Stable and Runtime Elements --- [][]
// [][] ---------------------------------------- [][]

#[derive(StableType, AsFixedSizeBytes, Debug, Default)]
pub struct Main {
    pub canister_data: CanisterSettings,
}

#[derive(CandidType, Deserialize, Default, Clone)]
pub struct RuntimeState{
    pub canister_logs: Vec<LogEntry>,
    pub news_store: Vec<NewsItem>,
    pub project_store_1: Vec<ProjectCard>,
    pub project_store_2: Vec<ProjectCard>,
    pub project_store_3: Vec<ProjectCard>,
    pub project_store_4: Vec<ProjectCard>,
    pub project_store_5: Vec<ProjectCard>,
    pub project_store_6: Vec<ProjectCard>,
    pub project_store_7: Vec<ProjectCard>,
    pub project_store_8: Vec<ProjectCard>,
    pub member_research: Option<Vec<NewsItem>>
}

impl RuntimeState {
    pub fn add_news(&mut self, title:String, sub_title:String, article_url:String, image_url:String) -> String {
        let data:NewsItem = NewsItem {
            title,
            sub_title,
            article_url,
            image_url,
        };
        self.news_store.push(data);
        return "News Item Added".to_string();
    }
    pub fn remove_news(&mut self, index: usize) -> String {
        self.news_store.remove(index);
        return "News Item Removed".to_string();
    }
    pub fn get_all_news(&self) -> Vec<NewsItem> {
        return self.news_store.clone();
    }

    // research methods
    pub fn add_research(&mut self, title:String, sub_title:String, article_url:String, image_url:String) -> String {

        let data:NewsItem = NewsItem {
            title,
            sub_title,
            article_url,
            image_url,
        };

        // Check if some/ none and add data
        if let Some(mut vec) = self.member_research.clone() {
            vec.push(data);
            self.member_research = Some(vec);
        } else {
            self.member_research = Some(vec![data]);
        }

        return "Research Item Added".to_string();
    }
    pub fn remove_research(&mut self, index: usize) -> String {
        // Check if some/ none and remove if possible
        if let Some(mut vec) = self.member_research.clone() {
            vec.remove(index);
            self.member_research = Some(vec);
            return "Research Item Removed".to_string();
        } else {
            return "There is nothing in the research store".to_string();
        }
    }
    pub fn get_all_research(&self) -> Vec<NewsItem> {
        if let Some(vec) = self.member_research.clone() {
            return vec;
        } else {
            let ret = Vec::new();
            return ret;
        }
    }

    // project methods
    pub fn add_project(&mut self, store:u8, title:String, sub_title:String, project_url:String, image_url:String) -> String {
        let data:ProjectCard = ProjectCard {
            title,
            sub_title,
            project_url,
            image_url,
        };
        match store {
            1 => { self.project_store_1.push(data); },
            2 => { self.project_store_2.push(data); },
            3 => { self.project_store_3.push(data); },
            4 => { self.project_store_4.push(data); },
            5 => { self.project_store_5.push(data); },
            6 => { self.project_store_6.push(data); },
            7 => { self.project_store_7.push(data); },
            8 => { self.project_store_8.push(data); },
            _ => { return "Invalid Store Choice".to_string() },
        }
        return "Project Added".to_string();
    }
    pub fn remove_project(&mut self, store:u8, index: usize) -> String {
        match store {
            1 => { self.project_store_1.remove(index); },
            2 => { self.project_store_2.remove(index); },
            3 => { self.project_store_3.remove(index); },
            4 => { self.project_store_4.remove(index); },
            5 => { self.project_store_5.remove(index); },
            6 => { self.project_store_6.remove(index); },
            7 => { self.project_store_7.remove(index); },
            8 => { self.project_store_8.remove(index); },
            _ => { return "Invalid Store Choice".to_string() },
        }
        return "Project Removed".to_string();
    }
    pub fn get_all_projects(&self) -> ProjectCollection {
        let data: ProjectCollection = ProjectCollection { 
            bucket1: self.project_store_1.clone(), 
            bucket2: self.project_store_2.clone(),  
            bucket3: self.project_store_3.clone(), 
            bucket4: self.project_store_4.clone(), 
            bucket5: self.project_store_5.clone(),  
            bucket6: self.project_store_6.clone(),  
            bucket7: self.project_store_7.clone(),  
            bucket8: self.project_store_8.clone(), 
        };
        return data;
    }
    pub fn get_all_single_project(&self, store: u8) -> Vec<ProjectCard> {
        let data: Vec<ProjectCard>; 
        match store {
            1 => { data = self.project_store_1.clone() },
            2 => { data = self.project_store_2.clone() },
            3 => { data = self.project_store_3.clone() },
            4 => { data = self.project_store_4.clone() },
            5 => { data = self.project_store_5.clone() },
            6 => { data = self.project_store_6.clone() },
            7 => { data = self.project_store_7.clone() },
            8 => { data = self.project_store_8.clone() },
            _ => { return Vec::new(); },
        }
        return data;
    }

}

thread_local! {
    pub static RUNTIME_STATE: RefCell<RuntimeState> = RefCell::default();
    pub static STABLE_STATE: RefCell<Option<Main>> = RefCell::default();
}

pub fn state_init(){
    stable_memory_init();
    // init stable state
    let mut stable_data = Main::default();
    let default_admin: IDKey = string_to_idkey(&"ADMIN_PRINCIPAL_HERE".to_string()).unwrap();
    let frontend: IDKey = string_to_idkey(&"FRONTEND_PRINCIPAL_HERE".to_string()).unwrap(); 
    let default_canister_name = string_to_idkey(&"Name Me Please!".to_string()).unwrap();
    stable_data.canister_data.authorised.push(default_admin).expect("Out of memory");
    stable_data.canister_data.authorised.push(frontend).expect("Out of memory");
    stable_data.canister_data.canister_name = default_canister_name;
    STABLE_STATE.with(|state| {
        *state.borrow_mut() = Some(stable_data);
    });
    
    // init runtime state
    let runtime_date = RuntimeState::default();
    RUNTIME_STATE.with(|state| {
        *state.borrow_mut() = runtime_date;
    });
    log("Canister Initialised");
}

pub fn state_pre_upgrade(){
    // stable state
    let state: Main = STABLE_STATE.with(|s| s.borrow_mut().take().unwrap());
    let boxed_state = SBox::new(state).expect("Out of memory");
    store_custom_data(0, boxed_state);

    // runtime state
    let rstate = RUNTIME_STATE.with(|s|{s.borrow_mut().to_owned()});
    let bytes = encode_one(rstate).expect("Unable to candid encode");
    let boxed_bytes = SBox::new(bytes).expect("Out of memory");
    store_custom_data(1, boxed_bytes);

    stable_memory_pre_upgrade().expect("Out of memory");
}

pub fn state_post_upgrade(){
    stable_memory_post_upgrade();
    let state: Main = retrieve_custom_data::<Main>(0).unwrap().into_inner();
    STABLE_STATE.with(|s| {
      *s.borrow_mut() = Some(state);
    });

    // Runtime Storage 
    let bytes: Vec<u8> = retrieve_custom_data::<Vec<u8>>(1).unwrap().into_inner();
    let mut rstate: RuntimeState = decode_one(&bytes).expect("Unable to candid decode");
    RUNTIME_STATE.with(|s| {
        *s.borrow_mut() = rstate;
    });
}