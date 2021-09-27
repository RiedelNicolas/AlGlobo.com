use super::error::{AppResult, InternalError};
use std_semaphore::Semaphore;
use std::ops::Range;
use std::sync::Arc;
use std::{thread, time};
use rand::Rng;

pub struct WebServiceConnection {
    permission: Arc<Semaphore>,
    work_time_range: Range<u64>,
    failure_probability: f32
}

impl WebServiceConnection {
    
    pub fn new(permission: Arc<Semaphore>, work_time_range: Range<u64>, failure_probability: f32) -> Self {
        let connection = WebServiceConnection {
            permission,
            work_time_range,
            failure_probability,
        };

        connection
    }

    pub fn resolve_request(&self) -> AppResult<()> {
        self.permission.acquire();
        let mut rng = rand::thread_rng();
        let work_time = rng.gen_range(self.work_time_range.clone());
        let ok = rng.gen::<f32>() >= self.failure_probability; //Esto deberia levantarse de un ENV
        thread::sleep(time::Duration::from_millis(work_time));
        self.permission.release();

        if ok { 
            Ok(()) 
        } else { 
            Err(Box::new(InternalError::new("Operation couldn't be done, please retry"))) 
        }
    }
}