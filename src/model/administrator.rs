use super::request::Request;
use super::airline::{Airline, AirlineRequest};
use actix::prelude::*;
use std::collections::HashMap;

#[derive(Message)]
#[rtype(result = "")]
pub struct NewRequest(pub Request);

#[derive(Message)]
#[rtype(result = "")]
pub struct FinishedWebServiceRequest(pub usize);

pub struct Administrator {
    pending_requests: HashMap<usize, (Request, u32)>,
    airlines: HashMap<String, Addr<Airline>>
}

impl Administrator {
    pub fn new() -> Administrator {
        Administrator {
            pending_requests: HashMap::new(),
            airlines: HashMap::new()
        }
    }
}

impl Actor for Administrator {
    type Context = Context<Self>;
}

impl Handler<NewRequest> for Administrator {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, ctx: &mut Context<Self>) -> Self::Result {

        let request = msg.0;

        let stages = 1 + request.with_hotel as u32;
        let id = request.get_id();
        
        let addr = self.airlines
            .entry(request.airline.clone())
            .or_insert_with(|| Airline::new(&request.airline, ctx.address()).start());
        
        self.pending_requests.insert(id, (request, stages));

        if addr.try_send(AirlineRequest(id)).is_err(){
            print!("Failing to send [{}] request to airline web service", id);
        }
    }
}

impl Handler<FinishedWebServiceRequest> for Administrator {
    type Result = ();

    fn handle(&mut self, msg: FinishedWebServiceRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let id = msg.0;
        if let Some((_, stages_left)) = self.pending_requests.get_mut(&id){
            *stages_left -= 1;
            if *stages_left == 0 {
                if let Some((req, _)) = self.pending_requests.remove(&id){
                    println!("Request [{}]: Finalizada request [{}->{}] por {}", req.get_id(), req.origin, req.destiny, req.airline);
                    println!("El hash de requests queda con {} conexiones", self.pending_requests.len());
                }
            }
        }  
    }
}