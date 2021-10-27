use super::env::Configuration;
use super::logger::Logger;
use super::web_service_connection::WebServiceConnection;
use std::collections::HashMap;
use std::sync::Arc;
use std_semaphore::Semaphore;

/// Clase que modela un webProvider
pub struct WebServiceProvider {
    airlines: HashMap<String, Arc<Semaphore>>,
    airline_limit: u32,
    hotels: Arc<Semaphore>,
    logger: Logger,
}

impl WebServiceProvider {
    /// Al ser construido recibe las limitaciones del provider que modela
    /// Limite de aerolinea y limite de hotel.
    pub fn new(airline_limit: u32, hotel_limit: u32, logger: Logger) -> Self {
        WebServiceProvider {
            airlines: HashMap::new(),
            airline_limit,
            hotels: Arc::new(Semaphore::new(hotel_limit as isize)),
            logger,
        }
    }

    /// Provee una WebServiceConnection inicializada para conectarse con una aerolinea.
    /// La aerolinea se especifica a traves de @airline_name
    pub fn airline_request(
        &mut self,
        airline_name: &str,
        envs: Configuration,
    ) -> WebServiceConnection {
        let semaphore = match self.airlines.get(airline_name) {
            Some(sema) => sema.clone(),
            None => {
                let sema = Arc::new(Semaphore::new(self.airline_limit as isize));
                self.airlines.insert(airline_name.to_string(), sema.clone());
                sema
            }
        };
        WebServiceConnection::new(
            airline_name.to_string(),
            semaphore,
            envs.air_min_work_time..envs.air_max_work_time,
            envs.air_failure_probability,
            self.logger.clone(),
        )
    }

    /// Provee una WebServiceConnection inicializada para conectarse con un hotel.
    pub fn hotel_request(&mut self, envs: Configuration) -> WebServiceConnection {
        WebServiceConnection::new(
            "Hotel".to_string(),
            self.hotels.clone(),
            envs.hotel_min_work_time..envs.hotel_max_work_time,
            0.0,
            self.logger.clone(),
        )
    }
}
