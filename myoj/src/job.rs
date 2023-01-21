use serde::{Serialize, Deserialize};
use std::sync::MutexGuard;
use actix_web::{get, Responder, HttpResponse};
use crate::{global::JOB_LIST};
use mysql::*;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub created_time: String,
    pub user: String,
    pub problem: usize,
    pub contest:usize,
    pub result: String,
    pub content:String,
    pub run_time: usize,
}

impl Job {
    pub fn new() -> Job {
        Job { 
            id: 0, 
            created_time: String::new(),
            user : String::new(),
            problem : 0,
            contest : 0,
            result: String::new(),
            content: String::new(),
            run_time: 0 ,
        }
    }
}

#[get("/api/jobs")]
async fn get_jobs() -> impl Responder {
    let job_lock: MutexGuard<Vec<Job>> = JOB_LIST.lock().unwrap();
    let response: Vec<Job> = (*job_lock).clone();
    drop(job_lock);
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