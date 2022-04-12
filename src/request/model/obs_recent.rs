use serde::{Serialize, Deserialize};
use serde_json::Value;

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
  data: ObsRecentData,
  meta: ObsRecentMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsRecentResp {
  result : ObsRecentResult,
}

impl ObsRecentResp {
    pub fn get_data(key : &str, location : &str) -> Result<Value, Box<dyn std::error::Error>> {
      //거제도 DT_0029
      //통영 DT_0014
      let url : String = ObsRecentResp::set_url("tideObsRecent", key, location);

      let resp = reqwest::blocking::get(url)?.text()?;

      let value : Value = serde_json::from_str(&resp).expect("json parse error!");

      Ok(value)
  }
}

impl RequestLib for ObsRecentResp {}