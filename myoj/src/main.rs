use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use contest::load_contests;
use problem::get_problems;
use problem::get_problems_id;
use problem::load_problems;

use crate::user::user_login;
use crate::user::user_register;
use crate::rank::get_rank;
use crate::contest::get_contests;
use crate::contest::get_contests_id;
use crate::contest::post_contest;
use crate::contest::admin_get_contests;
use crate::answer::get_answers;

mod user;
mod test;
mod contest;
mod global;
mod problem;
mod rank;
mod answer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
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

    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}