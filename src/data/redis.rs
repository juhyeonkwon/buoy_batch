use crate::db::maria_lib::Buoy;
use crate::db::redis_lib;
use chrono;
use chrono::prelude::*;
use chrono::Duration;
use redis::Commands;
use serde_json;
use serde_json::json;

pub fn get_data() {
    //key 설정
    let now = Local::now() + Duration::hours(-1);
    let now_str = now.to_string();

    let key = format!("{}_{}", &now_str[0..10], &now_str[11..13]);

    let mut conn = redis_lib::connect_redis();

    let items: Vec<String> = conn.lrange(key, 0, -1).expect("error!");

    println!("{:#?}", items);

    let mut bouys: Vec<Buoy> = Vec::new();

    for i in &items {
        let temp: Buoy = serde_json::from_str(i).unwrap();
        let t2 = json!(temp);
        bouys.push(temp);
        println!("{}", t2["model"]);
    }

    println!("{:#?}", bouys);
}
