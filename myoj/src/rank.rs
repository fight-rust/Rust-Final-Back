use actix_web::{get, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use mysql::prelude::*;
use mysql::*;

#[derive(Serialize, Deserialize, Clone)]
struct Request {
    current_page:usize,
}

#[derive(Serialize, Deserialize, Clone)]
struct Response {
    uname:Vec<String>,
    uacnum:Vec<i32>
}

fn bubble_sort(vec: &mut Vec<i32>,tag:&mut Vec<String>) {
    for i in 0..vec.len() {
        for j in 0..vec.len() - 1 - i {
            if vec[j] <= vec[j + 1] {
                vec[j] = vec[j] ^ vec[j + 1];
                vec[j + 1] = vec[j] ^ vec[j + 1];
                vec[j] = vec[j] ^ vec[j + 1];
                let temp=tag[j].to_string();
                tag[j]=tag[j+1].to_string();
                tag[j+1]=temp;
            }
        }
    }
}

pub fn update(username: String){
    let url = "mysql://root:123456@127.0.0.1:3306/oj";
    let opts = Opts::from_url(url).unwrap();
    let pool = Pool::new(opts).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let mut query="select acnums from user_info where username='".to_owned();
    query.push_str(username.as_str());
    query.push_str("'");

    let mut acnums=0;

   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
        acnums=row.unwrap().get(0).unwrap();
   });

   acnums=acnums+1;

   query="update user_info set acnums=".to_owned();
    query.push_str(&acnums.to_string());
    query.push_str(" where username='");
    query.push_str(username.as_str());
    query.push_str("'");

   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
        row.ok();
   });


}


#[get("/api/rank/list")]
async fn get_rank() -> impl Responder {
   let url = "mysql://root:123456@127.0.0.1:3306/oj";
   let opts = Opts::from_url(url).unwrap();
   let pool = Pool::new(opts).unwrap();
   let mut conn = pool.get_conn().unwrap();

   let query="select username,acnums from user_info".to_owned();
    let mut uname:Vec<String>=Vec::new();
    let mut uacnum:Vec<i32>=Vec::new();

   conn.query_iter(query)
   .unwrap()
   .for_each(|row| {
        uname.push(row.as_ref().unwrap().get(0).unwrap());
        uacnum.push(row.unwrap().get(1).unwrap());

   });

   bubble_sort(&mut uacnum,&mut uname);

    let response=Response{
        uname:uname,
        uacnum:uacnum
    };
    HttpResponse::Ok().json(response)

}

