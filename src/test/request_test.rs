#[cfg(test)]
mod tests {
    use chrono;
    use chrono::prelude::*;
    // use chrono::Duration;
    // use chrono::NaiveDateTime;

    use serde_json;
    use serde_json::Value;

    // use serde::{Deserialize, Serialize};

    #[test]
    fn date_test() {
        let now = Local::now();

        let now2 = DateTime::parse_from_str("2022-04-12 11:00:00 +09:00", "%Y-%m-%d %H:%M:%S %z")
            .expect("error");

        let now3 = DateTime::parse_from_str("2022-04-12 12:00:00 +09:00", "%Y-%m-%d %H:%M:%S %z")
            .expect("error");

        println!("{}", now.timestamp());
        println!("{}", now2.timestamp());
        println!("{}", now3.timestamp());
    }

    use crate::request::model::obs_recent::ObsRecentResp;
    use crate::request::model::obs_wave_hight::ObsWaveHightResp;
    use crate::request::model::tidal_current::TidalCurrentResp;

    // #[test]
    // fn curl_test() -> Result<(), Box<dyn std::error::Error>> {
    //     //let resp = reqwest::get("https://www.khoa.go.kr/api/oceangrid/tideObsRecent/search.do?ServiceKey=HefXKhyZpMNUAxmmMcpUg==&ObsCode=DT_0029&ResultType=json");

    //     let key = "HefXKhyZpMNUAxmmMcpUg==";

    //     let location = "DT_0029";

    //     let _data = ObsRecentResp::get_data(key, location).expect("error!");
    //     let data2 = ObsWaveHightResp::get_data(key, "KG_0025").expect("error!");
    //     let data3 = TidalCurrentResp::get_data(key, "16LTC09").expect("error!");

    //     let sib: ObsWaveHightResp = serde_json::from_value(data2).expect("Error!");

    //     let _js = serde_json::to_value(&sib.result.data[sib.result.data.len() - 1])
    //         .expect("parse Error!");

    //     // println!("{:#?}", data);
    //     // println!("{:#?}", js);
    //     // println!("{:#?}", data3);

    //     let tidal: TidalCurrentResp = serde_json::from_value(data3).expect("Error!");
    //     let _val: Value = tidal.get_close_data();

    //     Ok(())
    // }

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

    use crate::request::requests;
    use dotenv::dotenv;

    // #[test]
    // fn main_data_store() {
    //     dotenv().ok();
    //     requests::set_data("geojedo");
    //     requests::set_data("tongyeong");
    // }

    use crate::request::requests::set_all_obs_data;
    use crate::request::requests::set_all_tidal_data;
    use crate::request::requests::set_all_wave_height_data;

    #[test]
    fn set_all_data_test() {
        dotenv().ok();
        set_all_obs_data();
    }

    #[test]
    fn set_all_wave_height_data_test() {
        dotenv().ok();
        set_all_wave_height_data();
    }

    #[test]
    fn set_all_tidal_data_test() {
        dotenv().ok();
        set_all_tidal_data();
    }

    use crate::request::model::tidal_current::TidalRaderNowResp;

    #[test]
    fn tidal_test() {
        let key = "HefXKhyZpMNUAxmmMcpUg==";

        let _val: Value = TidalRaderNowResp::get_data(key, "HF_0064").expect("error!");
    }
}
