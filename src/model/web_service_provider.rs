use std_semaphore::Semaphore;
use std::sync::Arc;
use std::collections::HashMap;
use super::logger::Logger;
use super::web_service_connection::WebServiceConnection;

pub struct WebServiceProvider {
    airlines: HashMap<String, Arc<Semaphore>>,
    airline_limit: u32,
    hotels: Arc<Semaphore>,
    logger: Logger
}

impl WebServiceProvider {
    pub fn new(airline_limit: u32, hotel_limit: u32, in_logger : Logger) -> Self {

        let server = WebServiceProvider {
            airlines: HashMap::new(),
            airline_limit,
            hotels: Arc::new(Semaphore::new(hotel_limit as isize)),
            logger: in_logger.clone()
        };

        in_logger.log_info(String::from("Server Up") );
        server
    }

    pub fn airline_request(&mut self, airline_name: &str) -> WebServiceConnection {

        self.logger.log_info(format!("Sending request to the airline {}", airline_name));
        let semaphore = match self.airlines.get(airline_name) {
            Some(sema) => sema.clone(),
            None => {
                let sema = Arc::new(Semaphore::new(self.airline_limit as isize));
                self.airlines.insert(airline_name.to_string(), sema.clone());
                sema
            }
        };
        //Esto deberia levantarse de un ENV
        WebServiceConnection::new(semaphore, 1000..4000, 0.3,
             self.logger.clone() )
    }


    pub fn hotel_request(&mut self) -> WebServiceConnection {
        self.logger.log_info(format!("Sending request to the hotel (julian ayuda salvame)"));
        //Esto deberia levantarse de un ENV
        WebServiceConnection::new(self.hotels.clone(), 1000..2000, 0.0,
        self.logger.clone())
    }
}