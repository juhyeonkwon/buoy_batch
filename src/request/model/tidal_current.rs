use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::RequestLib;

use chrono;
use chrono::prelude::*;

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalCurrentData {
    pub current_dir: String,
    pub current_speed: String,
    pub pred_time: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalCurrentMeta {
    pub obs_code: String,
    pub obs_last_req_cnt: String,
    pub obs_location: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalCurrentResult {
    pub data: Vec<TidalCurrentData>,
    pub meta: TidalCurrentMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalCurrentResp {
    pub result: TidalCurrentResult,
}

impl TidalCurrentResp {
    pub fn get_data(key: &str, location: &str) -> Result<Value, Box<dyn std::error::Error>> {
        //통영 해만 16LTC09
        //거제 동부 18LTC12
        let date = TidalCurrentResp::get_today();

        let url: String =
            TidalCurrentResp::set_url_with_date("fcTidalCurrent", key, location, &date);

        let resp = reqwest::blocking::get(url)?.text()?;

        let value: Value = serde_json::from_str(&resp).expect("json parse error!");

        Ok(value)
    }

    //예보중에서 가장 가까운 값을 반환합니다.
    pub fn get_close_data(&self) -> Value {
        let data = &self.result.data;

        let now = Local::now();
        let mut now_min = 0;

        let mut current: i64 = 999999999;

        for (i, value) in data.iter().enumerate() {
            let string_time: String = String::from(&value.pred_time) + " +09:00";
            let temp_date = DateTime::parse_from_str(&string_time, "%Y-%m-%d %H:%M:%S %z")
                .expect("Date parse error");

            let temp_value = (now.timestamp() - temp_date.timestamp()).abs();

            if current > temp_value {
                now_min = i;
                current = temp_value;
            }
        }

        serde_json::to_value(&data[now_min]).expect("Error!")
    }
}

impl RequestLib for TidalCurrentResp {}
