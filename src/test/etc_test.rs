//test
#[cfg(test)]
mod tests {
    use chrono;
    use chrono::prelude::*;
    use chrono::Duration;

    use dotenv::dotenv;

    use mysql::prelude::*;
    use mysql::*;
    use serde::{Deserialize, Serialize};

    use mysql::params;

    // use std::env;

    use rand::prelude::*;

    use std::collections::HashMap;

    use crate::db::maria_lib::DataBase;

    #[test]
    fn get_time() {
        let now: DateTime<Local> = Local::now() - Duration::hours(1);

        let now_str = now.to_string();

        let cd = format!(
            "{}{}{}{}",
            &now_str[0..4],
            &now_str[5..7],
            &now_str[8..10],
            &now_str[11..13]
        );

        println!("{:?} 22", cd);
    }

    #[test]
    fn get_time2() {
        let now: DateTime<Local> = Local::now();

        let now_str = now.to_string();

        let ab = format!("{}{}{}", &now_str[0..4], &now_str[5..7], &now_str[8..10]);

        println!("{:?}", ab);
    }

    #[derive(Serialize, Debug)]
    pub struct Modelinfo {
        pub model: String,
        pub group_id: i32,
        pub latitude: f32,
        pub longitude: f32,
    }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Buoy {
        pub time: String,
        pub model: String,
        pub lat: f32,
        pub lon: f32,
        pub w_temp: f32,
        pub salinity: f32,
        pub height: f32,
        pub weight: f32,
    }

    #[derive(Debug)]
    pub struct Insertbuoy<'a> {
        pub buoy: Buoy,
        pub group_id: &'a i32,
    }

    #[test]
    fn send_data_test() {
        dotenv().ok();

        let mut db = DataBase::init();

        let row = db
            .conn
            .query_map(
                "SELECT model, group_id, latitude, longitude FROM buoy_model ORDER BY model_idx",
                |(model, group_id, latitude, longitude)| Modelinfo {
                    model,
                    group_id,
                    latitude,
                    longitude,
                },
            )
            .expect("queery Errror");

        let mut hashmap: HashMap<String, i32> = HashMap::new();

        for data in &row {
            hashmap.insert(String::from(&data.model), data.group_id);
        }

        println!(
            "{}, {}, {}",
            row[0].group_id, row[0].latitude, row[0].longitude
        );

        let now: DateTime<Local> = Local::now();
        let now_str = now.to_string();

        for n in 0..100 {
            let mut rng = rand::thread_rng();

            let mut model_name: String = String::from("buoy_");
            let number = (n + 1).to_string();

            model_name.push_str(&number);

            let data = Insertbuoy {
                group_id: hashmap.get(&model_name).expect("error! hash no type"),
                buoy: Buoy {
                    time: String::from(&now_str[0..19]),
                    model: model_name,
                    lat: row[n].latitude,
                    lon: row[n].longitude,
                    w_temp: rng.gen_range(12.5..13.5),
                    salinity: rng.gen_range(30.0..33.0),
                    height: rng.gen_range(10.0..20.0),
                    weight: rng.gen_range(40.0..50.0),
                },
            };

            println!("{:#?}", data);

            let stmt = db.conn.prep("INSERT INTO buoy_data(model, group_id, time, latitude, longitude, water_temp, salinity, height, weight) VALUES (:model, :gruop_id, :time, :latitude, :longitude, :water_temp, :salinity, :height, :weight)").expect("error");

            let row: Vec<Row> = db
                .conn
                .exec(
                    &stmt,
                    params! {
                      "model" => data.buoy.model,
                      "gruop_id" => data.group_id,
                      "time" => data.buoy.time,
                      "latitude" => data.buoy.lat,
                      "longitude" => data.buoy.lon,
                      "water_temp" => data.buoy.w_temp,
                      "salinity" => data.buoy.salinity,
                      "height" => data.buoy.height,
                      "weight" => data.buoy.weight
                    },
                )
                .expect("Error!");

            println!("{:#?}", row);
        }
    }

    #[derive(Serialize, Debug)]
    pub struct GroupAvg {
        group_id: i32,
        group_latitude: f32,
        group_longitude: f32,
        group_water_temp: f32,
        group_salinity: f32,
        group_height: f32,
        group_weight: f32,
    }
    #[test]
    fn update_avg_test() {
        dotenv().ok();
        let mut db = DataBase::init();

        let row = db
            .conn
            .query_map(
                "SELECT group_id, 
                                    AVG(latitude) AS group_latitude, 
                                    AVG(longitude) AS group_longitude, 
                                    AVG(water_temp) AS group_water_temp, 
                                    AVG(salinity) AS group_salinity, 
                                    AVG(height) AS group_height, 
                                    AVG(weight) AS group_weight
                                FROM buoy_model GROUP BY group_id",
                |(
                    group_id,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                )| GroupAvg {
                    group_id,
                    group_latitude,
                    group_longitude,
                    group_water_temp,
                    group_salinity,
                    group_height,
                    group_weight,
                },
            )
            .expect("error!");

        println!("{:#?}", row);

        let update_stmt = db
            .conn
            .prep(
                r"UPDATE buoy_group
                        SET                 
                            group_latitude   = :group_latitude,
                            group_longitude  = :group_longitude,
                            group_water_temp = :group_water_temp,
                            group_salinity   = :group_salinity,
                            group_height     = :group_height,
                            group_weight     = :group_weight
                        WHERE
                            group_id = :group_id",
            )
            .expect("Error on STMT");

        db.conn
            .exec_batch(
                update_stmt,
                row.iter().map(|group| {
                    params! {
                        "group_latitude" => group.group_latitude,
                        "group_longitude" => group.group_longitude,
                        "group_water_temp" => group.group_water_temp,
                        "group_salinity" => group.group_salinity,
                        "group_height" => group.group_height,
                        "group_weight" => group.group_weight,
                        "group_id" => group.group_id,
                    }
                }),
            )
            .expect("Error!!");
    }

    use crate::task::task;
    // #[test]
    // fn task_test() {
    //     dotenv().ok();
    //     task("test_task");
    // }

    // use crate::task::avg_task;
    // #[test]
    // fn avg_task_test() {
    //     dotenv().ok();
    //     avg_task("test_avg");
    // }

    // use crate::task::group_avg_task;
    // #[test]
    // fn group_avg_task_test() {
    //     dotenv().ok();
    //     group_avg_task("test_avg");
    // }

    // use crate::task::get_line_avg_task;
    // #[test]
    // fn get_line_avg_test() {
    //     dotenv().ok();
    //     get_line_avg_task("avg_line_test");
    // }

    // use crate::task::warn_task;
    // #[test]
    // fn warn_task_test() {
    //     dotenv().ok();
    //     warn_task("warn_test");
    // }

    #[derive(Serialize, Deserialize, Debug)]
    pub struct WarnData {
        pub group_id: i16,
        pub group_name: String,
        pub line: i8,
        pub warn_type: String,
        pub message: String,
    }

    impl<'a, 'b> PartialEq<WarnData> for WarnData {
        fn eq(&self, other: &WarnData) -> bool {
            self.group_id == other.group_id
                && self.group_name == other.group_name
                && self.line == other.line
                && self.warn_type == other.warn_type
                && self.message == other.message
        }
    }

    #[test]
    fn struct_eq_test() {
        let a = WarnData {
            group_id: 1,
            group_name: String::from("a"),
            line: 1,
            warn_type: String::from("ty"),
            message: String::from("abc"),
        };

        let b = WarnData {
            group_id: 1,
            group_name: String::from("a"),
            line: 1,
            warn_type: String::from("ty"),
            message: String::from("abc"),
        };

        if a == b {
            print!("같아요~")
        }
    }
}
