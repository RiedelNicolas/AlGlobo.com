use super::error::{AppResult};
use std_semaphore::Semaphore;
use std::sync::Arc;
use std::{thread, time};

pub struct WebServiceConnection {
    airline: Arc<Semaphore>,
    hotel: Arc<Semaphore>
}

impl WebServiceConnection {
    pub fn new(airline: Arc<Semaphore>, hotel: Arc<Semaphore>) -> Self {

        let connection = WebServiceConnection {
            airline,
            hotel
        };

        connection
    }

    pub fn resolve_airline_request(&self) -> AppResult<()> {
        self.airline.acquire();
        //Aca hay que cambiar esto por un random de tiempo de trabajo y random de resultado
        println!("Connection with airline web service established");
        thread::sleep(time::Duration::from_secs(3));
        self.airline.release();
        Ok(())
    }

    pub fn resolve_hotel_request(&self) {
        self.hotel.acquire();
        println!("Connection with hotel web service established");
        thread::sleep(time::Duration::from_secs(3));
        self.hotel.release();
    }
}