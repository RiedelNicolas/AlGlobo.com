use super::request::Request;
use super::airline::{Airline, AirlineRequest};
use super::statistics::{InfoRequest, Statistics, Update};
use super::hotel::{Hotel, HotelRequest};
use actix::prelude::*;
use std::collections::HashMap;
use super::configuration::Configuration;

/// Mensaje utilizado para iniciar una nueva Solicitud.
#[derive(Message)]
#[rtype(result = "")]
pub struct NewRequest(pub Request);

/// Mensaje utilizado para finalizar una solicitud.
#[derive(Message)]
#[rtype(result = "")]
pub struct FinishedWebServiceRequest(pub usize);

/// Mensaje utilizado para cerrar la comunicacion.
#[derive(Message)]
#[rtype(result = "")]
pub struct EndOfRequests;

pub struct Administrator {
    pending_requests: HashMap<usize, (Request, u32)>,
    airlines: HashMap<String, Addr<Airline>>,
    hotel: Option<Addr<Hotel>>,
    statistics: Addr<Statistics>,
    keep_going: bool,
    configuration: Configuration
}

impl Administrator {
    pub fn new(configuration: Configuration) -> Administrator {
        Administrator {
            pending_requests: HashMap::new(),
            airlines: HashMap::new(),
            hotel: None,
            statistics: Statistics::new(configuration).start(),
            keep_going: true,
            configuration
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


    fn started(&mut self, ctx: &mut Self::Context) {
        self.hotel = Some(Hotel::new(ctx.address(), self.configuration.clone()).start());
    }
}

impl Handler<NewRequest> for Administrator {
    type Result = ();

    fn handle(&mut self, msg: NewRequest, ctx: &mut Context<Self>) -> Self::Result {

        let request = msg.0;

        let with_hotel = request.with_hotel;
        let stages = 1 + with_hotel as u32;
        let id = request.get_id();
        
        let conf = &self.configuration;
        let addr = self.airlines
            .entry(request.airline.clone())
            .or_insert_with(|| Airline::new(&request.airline,
                                                    ctx.address(),
                                                    conf.clone()).start());
        
        self.pending_requests.insert(id, (request, stages));

        if addr.try_send(AirlineRequest(id)).is_err(){
            print!("Failing to send [{}] request to airline web service", id);
        }

        if with_hotel {
            if let Some(hotel_address) = &self.hotel {
                if hotel_address.try_send(HotelRequest(id)).is_err(){
                    print!("Failing to send [{}] request to hotel web service", id);
                }
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
                    self.update_statistics(req);
                    if !self.keep_going && self.pending_requests.is_empty() {
                        System::current().stop();
                    }
                }
            }
        }  
    }
}

impl Handler<EndOfRequests> for Administrator {
    type Result = ();

    fn handle(&mut self, msg: EndOfRequests, _ctx: &mut Context<Self>) -> Self::Result {
        self.keep_going = false;  
    }
}