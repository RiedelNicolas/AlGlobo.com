use super::env::Configuration;
use super::error::AppResult;
use super::request::Request;
use super::web_service_provider::WebServiceProvider;
use super::web_service_connection::WebServiceConnection;
use super::statistics::InfoRequest;
use std::thread::{self, JoinHandle};
use std::time::{self, Duration};
use std::sync::{Arc, RwLock};
use super::logger::Logger;

pub struct RequestHandler {
    request: Arc<RwLock<Request>>,
    airline: Option<JoinHandle<()>>,
    hotel: Option<JoinHandle<()>>,
    logger : Logger
}

impl RequestHandler {
    pub fn spawn(req: Request, provider: &mut WebServiceProvider, _envs: &Configuration,
                                                                        in_logger : Logger) -> AppResult<Self> {
        let connection = provider.airline_request(req.get_airline());
        let is_package = req.is_package();
        let protected_request_local = Arc::new(RwLock::new(req));
        let protected_request_airline = protected_request_local.clone();
        let protected_request_hotel = protected_request_local.clone();
        let logger_clone = in_logger.clone();
        let handler = RequestHandler {
            request: protected_request_local,
            airline: Some(thread::spawn( move || RequestHandler::process_request(protected_request_airline, connection, logger_clone ))),
            hotel: match is_package {
                true => {
                    let connection = provider.hotel_request();
                    let aux = in_logger.clone();
                    Some(thread::spawn( move || RequestHandler::process_request(protected_request_hotel, connection,  aux)))
                },
                false => None
            },
            logger : in_logger.clone()
        };

        Ok(handler)
    }

    fn process_request(request: Arc<RwLock<Request>>, connection: WebServiceConnection, logger : Logger) {
        logger.log_info(String::from("Trying to connect to extern web-service"));
        loop {
            if connection.resolve_request().is_ok() {
                break;
            }
            logger.log_warning(String::from("Error trying to resolve request. Retrying in a moment..."));
            
            thread::sleep(time::Duration::from_millis(1000));   //Deberia ser cargado desde un ENV
        }
        match request.write() {
            Ok(mut req) => {
                req.finish();
            },
            Err(_) => {
                logger.log_error(String::from("Fatal error: Poisoned Lock"));
                //println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
            }
        }
    }

    pub fn has_finished(&self) -> bool {
        match self.request.read() {
            Ok(req) => req.has_finished(),
            Err(_) => {
                self.logger.log_error(String::from("Fatal error: Poisoned Lock") );
                //println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
                false
            }
        }
    }
    
    pub fn join(self) -> InfoRequest { // VER SI PUEDE FALLAR JOIN QUE HACER
        if let Some(airline) = self.airline { let _ = airline.join(); }
        if let Some(hotel) = self.hotel { let _ = hotel.join(); }
        match self.request.write() {
            Ok(req) => {
                InfoRequest::new(req.get_route(), *req.get_completion_time())
            },
            Err(_) => {
                self.logger.log_error(String::from("Fatal error: Poisoned Lock"));
                println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
                InfoRequest::new(String::new(), Duration::from_secs(0)) // CAMBIAR ESTO
            }
        }
    }
}
