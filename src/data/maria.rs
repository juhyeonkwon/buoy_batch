use crate::db::maria_lib::{Buoy, DataBase};
use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::collections::HashMap;

use chrono;
use chrono::prelude::*;
use chrono::Duration;

#[derive(Debug)]
pub struct Insertbuoy {
    pub buoy: Buoy,
    pub group_id: i32,
}

#[derive(Serialize, Debug)]
pub struct Modelinfo {
    pub model: String,
    pub group_id: i32,
    pub line: i32,
    pub latitude: f32,
    pub longitude: f32,
}

#[derive(Serialize, Debug)]
pub struct GroupAvg {
    group_id: i32,
    group_latitude: f32,
    group_longitude: f32,
    group_water_temp: f32,
    group_salinity: f32,
    group_height: f32,
    group_weight: f32,
}

pub fn insert(data: &[Buoy]) -> HashMap<String, i32> {
    let mut db = DataBase::init();

    let row = db
        .conn
        .query_map(
            "SELECT model, group_id, line, latitude, longitude FROM buoy_model ORDER BY model_idx",
            |(model, group_id, line, latitude, longitude)| Modelinfo {
                model,
                group_id,
                line,
                latitude,
                longitude,
            },
        )
        .expect("queery Errror");

    let mut hashmap: HashMap<String, i32> = HashMap::new();

    for data in &row {
        hashmap.insert(String::from(&data.model), data.group_id);
    }

    let stmt = db.conn.prep("INSERT INTO buoy_data(model, group_id, time, latitude, longitude, water_temp, salinity, height, weight) VALUES (:model, :gruop_id, :time, :latitude, :longitude, :water_temp, :salinity, :height, :weight)").expect("error");

    db.conn
        .exec_batch(
            stmt,
            data.iter().map(|buoy| {
                params! {
                  "model" => &buoy.model,
                  "gruop_id" => hashmap.get(&buoy.model).expect("error! hash no type"),
                  "time" => &buoy.time,
                  "latitude" => buoy.lat,
                  "longitude" => buoy.lon,
                  "water_temp" => buoy.w_temp,
                  "salinity" => buoy.salinity,
                  "height" => buoy.height,
                  "weight" => buoy.weight
                }
            }),
        )
        .expect("error occured");

    update_buoy(db, data);

    hashmap
}

pub fn update_buoy(mut db: DataBase, data: &[Buoy]) {
    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_model 
                    SET latitude = :latitude, 
                        longitude = :longitude, 
                        water_temp = :water_temp,
                        salinity = :salinity,
                        height = :height,
                        weight = :weight
                    WHERE model = :model",
        )
        .expect("stmt error");

    db.conn
        .exec_batch(
            stmt,
            data.iter().map(|buoy| {
                params! {
                    "latitude" => buoy.lat,
                    "longitude" => buoy.lon,
                    "water_temp" => buoy.w_temp,
                    "salinity" => buoy.salinity,
                    "height" => buoy.height,
                    "weight" => buoy.weight,
                    "model" => &buoy.model
                }
            }),
        )
        .expect("Error!");

    println!("buoy_model update 완료");

    update_group_avg(db);
}

pub fn update_group_avg(mut db: DataBase) {
    let _row = db
        .conn
        .query_map(
            "SELECT group_id, 
                                AVG(latitude) AS group_latitude, 
                                AVG(longitude) AS group_longitude, 
                                AVG(water_temp) AS group_water_temp, 
                                AVG(salinity) AS group_salinity, 
                                AVG(height) AS group_height, 
                                AVG(weight) AS group_weight
                            FROM buoy_model GROUP BY group_id",
            |(
                group_id,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
            )| GroupAvg {
                group_id,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
            },
        )
        .expect("error!");

    let update_stmt = db
        .conn
        .prep(
            r"UPDATE buoy_group
                        SET                 
                            group_latitude   = :group_latitude,
                            group_longitude  = :group_longitude,
                            group_water_temp = :group_water_temp,
                            group_salinity   = :group_salinity,
                            group_height     = :group_height,
                            group_weight     = :group_weight
                        WHERE
                            group_id = :group_id",
        )
        .expect("Error on STMT");

    db.conn
        .exec_batch(
            update_stmt,
            _row.iter().map(|group| {
                params! {
                    "group_latitude" => group.group_latitude,
                    "group_longitude" => group.group_longitude,
                    "group_water_temp" => group.group_water_temp,
                    "group_salinity" => group.group_salinity,
                    "group_height" => group.group_height,
                    "group_weight" => group.group_weight,
                    "group_id" => group.group_id,
                }
            }),
        )
        .expect("Error!!");
}

//전날의 평균을 계산하기위해 하루치의 데이터들을 가져옵니다.
pub fn get_daily_data() -> Vec<Insertbuoy> {
    let now: DateTime<Local> = Local::now();

    let start_date = (now.date() - Duration::days(1)).to_string();
    let end_date = now.to_string();

    let mut db = DataBase::init();

    let query = r"
        SELECT group_id, model, CAST(time AS CHAR) as time, latitude, longitude, water_temp, salinity, height, weight FROM buoy_data 
            WHERE 
        buoy_data.time >= :start_date AND
        buoy_data.time <= :end_date;
    ";

    let stmt = db.conn.prep(query).expect("stmt error");

    let row = db
        .conn
        .exec_map(
            stmt,
            params! {
                "start_date" => &start_date[0..10],
                "end_date" => &end_date[0..10]
            },
            |(group_id, model, time, latitude, longitude, water_temp, salinity, height, weight)| {
                Insertbuoy {
                    buoy: Buoy {
                        time,
                        model,
                        lat: latitude,
                        lon: longitude,
                        w_temp: water_temp,
                        salinity,
                        height,
                        weight,
                    },
                    group_id,
                }
            },
        )
        .expect("error");

    row
}
