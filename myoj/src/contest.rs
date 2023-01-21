use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse, web, post};
use serde::{Serialize, Deserialize};
use mysql::prelude::*;
use mysql::*;
use crate::{global::{CONTEST_INFO, Contest}, problem::load_problems};

#[derive(Serialize, Deserialize, Clone)]
struct Response {
    pid:Vec<i32>,
    ptitle:Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct ContestList {
    cid:Vec<i32>,
    ctitle:Vec<String>
}

#[derive(Serialize, Deserialize, Clone)]
struct ContestCreate {
    problem:Vec<i32>,
    startTime:String,
    endTime:String,
    title:String
}

#[derive(Serialize, Deserialize, Clone)]
struct DeleteContest {
    deleteid:Vec<i32>,
}

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
            title: name,
            user: user,
            start_time: start_time,
            end_time:end_time,
            problem_ids: Vec::new(),
        },
    ).expect("Query failed.");
   
    for mut r in &mut res{
        let mut  query = "select questionId from contest_question where contestId=".to_owned();
        query.push_str(r.id.to_string().as_str());
        // println!("{:?}",query);
        let t:Vec<usize> = conn.query(query).unwrap();
        (*r).problem_ids=t;
    }
    
    *(CONTEST_INFO.lock().unwrap()) = res;
    // let mut contest_list = CONTEST_INFO.lock().unwrap();
    // *contest_list = res;
    // drop(contest_list);
}

pub fn add_contest(contest:Contest) ->bool {
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    //添加比赛
    let mut x = match "insert into contest_info values (?,?,?,?,?)"
            .with((contest.id,contest.title, contest.user,contest.start_time,contest.end_time))
            .run(&mut conn) {
            Ok(res) => {
                true
            }
            Err(e) => {
                return false
            }
        };
        println!("t");
    //添加比赛对应的题目
    for p in contest.problem_ids{
        println!("p");
        x = match "insert into contest_question values (?,?)"
                     .with((contest.id,p))
                    .run(&mut conn) {
                     Ok(res) => {
                        true
                        }
                     Err(e) => {
                        println!("{:?}", e);
                        return false
                     }
            };
    };
    true
}

#[get("/api/contests")]
async fn get_contests() -> impl Responder {
    // load_contests();
    // let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    // let response: Vec<Contest> = (*contest_lock).clone();
    // drop(contest_lock);
    // update_json_file();
    // HttpResponse::Ok().json(response)

    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
    let query = "select * from contest_info";

    let mut res = conn.query_map(
        query,
        |(id, name, user, start_time, end_time)| Contest {
            id: id,
            title: name,
            user: user,
            start_time: start_time,
            end_time:end_time,
            problem_ids: Vec::new(),
        },
    ).expect("Query failed.");

    for mut r in &mut res{
        let mut  query = "select questionId from contest_question where contestId=".to_owned();
        query.push_str(r.id.to_string().as_str());
        // println!("{:?}",query);
        let t:Vec<usize> = conn.query(query).unwrap();
        (*r).problem_ids=t;
    }
        HttpResponse::Ok().json(res)
}

#[get("/api/contests/{contest_id}")]
async fn get_contests_id(path: web::Path<usize>) -> impl Responder {
    let contest_id: usize = path.into_inner();
    let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    let contest_info: Vec<Contest> = (*contest_lock).clone();
   
    let mut i=0;
    while i<contest_info.len()
    {
        let contest = contest_info[i].clone();
        if contest.id==contest_id{
            drop(contest_lock);
            return HttpResponse::Ok().json(contest);
        }
        i=i+1;
    }
  
    drop(contest_lock);
    HttpResponse::Ok().json("不存在该比赛")
}

#[post("/api/admin/problem/getproblemlist")]
async fn admin_get_contests()->impl Responder{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
   let opts = Opts::from_url(url).unwrap();
   let pool = Pool::new(opts).unwrap();
   let mut conn = pool.get_conn().unwrap();

   let mut query="select questionId,questionTitle from question_info".to_owned();
   let mut pid:Vec<i32>=Vec::new();
   let mut ptitle:Vec<String>=Vec::new();

  conn.query_iter(query)
  .unwrap()
  .for_each(|row| {
    pid.push(row.as_ref().unwrap().get(0).unwrap());
    ptitle.push(row.unwrap().get(1).unwrap());

  });

    let response=Response{
        pid:pid,
        ptitle:ptitle
    };
    HttpResponse::Ok().json(response)
}

#[get("/api/admin/contest/get-contest-list")]
async fn admin_get_contests_list()->impl Responder{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
 
    let mut query="select contestId,contestTitle from contest_info".to_owned();
    let mut cid:Vec<i32>=Vec::new();
    let mut ctitle:Vec<String>=Vec::new();
 
   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
     cid.push(row.as_ref().unwrap().get(0).unwrap());
     ctitle.push(row.unwrap().get(1).unwrap());
 
   });
 
     let response=ContestList{
         cid:cid,
         ctitle:ctitle
     };
     HttpResponse::Ok().json(response)
}

#[post("/api/admin/contest")]
async fn admin_add_contest(body: web::Json<ContestCreate>)->impl Responder{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();
 
    let mut query="insert into contest_info values(NULL,'".to_owned();
    query.push_str(&body.title);
    query.push_str("','111','");
    query.push_str(&body.startTime);
    query.push_str("','");
    query.push_str(&body.endTime);
    query.push_str("')");
    println!("{}",query);
 
   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
     row.ok();
   });

   query="select contestId from contest_info where contestTitle='".to_owned();
    query.push_str(&body.title);
    query.push_str("'");
    println!("{}",query);

    let mut id=1;
 
   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
    id=row.unwrap().get(0).unwrap();
   });

   for i in 0..body.problem.len(){
    query="insert into contest_question values(".to_owned();
    query.push_str(id.to_string().as_str());
    query.push_str(",");
    query.push_str(&body.problem[i].to_string());
    query.push_str(")");
    println!("{}",query);
 
   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
    row.ok();
   });
   }


    HttpResponse::Ok()
}

#[post("/api/admin/contestdelete")]
async fn admin_delete_contests(body: web::Json<DeleteContest>) -> impl Responder{
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    println!("success accept");

    for i in 0..body.deleteid.len(){
        let mut query="delete from contest_info where contestId=".to_owned();
        query.push_str(&body.deleteid[i].to_string());

        conn.query_iter(query)
        .unwrap()
        .for_each(|row| {
            row.ok();
        });

        query="delete from contest_question where contestId=".to_owned();
        query.push_str(&body.deleteid[i].to_string());

        conn.query_iter(query)
        .unwrap()
        .for_each(|row| {
            row.ok();
        });
    }
 
    HttpResponse::Ok() 
}

#[post("/api/addContest")]
async fn post_contest(body: web::Json<Contest>) -> impl Responder {

    let mut contest_info: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    let contest_num: usize = contest_info.len(); // id from 1 to contest_num

    // let contest_id = body.id.clone();
    // if contest_id > contest_num { // the id is invalid
    //     return HttpResponse::NotFound().json("Contest 114514 not found.");
    // }
        
        let mut user_and_problem_valid: bool = true;
       
        let user_id = body.user.clone();
        //判断是否为管理员的函数

        //判断是否存在该问题
        let problem_list = load_problems();
        let problem_ids: Vec<usize> = body.problem_ids.clone();
        for i in 0..problem_ids.len() {
            let mut if_find_problem_id: bool = false;
            for j in 0..problem_list.len() {
                if problem_ids[i] == problem_list[j].id as usize {
                    if_find_problem_id = true;
                    break;
                }
            } // search the problem_id in the configuration
            if if_find_problem_id == false {
                user_and_problem_valid = false;
                break;
            }
        } // check the validity of problem_id
        if user_and_problem_valid == false {
            return HttpResponse::NotFound().json("不存在该问题".to_string());
        } 
        
        let res = add_contest(body.clone());
        if res==true
        {
            load_contests();
            return HttpResponse::Ok().json("发起比赛成功");
        }
        else {
            return HttpResponse::ExpectationFailed().json("数据库添加失败".to_string());
        }
}