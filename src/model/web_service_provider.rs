use std_semaphore::Semaphore;
use std::sync::Arc;
use std::collections::HashMap;
use super::env::Configuration;
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
        WebServiceProvider {
            airlines: HashMap::new(),
            airline_limit,
            hotels: Arc::new(Semaphore::new(hotel_limit as isize)),
            logger: in_logger
        }
    }

    pub fn airline_request(&mut self, airline_name: &str, envs: Configuration) -> WebServiceConnection {

        self.logger.log_info(format!("[WebServiceProvider] Sending request to the airline {}", airline_name));
        let semaphore = match self.airlines.get(airline_name) {
            Some(sema) => sema.clone(),
            None => {
                let sema = Arc::new(Semaphore::new(self.airline_limit as isize));
                self.airlines.insert(airline_name.to_string(), sema.clone());
                sema
            }
        };
        WebServiceConnection::new(semaphore, envs.air_min_work_time..envs.air_max_work_time, envs.air_failure_probability )
    }


    pub fn hotel_request(&mut self, envs: Configuration) -> WebServiceConnection {
        self.logger.log_info("[WebServiceProvider] Sending request to the hotel".to_string()); //CHEQUEAR
        WebServiceConnection::new(self.hotels.clone(), envs.hotel_min_work_time..envs.hotel_max_work_time, envs.hotel_failure_probability)
    }
}