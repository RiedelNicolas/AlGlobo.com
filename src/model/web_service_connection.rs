use super::error::{AppResult, InternalError};
use std_semaphore::Semaphore;
use std::sync::Arc;
use std::{thread, time};
use rand::Rng;

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
        println!("Connection with airline web service established");
        let mut rng = rand::thread_rng();
        let work_time = rng.gen_range(1000..4000);
        let ok = rng.gen::<f32>() > 0.3; //Esto deberia levantarse de un ENV
        thread::sleep(time::Duration::from_millis(work_time));
        self.airline.release();

        if ok { 
            Ok(()) 
        } else { 
            Err(Box::new(InternalError::new("Operation couldn't be done, please retry"))) 
        }
    }

    pub fn resolve_hotel_request(&self) {
        self.hotel.acquire();
        let mut rng = rand::thread_rng();
        let work_time = rng.gen_range(1000..4000);
        println!("Connection with hotel web service established");
        thread::sleep(time::Duration::from_millis(work_time));
        self.hotel.release();
    }
}