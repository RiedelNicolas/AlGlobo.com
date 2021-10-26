use std::collections::HashMap;
use std::time::Duration;
use actix::prelude::*;
use super::configuration::Configuration;
use super::logger::Logger;
use crate::model::logger_message::LoggerMessage;

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

pub struct Statistics {
    routes_requested: HashMap<String, u32>,
    total_time: Duration,
    requests_amount: usize,
    log_rate: usize,
    top_req_amount: usize,
    logger: Addr<Logger>
}

impl Statistics {
    pub fn new( configuration: Configuration,
                logger: Addr<Logger>) -> Self { 
        Self { 
            routes_requested: HashMap::new(),
            total_time: Duration::from_secs(0),
            requests_amount: 0,
            log_rate: configuration.stats_log_rate,
            top_req_amount: configuration.stats_top_req_amount,
            logger
        }
    }

    /// Imprime las estadisticas generadas en el log file
    pub fn log_data(&self) {
        let mut top_requested: Vec<(&String, &u32)> = Vec::new();
        for (key,value) in self.routes_requested.iter() {
            top_requested.push((key, value));
        }
        top_requested.sort_by_key(|k| k.1);
        
        let mut top_req_str: String = String::new();
        let index = if top_requested.len() > self.top_req_amount { self.top_req_amount } else { top_requested.len() };
        
        for (i, item) in top_requested.iter().enumerate().take(index) {
            top_req_str.push_str(item.0);
            if i != index-1 { top_req_str.push_str(" // "); };
        }

        self.logger.do_send(LoggerMessage::new_info(format!("[Statistics] Requests Amount: {}", self.requests_amount )));
        self.logger.do_send(LoggerMessage::new_info(format!("[Statistics] Total Request Time: {}s", self.total_time.as_secs() )));
        self.logger.do_send(LoggerMessage::new_info(format!("[Statistics] Average Request Time: {}s", (self.total_time.as_secs() / self.requests_amount as u64) )));
        self.logger.do_send(LoggerMessage::new_info(format!("[Statistics] Top 10 Requested Routes: {}", top_req_str )));
    }
}

impl Actor for Statistics {
    type Context = Context<Self>;
}

impl Handler<Update> for Statistics {
    type Result = ();

    fn handle(&mut self, msg: Update, _ctx: &mut Context<Self>) -> Self::Result {
        let req = msg.0;
        let route = req.route;
        let time = req.time;

        self.requests_amount += 1;
        self.total_time += time;
        
        *self.routes_requested.entry(route).or_insert(0) += 1;

        if self.requests_amount % self.log_rate == 0 {
            self.log_data();
        }
    }
}


impl Handler<LogData> for Statistics {
    type Result = ();

    fn handle(&mut self, _msg: LogData, _ctx: &mut Context<Self>) -> Self::Result {
        self.log_data();
    }
}
