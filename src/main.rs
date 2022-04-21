mod data;
mod db;
mod request;
mod task;
mod test;

use dotenv::dotenv;
extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;

use std::thread;

fn main() {
    dotenv().ok();

    //레디스에서 값을 가져와서 RDBMS, CSV 작업을하는 크론잡 정의
    let rdbms_thread = thread::spawn(|| {
        println!("CronJob 1 : RDBMS 저장 실행");
        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 1 * * * *".parse().unwrap(), || {
            task::task("Batch");
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    //RDBMS에서 값을 가져와서 각 평균을 저장하는 크론잡 정의
    let avg_thread = thread::spawn(|| {
        println!("CronJob 2 : 평균값 계산 및 저장 실행");

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 5 0 * * *".parse().unwrap(), || {
            task::avg_task("AvgBatch");
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    //RDBMS에서 값을 가져와서 그룹의 평균을 저장하는 크론잡 정의
    let group_avg_thread = thread::spawn(|| {
        println!("CronJob 5 : 그룹의 평균값 계산 및 저장 실행");

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 55 23 * * *".parse().unwrap(), || {
            task::group_avg_task("GruopAvgBatch");
            task::get_line_avg_task("GroupLineBatchTask");
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    let main_data_thread = thread::spawn(|| {
        println!("CronJob 3 : 메인데이터(관측, 파고) 저장 실행");

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 0,15,30,45 * * * *".parse().unwrap(), || {
            task::obs_all_task("Main 데이터 저장(obs, wave)");
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    let main_tidal_thread = thread::spawn(|| {
        println!("CronJob 4 : 메인데이터(조류 데이터) 저장 실행");

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 0,30 * * * *".parse().unwrap(), || {
            task::tidal_all_task("Main 데이터 저장(조류)");
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    rdbms_thread.join().unwrap();
    avg_thread.join().unwrap();
    group_avg_thread.join().unwrap();
    main_data_thread.join().unwrap();
    main_tidal_thread.join().unwrap();
}
