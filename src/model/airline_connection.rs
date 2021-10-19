use actix::prelude::*;
use super::airline::{Airline, ConnectionFinished};
use actix::clock::sleep;
use std::time::Duration;
use std::collections::VecDeque;

#[derive(Message)]
#[rtype(result = "")]
pub struct ProcessRequest;


#[derive(Message)]
#[rtype(result = "")]
pub struct Request(pub usize);

pub struct AirlineConnection {
    pending_requests: VecDeque<usize>,
    airline: Addr<Airline>
}

impl AirlineConnection {
    
    pub fn new(airline: Addr<Airline>) -> AirlineConnection {
        AirlineConnection {
            pending_requests: VecDeque::new(),
            airline
        }
    }
}

impl Actor for AirlineConnection {
    type Context = Context<Self>;
}

impl Handler<Request> for AirlineConnection {
    type Result = ();

    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Self::Result {
        if self.pending_requests.is_empty() && 
            ctx.address().try_send(ProcessRequest).is_err(){
            
            println!("Failing to process request");
        }
        self.pending_requests.push_back(msg.0);
    }
}


impl Handler<ProcessRequest> for AirlineConnection {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: ProcessRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let id = self.pending_requests.pop_front().unwrap();
        //thread_rng().gen_range(5000..5000)
        Box::pin(sleep(Duration::from_millis(5000))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                println!("Request [{}]: Request a la aerolinea resuelta", id);
                
                if me.airline.try_send(ConnectionFinished(id)).is_err(){
                    println!("Failing to send [{}] completed request to airline", id);
                }
                if !me.pending_requests.is_empty() &&
                    ctx.address().try_send(ProcessRequest).is_err() {
                    println!("Failing to process request");
                }
            }))
    }
}