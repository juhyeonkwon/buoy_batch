use serde_json;
use serde_json::json;
use serde_json::Value;

use super::model::obs_recent::ObsRecentResp;
use super::model::obs_wave_hight::ObsWaveHightResp;
use super::model::tidal_current::TidalCurrentResp;

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
    let wave_hight = ObsWaveHightResp::get_data(&key, location_data.wave_hight).expect("error!");
    let tidal = TidalCurrentResp::get_data(&key, location_data.tidal).expect("error!");

    //데이터 정리
    let recent_struct :ObsRecentResp = serde_json::from_value(recent).expect("Error!");

    let recent_val : Value = serde_json::to_value(&recent_struct.result.data).expect("Error!");

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
