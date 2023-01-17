extern crate dotenv;
extern crate sqlx;

use actix_web::{post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use mysql::prelude::*;
use mysql::*;

#[derive(Serialize, Deserialize, Clone)]
struct User {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Response {
    response:String,
}


#[post("/api/users/login")]
async fn user_login(body: web::Json<User>) -> impl Responder {

    let url = "mysql://root:123456@127.0.0.1:3306/oj";
   let opts = Opts::from_url(url).unwrap();
   let pool = Pool::new(opts).unwrap();
   let mut conn = pool.get_conn().unwrap();

   let mut query="select * from user_info where username='".to_owned();
   query.push_str(&body.username);
   query.push_str("' and password='");
   query.push_str(&body.password);
   query.push_str("'");

    let mut isexist=0;

   conn.query_iter(query)
       .unwrap()
       .for_each(|row| {
           row.ok();
           isexist=1;
       });

    if isexist == 1{
        let response=Response{
            response:String::from("登录成功！")
        };
        HttpResponse::Ok().json(response)
    }
    else{
        let response=Response{
            response:String::from("用户名或密码错误！")
        };
        HttpResponse::Ok().json(response)
    }
}


#[post("/users/register")]
async fn user_register(body: web::Json<User>) -> impl Responder {
   let url = "mysql://root:123456@127.0.0.1:3306/oj";
   let opts = Opts::from_url(url).unwrap();
   let pool = Pool::new(opts).unwrap();
   let mut conn = pool.get_conn().unwrap();

   let mut query="select * from user_info where username='".to_owned();
   query.push_str(&body.username);
   query.push_str("'");

    let mut isexist=0;

   conn.query_iter(query)
       .unwrap()
       .for_each(|row| {
           row.ok();
           isexist=1;
       });

    if isexist == 1{
        let response=Response{
            response:String::from("用户名已存在！")
        };
        HttpResponse::Ok().json(response)
    }
    else{
        let mut query="insert into user_info values('".to_owned();
        query.push_str(&body.username);
        query.push_str("','");
        query.push_str(&body.password);
        query.push_str("')");
        conn.query_iter(query).unwrap();

        let response=Response{
            response:String::from("注册成功！")
        };
        HttpResponse::Ok().json(response)
    }
    
    
}