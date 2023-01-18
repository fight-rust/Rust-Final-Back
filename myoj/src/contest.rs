use std::sync::{MutexGuard};
use actix_web::{get, Responder, HttpResponse, web, post};
use mysql::prelude::*;
use mysql::*;
use crate::{global::{CONTEST_INFO, Contest}, problem::load_problems};

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
    load_contests();
    let contest_lock: MutexGuard<Vec<Contest>> = CONTEST_INFO.lock().unwrap();
    let response: Vec<Contest> = (*contest_lock).clone();
    drop(contest_lock);
    // update_json_file();
    HttpResponse::Ok().json(response)
}

#[get("/api/contests/{contest_id}")]
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
            return HttpResponse::Ok().json("发起比赛成功");
        }
        else {
            return HttpResponse::ExpectationFailed().json("数据库添加失败".to_string());
        }
    // `Id` is provided
}