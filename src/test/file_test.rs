#[cfg(test)]
mod tests {
    // use crate::db::maria_lib::Buoy;

    // use serde::{Deserialize, Serialize};

    use std::error::Error;
    use std::fs::File;
    // use std::io;

    extern crate csv;

    #[test]
    fn create_file() -> Result<(), Box<dyn Error>> {
        File::create("./csv/test.csv").expect("Error occured");

        let file = File::options()
            .write(true)
            .open("./csv/test.csv")
            .expect("err");

        let mut wtr = csv::Writer::from_writer(file);

        // We still need to write headers manually.
        wtr.write_record(&["City", "State", "Population", "Latitude", "Longitude"])?;

        // But now we can write records by providing a normal Rust value.
        //
        // Note that the odd `None::<u64>` syntax is required because `None` on
        // its own doesn't have a concrete type, but Serde needs a concrete type
        // in order to serialize it. That is, `None` has type `Option<T>` but
        // `None::<u64>` has type `Option<u64>`.
        wtr.serialize((
            "Davidsons Landing",
            "AK",
            None::<u64>,
            65.2419444,
            -165.2716667,
        ))?;
        wtr.serialize(("Kenai", "AK", Some(7610), 60.5544444, -151.2583333))?;
        wtr.serialize(("Oakman", "AL", None::<u64>, 33.7133333, -87.3886111))?;

        wtr.flush()?;

        Ok(())
    }
}
