use actix::prelude::*;
use crate::model::airline_connection::{AirlineConnection, Request};
use super::administrator::{Administrator, FinishedWebServiceRequest};
use super::configuration::{Configuration};
use actix::clock::sleep;
use std::time::Duration;
use std::ops::Range;
use super::logger::Logger;

#[derive(Message)]
#[rtype(result = "")]
pub struct AirlineRequest(pub usize);


#[derive(Message)]
#[rtype(result = "")]
pub struct WaitAndProcessRequest;

#[derive(Message)]
#[rtype(result = "")]
pub struct ConnectionFinished(pub usize);

#[derive(Message)]
#[rtype(result = "")]
pub struct ConnectionFailed(pub usize);

pub struct Airline {
    name: String,
    connections: Vec<Addr<AirlineConnection>>,
    next_connection: usize,
    max_concurrent_connections: usize,
    sleeping_retry_time: usize,
    admin: Addr<Administrator>,
    configuration: Configuration,
    logger: Addr<Logger>
}

impl Airline {
    
    pub fn new( name: &str, 
                admin: Addr<Administrator>,
                configuration: Configuration,
                logger: Addr<Logger>) -> Airline {

        Airline {
            name: name.to_string(),
            connections: Vec::new(),
            next_connection: 0,
            max_concurrent_connections: configuration.airline_limit,
            sleeping_retry_time: configuration.sleeping_retry_time,
            admin,
            configuration,
            logger
        }
    }
    
    pub fn get_next_connection(&mut self, airline_address: Addr<Airline>) -> Addr<AirlineConnection> {
        
        let connection = match self.connections.get_mut(self.next_connection){
            Some(conn) => conn.clone(),
            None => {
                let conn = AirlineConnection::new(
                    self.name.clone(),
                    airline_address,
                    Range {
                        start: self.configuration.air_min_work_time, 
                        end: self.configuration.air_max_work_time 
                    },
                    self.configuration.air_failure_probability,
                    self.logger.clone()
                ).start();
                self.connections.push(conn.clone());
                conn
            }
        };

        if self.next_connection + 1 >= self.max_concurrent_connections {
            self.next_connection = 0;
        } else {
            self.next_connection += 1;
        }

        connection
    }

}

impl Actor for Airline {
    type Context = Context<Self>;
}

impl Handler<AirlineRequest> for Airline {
    type Result = ();

    fn handle(&mut self, msg: AirlineRequest, ctx: &mut Context<Self>) -> Self::Result {
        let req_id = msg.0;
        let addr = self.get_next_connection(ctx.address());

        addr.do_send(Request(req_id));
    }
}


impl Handler<ConnectionFinished> for Airline {
    type Result = ();

    fn handle(&mut self, msg: ConnectionFinished, _ctx: &mut Context<Self>) -> Self::Result {
        self.admin.do_send(FinishedWebServiceRequest(msg.0));
    }
}

impl Handler<ConnectionFailed> for Airline {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: ConnectionFailed, _ctx: &mut Context<Self>) -> Self::Result {
        Box::pin(sleep(Duration::from_millis(self.sleeping_retry_time as u64))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                let addr = me.get_next_connection(ctx.address());
                let id = msg.0;
                addr.do_send(Request(id));
            }))
    }
}