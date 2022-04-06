extern crate dotenv;
extern crate redis;
use std::env;

pub fn connect_redis() -> redis::Connection {
    let redis = env::var("REDIS").expect("ENV not Found");

    redis::Client::open(redis)
        .expect("error in open Redis.")
        .get_connection()
        .expect("faild to connect to Redis.")

    // let mut con = client.get_connection();

    // let a : String = redis::cmd("GET").arg("welding_tbar_2021-12-21").query(&mut con).expect("error!");

    // let items: Vec<String> = con.lrange("N-225_2020-06-01", 0, -1).expect("error!");

    // println!("{:?}", items);

    // println!("{}", a);

    // let str2 = con.get("welding_tbar_2021-12-21")?;

    // println!("{:?}", str2);

    // con
}
