mod data;
mod db;
mod task;
mod test;
mod request;

use dotenv::dotenv;
// use std::collections::HashMap;

extern crate cronjob;
use cronjob::CronJob;

use std::thread;

fn main() {
    dotenv().ok();

    //레디스에서 값을 가져와서 RDBMS, CSV 작업을하는 크론잡 정의
    let rdbms_thread = thread::spawn(|| {
        println!("CronJob 1 : RDBMS 저장 실행");
        // cron 잡 정의
        let mut cron1 = CronJob::new("Batch", task::task);

        //각 1분마다 Job 실행
        cron1.seconds("0");
        cron1.minutes("5");
        cron1.hours("*");

        cron1.start_job();
    });

    //RDBMS에서 값을 가져와서 각 평균을 저장하는 크론잡 정의
    let avg_thread = thread::spawn(|| {
        println!("CronJob 2 : 평균값 계산 및 저장 실행");

        let mut cron2 = CronJob::new("AvgBatch", task::avg_task);

        //매일 00:05:00에 작업 시작
        cron2.seconds("0");
        cron2.minutes("5");
        cron2.hours("0");

        cron2.start_job();
    });

    rdbms_thread.join().unwrap();
    avg_thread.join().unwrap();
}
