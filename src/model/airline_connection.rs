use super::airline::{Airline, ConnectionFailed, ConnectionFinished};
use super::logger::Logger;
use crate::model::logger_message::LoggerMessage;
use actix::clock::sleep;
use actix::prelude::*;
use rand::Rng;
use std::collections::VecDeque; //TODO: Modificar por una cola normal
use std::ops::Range;
use std::time::Duration;

/// Mensaje que indica que se debe procesar una solicitud
#[derive(Message)]
#[rtype(result = "")]
pub struct ProcessRequest;

/// Mensaje que representa un request al webservice simulado.
#[derive(Message)]
#[rtype(result = "")]
pub struct Request(pub usize);

/// Clase que simula un webservice de una aerolinea
pub struct AirlineConnection {
    pending_requests: VecDeque<usize>,
    airline_name: String,
    airline: Addr<Airline>,
    work_time_range: Range<usize>,
    failure_probability: f32,
    logger: Addr<Logger>,
}


impl AirlineConnection {
    /// Devuelve una instancia de un webservice de una aerolinea, 
    /// en el constructor recibe los parametros necesarios para construirla
    pub fn new(
        airline_name: String,
        airline: Addr<Airline>,
        work_time_range: Range<usize>,
        failure_probability: f32,
        logger: Addr<Logger>,
    ) -> AirlineConnection {
        AirlineConnection {
            pending_requests: VecDeque::new(),
            airline_name,
            airline,
            work_time_range,
            failure_probability,
            logger,
        }
    }
}

impl Actor for AirlineConnection {
    type Context = Context<Self>;
}

impl Handler<Request> for AirlineConnection {
    type Result = ();
    /// Handler que maneja la solicitud de una reserva a dicho webservice
    fn handle(&mut self, msg: Request, ctx: &mut Context<Self>) -> Self::Result {
        if self.pending_requests.is_empty() {
            ctx.address().do_send(ProcessRequest);
        }
        self.pending_requests.push_back(msg.0);
    }
}

impl Handler<ProcessRequest> for AirlineConnection {
    type Result = ResponseActFuture<Self, ()>;
    /// Handler que maneja el procesamiento de una request, en caso de que la aerolinea rechace
    /// la reserva volvera a intentar en unos segundos hasta que la misma sea aceptada.
    fn handle(&mut self, _msg: ProcessRequest, _ctx: &mut Context<Self>) -> Self::Result {
        let id = self
            .pending_requests
            .pop_front()
            .expect("[AirlineConnection]: Found an empty queue on ProcessRequest");

        self.logger.do_send(LoggerMessage::new_info(format!(
            "[AirlineConnection: {}]: Conectando con la aerolinea {}",
            id, self.airline_name
        )));

        let rand_numb = rand::thread_rng().gen_range(self.work_time_range.clone());

        Box::pin(
            sleep(Duration::from_millis(rand_numb as u64))
                .into_actor(self)
                .map(move |_result, me, ctx| {
                    let num_random = rand::thread_rng().gen::<f32>();
                    let request_solved = num_random >= me.failure_probability;

                    if request_solved {
                        me.logger.do_send(LoggerMessage::new_info(format!(
                            "[AirlineConnection: {}]: Request a la aerolinea resuelta",
                            id
                        )));
                        me.airline.do_send(ConnectionFinished(id));
                    } else {
                        me.logger.do_send(LoggerMessage::new_warning(format!(
                            "[AirlineConnection: {}]: Fallo al resolver la request",
                            id
                        )));
                        me.airline.do_send(ConnectionFailed(id));
                    }

                    if !me.pending_requests.is_empty() {
                        ctx.address().do_send(ProcessRequest);
                    }
                }),
        )
    }
}
