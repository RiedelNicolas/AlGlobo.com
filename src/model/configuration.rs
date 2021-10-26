use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use serde::{Deserialize, Serialize};
use crate::model::logger_message::LoggerMessage;
use super::logger::Logger;
use actix::prelude::Addr;

#[derive(Serialize, Deserialize, Clone, Copy)]
pub struct Configuration {
    pub airline_limit: usize,
    pub air_failure_probability: f32,
    pub air_min_work_time: usize,
    pub air_max_work_time: usize,
    pub hotel_limit: usize,
    pub hotel_min_work_time: usize,
    pub hotel_max_work_time: usize,
    pub sleeping_retry_time: usize,
    pub stats_log_rate: usize,
    pub stats_top_req_amount: usize
}

/// Clase utilizada para la configuracion de variables de entorno.
impl Configuration {
    fn new() -> Self {
        Self {
            airline_limit: 10, 
            air_failure_probability: 0.1, 
            air_min_work_time: 1000,
            air_max_work_time: 4000, 
            hotel_limit: 10, 
            hotel_min_work_time: 1000, 
            hotel_max_work_time: 4000, 
            sleeping_retry_time: 1000,
            stats_log_rate: 5,
            stats_top_req_amount: 10
        }
    }
}

pub fn get_envs<P: AsRef<Path>>(path: P, logger: Addr<Logger>) -> Configuration {

    let file = match File::open(path) {
        Ok(r) => r,
        Err(_) => {
            logger.do_send(LoggerMessage::new_warning(
                "[Configuration]: Could not open specified configurations file".to_string()));
            return Configuration::new()
        }
    };

    let reader = BufReader::new(file);

    let config: Configuration = match serde_json::from_reader(reader) {
        Ok(r) => r,
        Err(_) => Configuration::new()
    };

    config
}
