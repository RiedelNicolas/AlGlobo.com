use super::error::AppResult;
use super::request::Request;
use super::web_service_provider::WebServiceProvider;
use super::web_service_connection::WebServiceConnection;
use std::thread::{self, JoinHandle};

pub struct RequestHandler {
    handle: Option<JoinHandle<()>>
}

impl RequestHandler {
    pub fn spawn(request: Request, provider: &mut WebServiceProvider) -> AppResult<Self> {
        let connection = provider.airline_request(request.get_airline());
        let handler = RequestHandler {
            handle: Some(thread::spawn( move || RequestHandler::process_request(request, connection)))
        };

        Ok(handler)
    }

    fn process_request(request: Request, connection: WebServiceConnection) {
        println!("Trying to connect to extern airline web-service");
        loop {
            if let Ok(_) = connection.resolve_airline_request() {
                break;
            }
            println!("Error trying to resolve airline request. Retrying...");
        }

        if request.is_package(){
            println!("Trying to connect to extern airline web-service");
            connection.resolve_hotel_request();
        }
        
        println!("{:?}", request);
        println!("Finished");
    }

    // Puede ser necesario para poder limpiar hilos finalizados desde el main
    //fn finished(&self) -> bool {
    //    self.finished
    //}
    
    //Quizas un join_if_finished que haga todo junto
    pub fn join(mut self) -> std::thread::Result<()> {
        match self.handle.take() {
            Some(h) => h.join(),
            None => Ok(()),
        }
    }
}
