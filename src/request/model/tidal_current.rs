use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::USER_AGENT;

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
    pub fn get_data(key: &str, location: &str, client : &reqwest::blocking::Client) -> Result<Value, Box<dyn std::error::Error>> {
        //통영 해만 16LTC09
        //거제 동부 18LTC12
        let date = TidalCurrentResp::get_today();

        let url: String =
            TidalCurrentResp::set_url_with_date("fcTidalCurrent", key, location, &date);

        let resp = client.get(&url).header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36").send().expect("Error!").text()?;

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

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalObsNowData {
    pub obs_time: String,
    pub current_direct: String,
    pub current_speed: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalObsNowMeta {
    pub obs_post_id: String,
    pub obs_post_name: String,
    pub obs_last_req_cnt: String,
    pub obs_lat: String,
    pub obs_lon: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalObsNowResult {
    pub data: Vec<TidalObsNowData>,
    pub meta: TidalObsNowMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalObsNowResp {
    pub result: TidalObsNowResult,
}

impl TidalObsNowResp {
    pub fn get_data(key: &str, location: &str, client : &reqwest::blocking::Client) -> Result<Value, Box<dyn std::error::Error>> {
        //tidalBu 조류관측소
        //tidalHfRadar 해수유동 관측소 (HF로 시작)

        let date = TidalObsNowResp::get_today();

        let mut url: String = TidalObsNowResp::set_url_with_date("tidalBu", key, location, &date);

        let resp = client.get(&url).header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36").send().expect("Error!").text()?;

        let value: Value = serde_json::from_str(&resp).expect("json parse error!");

        Ok(value)
    }

    pub fn get_last_data(value: &TidalObsNowResp) -> Value {
        let val = serde_json::to_value(&value.result.data[value.result.data.len() - 1])
            .expect("parse Error!");

        val
    }
}

impl RequestLib for TidalObsNowResp {}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalRaderNowData {
    pub lat: String,
    pub lon: String,
    pub current_direct: String,
    pub current_speed: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalRaderNowMeta {
    pub obs_post_id: String,
    pub obs_post_name: String,
    pub obs_last_req_cnt: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalRaderNowResult {
    pub data: Vec<TidalRaderNowData>,
    pub meta: TidalRaderNowMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalRaderNowResp {
    pub result: TidalRaderNowResult,
}

impl TidalRaderNowResp {
    pub fn get_data(key: &str, location: &str, client : &reqwest::blocking::Client) -> Result<Value, Box<dyn std::error::Error>> {
        //tidalBu 조류관측소
        //tidalHfRadar 해수유동 관측소 (HF로 시작)

        let date = TidalRaderNowResp::get_today_with_time();

        let url: String =
            TidalRaderNowResp::set_url_with_date("tidalHfRadar", key, location, &date);

        let resp = client.get(&url).header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36").send().expect("Error!").text()?;

        let value: Value = serde_json::from_str(&resp).expect("json parse error!");

        Ok(value)
    }

    pub fn get_last_data(value: &TidalRaderNowResp) -> Value {
        let val = serde_json::to_value(&value.result.data[value.result.data.len() - 1])
            .expect("parse Error!");

        val
    }
}

impl RequestLib for TidalRaderNowResp {}
