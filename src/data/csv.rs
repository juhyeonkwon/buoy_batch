use crate::db::maria_lib::Buoy;
extern crate csv;

use chrono::prelude::*;
use chrono::Duration;
use serde::Serialize;
use std::collections::HashMap;
use std::fs::File;

#[derive(Debug, Serialize)]
pub struct Insertbuoy {
    pub time: String,
    pub model: String,
    pub lat: f64,
    pub lon: f64,
    pub w_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
    pub group_id: i32,
}

pub fn write_csv(data: &[Buoy], hashmap: HashMap<String, i32>) {
    //현재시간에서 한시간 뺀거
    let now: DateTime<Local> = Local::now() - Duration::hours(1);

    let now_str = now.to_string();

    let file_name = format!("{}_{}", &now_str[0..10], &now_str[11..13]);

    //파일 생성
    let file = File::create(format!("./csv/{}", file_name)).expect("Can't create File");

    //csv파일로 저장
    let mut wtr = csv::Writer::from_writer(file);

    // let mut wtr = csv::Writer::from_writer
    for _buoy in data.iter() {
        wtr.serialize(Insertbuoy {
            time: String::from(&_buoy.time),
            model: String::from(&_buoy.model),
            lat: _buoy.lat,
            lon: _buoy.lon,
            w_temp: _buoy.w_temp,
            salinity: _buoy.salinity,
            height: _buoy.height,
            weight: _buoy.weight,
            group_id: *hashmap.get(&_buoy.model).expect("Error! hash no type"),
        })
        .expect("err");
    }

    wtr.flush().expect("err");
}
