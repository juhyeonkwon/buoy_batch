use chrono;
use chrono::prelude::*;

pub mod obs_recent;
pub mod obs_wave_hight;
pub mod tidal_current;

pub trait RequestLib {
    fn set_url(request_type: &str, key: &str, location: &str) -> String {
        String::from("https://www.khoa.go.kr/api/oceangrid/")
            + request_type
            + "/search.do?ServiceKey="
            + key
            + "&ObsCode="
            + location
            + "&ResultType=json"
    }

    fn set_url_with_date(request_type: &str, key: &str, location: &str, date: &str) -> String {
        String::from("https://www.khoa.go.kr/api/oceangrid/")
            + request_type
            + "/search.do?ServiceKey="
            + key
            + "&ObsCode="
            + location
            + "&Date="
            + date
            + "&ResultType=json"
    }

    fn get_today() -> String {
        let now: DateTime<Local> = Local::now();
        let now_str = now.to_string();

        format!("{}{}{}", &now_str[0..4], &now_str[5..7], &now_str[8..10])
    }
}
