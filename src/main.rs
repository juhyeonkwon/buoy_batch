mod data;
mod db;
mod test;
use dotenv::dotenv;
// use std::collections::HashMap;
use chrono::prelude::*;

extern crate cronjob;
use cronjob::CronJob;

fn main() {
    dotenv().ok();

    //cron 잡 정의

    // let mut cron = CronJob::new("Batch", task);

    // //각 5분마다 Job 실행
    // cron.seconds("0");
    // cron.minutes("10");
    // cron.start_job();

    task("cron");
}

fn task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();

    println!("{} 작업 실행 : {}", name, now_str);
    //redis에서 값을 가져옵니다.
    let buoys: Vec<db::maria_lib::Buoy> = data::redis::get_data();
    println!("redis 값 불러오기 완료");

    //redis에 가져온 값을 정제 후에 Maria에 Insert 합니다. 또한 각 모델의 gruop에 대한 hash값을 가져옵니다.
    let hash = data::maria::insert(&buoys);
    println!("insert 완료");

    //각 값을 csv파일로 저장 합니다.
    data::csv::write_csv(&buoys, hash);
    println!("csv저장 완료");
}
