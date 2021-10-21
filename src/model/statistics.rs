use std::collections::HashMap;
use std::time::Duration;
use actix::prelude::*;

#[derive(Debug)]
pub struct InfoRequest {
    route: String,
    time: Duration
}

#[derive(Message)]
#[rtype(result = "")]
pub struct Update(pub InfoRequest);

#[derive(Message)]
#[rtype(result = "")]
pub struct LogData;

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
}

impl Actor for Statistics {
    type Context = Context<Self>;
}

impl Handler<Update> for Statistics {
    type Result = ();

    fn handle(&mut self, msg: Update, ctx: &mut Context<Self>) -> Self::Result {
        let req = msg.0;
        let route = req.route();
        let time = *req.time();

        self.requests_amount += 1;
        self.total_time += time;
        
        *self.routes_requested.entry(route).or_insert(0) += 1;
    }
}

impl Handler<LogData> for Statistics {
    type Result = ();

    fn handle(&mut self, msg: LogData, ctx: &mut Context<Self>) -> Self::Result {
        println!("Tiempo: {}", self.total_time.as_secs() / self.requests_amount); //CAMBIAR
    }
}
