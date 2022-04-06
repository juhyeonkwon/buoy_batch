mod data;
mod db;
mod test;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    data::redis::get_data();
}
