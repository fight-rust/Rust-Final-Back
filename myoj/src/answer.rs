use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse, web, post};
use mysql::prelude::*;
use mysql::*;
use crate::global::{Answer};

pub static mut  ANSWER_LIST: Vec<Answer> = Vec::new();

pub fn load_answers() ->Vec<Answer>{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from question_info";
    
    let  res = conn.query_map(
        query,
        |(id, title, content, example)| Answer {
            id: id,
            title: title,
            content: content,
            example: example,
        },
    ).expect("Query failed.");
    res
}

#[get("/api/answers")]
async fn get_answers() -> impl Responder {
    let response = load_answers();
    // update_json_file();
    HttpResponse::Ok().json(response)
}

#[get("/api/answers/{answer_id}")]
async fn get_answers_id(path: web::Path<usize>) -> impl Responder {
    let answer_list = load_answers();
    let answer_id: usize = path.into_inner();
    let mut if_find_answer_id: bool = false;
    for j in 0..answer_list.len() {
        if answer_id == answer_list[j].id as usize {
            if_find_answer_id = true;
            break;
        }
    }
    if if_find_answer_id == false {
        println!("fail");
        return HttpResponse::NotFound().json("不存在该比赛");
    } 

    let response: Answer = answer_list[answer_id - 1].clone();
    HttpResponse::Ok().json(response)
}

