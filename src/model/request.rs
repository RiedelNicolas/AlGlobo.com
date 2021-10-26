use std::time::{Duration, Instant};
use uid::Id;

#[derive(Debug)]
pub struct Request {
    pub id: Id<usize>,
    pub origin: String,
    pub destiny: String,
    pub airline: String,
    pub with_hotel: bool,
    pub arrival_time: Instant,
    pub completion_time: Duration,
}

impl Request {
    pub fn new(origin: &str, destiny: &str, airline: &str, with_hotel: bool) -> Self {
        Request {
            id: Id::new(),
            origin: origin.to_string(),
            destiny: destiny.to_string(),
            airline: airline.to_string(),
            with_hotel,
            arrival_time: Instant::now(),
            completion_time: Duration::from_secs(0),
        }
    }

    pub fn get_id(&self) -> usize {
        self.id.get()
    }

    pub fn finish(&mut self) {
        self.completion_time = self.arrival_time.elapsed();
    }

    pub fn get_completion_time(&self) -> &Duration {
        &self.completion_time
    }

    pub fn get_route(&self) -> String {
        format!("{}->{}", self.origin, self.destiny)
    }
}
