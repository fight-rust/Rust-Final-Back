
use actix_web::{get, Responder, HttpResponse, web, post};
use serde::{Serialize, Deserialize};
use mysql::prelude::*;
use mysql::*;
use crate::global::{Problem};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct NewProblem {
    title: String,
    content: String,
    example: String,
}

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

    let mut i=0;
    while i<problem_list.len()
    {
        let problem = problem_list[i].clone();
        if problem.id==problem_id{
            return HttpResponse::Ok().json(problem);
        }
        i=i+1;
    }
  
    HttpResponse::Ok().json("不存在该题目")
}

#[post("/api/admin/problem")]
async fn admin_add_problem(body: web::Json<NewProblem>) -> impl Responder {
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
   let opts = Opts::from_url(url).unwrap();
   let pool = Pool::new(opts).unwrap();
   let mut conn = pool.get_conn().unwrap();
    
   let mut query="insert into question_info values(NULL,'".to_owned();
   query.push_str(&body.title);
   query.push_str("','");
   query.push_str(&body.content);
   query.push_str("','");
   query.push_str(&body.example);
   query.push_str("')");

   conn.query_iter(query)
       .unwrap()
       .for_each(|row| {
            row.ok();
       });

    HttpResponse::Ok()
}


