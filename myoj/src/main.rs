use actix_web::{get, middleware::Logger, post, web, App, HttpServer, Responder};
use crate::user::user_login;
use crate::user::user_register;
use crate::contest::get_contests;
use crate::contest::get_contests_id;

mod user;
mod test;
mod contest;
mod global;

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

    })
    .bind(("127.0.0.1", 12345))?
    .run()
    .await
}
