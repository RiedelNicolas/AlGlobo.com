use actix::prelude::*;
use crate::model::hotel_connection::{HotelConnection, Request};
use super::administrator::{Administrator, FinishedWebServiceRequest};
use super::configuration::Configuration;
use std::ops::Range;

#[derive(Message)]
#[rtype(result = "")]
pub struct HotelRequest(pub usize);

#[derive(Message)]
#[rtype(result = "")]
pub struct WaitAndProcessRequest;

#[derive(Message)]
#[rtype(result = "")]
pub struct ConnectionFinished(pub usize);

pub struct Hotel {
    connections: Vec<Addr<HotelConnection>>,
    next_connection: usize,
    max_concurrent_connections: usize,
    admin: Addr<Administrator>,
    configuration: Configuration
}

impl Hotel {
    
    pub fn new( admin: Addr<Administrator>,
                configuration: Configuration) -> Hotel {

        Hotel {
            connections: Vec::new(),
            next_connection: 0,
            max_concurrent_connections: configuration.hotel_limit,
            admin,
            configuration
        }
    }
    
    pub fn get_next_connection(&mut self, hotel_address: Addr<Hotel>) -> Addr<HotelConnection> {
        
        let connection = match self.connections.get_mut(self.next_connection){
            Some(conn) => conn.clone(),
            None => {
                let conn = HotelConnection::new(
                    hotel_address,
                    Range {
                        start: self.configuration.hotel_min_work_time, 
                        end: self.configuration.hotel_max_work_time 
                    }
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

impl Actor for Hotel {
    type Context = Context<Self>;
}

impl Handler<HotelRequest> for Hotel {
    type Result = ();

    fn handle(&mut self, msg: HotelRequest, ctx: &mut Context<Self>) -> Self::Result {
        let req_id = msg.0;
        let addr = self.get_next_connection(ctx.address());

        if addr.try_send(Request(req_id)).is_err(){
            println!("Failing to process request");
        }
    }
}

impl Handler<ConnectionFinished> for Hotel {
    type Result = ();

    fn handle(&mut self, msg: ConnectionFinished, _ctx: &mut Context<Self>) -> Self::Result {
        if self.admin.try_send(FinishedWebServiceRequest(msg.0)).is_err(){
            println!("Failing to process request");
        }
    }
}

