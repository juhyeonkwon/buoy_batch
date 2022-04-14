use chrono::prelude::*;

//rdbms task
pub fn task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();

    println!("{} 작업 실행 : {}", name, now_str);
    //redis에서 값을 가져옵니다.
    let buoys: Vec<super::db::maria_lib::Buoy> = super::data::redis::get_data();
    println!("redis 값 불러오기 완료");

    //redis에 가져온 값을 정제 후에 Maria에 Insert 합니다. 또한 각 모델의 gruop에 대한 hash값을 가져옵니다.
    let hash = super::data::maria::insert(&buoys);
    println!("insert 및 update 완료");

    //각 값을 csv파일로 저장 합니다.
    super::data::csv::write_csv(&buoys, hash);
    println!("csv저장 완료");
}

//avg_batch
pub fn avg_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    //전날의 모든 데이터를 가져옵니다.
    let _data = super::data::maria::get_daily_data();
    println!("데이터 불러오기 완료.");

    //데이터들을 정제합니다.
    let proceed_data = super::data::processing::avg_buoy_processing(_data);
    println!("데이터 정제 완료.");

    //데이터들을 redis에 저장합니다.
    super::data::redis::set_avg_data(proceed_data);
    println!("redis 저장 완료.");
}

//기상, 해양값을 가져오는 크론 잡 정의
pub fn obs_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_data("tongyeong");
    super::request::requests::set_data("geojedo");
    println!("기상, 해양 메인정보 저장 완료.");
}

//기상, 해양값 15분 간격
pub fn obs_all_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_all_obs_data();
    super::request::requests::set_all_wave_height_data();
}

//조류의 유속 등 30분 간격
pub fn tidal_all_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_all_tidal_data();
}
