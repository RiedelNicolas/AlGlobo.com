use actix::prelude::*;
use crate::model::airline_connection::{AirlineConnection, Request};
use super::administrator::{Administrator, FinishedWebServiceRequest};

#[derive(Message)]
#[rtype(result = "")]
pub struct AirlineRequest(pub usize);


#[derive(Message)]
#[rtype(result = "")]
pub struct WaitAndProcessRequest;

#[derive(Message)]
#[rtype(result = "")]
pub struct ConnectionFinished(pub usize);

pub struct Airline {
    name: String,
    connections: Vec<Addr<AirlineConnection>>,
    next_connection: usize,
    max_concurrent_connections: usize,
    admin: Addr<Administrator>
}

impl Airline {
    
    pub fn new(name: &str, admin: Addr<Administrator>) -> Airline {

        Airline {
            name: name.to_string(),
            connections: Vec::new(),
            next_connection: 0,
            max_concurrent_connections: 2,
            admin
        }
    }
    
    pub fn get_next_connection(&mut self, airline_address: Addr<Airline>) -> Addr<AirlineConnection> {
        
        let connection = match self.connections.get_mut(self.next_connection){
            Some(conn) => conn.clone(),
            None => {
                let conn = AirlineConnection::new(airline_address).start();
                self.connections.push(conn.clone());
                conn
            }
        };

        if self.next_connection+1 >= self.max_concurrent_connections {
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

        println!("Request [{}]: Conectando con la aerolinea {}", self.name, req_id);

        let addr = self.get_next_connection(ctx.address());

        if addr.try_send(Request(req_id)).is_err(){
            println!("Failing to process request");
        }
    }
}


impl Handler<ConnectionFinished> for Airline {
    type Result = ();

    fn handle(&mut self, msg: ConnectionFinished, _ctx: &mut Context<Self>) -> Self::Result {
        if self.admin.try_send(FinishedWebServiceRequest(msg.0)).is_err(){
            println!("Failing to process request");
        }
    }
}