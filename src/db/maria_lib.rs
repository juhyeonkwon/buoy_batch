// use mysql::prelude::*;
use mysql::*;
use serde::{Deserialize, Serialize};
use std::env;

// use actix_web::web::Json;

#[derive(Serialize)]
pub struct Test {
    pub idx: u32,
    pub test: String,
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

pub struct DataBase {
    pub pool: Pool,
    pub conn: PooledConn,
}

impl DataBase {
    pub fn init() -> DataBase {
        let user = env::var("MYSQL_USER_NAME").expect("ENV not Found");
        let password = env::var("MYSQL_PASSWORD").expect("ENV not Found");
        let ip = env::var("MYSQL_IP").expect("ENV not Found");
        let port = env::var("MYSQL_PORT").expect("").parse().unwrap();
        let db_name = env::var("MYSQL_DB_NAME").expect("ENV not Found");

        let opts = OptsBuilder::new()
            .user(Some(user))
            .pass(Some(password))
            .ip_or_hostname(Some(ip))
            .tcp_port(port)
            .db_name(Some(db_name));

        let pool = Pool::new(opts).unwrap();
        let conn = pool.get_conn().unwrap();

        DataBase { pool, conn }
    }
}

// pub fn init() -> mysql::Pool {

//   let MYSQL_USER_NAME : String = env::var("MYSQL_USER_NAME").expect("ENV not Found");
//   let MYSQL_PASSWORD : String = env::var("MYSQL_PASSWORD").expect("ENV not Found");
//   let MYSQL_IP : String = env::var("MYSQL_IP").expect("ENV not Found");
//   let MYSQL_PORT : u16 = env::var("MYSQL_PORT").expect("").parse().unwrap();
//   let MYSQL_DB_NAME : String = env::var("MYSQL_DB_NAME").expect("ENV not Found");

//   let opts = OptsBuilder::new()
//               .user(Some(MYSQL_USER_NAME))
//               .pass(Some(MYSQL_PASSWORD))
//               .ip_or_hostname(Some(MYSQL_IP))
//               .tcp_port(MYSQL_PORT)
//               .db_name(Some(MYSQL_DB_NAME));

//   Pool::new(opts).unwrap()

//   // let connection_url = format!("mysql://{}:{}@{}:{}/{}", USER_NAME, PASSWORD, IP, PORT, DB_NAME);

// }
