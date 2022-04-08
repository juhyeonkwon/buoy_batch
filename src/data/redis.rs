use crate::db::maria_lib::Buoy;
use crate::db::redis_lib;
use chrono;
use chrono::prelude::*;
use chrono::Duration;
use redis::Commands;
use serde_json;
// use serde_json::json;

pub fn get_data() -> Vec<Buoy> {
    //key 설정

    //각 정각에 한시간 이전의 모든 데이터들을 불러옵니다.
    let now = Local::now() + Duration::hours(-1);
    // let now = Local::now();

    let now_str = now.to_string();

    let key = format!("{}_{}", &now_str[0..10], &now_str[11..13]);

    //레디스에 연결합니다.
    let mut conn = redis_lib::connect_redis();

    //레디스에서 Value들을 받아옵니다.
    let items: Vec<String> = conn.lrange(key, 0, -1).expect("error!");

    let mut buoys: Vec<Buoy> = Vec::new();

    //받아온 Value들을 serde_json을 통해 Json객체로 변환한뒤, 그 객체를 Buoy Struct에 집어넣습니다.
    for i in &items {
        let temp: Buoy = serde_json::from_str(i).unwrap();
        // let t2 = json!(temp);
        buoys.push(temp);
        // println!("{}", t2["model"]);
    }

    //Redis에서 받아온 Vec<Buoy>를 반환합니다.
    buoys
}
