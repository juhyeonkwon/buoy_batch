use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::USER_AGENT;

use super::RequestLib;

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightData {
    pub record_time: String,
    pub wave_height: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightMeta {
    pub obs_last_req_cnt: String,
    pub obs_lat: String,
    pub obs_lon: String,
    pub obs_post_id: String,
    pub obs_post_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightResult {
    pub data: Vec<ObsWaveHightData>,
    pub meta: ObsWaveHightMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightResp {
    pub result: ObsWaveHightResult,
}

impl ObsWaveHightResp {
    pub fn get_data(key: &str, location: &str, client : &reqwest::blocking::Client) -> Result<Value, Box<dyn std::error::Error>> {
        //남해동부 KG_0025

        let date = ObsWaveHightResp::get_today();

        let url: String = ObsWaveHightResp::set_url_with_date("obsWaveHight", key, location, &date);

        let resp = client.get(&url).header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36").send().expect("Error!").text()?;

        let value: Value = serde_json::from_str(&resp).expect("json parse error!");

        Ok(value)
    }
}
impl RequestLib for ObsWaveHightResp {}
