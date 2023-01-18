use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse, web, post};
use mysql::prelude::*;
use mysql::*;
use crate::global::{Problem};

pub static mut  PROBLEM_LIST: Vec<Problem> = Vec::new();

pub fn load_problems() ->Vec<Problem>{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from question_info";
    
    let  res = conn.query_map(
        query,
        |(id, title, content, example)| Problem {
            id: id,
            title: title,
            content: content,
            example: example,
        },
    ).expect("Query failed.");
    res
}

#[get("/api/problems")]
async fn get_problems() -> impl Responder {
    let response = load_problems();
    // update_json_file();
    HttpResponse::Ok().json(response)
}

#[get("/api/problems/{problem_id}")]
async fn get_problems_id(path: web::Path<usize>) -> impl Responder {
    let problem_list = load_problems();
    let problem_id: usize = path.into_inner();
    let mut if_find_problem_id: bool = false;
    for j in 0..problem_list.len() {
        if problem_id == problem_list[j].id as usize {
            if_find_problem_id = true;
            break;
        }
    }
    if if_find_problem_id == false {
        println!("fail");
        return HttpResponse::NotFound().json("不存在该比赛");
    } 

    let response: Problem = problem_list[problem_id - 1].clone();
    HttpResponse::Ok().json(response)
}

