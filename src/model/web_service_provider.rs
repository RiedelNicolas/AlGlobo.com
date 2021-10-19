use std_semaphore::Semaphore;
use std::sync::Arc;
use std::collections::HashMap;
use super::web_service_connection::WebServiceConnection;

pub struct WebServiceProvider {
    airlines: HashMap<String, Arc<Semaphore>>,
    airline_limit: u32,
    hotels: Arc<Semaphore>
}

impl WebServiceProvider {
    pub fn new(airline_limit: u32, hotel_limit: u32) -> Self {

        WebServiceProvider {
            airlines: HashMap::new(),
            airline_limit,
            hotels: Arc::new(Semaphore::new(hotel_limit as isize))
        }
    }

    pub fn airline_request(&mut self, airline_name: &str) -> WebServiceConnection {

        let semaphore = match self.airlines.get(airline_name) {
            Some(sema) => sema.clone(),
            None => {
                let sema = Arc::new(Semaphore::new(self.airline_limit as isize));
                self.airlines.insert(airline_name.to_string(), sema.clone());
                sema
            }
        };
        //Esto deberia levantarse de un ENV
        WebServiceConnection::new(semaphore, 1000..4000, 0.3)
    }


    pub fn hotel_request(&mut self) -> WebServiceConnection {
        //Esto deberia levantarse de un ENV
        WebServiceConnection::new(self.hotels.clone(), 1000..2000, 0.0)
    }
}