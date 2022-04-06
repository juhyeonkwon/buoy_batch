//test
#[cfg(test)]
mod tests {
    use crate::db;

    use dotenv::dotenv;
    use mysql::prelude::*;
    use mysql::*;
    use serde::Serialize;
    extern crate redis;
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
}
