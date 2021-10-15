use super::env::Configuration;
use super::error::AppResult;
use super::request::Request;
use super::web_service_provider::WebServiceProvider;
use super::web_service_connection::WebServiceConnection;
use super::statistics::InfoRequest;
use std::thread::{self, JoinHandle};
use std::time::{self, Duration};
use std::sync::{Arc, RwLock};

pub struct RequestHandler {
    request: Arc<RwLock<Request>>,
    airline: Option<JoinHandle<()>>,
    hotel: Option<JoinHandle<()>>
}

impl RequestHandler {
    pub fn spawn(req: Request, provider: &mut WebServiceProvider, envs: &Configuration) -> AppResult<Self> {
        let connection = provider.airline_request(req.get_airline());
        let is_package = req.is_package();
        let protected_request_local = Arc::new(RwLock::new(req));
        let protected_request_airline = protected_request_local.clone();
        let protected_request_hotel = protected_request_local.clone();

        let handler = RequestHandler {
            request: protected_request_local,
            airline: Some(thread::spawn( move || RequestHandler::process_request(protected_request_airline, connection))),
            hotel: match is_package {
                true => {
                    let connection = provider.hotel_request();
                    Some(thread::spawn( move || RequestHandler::process_request(protected_request_hotel, connection)))
                },
                false => None
            }
        };

        Ok(handler)
    }

    fn process_request(request: Arc<RwLock<Request>>, connection: WebServiceConnection) {
        println!("Trying to connect to extern web-service");
        loop {
            if let Ok(_) = connection.resolve_request() {
                break;
            }
            println!("Error trying to resolve request. Retrying in a moment...");
            thread::sleep(time::Duration::from_millis(1000));   //Deberia ser cargado desde un ENV
        }
        match request.write() {
            Ok(mut req) => {
                req.finish();
            },
            Err(_) => {
                println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
            }
        }
    }

    pub fn has_finished(&self) -> bool {
        match self.request.read() {
            Ok(req) => req.has_finished(),
            Err(_) => {
                println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
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
                println!("Fatal error: Poisoned Lock"); //No me gusta mucho esto
                InfoRequest::new(String::new(), Duration::from_secs(0)) // CAMBIAR ESTO
            }
        }
    }
}
