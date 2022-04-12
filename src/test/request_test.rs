#[cfg(test)]
mod tests {
    use chrono;
    use chrono::prelude::*;
    use chrono::Duration;
    use chrono::NaiveDateTime;

    use serde_json;
    use serde_json::Value;

    use serde::{Deserialize, Serialize};
    use std::collections::HashMap;

    #[test]
    fn date_test() {
      let now = Local::now();

      let now2 = DateTime::parse_from_str("2022-04-12 11:00:00 +09:00", "%Y-%m-%d %H:%M:%S %z").expect("error");

      let now3 = DateTime::parse_from_str("2022-04-12 12:00:00 +09:00", "%Y-%m-%d %H:%M:%S %z").expect("error");
      
      println!("{}", now.timestamp());
      println!("{}", now2.timestamp());
      println!("{}", now3.timestamp());      

    }

    use crate::request::model::obs_recent::ObsRecentResp;
    use crate::request::model::obs_wave_hight::ObsWaveHightResp;
    use crate::request::model::tidal_current::TidalCurrentResp;

    #[test]
    fn curl_test() {
      resq();
    }

    fn resq() -> Result<(), Box<dyn std::error::Error>>{

      //let resp = reqwest::get("https://www.khoa.go.kr/api/oceangrid/tideObsRecent/search.do?ServiceKey=HefXKhyZpMNUAxmmMcpUg==&ObsCode=DT_0029&ResultType=json");

      let key = "HefXKhyZpMNUAxmmMcpUg==";

      let location = "DT_0029";

      let data = ObsRecentResp::get_data(key, location).expect("error!");
      let data2 = ObsWaveHightResp::get_data(key, "KG_0025").expect("error!");
      let data3 = TidalCurrentResp::get_data(key, "16LTC09").expect("error!");

      let sib : ObsWaveHightResp = serde_json::from_value(data2).expect("Error!");

      let js = serde_json::to_value(&sib.result.data[sib.result.data.len()-1]).expect("parse Error!");

      println!("{:#?}", data);
      println!("{:#?}", js);
      println!("{:#?}", data3);
      Ok(())
    }

    use serde_json::json;

    #[test]
    fn json_test() {
      let mut a = json!({});
      let b = json!({
        "a" : 1234,
      });

      a["x"] = json!(2);
      a["b"] = b;

      println!("{:#?}", a);

    }
}
