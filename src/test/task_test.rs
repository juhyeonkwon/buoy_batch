//test
#[cfg(test)]
mod tests {
    use crate::db;

    use dotenv::dotenv;
    use std::sync::{Mutex, Arc};
    use crate::task;
    use std::thread;
    
    #[test]
    fn tesk_test() {
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
    
            task::task("Batch", &pool);

            //task에서 값을 가져온것을 토대로 경고 리스트를 저장합니다.
            task::warn_task("set Warn list", &pool);

        });
    
        //RDBMS에서 값을 가져와서 각 평균을 저장하는 크론잡 정의
        let avg_thread = thread::spawn(move || {
            let pool = Arc::clone(&pool2);
    
            println!("CronJob 2 : 평균값 계산 및 저장 실행");
    
            task::avg_task("AvgBatch", &pool);
        });
    
        //RDBMS에서 값을 가져와서 그룹의 평균을 저장하는 크론잡 정의
        let group_avg_thread = thread::spawn(move || {
            println!("CronJob 5 : 그룹의 평균값 계산 및 저장 실행");
            let pool = Arc::clone(&pool3);

            task::group_avg_task("GruopAvgBatch", &pool);
            task::get_line_avg_task("GroupLineBatchTask", &pool);

        });
    
        let main_data_thread = thread::spawn(move || {
            println!("CronJob 3 : 메인데이터(관측, 파고) 저장 실행");
            let pool = Arc::clone(&pool4);

            task::obs_all_task("Main 데이터 저장(obs, wave)", &pool);

        });
    
        let main_tidal_thread = thread::spawn(move || {
            let pool = Arc::clone(&pool5);
    
            println!("CronJob 4 : 메인데이터(조류 데이터) 저장 실행");
    
          
            task::tidal_all_task("Main 데이터 저장(조류)", &pool);

        });
    
        rdbms_thread.join().unwrap();
        avg_thread.join().unwrap();
        group_avg_thread.join().unwrap();
        main_data_thread.join().unwrap();
        main_tidal_thread.join().unwrap();
    }
    



}
