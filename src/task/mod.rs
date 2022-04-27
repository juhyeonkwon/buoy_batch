use crate::db::maria_lib::DataBase;
use chrono::prelude::*;
use mysql::prelude::*;
use mysql::*;

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

//avg_batch 각 스마트 부표별 하루치 평균을 저장
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

    println!("{} 작업 완료 : {}", name, now_str);
}

//그룹 평균
pub fn group_avg_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    let _avg_data = super::data::maria::get_group_avg();
    println!("그룹 평균 데이터 불러오기 완료.");

    super::data::redis::set_group_avg_data(_avg_data);
    println!("그룹데이터 평균 redis 저장 완료.");

    println!("{} 작업 완료 : {}", name, now_str);
}

use crate::data::maria::List;

pub fn get_line_avg_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    let mut db = DataBase::init();

    let query = r"SELECT group_id, group_name FROM buoy_group";

    //그룹 리스트 불러옴
    let row: Vec<List> = db
        .conn
        .query_map(query, |(group_id, group_name)| List {
            group_id,
            group_name,
        })
        .expect("select Error");

    //그룹별 라인 평균값 가져옴
    let mut data: serde_json::Value = super::data::maria::get_line_avg(&row, &mut db);
    println!("그룹 라인별 평균 데이터 불러오기 완료.");

    super::data::redis::set_group_line_avg_data(&mut data, &row);
    println!("그룹 라인별 평균 데이터 저장 완료.");

    println!("{} 작업 완료 : {}", name, now_str);
}

//기상, 해양값을 가져오는 크론 잡 정의
pub fn obs_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_data("tongyeong");
    super::request::requests::set_data("geojedo");
    println!("{} 작업 완료 : {}", name, now_str);
}

//기상, 해양값 15분 간격
pub fn obs_all_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_all_obs_data();
    super::request::requests::set_all_wave_height_data();

    println!("{} 작업 완료 : {}", name, now_str);
}

//조류의 유속 등 30분 간격
pub fn tidal_all_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    super::request::requests::set_all_tidal_data();

    println!("{} 작업 완료 : {}", name, now_str);
}

//경고 TASK
pub fn warn_task(name: &str) {
    let now: DateTime<Local> = Local::now();

    let now_str = now.to_string();
    println!("{} 작업 실행 : {}", name, now_str);

    let mut db = DataBase::init();

    //현재 DB에서 경고 리스트를 불러옵니다.
    let mut warn_list = super::data::maria::get_warn_list(&mut db);

    //경고 리스트를 저장하고, 알람 리스트를 갱신합니다람쥐.

    super::data::redis::set_warn_redis(&mut warn_list);

    println!("{} 작업 완료 : {}", name, now_str);
}
