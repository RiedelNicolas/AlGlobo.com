use std::collections::HashMap;
use std::f32::consts::LOG10_2;
use std::time::Duration;
use super::logger::Logger;

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
  requests_amount: u64,
  logger : Logger
}

impl Statistics {
    pub fn new( in_logger : Logger) -> Self { 
        Self { 
            routes_requested: HashMap::new(),
            total_time: Duration::from_secs(0),
            requests_amount: 0,
            logger : in_logger.clone()
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
        let mut top_requested: Vec<(&String, &u32)> = Vec::new();
        for (key,value) in self.routes_requested.iter() {
            top_requested.push((key, value));
        }
        top_requested.sort_by_key(|k| k.1);
        
        let mut top_req_str: String = String::new();
        let index = if top_requested.len() > 5 { 5 } else { top_requested.len() };
        for i in 0..index {
            top_req_str.push_str(top_requested[i].0);
            if i != index-1 { top_req_str.push_str(" // "); };
        }

        self.logger.log_info(format!("Average Request Time: {}", (self.total_time.as_secs() / self.requests_amount) ));
        self.logger.log_info(format!("Top Requested Routes: {}", top_req_str ));
    }
}