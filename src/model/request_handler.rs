use super::error::AppResult;
use super::request::Request;
use std::thread::{self, JoinHandle};
use std::time::Duration;

#[derive(Debug)]
pub struct RequestHandler {
    handle: Option<JoinHandle<()>>
    //finished: bool
}

impl RequestHandler {
    pub fn spawn(request: Request) -> AppResult<Self> {

        let handler = RequestHandler {
            handle: Some(thread::spawn( move || RequestHandler::process_request(request))),
            //finished: false
        };

        Ok(handler)
    }

    fn process_request(request: Request) {
        println!("Trying to connect to extern web-service");
        std::thread::sleep(Duration::from_secs(3));
        println!("Connection established");
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
