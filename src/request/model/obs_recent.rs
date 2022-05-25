use serde::{Deserialize, Serialize};
use serde_json::Value;
use reqwest::header::USER_AGENT;

use super::RequestLib;

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsRecentData {
    pub Salinity: String,
    pub air_press: String,
    pub air_temp: String,
    pub record_time: String,
    pub tide_level: String,
    pub water_temp: String,
    pub wind_dir: String,
    pub wind_gust: String,
    pub wind_speed: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsRecentMeta {
    pub obs_last_req_cnt: String,
    pub obs_lat: String,
    pub obs_lon: String,
    pub obs_post_id: String,
    pub obs_post_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsRecentResult {
    pub data: ObsRecentData,
    pub meta: ObsRecentMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsRecentResp {
    pub result: ObsRecentResult,
}

impl ObsRecentResp {
    pub fn get_data(key: &str, location: &str, client : &reqwest::blocking::Client) -> Result<Value, Box<dyn std::error::Error>> {
        //거제도 DT_0029
        //통영 DT_0014
        let url: String = ObsRecentResp::set_url("tideObsRecent", key, location);

        let resp = client.get(&url).header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/101.0.4951.54 Safari/537.36").send().expect("Error!").text()?;
        // println!("{:#?}", res);
        // let resp = reqwest::blocking::get(url)?.text()?;

        let value: Value = serde_json::from_str(&resp).expect("json parse error!");

        Ok(value)
    }
}

impl RequestLib for ObsRecentResp {}
