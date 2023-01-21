use std::sync::MutexGuard;
use actix_web::{post, web, Responder, HttpResponse};
use serde::{Serialize, Deserialize};
use crate::{global::JOB_LIST, job::Job};

#[derive(Clone, Deserialize, Serialize, Default, Debug)]
struct ListFilter {
    user_name: Option<String>,
    problem_id: Option<usize>,
} // the arguments for filtering jobs

#[post("api/jobs/filter")]
async fn get_filter_jobs(info: web::Json<ListFilter>) -> impl Responder {

    let job_lock: MutexGuard<Vec<Job>> = JOB_LIST.lock().unwrap();
    let job_list: Vec<Job> = (*job_lock).clone();
    let mut job: Vec<Job> = Vec::new();

    // obey the ascending order for times
    let mut job_index: i32 = 0; // use i32, as if use `usize`, 

    // it can't be minused 1 to `-1`
    while job_index <= job_list.len() as i32 - 1 {
        let i:usize = job_index as usize;
        let mut valid: bool = true;
        let problem_id: Option<usize> = info.problem_id.clone();
        let user_name: Option<String> = info.user_name.clone();

        if problem_id.is_some() == true {
            if job_list[i].problem != problem_id.unwrap() {
                valid = false;
            }
        } // problem_id

        if user_name.is_some() == true {
            if &job_list[i].user != &user_name.unwrap() {
                valid = false;
            }
        } // user_name
        if valid == true {
            job.push(job_list[i].clone());
        }
        job_index += 1; // search the next job
    }

    drop(job_lock);

    HttpResponse::Ok().json(job)
}
