use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug)]
pub struct InfoRequest {
  route: String,
  time: Duration
}

impl InfoRequest {
    pub fn new(route: String, time: Duration) -> Self { Self { route, time } }

    pub fn route(&self) -> String {
        String::from(&self.route)
    }

    pub fn time(&self) -> &Duration {
        &self.time
    }
}

#[derive(Debug)]
pub struct Statistics {
  routes_requested: HashMap<String, u32>,
  total_time: Duration,
  requests_amount: u64
}

impl Statistics {
    pub fn new() -> Self { 
        Self { 
            routes_requested: HashMap::new(),
            total_time: Duration::from_secs(0),
            requests_amount: 0
        }
    }

    pub fn update(&mut self, req: InfoRequest) { 
        let route = req.route();
        let time = *req.time();

        self.requests_amount += 1;
        self.total_time += time;
        
        *self.routes_requested.entry(route).or_insert(0) += 1;
    }

    pub fn log_data(&self) {
        println!("Tiempo: {}", self.total_time.as_secs() / self.requests_amount);
    }
}