use super::maria::Insertbuoy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize, Serialize)]
pub struct BuoyAvg {
    pub model: String,
    pub avg_w_temp: f32,
    pub avg_salinity: f32,
    pub avg_height: f32,
    pub avg_weight: f32,
}

pub fn avg_buoy_processing(row: Vec<Insertbuoy>) -> Vec<BuoyAvg> {
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

    let mut vector: Vec<BuoyAvg> = Vec::new();

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
            count += 1.0;
        }

        let avg_w_temp: f32 = sum_w_temp / count;
        let avg_salinity: f32 = sum_salinity / count;
        let avg_height: f32 = sum_height / count;
        let avg_weight: f32 = sum_weight / count;

        vector.push(BuoyAvg {
            model: String::from(&_data.1[0].buoy.model),
            avg_w_temp,
            avg_salinity,
            avg_height,
            avg_weight,
        });
    }

    vector
}
