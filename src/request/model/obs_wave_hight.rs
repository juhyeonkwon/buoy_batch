use serde::{Serialize, Deserialize};
use serde_json::Value;

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
  pub obs_post_id:String,
  pub obs_post_name: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightResult {
  pub data : Vec<ObsWaveHightData>,
  pub meta : ObsWaveHightMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ObsWaveHightResp {
  pub result : ObsWaveHightResult,
}

impl ObsWaveHightResp {
    pub fn get_data(key : &str, location : &str) -> Result<Value, Box<dyn std::error::Error>> {
      //남해동부 KG_0025

      let date = ObsWaveHightResp::get_today();

      let url : String = ObsWaveHightResp::set_url_with_date("obsWaveHight", key, location, &date);

      println!("url : {}", url);
  
      
      let resp = reqwest::blocking::get(url)?.text()?;

      let value : Value = serde_json::from_str(&resp).expect("json parse error!");

      Ok(value)
    }
}
impl RequestLib for ObsWaveHightResp {}
