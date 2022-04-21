use super::maria::Group;
use super::processing::BuoyAvg;
use crate::db::maria_lib::Buoy;
use crate::db::redis_lib;

use chrono;
use chrono::prelude::*;
use chrono::Duration;
use redis::Commands;
use serde_json;
use serde_json::{json, Value};
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

pub fn set_avg_data(proceed_data: Vec<BuoyAvg>) {
    //키 날짜 세팅
    let now: DateTime<Local> = Local::now() - Duration::days(1);

    let now_str = now.to_string();
    let key_date = &now_str[0..10];

    //redis 세팅
    let mut conn = redis_lib::connect_redis();

    for data in proceed_data {
        let _key = format!("{}", &data.model);

        let _text: String = serde_json::to_string(&data).expect("parse Text error!");

        let _: () = redis::cmd("LPUSH")
            .arg(_key)
            .arg(_text)
            .query(&mut conn)
            .expect("redis SET Error ocuured");
    }
}

pub fn set_line_avg_data(proceed_data: Vec<Group>) {
    //키 날짜 세팅
    let now: DateTime<Local> = Local::now();
    let now_str = now.to_string();
    let key_date = &now_str[0..10];

    //redis 세팅
    let mut conn = redis_lib::connect_redis();

    for data in proceed_data {
        let mut val: Value = serde_json::to_value(&data).expect("Error! on ~");

        val["date"] = json!(key_date);

        let _key = format!("{}_group", data.group_name);

        let _text: String = serde_json::to_string(&val).expect("parse Text error!");

        let _: () = redis::cmd("LPUSH")
            .arg(_key)
            .arg(_text)
            .query(&mut conn)
            .expect("redis SET Error ocuured");
    }
}

pub fn set_group_avg_data(proceed_data: Vec<Group>) {
    //키 날짜 세팅
    let now: DateTime<Local> = Local::now();
    let now_str = now.to_string();
    let key_date = &now_str[0..10];

    //redis 세팅
    let mut conn = redis_lib::connect_redis();

    for data in proceed_data {
        let mut val: Value = serde_json::to_value(&data).expect("Error! on ~");

        val["date"] = json!(key_date);

        let _key = format!("{}_group", data.group_name);

        let _text: String = serde_json::to_string(&val).expect("parse Text error!");

        let _: () = redis::cmd("LPUSH")
            .arg(_key)
            .arg(_text)
            .query(&mut conn)
            .expect("redis SET Error ocuured");
    }
}
use crate::data::maria::List;
use crate::data::maria::Line;

//라인별 avg 데이터를 저장한다
pub fn set_group_line_avg_data(data : &mut Value, list : &Vec<List> ) {
    //키 날짜 세팅
    let now: DateTime<Local> = Local::now();
    let now_str = now.to_string();
    let key_date = &now_str[0..10];

    //redis 세팅
    let mut conn = redis_lib::connect_redis();

    for group in list.iter() {

        let mut temp : Vec<Value> = serde_json::from_value(data[&group.group_name].take()).expect("Parse Error!");

        for data in temp.iter_mut() {
            data["date"] = json!(key_date);
            let _key = format!("{}_group_line_{}", group.group_name, data["line"]);
            let _text: String = serde_json::to_string(&data).expect("parse Text error!");

            let _: () = redis::cmd("LPUSH")
                .arg(_key)
                .arg(_text)
                .query(&mut conn)
                .expect("redis SET Error ocuured");
        }
        
        // let _key = format!("{}_group_line_{}", group.group_name);

        // let _text: String = serde_json::to_string(&val).expect("parse Text error!");

        // let _: () = redis::cmd("LPUSH")
        //     .arg(_key)
        //     .arg(_text)
        //     .query(&mut conn)
        //     .expect("redis SET Error ocuured");
    }
}
