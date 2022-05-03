use crate::db::maria_lib::{Buoy, DataBase};
use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
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
    group_latitude: f64,
    group_longitude: f64,
    group_water_temp: f64,
    group_salinity: f64,
    group_height: f64,
    group_weight: f64,
}

#[derive(Serialize, Debug)]
struct BuoyModel {
    pub model_idx: i16,
    pub model: String,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f32,
    pub salinity: f32,
    pub height: f32,
    pub weight: f32,
}

pub fn insert(data: &[Buoy]) -> HashMap<String, i32> {
    let mut db = DataBase::init();
    let mut db2 = DataBase::init();
    //Buoy_Model에서 모델의 정보들을 가져옵니다.
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
                  "gruop_id" => match hashmap.get(&buoy.model) {
                      Some(v) => v,
                      None => {
                        hashmap.insert(String::from(&buoy.model), 0);
                        create_buoy_model(&buoy.model, &mut db2);
                        &0
                      }
                  },
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

    //buoy의 각 값들을 최신값으로 업데이트
    update_buoy(&mut db, data);

    //최신값을 토대로 경고값을 설정합니다
    update_warn_buoy(&mut db);

    hashmap
}

//만약 model에 없는 새로운 모델이 값으로 들어왔다면 해당 모델을 buoy_model에 Insert합니다.
pub fn create_buoy_model(model: &String, db: &mut DataBase) {
    let stmt = db.conn.prep("INSERT INTO 
                  buoy_model (model, 
                                group_id, 
                                line, 
                                latitude, 
                                longitude, 
                                water_temp, 
                                salinity, 
                                height, 
                                weight, 
                                warn
                            ) 
                                VALUES (:model, 0, 0, 0, 0, 0, 0, 0, 0, 0)").expect("Error!");
    
    db.conn.exec_drop(stmt, params! {
        "model" => model 
    }).expect("Error!");
    
}

//buoy의 각 값들을 최신값으로 업데이트 합니다.
pub fn update_buoy(db: &mut DataBase, data: &[Buoy]) {
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

    //그룹의 평균값을 저장
    update_group_avg(db);
}


#[derive(Serialize, Debug)]
struct UserId{
    pub idx : i32
}

//각 최신값들을 토대로 그룹들의 현재 평균값을 저장합니다.
pub fn update_group_avg(db: &mut DataBase) {
    
    //1. 유저 id를 가져옵니다

    let user_idx : Vec<UserId> = db.conn.query_map("SELECT idx from users", |idx| {
        UserId { idx }
    }).expect("DB ERROR!");


    //유저별 그룹의 평균값 저장
    for idx in user_idx.iter() {
        let stmt = db.conn.prep("SELECT group_id, 
                                    AVG(latitude) AS group_latitude, 
                                    AVG(longitude) AS group_longitude, 
                                    AVG(water_temp) AS group_water_temp, 
                                    AVG(salinity) AS group_salinity, 
                                    AVG(height) AS group_height, 
                                    AVG(weight) AS group_weight
                                FROM buoy_model WHERE user_idx = :idx AND group_id > 0 GROUP BY group_id").expect("Err");
        let _row = db
        .conn
        .exec_map(
            stmt, params!{"idx" => idx.idx},
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

#[derive(Debug, Deserialize, Serialize)]
pub struct Group {
    pub group_id: String,
    pub group_name: String,
    pub group_latitude: f64,
    pub group_longitude: f64,
    pub group_water_temp: f64,
    pub group_salinity: f64,
    pub group_height: f64,
    pub group_weight: f64,
    pub user_idx : i32,
}

pub fn get_group_avg() -> Vec<Group> {
    let mut db = DataBase::init();

    let data: Vec<Group> = db
        .conn
        .query_map(
            "SELECT group_id,
            group_name,
            group_latitude,
            group_longitude,
            group_water_temp,
            group_salinity,
            group_height,
            group_weight,
            user_idx FROM buoy_group where group_id > 0",
            |(
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
                user_idx
            )| Group {
                group_id,
                group_name,
                group_latitude,
                group_longitude,
                group_water_temp,
                group_salinity,
                group_height,
                group_weight,
                user_idx
            },
        )
        .expect("query Error occured");

    data
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Line {
    pub group_id: i16,
    pub group_name: String,
    pub line: i16,
    pub latitude: f64,
    pub longitude: f64,
    pub water_temp: f64,
    pub salinity: f64,
    pub height: f64,
    pub weight: f64,
}

pub struct List {
    pub group_id: i16,
    pub group_name: String,
}

pub fn get_line_avg(row: &Vec<List>, db: &mut DataBase) -> Value {
    let stmt = db
        .conn
        .prep(
            "SELECT b.group_id, b.group_name, 
                line,
                AVG(latitude) as latitude,
                AVG(longitude) as longitude,
                AVG(water_temp) as water_temp,
                AVG(salinity) as salinity,
                AVG(height) as height,
                AVG(weight) as weight
            FROM
                buoy_model a
            INNER JOIN
                buoy_group b ON a.group_id = b.group_id
            WHERE
                a.group_id = :group_id GROUP BY a.line",
        )
        .expect("Error");

    let mut json: Value = json!({});

    for value in row.iter() {
        let data: Vec<Line> = db
            .conn
            .exec_map(
                &stmt,
                params! {
                    "group_id" => &value.group_id
                },
                |(
                    group_id,
                    group_name,
                    line,
                    latitude,
                    longitude,
                    water_temp,
                    salinity,
                    height,
                    weight,
                )| Line {
                    group_id,
                    group_name,
                    line,
                    latitude,
                    longitude,
                    water_temp,
                    salinity,
                    height,
                    weight,
                },
            )
            .expect("query Error occured");

        json[value.group_id.to_string()] = json!(data);
    }

    json
}

#[derive(Debug)]
struct Warn {
    pub model_idx: i16,
    pub model: String,
    pub warn: i8,
    pub temp_warn: i8,
    pub salinity_warn: i8,
    pub height_warn: i8,
    pub weight_warn: i8,
    pub location_warn: i8,
}

/*
latitude: f64,
longitude: f64,
water_temp: f32,
salinity: f32,
height: f32,
weight: f32,
*/

//각 스마트 부표별 경고여부를 설정합니다.
pub fn update_warn_buoy(db: &mut DataBase) {
    let buoy_model = get_buoy_model(db);
    // let mut warn = get_buoy_warn(db);

    let mut warn: Vec<Warn> = Vec::new();

    for value in buoy_model.iter() {
        let salinity_warn = set_salinity_warn(&value);
        let height_warn = set_height_warn(&value);
        let weight_warn = set_weight_warn(&value);

        let mut warn_yn: i8 = 0;

        if salinity_warn > 0 || height_warn > 0 || weight_warn > 0 {
            warn_yn = 1;
        }

        let temp = Warn {
            model_idx: value.model_idx,
            model: String::from(&value.model),
            warn: warn_yn,
            temp_warn: 0, //수온경고는 임시로 놔둠
            salinity_warn: salinity_warn,
            height_warn: height_warn,
            weight_warn: weight_warn,
            location_warn: 0, //위치 경고도 임시로 놔둠
        };

        warn.push(temp);
    }

    let stmt = db
        .conn
        .prep(
            "UPDATE buoy_model 
                                SET 
                                    temp_warn = :temp_warn, 
                                    salinity_warn = :salinity_warn, 
                                    height_warn = :height_warn, 
                                    weight_warn = :weight_warn, 
                                    location_warn = :location_warn,
                                    warn = :warn
                                WHERE
                                    model = :model",
        )
        .expect("Prep Error");

    db.conn
        .exec_batch(
            stmt,
            warn.iter().map(|warn| {
                params! {
                    "temp_warn" => warn.temp_warn,
                    "salinity_warn" => warn.salinity_warn,
                    "height_warn" => warn.height_warn,
                    "weight_warn" => warn.weight_warn,
                    "location_warn" => warn.location_warn,
                    "model" => &warn.model,
                    "warn" => warn.warn
                }
            }),
        )
        .expect("error occured");
}

fn get_buoy_model(db: &mut DataBase) -> Vec<BuoyModel> {
    let buoy: Vec<BuoyModel> = db
        .conn
        .query_map(
            "SELECT 
            model_idx,
            model,
            latitude,
            longitude,
            water_temp,
            salinity,
            height,
            weight
         FROM buoy_model order by model_idx asc",
            |(model_idx, model, latitude, longitude, water_temp, salinity, height, weight)| {
                BuoyModel {
                    model_idx,
                    model,
                    latitude,
                    longitude,
                    water_temp,
                    salinity,
                    height,
                    weight,
                }
            },
        )
        .expect("DB Error!");

    buoy
}

fn set_salinity_warn(val: &BuoyModel) -> i8 {
    if val.salinity < 28.3 {
        1
    } else if val.salinity > 33.0 {
        2
    } else {
        0
    }
}

fn set_height_warn(val: &BuoyModel) -> i8 {
    if val.height < 8.5 {
        1
    } else {
        0
    }
}

fn set_weight_warn(val: &BuoyModel) -> i8 {
    if val.weight > 70.0 {
        1
    } else {
        0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarnInfo {
    pub group_id: i16,
    pub group_name: String,
    pub line: i8,
    pub low_temp_warn: i8,
    pub high_temp_warn: i8,
    pub low_salinity_warn: i8,
    pub high_salinity_warn: i8,
    pub low_height_warn: i8,
    pub weight_warn: i8,
    pub location_warn: i8,
    pub mark: f32,
    pub user_idx : i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WarnData {
    pub group_id: i16,
    pub group_name: String,
    pub line: i8,
    pub warn_type: String,
    pub message: String,
    pub user_idx : i64,
}

impl<'a, 'b> PartialEq<WarnData> for WarnData {
    fn eq(&self, other: &WarnData) -> bool {
        self.group_id == other.group_id
            && self.group_name == other.group_name
            && self.line == other.line
            && self.warn_type == other.warn_type
            && self.message == other.message
    }
}

#[derive(Serialize, Debug)]
pub struct GroupList {
    pub group_id: i32,
    pub group_name: String,
}

pub fn get_warn_list(db: &mut DataBase) -> Vec<WarnData> {
    //배열 저장
    let mut vector: Vec<WarnData> = Vec::new();

    let group_list: Vec<GroupList> = db
        .conn
        .query_map(
            "SELECT group_id, group_name from buoy_group",
            |(group_id, group_name)| GroupList {
                group_id,
                group_name,
            },
        )
        .expect("db Error!");

    for list in group_list.iter() {
        let stmt = db
            .conn
            .prep(
                "SELECT a.group_id, b.group_name,
                                        a.line,
                                        SUM(temp_warn = 1) AS low_temp_warn,
                                        SUM(temp_warn = 2) AS high_temp_warn,
                                        SUM(salinity_warn = 1) AS low_salinity_warn,
                                        SUM(salinity_warn = 2) AS high_salinity_warn,
                                        SUM(height_warn = 1) AS low_height_warn,
                                        SUM(weight_warn = 1) AS weight_warn,
                                        SUM(location_warn = 1) AS location_warn,
                                        COUNT(*) * 0.5 AS mark,
                                        a.user_idx
                                    FROM buoy_model a, buoy_group b 
                                    WHERE a.group_id = b.group_id AND a.group_id = :group_id AND a.group_id > 0
                                    GROUP BY line",
            )
            .expect("STMT Error");

        let temp: Vec<WarnInfo> = db
            .conn
            .exec_map(
                stmt,
                params! {
                    "group_id" => list.group_id
                },
                |(
                    group_id,
                    group_name,
                    line,
                    low_temp_warn,
                    high_temp_warn,
                    low_salinity_warn,
                    high_salinity_warn,
                    low_height_warn,
                    weight_warn,
                    location_warn,
                    mark,
                    user_idx,
                )| WarnInfo {
                    group_id,
                    group_name,
                    line,
                    low_temp_warn,
                    high_temp_warn,
                    low_salinity_warn,
                    high_salinity_warn,
                    low_height_warn,
                    weight_warn,
                    location_warn,
                    mark,
                    user_idx,
                },
            )
            .expect("Error!");

        let mut temp_vec = set_warn_struct(&temp);

        vector.append(&mut temp_vec);
    }

    vector
}

fn set_warn_struct(warn_list: &[WarnInfo]) -> Vec<WarnData> {
    let mut temp_list: Vec<WarnData> = Vec::new();

    for warn in warn_list.iter() {
        if warn.low_temp_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("temperature"),
                message: String::from("low"),
                user_idx : warn.user_idx,
            });
        }

        if warn.high_temp_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("temperature"),
                message: String::from("high"),
                user_idx : warn.user_idx,
            });
        }

        if warn.low_salinity_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("salinity"),
                message: String::from("low"),
                user_idx : warn.user_idx,
            });
        }

        if warn.high_salinity_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("salinity"),
                message: String::from("high"),
                user_idx : warn.user_idx,
            });
        }

        if warn.low_height_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("height"),
                message: String::from("low"),
                user_idx : warn.user_idx,
            });
        }

        if warn.weight_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("weight"),
                message: String::from("high"),
                user_idx : warn.user_idx,
            });
        }

        if warn.location_warn as f32 > warn.mark {
            temp_list.push(WarnData {
                group_id: warn.group_id,
                group_name: String::from(&warn.group_name),
                line: warn.line,
                warn_type: String::from("location"),
                message: String::from("missing"),
                user_idx : warn.user_idx,
            });
        }
    }

    temp_list
}
