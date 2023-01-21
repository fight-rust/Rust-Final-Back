use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse};
use mysql::prelude::*;
use mysql::*;
use crate::global::{Answer,ANSWER_LIST};


pub fn load_answers(){
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from answer_info";
    
   let res= conn.query_map(
        query,
        |(id,contest, problem,user, answer_time, content,result,run_time)| Answer {
            id:id,
            user: user,
            problem: problem,
            contest: contest,
            result: result,
            answer_time:answer_time,
            content:content,
            run_time:run_time
        },
    ).expect("Query failed.");

    let mut answer_list = ANSWER_LIST.lock().unwrap();
    *answer_list = res;
    drop(answer_list);
}

#[get("/api/answers")]
async fn get_answers() -> impl Responder {
    load_answers();
    let answer_lock: MutexGuard<Vec<Answer>> = ANSWER_LIST.lock().unwrap();
    let response: Vec<Answer> = (*answer_lock).clone();
    drop(answer_lock);
    HttpResponse::Ok().json(response)
}


pub fn add_answer(answer:Answer)->bool {
    println!("a");
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    
    //添加答题记录
    let mut x = match "insert into answer_info(contestId,questionId,username,answerTime,answerContent,result,runTime) values (?,?,?,?,?,?,?)"
        .with((answer.contest,answer.problem, answer.user,answer.answer_time,answer.content,answer.result,answer.run_time))
        .run(&mut conn) {
            Ok(_res) => {
                println!("ok");
                return true
            }
            Err(e) => {
                println!("{}",e);
                return false
            }
        };
}

impl Answer {
    pub fn new() -> Answer {
        Answer { 
            id: 0,
            user: String::new(),
            problem: 0,
            contest: 0,
            result:  String::new(),
            answer_time: String::new(),
            content: String::new(),
            run_time: 0,
        }
    }
}