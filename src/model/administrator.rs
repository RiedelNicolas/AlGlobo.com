use crate::model::logger_message::LoggerMessage;
use crate::model::statistics::LogData;
use super::logger::Logger;
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

/// Clase encarga de manejar los distintos webservice y derivar las solicitudes.
pub struct Administrator {
    pending_requests: HashMap<usize, (Request, u32)>,
    airlines: HashMap<String, Addr<Airline>>,
    hotel: Option<Addr<Hotel>>,
    statistics: Addr<Statistics>,
    keep_going: bool,
    configuration: Configuration,
    logger: Addr<Logger>
}

impl Administrator {
    pub fn new( statistics: Addr<Statistics>, 
                configuration: Configuration,
                logger: Addr<Logger>) -> Administrator {
        Administrator {
            pending_requests: HashMap::new(),
            airlines: HashMap::new(),
            hotel: None,
            statistics,
            keep_going: true,
            configuration,
            logger
        }
    }
    /// Actualiza las estadisticas internas. (Deberia ser privada?)
    pub fn update_statistics(&mut self, req: Request) {

        let info = InfoRequest {
            route: req.get_route(),
            time: *req.get_completion_time()
        };

        self.statistics.do_send(Update(info));
    }
}

impl Actor for Administrator {
    type Context = Context<Self>;


    fn started(&mut self, ctx: &mut Self::Context) {
        self.hotel = Some(Hotel::new(
            ctx.address(), 
            self.configuration,
            self.logger.clone()).start());
    }

}


impl Handler<NewRequest> for Administrator {
    type Result = ();
    /// Handler que maneja la creacion de solicitudes.
    fn handle(&mut self, msg: NewRequest, ctx: &mut Context<Self>) -> Self::Result {

        let request = msg.0;

        let with_hotel = request.with_hotel;
        let stages = 1 + with_hotel as u32;
        let id = request.get_id();
        
        let conf = &self.configuration;
        let log = &self.logger;
        let addr = self.airlines
            .entry(request.airline.clone())
            .or_insert_with(|| Airline::new(&request.airline,
                                                    ctx.address(),
                                                    *conf,
                                                    log.clone()).start());
        
        self.pending_requests.insert(id, (request, stages));

        addr.do_send(AirlineRequest(id));

        if with_hotel {
            if let Some(hotel_address) = &self.hotel {
                hotel_address.do_send(HotelRequest(id));
            }
        }
    }
}

/// Handler para manejar la finalizacion de una solicitud.
impl Handler<FinishedWebServiceRequest> for Administrator {
    type Result = ();

    fn handle(&mut self, msg: FinishedWebServiceRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let id = msg.0;
        if let Some((_, stages_left)) = self.pending_requests.get_mut(&id){
            *stages_left -= 1;
            if *stages_left == 0 {
                if let Some((mut req, _)) = self.pending_requests.remove(&id) {
                    req.finish();
                    
                    self.logger.do_send(LoggerMessage::new_info(
                    format!("[Administrator: {}]: Finalizada request [{}->{}] por {}", 
                            req.get_id(), 
                            req.origin, 
                            req.destiny, 
                            req.airline)));
                    self.update_statistics(req);
                    if !self.keep_going && self.pending_requests.is_empty() {
                        self.statistics.do_send(LogData);
                        System::current().stop();
                    }
                }
            }
        }  
    }
}

/// Handler que maneja el fin de las solicitudes. (Cerrar la comunicacion)
impl Handler<EndOfRequests> for Administrator {
    type Result = ();

    fn handle(&mut self, _msg: EndOfRequests, _ctx: &mut Context<Self>) -> Self::Result {
        self.keep_going = false;  
    }
}