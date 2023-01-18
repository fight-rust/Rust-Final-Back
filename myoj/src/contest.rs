use std::sync::{MutexGuard, Arc, Mutex};
use actix_web::{get, Responder, HttpResponse, web};
use mysql::prelude::*;
use mysql::*;
use crate::global::{CONTEST_INFO, Contest,Response};

pub fn load_contests() {
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from contest_info";
    let mut res = conn.query_map(
        query,
        |(id, name, user, start_time, end_time)| Contest {
            id: id,
            name: name,
            user: user,
            start_time: start_time,
            end_time:end_time,
            problem_ids: Vec::new(),
        },
    ).expect("Query failed.");
   
    for mut r in &mut res{
        let mut  query = "select questionId from contest_question where contestId=".to_owned();
        query.push_str(r.id.to_string().as_str());
        println!("{:?}",query);
        let t:Vec<usize> = conn.query(query).unwrap();
        (*r).problem_ids=t;
    }
    
    *(CONTEST_INFO.lock().unwrap()) = res;
}

#[get("/api/contests")]
async fn get_contests() -> impl Responder {
    load_contests();
    let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    let response: Vec<Contest> = (*contest_lock).clone();
    drop(contest_lock);
    // update_json_file();
    HttpResponse::Ok().json(response)
}

#[get("/contests/{contest_id}")]
async fn get_contests_id(path: web::Path<usize>) -> impl Responder {
    let contest_id: usize = path.into_inner();
    let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    let contest_info: Vec<Contest> = (*contest_lock).clone();
    let contest_num: usize = contest_info.len();
    if contest_id > contest_num {
        println!("fail");
        return HttpResponse::NotFound().json("不存在该比赛");
    } 
    let response: Contest = contest_info[contest_id - 1].clone();
    drop(contest_lock);
    // update_json_file();
    HttpResponse::Ok().json(response)
}
