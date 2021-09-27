use super::error::{AppResult, InternalError};
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct Request {
    origin: String,
    destiny: String,
    airline: String,
    with_hotel: bool,
    arrival_time: Instant,
    completion_time: Duration
}

impl Request {
    pub fn new(origin: &str, destiny: &str, airline: &str, with_hotel: bool) -> AppResult<Self> {
        let request = Request {
            origin: origin.to_string(),
            destiny: destiny.to_string(),
            airline: airline.to_string(),
            with_hotel,
            arrival_time: Instant::now(),
            completion_time: Duration::from_secs(0)
        };

        Ok(request)
    }

    pub fn get_airline(&self) -> &str {
        &self.airline[..]
    }

    pub fn is_package(&self) -> bool {
        self.with_hotel
    }

    pub fn finish(&mut self) {
        self.completion_time = self.arrival_time.elapsed();
    }

    pub fn get_completion_time(&self) -> &Duration {
        &self.completion_time
    }
}
