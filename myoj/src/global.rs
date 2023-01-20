use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;
use crate::job::Job;
use serde::{Serialize, Deserialize};


//结构体
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub name: String,
    pub password:String,
    pub is_manager:usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Contest {
    pub id: usize,
    pub title: String,
    pub user: String,
    pub start_time: String,
    pub end_time: String,
    pub problem_ids: Vec<usize>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Answer {
    pub id:usize,
    pub user: String,
    pub problem: usize,
    pub contest:usize,
    pub result: String,
    pub answer_time: String,
    pub content:String,
    pub run_time: usize,
} 

#[derive(Serialize, Deserialize, Clone)]
pub struct Problem {
    pub id: usize,
    pub title: String,
    pub content: String,
    pub example: String,
} 


//全局变量
lazy_static! {
    pub static ref JOB_NUM: Arc<Mutex<u64>> = 
        Arc::new(Mutex::new(0));
} // record the serial number of judge jobs

lazy_static! {
    pub static ref JOB_LIST: Arc<Mutex<Vec<Job>>>
        = Arc::new(Mutex::new(Vec::new()));
} // Record all the judge jobs submitted

lazy_static! {
    pub static ref USER_LIST: Arc<Mutex<Vec<User>>>
         = Arc::new(Mutex::new(vec![User {
            name: "111".to_string(), // the default user
            password:"222".to_string(),
            is_manager:1
         }]));
} // Record the information of all the users

lazy_static! {
    pub static ref ANSWER_LIST: Arc<Mutex<Vec<Answer>>>
        = Arc::new(Mutex::new(Vec::new()));
    // contest_id = 0 means the global ranking list
    // for the basic requirements 5
}

lazy_static! {
    pub static ref CONTEST_INFO: Arc<Mutex<Vec<Contest>>>
        = Arc::new(Mutex::new(Vec::new()));
    // the advanced_requirements
    // record the information of contests
}