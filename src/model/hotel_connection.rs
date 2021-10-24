use actix::prelude::*;
use super::hotel::{ 
    Hotel, 
    ConnectionFinished
};
use actix::clock::sleep;
use std::time::Duration;
use rand::Rng;
use std::collections::VecDeque; //TODO: Modificar por una cola normal
use std::ops::Range;

#[derive(Message)]
#[rtype(result = "")]
pub struct ProcessRequest;


#[derive(Message)]
#[rtype(result = "")]
pub struct Request(pub usize);

pub struct HotelConnection {
    pending_requests: VecDeque<usize>,
    hotel: Addr<Hotel>,
    work_time_range: Range<u64>
}

impl HotelConnection {
    
    pub fn new( hotel: Addr<Hotel>, 
                work_time_range: Range<u64>) -> HotelConnection {
        HotelConnection {
            pending_requests: VecDeque::new(),
            hotel,
            work_time_range
        }
    }
}

impl Actor for HotelConnection {
    type Context = Context<Self>;
}

impl Handler<Request> for HotelConnection {
    type Result = ();

    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Self::Result {
        if self.pending_requests.is_empty() && 
            ctx.address().try_send(ProcessRequest).is_err(){
            
            println!("Failing to process request");
        }
        self.pending_requests.push_back(msg.0);
    }
}


impl Handler<ProcessRequest> for HotelConnection {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, _msg: ProcessRequest, _ctx: &mut Context<Self>) -> Self::Result {

        //TODO: Eliminar este unwrap
        let id = self.pending_requests.pop_front().unwrap();
        
        println!("Request [{}]: Conectando con el hotel", id);

        Box::pin(sleep(Duration::from_millis(rand::thread_rng().gen_range(self.work_time_range.clone())))
            .into_actor(self)
            .map(move |_result, me, ctx| {
                
                println!("Request [{}]: Request al hotel resuelta", id);
                if me.hotel.try_send(ConnectionFinished(id)).is_err(){
                    println!("Failing to send [{}] completed request to airline", id);
                }

                if !me.pending_requests.is_empty() &&
                    ctx.address().try_send(ProcessRequest).is_err() {
                    println!("Failing to process request");
                }
            }))
    }
}

