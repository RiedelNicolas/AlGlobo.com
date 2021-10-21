use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Configuration {
    pub airline_limit: u32,
    pub air_failure_probability: f32,
    pub air_min_work_time: u32,
    pub air_max_work_time: u32,
    pub hotel_limit: u32,
    pub hotel_failure_probability: f32,
    pub hotel_min_work_time: u32,
    pub hotel_max_work_time: u32,
    pub sleeping_retry_time: u64
}

impl Configuration {
    fn new() -> Self { 
        Self { 
            airline_limit: 10, 
            air_failure_probability: 0.1, 
            air_min_work_time: 1000,
            air_max_work_time: 4000, 
            hotel_limit: 10, 
            hotel_failure_probability: 0.1, 
            hotel_min_work_time: 1000, 
            hotel_max_work_time: 4000, 
            sleeping_retry_time: 1000 
        } 
    }
}

pub fn get_envs<P: AsRef<Path>>(path: P) -> Configuration {
    let file = File::open(path).unwrap();
    let reader = BufReader::new(file);

    let config: Configuration = match serde_json::from_reader(reader) {
        Ok(r) => r,
        Err(_) => Configuration::new()
    };

    config
}
