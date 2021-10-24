use std::collections::HashMap;
use std::time::Duration;
use actix::prelude::*;

#[derive(Debug)]
pub struct InfoRequest {
    pub route: String,
    pub time: Duration
}

#[derive(Message)]
#[rtype(result = "")]
pub struct Update(pub InfoRequest);

#[derive(Message)]
#[rtype(result = "")]
pub struct LogData;

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

    fn log(&self) {
        println!("Estadisticas:  
        - Tiempo medio de resolucion: {}
        - Tiempo total: {},
        - Total requests: {}", 
        self.total_time.as_millis() as f64 / self.requests_amount as f64,
        self.total_time.as_millis() as f64,
        self.requests_amount); //CAMBIAR
    }
}

impl Actor for Statistics {
    type Context = Context<Self>;
}

impl Handler<Update> for Statistics {
    type Result = ();

    fn handle(&mut self, msg: Update, ctx: &mut Context<Self>) -> Self::Result {
        let req = msg.0;
        let route = req.route;
        let time = req.time;

        self.requests_amount += 1;
        self.total_time += time;
        
        *self.routes_requested.entry(route).or_insert(0) += 1;

        // TODO: Numeros magicos

        if self.requests_amount % 2 == 0 {
            self.log();
        }

    }
}
