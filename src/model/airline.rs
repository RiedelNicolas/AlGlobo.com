use actix::prelude::*;
use super::administrator::{Administrator, FinishedWebServiceRequest};
use rand::{thread_rng, Rng};
use actix::clock::sleep;
use std::time::Duration;

#[derive(Message)]
#[rtype(result = "")]
pub struct AirlineRequest(pub usize);

pub struct Airline {
    name: String,
    conections: Vec<usize>,
    next_connection: usize,
    admin: Addr<Administrator>
}

impl Airline {
    
    pub fn new(name: &str, admin: Addr<Administrator>) -> Airline {
        Airline {
            conections: vec!(),
            next_connection: 0,
            name: name.to_string(),
            admin
        }
    }
    
    pub fn calculate_next_connection(&mut self) -> usize{
        if self.next_connection >= self.conections.len() {
            self.next_connection = 0;
        } else {
            self.next_connection += 1;
        }

        self.next_connection
    }

}

impl Actor for Airline {
    type Context = Context<Self>;
}

impl Handler<AirlineRequest> for Airline {
    type Result = ResponseActFuture<Self, ()>;

    fn handle(&mut self, msg: AirlineRequest, ctx: &mut Context<Self>) -> Self::Result {
        /*
        Pasos:
    
        - Enviar la request a alguna de las conexiones, puede ser en round robin
    
        */
        let id = msg.0;
        println!("Request [{}]: Conectando con la aerolinea {}", id, self.name);

        /*
        Lo que viene a continuacion es simplemente una prueba, este actor no debe dormir.
        */

        Box::pin(sleep(Duration::from_millis(thread_rng().gen_range(1000..2000)))
            .into_actor(self)
            .map(move |_result, me, _ctx| {
                println!("Request [{}]: Request a la aerolinea resuelta", id);
                
                if let Err(_) = me.admin.try_send(FinishedWebServiceRequest(id)){
                    println!("Failing to send [{}] completed request to adminstrator", id);
                }
            }))
        //Algo asi deberia hacerse
        //connections[self.calculate_next_connection()].send(...)
    }
}
