use super::error::{AppResult, InternalError};
use super::logger::Logger;
use std_semaphore::Semaphore;
use std::ops::Range;
use std::sync::Arc;
use std::{thread, time};
use rand::Rng;

pub struct WebServiceConnection {
    permission: Arc<Semaphore>,
    work_time_range: Range<u64>,
    failure_probability: f32,
    logger : Logger
}

impl WebServiceConnection {
    
<<<<<<< HEAD
    pub fn new(permission: Arc<Semaphore>, work_time_range: Range<u64>, failure_probability: f32, in_logger : Logger) -> Self {
        let connection = WebServiceConnection {
            permission,
            work_time_range,
            failure_probability,
            logger : in_logger.clone()
        };

        connection
=======
    pub fn new(permission: Arc<Semaphore>, work_time_range: Range<u64>, failure_probability: f32) -> Self {
        WebServiceConnection {
            permission,
            work_time_range,
            failure_probability,
        }
>>>>>>> 4d586075e83659bebfab1d13c59ba8a8f5f582ef
    }

    pub fn resolve_request(&self) -> AppResult<()> {
        self.permission.acquire();
        let mut rng = rand::thread_rng();
        let work_time = rng.gen_range(self.work_time_range.clone());
        let ok = rng.gen::<f32>() >= self.failure_probability;
        thread::sleep(time::Duration::from_millis(work_time));
        self.permission.release();

        if ok {
            self.logger.log_info(String::from("se acepto un request toquen algo aca xd"));
            Ok(()) 
        } else {
            self.logger.log_error(String::from("op could not be done"));
            Err(Box::new(InternalError::new("Operation couldn't be done, please retry"))) 
        }
    }
}