use actix::prelude::*;
use super::airline::{ 
    Airline, 
    ConnectionFinished,
    ConnectionFailed
};
use actix::clock::sleep;
use std::time::Duration;
use rand::Rng;
use std::collections::VecDeque; //TODO: Modificar por una cola normal
use std::ops::Range;

/// Mensaje que indica que se debe procesar una solicitud
#[derive(Message)]
#[rtype(result = "")]
pub struct ProcessRequest;

/// Mensaje que representa un request al webservice simulado.
#[derive(Message)]
#[rtype(result = "")]
pub struct Request(pub usize);

/// Mensaje que representa un request al webservice simulado.
pub struct AirlineConnection {
    pending_requests: VecDeque<usize>,
    airline_name: String,
    airline: Addr<Airline>,
    work_time_range: Range<usize>,
    failure_probability: f32,
}

/// 
impl AirlineConnection {
    
    pub fn new( airline_name: String,
                airline: Addr<Airline>, 
                work_time_range: Range<usize>, 
                failure_probability: f32) -> AirlineConnection {
        AirlineConnection {
            pending_requests: VecDeque::new(),
            airline_name,
            airline,
            work_time_range,
            failure_probability
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

        //TODO: Eliminar este unwrap
        let id = self.pending_requests.pop_front().unwrap();
        
        println!("Request [{}]: Conectando con la aerolinea {}", id, self.airline_name);

        let rand_numb = rand::thread_rng().gen_range(self.work_time_range.clone());
        Box::pin(sleep(Duration::from_millis(rand_numb as u64))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                
                let num_random = rand::thread_rng().gen::<f32>();
                let request_solved =  num_random >= me.failure_probability;
                
                if request_solved {
                    println!("Request [{}]: Request a la aerolinea resuelta", id);
                    if me.airline.try_send(ConnectionFinished(id)).is_err(){
                        println!("Failing to send [{}] completed request to airline", id);
                    }
                } else {
                    println!("Request [{}]: Fallo al resolver la request", id);
                    if me.airline.try_send(ConnectionFailed(id)).is_err(){
                        println!("Failing to send [{}] completed request to airline", id);
                    }
                }

                if !me.pending_requests.is_empty() &&
                        ctx.address().try_send(ProcessRequest).is_err() {
                        println!("Failing to process request");
                    }
            }))
    }
}

