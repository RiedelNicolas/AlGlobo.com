use actix::fut::future::*;
use actix::prelude::*;
use super::hotel::{ 
    Hotel, 
    ConnectionFinished
};
use actix::clock::sleep;
use std::time::Duration;
use rand::Rng;
use std::collections::VecDeque;
use std::ops::Range;
use super::logger::Logger;
use crate::model::logger_message::LoggerMessage;

#[derive(Message)]
#[rtype(result = "")]
pub struct ProcessRequest;


#[derive(Message)]
#[rtype(result = "")]
pub struct Request(pub usize);

pub struct HotelConnection {
    pending_requests: VecDeque<usize>,
    hotel: Addr<Hotel>,
    work_time_range: Range<usize>,
    logger: Addr<Logger>
}

impl HotelConnection {
    
    pub fn new( hotel: Addr<Hotel>, 
                work_time_range: Range<usize>,
                logger: Addr<Logger>) -> HotelConnection {
        HotelConnection {
            pending_requests: VecDeque::new(),
            hotel,
            work_time_range,
            logger
        }
    }
}

impl Actor for HotelConnection {
    type Context = Context<Self>;
}

impl Handler<Request> for HotelConnection {
    type Result = ();

    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Self::Result {
        if self.pending_requests.is_empty() {
            ctx.address().do_send(ProcessRequest);
        }

        self.pending_requests.push_back(msg.0);
    }
}


impl Handler<ProcessRequest> for HotelConnection {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: ProcessRequest, _ctx: &mut Context<Self>) -> Self::Result {

        let id = self.pending_requests
            .pop_front()
            .expect("[HotelConnection]: Found an empty queue on ProcessRequest");
        
        self.logger.do_send(LoggerMessage::new_info(format!("[HotelConnection: {}]: Conectando con el hotel", id)));
        let rand_number = rand::thread_rng().gen_range(self.work_time_range.clone());

        Box::pin(sleep(Duration::from_millis(rand_number as u64))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                
                me.logger.do_send(LoggerMessage::new_info(format!("[HotelConnection: {}]: Request al hotel resuelta", id)));
                me.hotel.do_send(ConnectionFinished(id));

                if !me.pending_requests.is_empty(){
                    ctx.address().do_send(ProcessRequest);
                }
            }))
    }
}

