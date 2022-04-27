//test
#[cfg(test)]
mod tests {
    use crate::db;

    use dotenv::dotenv;
    use mysql::prelude::*;
    use mysql::*;
    use serde::Serialize;
    extern crate redis;
    use crate::db::maria_lib::Buoy;
    use std::env;

    #[derive(Serialize, Debug)]
    pub struct Test {
        pub idx: u32,
        pub test: String,
    }

    #[test]
    fn mysql_connection_test() {
        dotenv().ok();

        let mut db = db::maria_lib::DataBase::init();

        db.conn
            .query_map("SELECT * FROM test", |(idx, test)| Test { idx, test })
            .expect("query Error occured");
    }

    #[test]
    fn redis_connection_test() {
        dotenv().ok();
        let redis = env::var("REDIS").expect("ENV not Found");

        redis::Client::open(redis)
            .expect("error in open Redis.")
            .get_connection()
            .expect("faild to connect to Redis.");
    }

    use chrono;
    use chrono::prelude::*;
    use chrono::Duration;
    use std::collections::HashMap;
    #[derive(Debug)]
    pub struct Insertbuoy {
        pub buoy: Buoy,
        pub group_id: i32,
    }

    #[test]
    fn select_time_test() {
        dotenv().ok();

        let now: DateTime<Local> = Local::now() + Duration::days(1);

        let start_date = (now.date() - Duration::days(1)).to_string();
        let end_date = now.to_string();

        println!("{} {}", &start_date[0..10], &end_date[0..10]);

        let mut db = db::maria_lib::DataBase::init();

        let query = r"
            SELECT group_id, model, CAST(time AS CHAR) as time, latitude, longitude, water_temp, salinity, height, weight FROM buoy_data 
                WHERE 
            buoy_data.time >= :start_date AND
            buoy_data.time <= :end_date;
        ";

        let stmt = db.conn.prep(query).expect("stmt error");

        let row = db
            .conn
            .exec_map(
                stmt,
                params! {
                    "start_date" => &start_date[0..10],
                    "end_date" => &end_date[0..10]
                },
                |(
                    group_id,
                    model,
                    time,
                    latitude,
                    longitude,
                    water_temp,
                    salinity,
                    height,
                    weight,
                )| Insertbuoy {
                    buoy: Buoy {
                        time: time,
                        model: model,
                        lat: latitude,
                        lon: longitude,
                        w_temp: water_temp,
                        salinity: salinity,
                        height: height,
                        weight: weight,
                    },
                    group_id,
                },
            )
            .expect("error");

        let mut buoy_hashmap: HashMap<&String, Vec<&Insertbuoy>> = HashMap::new();
        for buoys in row.iter() {
            match buoy_hashmap.get_mut(&buoys.buoy.model) {
                Some(v) => {
                    v.push(buoys);
                }
                None => {
                    let mut vec: Vec<&Insertbuoy> = Vec::new();
                    vec.push(buoys);
                    buoy_hashmap.insert(&buoys.buoy.model, vec);
                }
            }
        }

        for _data in buoy_hashmap {
            let mut sum_w_temp: f32 = 0.0;
            let mut sum_salinity: f32 = 0.0;
            let mut sum_height: f32 = 0.0;
            let mut sum_weight: f32 = 0.0;

            let mut count: f32 = 0.0;

            //0은 키고 1은 값이네..
            for _insertbuoy in &_data.1 {
                sum_w_temp += _insertbuoy.buoy.w_temp;
                sum_salinity += _insertbuoy.buoy.salinity;
                sum_height += _insertbuoy.buoy.height;
                sum_weight += _insertbuoy.buoy.weight;
                count = count + 1.0;
            }

            let avg_w_temp: f32 = sum_w_temp / count;
            let avg_salinity: f32 = sum_salinity / count;
            let avg_height: f32 = sum_height / count;
            let avg_weight: f32 = sum_weight / count;

            println!(
                "{}의 값들 {} {} {} {} 개수 {}",
                _data.1[0].buoy.model, avg_w_temp, avg_salinity, avg_height, avg_weight, count
            );
        }
    }

    #[test]
    fn hash_tast() {
        let mut hashmap: HashMap<String, Vec<String>> = HashMap::new();

        match hashmap.get_key_value("1") {
            Some(v) => {
                println!("{:#?}", v)
            }
            None => {}
        };

        let key = "1";
        match hashmap.get_mut(key) {
            Some(v) => println!("{:#?}", v),
            None => {
                let strvec: Vec<String> = Vec::new();
                hashmap.insert(String::from(key), strvec);
            }
        }

        match hashmap.get_mut(key) {
            Some(v) => {
                println!("{:#?}", v);
                v.push(String::from("1123"));
            }
            None => {
                let strvec: Vec<String> = Vec::new();
                hashmap.insert(String::from(key), strvec);
            }
        }
    }

    #[derive(Debug)]
    struct Obs {
        number: String,
        name: String,
    }

    use crate::request::model::obs_recent::ObsRecentResp;

    #[test]
    fn db_test2() {
        dotenv().ok();

        let mut db = db::maria_lib::DataBase::init();

        let data : Vec<Obs> = db.conn
            .query_map("SELECT number, name FROM observation_list WHERE tide_level = 1 AND w_temperature = 1 AND salinity = 1 AND air_temperature = 1", |(number, name)| Obs { number, name })
            .expect("query Error occured");

        let key = "HefXKhyZpMNUAxmmMcpUg==";

        for val in data.iter() {
            let temp = ObsRecentResp::get_data(key, &val.number).expect("error!");
            println!("{:#?}", temp);
        }
    }

    #[derive(Debug)]
    struct Group {
        group_id: String,
        group_name: String,
        group_latitude: f64,
        group_longitude: f64,
        group_water_temp: f64,
        group_salinity: f64,
        group_height: f64,
        group_weight: f64,
        plain_buoy: i16,
    }

    #[test]
    fn group_avg() {
        dotenv().ok();

        let mut db = db::maria_lib::DataBase::init();
        let now: DateTime<Local> = Local::now();
        let now_str = now.to_string();

        let data: Vec<Group> = db
            .conn
            .query_map(
                "SELECT * FROM buoy_group where group_id > 0",
                |(
                    group_id,
                    group_name,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                    plain_buoy,
                )| Group {
                    group_id,
                    group_name,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                    plain_buoy,
                },
            )
            .expect("query Error occured");

        println!("{:#?}, {}", data, &now_str[0..10]);
    }

    #[derive(Debug)]
    struct BuoyModel {
        pub model_idx: i16,
        pub model: String,
        pub group_name: String,
        pub line: i8,
        pub latitude: f64,
        pub longitude: f64,
        pub water_temp: f32,
        pub salinity: f32,
        pub height: f32,
        pub weight: f32,
    }

    #[derive(Debug)]
    struct Warn {
        pub model_idx: i16,
        pub model: String,
        pub warn: i8,
        pub temp_warn: i8,
        pub salinity_warn: i8,
        pub height_warn: i8,
        pub weight_warn: i8,
        pub location_warn: i8,
    }

    use crate::data::maria::update_warn_buoy;

    #[test]
    fn warn_test() {
        dotenv().ok();

        let mut db = db::maria_lib::DataBase::init();

        // 높이 8.5cm 밑으로 가면 경고
        // 무게는 50 이상으로 가면 경고
        // 염도는 30 이하로 내려가면 경고
        // 33 이상이면 경고
        // 수온은 일단 보류
        // 한 라인의 부이 다수가 경고를 뛰우면 실질적인 경고를 내야하는듯...
        // batch 작업 시 각 값들을 가져와서 경고만 따로 Update를 한다음
        // 일정 시간에 경고수를 체크해서, 경고를 레디스나 maria에 저장한다음, 가져온다..
        update_warn_buoy(&mut db);
    }
}
