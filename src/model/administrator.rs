use super::request::Request;
use super::airline::{Airline, AirlineRequest};
use super::statistics::{InfoRequest, Statistics, Update};
use super::hotel::{Hotel, HotelRequest};
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
    airlines: HashMap<String, Addr<Airline>>,
    hotel: Option<Addr<Hotel>>,
    statistics: Addr<Statistics>
}

impl Administrator {
    pub fn new() -> Administrator {
        Administrator {
            pending_requests: HashMap::new(),
            airlines: HashMap::new(),
            hotel: None,
            statistics: Statistics::new().start()
        }
    }
    


    pub fn update_statistics(&mut self, req: Request) {

        let info = InfoRequest {
            route: req.get_route(),
            time: *req.get_completion_time()
        };

        let _ = self.statistics.try_send(Update(info));
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

        match &self.hotel {
            Some(hotel_address) => {
                if hotel_address.try_send(HotelRequest(id)).is_err(){
                    print!("Failing to send [{}] request to hotel web service", id);
                }
            },
            None => {
                let hotel = Hotel::new(ctx.address()).start();
                if hotel.try_send(HotelRequest(id)).is_err(){
                    print!("Failing to send [{}] request to hotel web service", id);
                }
                self.hotel = Some(hotel);
            }
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
                if let Some((mut req, _)) = self.pending_requests.remove(&id) {
                    req.finish();
                    println!("Request [{}]: Finalizada request [{}->{}] por {}", req.get_id(), req.origin, req.destiny, req.airline);
                    self.update_statistics(req)
                }
            }
        }  
    }
}