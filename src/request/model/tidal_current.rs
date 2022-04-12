use serde::{Serialize, Deserialize};
use serde_json::Value;

use super::RequestLib;


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
  pub data : Vec<TidalCurrentData>,
  pub meta : TidalCurrentMeta,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct TidalCurrentResp {
  pub result : TidalCurrentResult,
}


impl TidalCurrentResp {
  pub fn get_data(key : &str, location : &str) -> Result<Value, Box<dyn std::error::Error>> {
    //통영 해만 16LTC09
    //거제 동부 18LTC12
    let date = TidalCurrentResp::get_today();

    let url : String = TidalCurrentResp::set_url_with_date("fcTidalCurrent", key, location, &date);

    println!("url : {}", url);

    
    let resp = reqwest::blocking::get(url)?.text()?;

    let value : Value = serde_json::from_str(&resp).expect("json parse error!");

    Ok(value)
  }
}

impl RequestLib for TidalCurrentResp {}