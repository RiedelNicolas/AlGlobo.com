use super::administrator::{Administrator, FinishedWebServiceRequest};
use super::configuration::Configuration;
use super::logger::Logger;
use crate::model::hotel_connection::{HotelConnection, Request};
use actix::prelude::*;
use std::ops::Range;

#[derive(Message)]
#[rtype(result = "")]
pub struct HotelRequest(pub usize);

#[derive(Message)]
#[rtype(result = "")]
pub struct WaitAndProcessRequest;

#[derive(Message)]
#[rtype(result = "")]
pub struct ConnectionFinished(pub usize);

/// Clase que representa un hotel, encargado de manejar las distintas conexiones simuladas.
/// ACLARACION : Se considera a el hotel como unico.
pub struct Hotel {
    connections: Vec<Addr<HotelConnection>>,
    next_connection: usize,
    max_concurrent_connections: usize,
    admin: Addr<Administrator>,
    configuration: Configuration,
    logger: Addr<Logger>,
}

impl Hotel {
    pub fn new(
        admin: Addr<Administrator>,
        configuration: Configuration,
        logger: Addr<Logger>,
    ) -> Hotel {
        Hotel {
            connections: Vec::new(),
            next_connection: 0,
            max_concurrent_connections: configuration.hotel_limit,
            admin,
            configuration,
            logger,
        }
    }
    /// Devuelve una nueva conexion a la aerolinea.
    pub fn get_next_connection(&mut self, hotel_address: Addr<Hotel>) -> Addr<HotelConnection> {
        let connection = match self.connections.get_mut(self.next_connection) {
            Some(conn) => conn.clone(),
            None => {
                let conn = HotelConnection::new(
                    hotel_address,
                    Range {
                        start: self.configuration.hotel_min_work_time,
                        end: self.configuration.hotel_max_work_time,
                    },
                    self.logger.clone(),
                )
                .start();
                self.connections.push(conn.clone());
                conn
            }
        };

        if self.next_connection + 1 >= self.max_concurrent_connections {
            self.next_connection = 0;
        } else {
            self.next_connection += 1;
        }

        connection
    }
}

impl Actor for Hotel {
    type Context = Context<Self>;
}

impl Handler<HotelRequest> for Hotel {
    type Result = ();

    fn handle(&mut self, msg: HotelRequest, ctx: &mut Context<Self>) -> Self::Result {
        let req_id = msg.0;
        let addr = self.get_next_connection(ctx.address());

        addr.do_send(Request(req_id));
    }
}

impl Handler<ConnectionFinished> for Hotel {
    type Result = ();

    fn handle(&mut self, msg: ConnectionFinished, _ctx: &mut Context<Self>) -> Self::Result {
        self.admin.do_send(FinishedWebServiceRequest(msg.0));
    }
}
