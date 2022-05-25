mod data;
mod db;
mod request;
mod task;
mod test;

use dotenv::dotenv;
extern crate job_scheduler;
use job_scheduler::{Job, JobScheduler};
use std::time::Duration;
use std::sync::{Mutex, Arc};

use std::thread;

fn main() {
    dotenv().ok();

    let pool = Arc::new(Mutex::new(db::maria_lib::DataBase::init().pool));
    let pool2 = Arc::clone(&pool);
    let pool3 = Arc::clone(&pool);
    let pool4 = Arc::clone(&pool);
    let pool5 = Arc::clone(&pool);

    //레디스에서 값을 가져와서 RDBMS, CSV 작업을하는 크론잡 정의
    let rdbms_thread = thread::spawn(move|| {
        let pool = Arc::clone(&pool);
        println!("CronJob 1 : RDBMS 저장 실행");
        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 1 * * * *".parse().unwrap(), || {
            task::task("Batch", &pool);

            //task에서 값을 가져온것을 토대로 경고 리스트를 저장합니다.
            task::warn_task("set Warn list", &pool);
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    //RDBMS에서 값을 가져와서 각 평균을 저장하는 크론잡 정의
    let avg_thread = thread::spawn(move || {
        let pool = Arc::clone(&pool2);

        println!("CronJob 2 : 평균값 계산 및 저장 실행");

        let mut sched = JobScheduler::new();

        //크론잡의 시간이 UTC로 되기 때문에 나는 어찌할 바를 모르겠다..
        // 15 + 9 = 24 (한국 시간 기준 0시)
        sched.add(Job::new("0 7 15 * * *".parse().unwrap(), || {
            task::avg_task("AvgBatch", &pool);
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    //RDBMS에서 값을 가져와서 그룹의 평균을 저장하는 크론잡 정의
    let group_avg_thread = thread::spawn(move || {
        println!("CronJob 5 : 그룹의 평균값 계산 및 저장 실행");
        let pool = Arc::clone(&pool3);

        let mut sched = JobScheduler::new();

        // 14 + 9 = 23 (한국 시간 기준 23시)
        sched.add(Job::new("0 55 14 * * *".parse().unwrap(), || {
            task::group_avg_task("GruopAvgBatch", &pool);
            task::get_line_avg_task("GroupLineBatchTask", &pool);
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    let main_data_thread = thread::spawn(move || {
        println!("CronJob 3 : 메인데이터(관측, 파고) 저장 실행");
        let pool = Arc::clone(&pool4);

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 0,15,30,45 * * * *".parse().unwrap(), || {
            task::obs_all_task("Main 데이터 저장(obs, wave)", &pool);
        }));

        loop {
            sched.tick();

            std::thread::sleep(Duration::from_millis(500));
        }
    });

    let main_tidal_thread = thread::spawn(move || {
        let pool = Arc::clone(&pool5);

        println!("CronJob 4 : 메인데이터(조류 데이터) 저장 실행");

        let mut sched = JobScheduler::new();

        sched.add(Job::new("0 0,30 * * * *".parse().unwrap(), || {
            task::tidal_all_task("Main 데이터 저장(조류)", &pool);
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
