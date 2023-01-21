use actix_web::{post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::global::{JOB_NUM, JOB_LIST, Answer};
use crate::answer::add_answer;
use crate::rank::update;
use crate::job::{Job};
use std::time::{Duration};
use std::{thread,};
use chrono::{Utc, SecondsFormat};
use mysql::prelude::*;
use mysql::*;
use rand::Rng;


#[derive(Clone, Deserialize, Serialize, Default, Debug)]
pub struct PostJob {
    pub source_code: String,
    pub user_name: String,
    pub contest_id: usize,
    pub problem_id: usize,
}

//初始化joblist
pub fn init_joblist(){
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from answer_info";
    
   let res= conn.query_map(
        query,
        |(id,contest, problem,user, answer_time, content,result,run_time)| Job {
            id:id,
            user: user,
            problem: problem,
            contest: contest,
            result: result,
            created_time:answer_time,
            content:content,
            run_time:run_time,
        },
    ).expect("Query failed.");
    let mut job_list = JOB_LIST.lock().unwrap();
    *job_list = res;
    let mut job_num = JOB_NUM.lock().unwrap();
    *job_num = (*(job_list)).len() as u64; 
    *job_num += 1;
    drop(job_list);
    drop(job_num);
}

#[post("/api/jobs")]
async fn post_jobs(body: web::Json<PostJob>) -> impl Responder {
    let mut job: Job = Job::new();
    job.created_time = Utc::now().
        to_rfc3339_opts(SecondsFormat::Millis, true);

    //生成job id
    let mut lock = JOB_NUM.lock().unwrap();
    *lock += 1;
    let job_num = *lock - 1; // get the global variable
    job.id = job_num;

    
    //job赋值
    job.result = String::from("Compiling");
    job.user = body.user_name.clone();
    job.problem = body.problem_id as usize;
    job.contest = body.contest_id as usize;
    job.run_time = 0 as usize;
    job.content = body.source_code.clone();

    //将job加入job_list
    let mut lock = JOB_LIST.lock().unwrap();
    (*lock).push(job.clone());
    drop(lock);

    //模拟判题过程
    thread::sleep(Duration::from_secs(3));

    //将answer写入数据库
    let mut answer: Answer = Answer::new();
    answer.user = body.user_name.clone();
    answer.problem = body.problem_id as usize;
    answer.contest = body.contest_id as usize;
    answer.answer_time = job.created_time.clone();
    let mut rng = rand::thread_rng();
    let t = rng.gen::<f64>();
    let rt = (t*1000.0+200.0) as usize;
    answer.run_time = rt;
    if rt > 1000 {
        answer.result = String::from("Time Limit Exceeded");
    }
    else if rt <= 200 {
        answer.result = String::from("Memory Limit Exceeded");
    }
    else if rt > 200 && rt <= 400 {
        answer.result = String::from("Compilation Error");
    }
    else if rt > 400 && rt <= 600 {
        answer.result = String::from("Wrong Answer");
    }
    else{
        answer.result = String::from("Answer Correct");
        update(body.user_name.clone());

    }
    answer.content = body.source_code.clone();
    
    let response = answer.clone();

    let mut lock = JOB_LIST.lock().unwrap();
    let mut job_id = job.id as usize;
    job_id -= 1;

    (*lock)[job_id].result = answer.result.clone();
    (*lock)[job_id].run_time = rt;
    
    add_answer(answer);
    drop(lock);

    HttpResponse::Ok().json(response)
}