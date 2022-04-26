use serde_json;
use serde_json::json;
use serde_json::Value;

use mysql::prelude::*;
use mysql::*;

use super::model::obs_recent::ObsRecentResp;
use super::model::obs_wave_hight::ObsWaveHightResp;
use super::model::tidal_current::TidalCurrentResp;
use super::model::tidal_current::TidalObsNowResp;
use super::model::tidal_current::TidalRaderNowResp;

use crate::db::maria_lib;
use crate::db::redis_lib;

use std::env;

pub struct Location<'a> {
    pub obs_recent: &'a str,
    pub wave_hight: &'a str,
    pub tidal: &'a str,
}

static TONGYEONG: Location = Location {
    obs_recent: "DT_0014",
    wave_hight: "KG_0025",
    tidal: "16LTC09",
};

static GEOJEDO: Location = Location {
    obs_recent: "DT_0029",
    wave_hight: "KG_0025",
    tidal: "18LTC12",
};

pub fn set_data(location: &str) {
    let key = env::var("KHOA_API").expect("ENV not Found");

    let location_data = set_location(location);

    //데이터 가져 옴
    let recent = ObsRecentResp::get_data(&key, location_data.obs_recent).expect("error!");
    let tidal = TidalCurrentResp::get_data(&key, location_data.tidal).expect("error!");
    let wave_hight = ObsWaveHightResp::get_data(&key, location_data.wave_hight).expect("error!");

    //데이터 정리
    let recent_struct: ObsRecentResp = serde_json::from_value(recent).expect("Error!");

    let recent_val: Value = serde_json::to_value(&recent_struct.result.data).expect("Error!");

    let wave_hight_struct: ObsWaveHightResp = serde_json::from_value(wave_hight).expect("Error!");

    let wave_hight_val = serde_json::to_value(
        &wave_hight_struct.result.data[wave_hight_struct.result.data.len() - 1],
    )
    .expect("parse Error!");

    let tidal_str: TidalCurrentResp = serde_json::from_value(tidal).expect("Error!");
    let tidal_val: Value = tidal_str.get_close_data();

    //한곳으로 모음
    let mut json = json!({});

    json["obs_data"] = json!(recent_val);
    json["wave_hight"] = json!(wave_hight_val);
    json["tidal"] = json!(tidal_val);

    //레디스에 저장하기 위해 직렬화
    let redis_val = serde_json::to_string(&json).expect("Prase Error!");

    let _key = String::from(location) + "_main_data";

    let mut conn = redis_lib::connect_redis();

    let _: () = redis::cmd("SET")
        .arg(&_key)
        .arg(redis_val)
        .query(&mut conn)
        .expect("redis SET Error!");
}

pub fn set_location<'a>(location: &str) -> Location<'a> {
    let location_struct = match location {
        "tongyeong" => Location {
            obs_recent: TONGYEONG.obs_recent,
            wave_hight: TONGYEONG.wave_hight,
            tidal: TONGYEONG.tidal,
        },
        "geojedo" => Location {
            obs_recent: GEOJEDO.obs_recent,
            wave_hight: GEOJEDO.wave_hight,
            tidal: GEOJEDO.tidal,
        },
        _ => panic!("wrong data come"),
    };

    location_struct
}

#[derive(Debug)]
struct Obs {
    number: String,
    name: String,
}

//위치기반으로 하기위해서 데이터가 존재하는 모든것을 요청하고 받아온다음에 redis에 저장을 한다요.
pub fn set_all_obs_data() {
    //요청할 값들을 가져옴
    let mut db = maria_lib::DataBase::init();

    let data : Vec<Obs> = db.conn
    .query_map("SELECT number, name FROM observation_list WHERE tide_level = 1 AND w_temperature = 1 AND salinity = 1 AND air_temperature = 1 AND wind_velocity = 1", |(number, name)| Obs { number, name })
    .expect("query Error occured");

    //레디스 연결
    let mut conn = redis_lib::connect_redis();

    let key = "HefXKhyZpMNUAxmmMcpUg==";

    for val in data.iter() {
        let recent = match ObsRecentResp::get_data(key, &val.number) {
            Ok(v) => v,
            Err(_) => {
                println!("{}의 OBS 데이터가 존재하지 않습니다. ", val.number);
                continue;
            }
        };

        let recent_struct: ObsRecentResp = serde_json::from_value(recent).expect("Error!");
        let recent_val: String = serde_json::to_string(&recent_struct.result.data).expect("Error!");

        let _key = String::from("obs_") + &val.number;

        let _: () = redis::cmd("SET")
            .arg(_key)
            .arg(&recent_val)
            .query(&mut conn)
            .expect("redis SET Error!");
    }
}

pub fn set_all_wave_height_data() {
    //요청할 값들을 가져옴
    let mut db = maria_lib::DataBase::init();

    let data: Vec<Obs> = db
        .conn
        .query_map(
            "SELECT number, name FROM observation_list WHERE digging = 1",
            |(number, name)| Obs { number, name },
        )
        .expect("query Error occured");

    //레디스 연결
    let mut conn = redis_lib::connect_redis();

    let key = "HefXKhyZpMNUAxmmMcpUg==";

    for val in data.iter() {
        let wave_hight = match ObsWaveHightResp::get_data(&key, &val.number) {
            Ok(v) => v,
            Err(_) =>  {
                println!("{}의 파도, 파고 데이터 존재하지 않습니다.", val.number);
                continue;
            }
        };
        let wave_hight_struct: ObsWaveHightResp = match serde_json::from_value(wave_hight) {
            Ok(v) => v,
            Err(_) => {
                println!("{}의 파도, 파고 데이터 존재하지 않습니다.", val.number);
                continue;
            }
        };

        let wave_hight_val = serde_json::to_string(
            &wave_hight_struct.result.data[wave_hight_struct.result.data.len() - 1],
        )
        .expect("parse Error!");

        let _key = String::from("wave_hight_") + &val.number;

        let _: () = redis::cmd("SET")
            .arg(&_key)
            .arg(&wave_hight_val)
            .query(&mut conn)
            .expect("redis SET Error!");
    }
}

struct Tidal {
    number: String,
    name: String,
    tide_velocity: i16,
}

pub fn set_all_tidal_data() {
    //요청할 값들을 가져옴
    let mut db = maria_lib::DataBase::init();

    let data: Vec<Tidal> = db
        .conn
        .query_map(
            "SELECT number, name, tide_velocity FROM observation_list WHERE tide_velocity > 0",
            |(number, name, tide_velocity)| Tidal {
                number,
                name,
                tide_velocity,
            },
        )
        .expect("query Error occured");

    //레디스 연결
    let mut conn = redis_lib::connect_redis();

    let key = "HefXKhyZpMNUAxmmMcpUg==";

    for val in data.iter() {
        let mut tidal: Value;
        let mut tidal_val: String = String::from("");

        if val.tide_velocity == 1 {
            tidal = TidalObsNowResp::get_data(&key, &val.number).expect("데이터 가져오기 error!");
            // println!("{:#?}", tidal);
            let tidal_struct: TidalObsNowResp = match serde_json::from_value(tidal) {
                Ok(v) => v,
                Err(_) => {
                    println!("data not exsist in {}", val.number);
                    continue;
                }
            };

            tidal_val = serde_json::to_string(
                &tidal_struct.result.data[tidal_struct.result.data.len() - 1],
            )
            .expect("parse Error!");
        } else if val.tide_velocity == 2 {
            tidal = TidalRaderNowResp::get_data(&key, &val.number).expect("error!");
            let tidal_struct: TidalRaderNowResp = match serde_json::from_value(tidal) {
                Ok(v) => v,
                Err(e) => {
                    println!(
                        "{:#?}, {} 조류 데이터가 아직 존재하지 않습니다.",
                        e, val.number
                    );
                    continue;
                }
            };

            tidal_val = serde_json::to_string(&tidal_struct.result.data).expect("parse Error!");
        }

        let _key = String::from("tidal_") + &val.number;

        let _: () = redis::cmd("SET")
            .arg(&_key)
            .arg(&tidal_val)
            .query(&mut conn)
            .expect("redis SET Error!");
    }
}
