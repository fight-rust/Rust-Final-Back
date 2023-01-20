use serde::{Serialize, Deserialize};
use crate::judge::PostJob;
use std::sync::MutexGuard;
use actix_web::{get, put, web, Responder, HttpResponse};
use chrono::{NaiveDateTime, Utc, SecondsFormat};
use crate::{global::JOB_LIST};
use mysql::prelude::*;
use mysql::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub created_time: String,
    // pub updated_time: String,
    // pub submission: PostJob,
    // pub state: State,
    // pub result: Result,
    // pub score: f64,
    // pub cases: Vec<CaseResult>,
    // pub id:usize,
    pub user: String,
    pub problem: usize,
    pub contest:usize,
    pub result: String,
    // pub answer_time: String,
    pub content:String,
    pub run_time: usize,
    // pub state: String,
}

impl Job {
    pub fn new() -> Job {
        Job { 
            id: 0, 
            created_time: String::new(),
            user : String::new(),
            problem : 0,
            contest : 0,
            // updated_time: String::new(), 
            result: String::new(),
            content: String::new(),
            run_time: 0 ,
            // state: String::new(),
            // submission: PostJob::default(),
            // state: State::Default, 
            // result: Result::Default, 
            // score: 0.0, 
            // cases: Vec::new(), 
        }
    }
}

#[get("/api/jobs")]
async fn get_jobs() -> impl Responder {
    let job_lock: MutexGuard<Vec<Job>> = JOB_LIST.lock().unwrap();
    let response: Vec<Job> = (*job_lock).clone();
    drop(job_lock);
    // update_json_file();
    HttpResponse::Ok().json(response)
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum State {
    Queueing,
    Running,
    Finished,
    Canceled,
    Default,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub enum Result {
    Waiting,
    Running,
    Accepted,
    #[serde(rename = "Compilation Error")]
    CompilationError,
    #[serde(rename = "Compilation Success")]
    CompilationSuccess,
    #[serde(rename = "Wrong Answer")]
    WrongAnswer,
    #[serde(rename = "Runtime Error")]
    RuntimeError,
    #[serde(rename = "Time Limit Exceeded")]
    TimeLimitExceeded,
    #[serde(rename = "Memory Limit Exceeded")]
    MemoryLimitExceeded,
    #[serde(rename = "System Error")]
    SystemError,
    #[serde(rename = "SPJ Error")]
    SPJError,
    Skipped,
    Default,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CaseResult {
    pub id: u64,
    pub result: Result,
    pub time: u64,
    pub memory: u64,
    pub info: String,
}