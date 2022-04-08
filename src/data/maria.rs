use crate::db::maria_lib::{Buoy, DataBase};
use mysql::prelude::*;
use mysql::*;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Insertbuoy {
    pub buoy: Buoy,
    pub group_id: i32,
}

#[derive(Serialize, Debug)]
pub struct Modelinfo {
    pub model: String,
    pub group_id: i32,
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

pub fn insert(data: &Vec<Buoy>) -> HashMap<String, i32> {
    let mut db = DataBase::init();

    let row = db
        .conn
        .query_map(
            "SELECT model, group_id, latitude, longitude FROM buoy_model ORDER BY model_idx",
            |(model, group_id, latitude, longitude)| Modelinfo {
                model,
                group_id,
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

pub fn update_buoy(mut db: DataBase, data: &Vec<Buoy>) {
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
    let row = db
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

}
