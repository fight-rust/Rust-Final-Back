use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use contest::load_contests;
use problem::get_problems;
use problem::get_problems_id;
use problem::load_problems;
use judge::post_jobs;
use judge::init_joblist;
use job::Job;
use job::get_jobs;
use task::get_filter_jobs;

use crate::user::user_login;
use crate::user::user_register;
use crate::rank::get_rank;
use crate::contest::get_contests;
use crate::contest::get_contests_id;
use crate::contest::post_contest;
use crate::contest::admin_get_contests;
use crate::contest::admin_get_contests_list;
use crate::contest::admin_add_contest;
use crate::contest::admin_delete_contests;
use crate::problem::admin_add_problem;
use crate::answer::get_answers;
use crate::global::{JOB_LIST};

mod user;
mod test;
mod contest;
mod global;
mod problem;
mod rank;
mod answer;
mod judge;
mod job;
mod task;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    init_joblist();
    load_contests();
    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .service(user_login)
            .service(user_register)
            .service(get_contests)
            .service(get_contests_id)
            .service(admin_get_contests)
            .service(post_contest)
            .service(get_problems)
            .service(get_problems_id)
            .service(get_rank)
            .service(get_answers)
            .service(admin_get_contests_list)
            .service(admin_add_contest)
            .service(admin_delete_contests)
            .service(get_jobs)
            .service(post_jobs)
            .service(get_filter_jobs)
            .service(admin_add_problem)

    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}