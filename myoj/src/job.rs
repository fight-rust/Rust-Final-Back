use serde::{Serialize, Deserialize};
use crate::judge::PostJob;

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Job {
    pub id: u64,
    pub created_time: String,
    pub updated_time: String,
    // pub submission: PostJob,
    // pub state: State,
    // pub result: Result,
    // pub score: f64,
    // pub cases: Vec<CaseResult>,
    // pub id:usize,
    // pub user: String,
    // pub problem: usize,
    // pub contest:usize,
    pub result: String,
    // pub answer_time: String,
    pub content:String,
    // pub run_time: usize,
    pub state: String,
}

impl Job {
    pub fn new() -> Job {
        Job { 
            id: 0, 
            created_time: String::new(),
            updated_time: String::new(), 
            result: String::new(),
            content: String::new(),
            state: String::new(),
            // submission: PostJob::default(),
            // state: State::Default, 
            // result: Result::Default, 
            // score: 0.0, 
            // cases: Vec::new(), 
        }
    }
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