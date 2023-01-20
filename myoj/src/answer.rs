use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse, web, post};
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
    *(ANSWER_LIST.lock().unwrap()) = res;
}

#[get("/api/answers")]
async fn get_answers() -> impl Responder {
    load_answers();
    let answer_lock: MutexGuard<Vec<Answer>> = ANSWER_LIST.lock().unwrap();
    let response: Vec<Answer> = (*answer_lock).clone();
    drop(answer_lock);
    // update_json_file();
    HttpResponse::Ok().json(response)
}

// #[get("/api/answers/{answer_id}")]
// async fn get_answers_id(path: web::Path<usize>) -> impl Responder {
//     let answer_list = load_answers();
//     let answer_id: usize = path.into_inner();
//     let mut if_find_answer_id: bool = false;
//     for j in 0..answer_list.len() {
//         if answer_id == answer_list[j].id as usize {
//             if_find_answer_id = true;
//             break;
//         }
//     }
//     if if_find_answer_id == false {
//         println!("fail");
//         return HttpResponse::NotFound().json("不存在该比赛");
//     } 

//     let response: Answer = answer_list[answer_id - 1].clone();
//     HttpResponse::Ok().json(response)
// }