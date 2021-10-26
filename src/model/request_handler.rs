use super::env::Configuration;
use super::error::AppResult;
use super::logger::Logger;
use super::request::Request;
use super::statistics::{InfoRequest, Statistics};
use super::web_service_connection::WebServiceConnection;
use super::web_service_provider::WebServiceProvider;
use std::sync::{Arc, Barrier, Mutex, RwLock};
use std::thread::{self, JoinHandle};
use std::time::Duration;
use uid::Id;

/// Maneja y distribuye los distintos request que recibe el sistema.
pub struct RequestHandler {
    id: Id<usize>,
    request: Arc<RwLock<Request>>,
    airline: Option<JoinHandle<()>>,
    hotel: Option<JoinHandle<()>>,
    logger: Logger,
}

impl RequestHandler {
    /// Instancia un handler, recibe struct de configuracion y logger.
    pub fn spawn(
        req: Request,
        provider: &mut WebServiceProvider,
        envs: Configuration,
        in_logger: Logger,
        stats: Arc<Mutex<Statistics>>,
    ) -> AppResult<Self> {
        let connection = provider.airline_request(req.get_airline(), envs);
        let is_package = req.is_package();
        let barrier_airline = Arc::new(Barrier::new(1 + req.is_package() as usize));
        let barrier_hotel = barrier_airline.clone();
        let protected_request_local = Arc::new(RwLock::new(req));
        let protected_request_airline = protected_request_local.clone();
        let logger_clone = in_logger.clone();
        let id: Id<usize> = Id::new();

        let handler = RequestHandler {
            id,
            request: protected_request_local,
            airline: Some(thread::spawn(move || {
                RequestHandler::process_airline(
                    protected_request_airline,
                    connection,
                    envs,
                    logger_clone,
                    barrier_airline,
                    stats,
                    id.get(),
                )
            })),
            hotel: if is_package {
                let connection = provider.hotel_request(envs);
                let aux = in_logger.clone();
                Some(thread::spawn(move || {
                    RequestHandler::process_hotel(connection, aux, barrier_hotel, id.get())
                }))
            } else {
                None
            },
            logger: in_logger,
        };

        Ok(handler)
    }

    /// Procesa una request de aerolinea, recibe una conexiona al webservice donde debe derivar la misma.
    fn process_airline(
        request: Arc<RwLock<Request>>,
        connection: WebServiceConnection,
        envs: Configuration,
        logger: Logger,
        barrier: Arc<Barrier>,
        stats: Arc<Mutex<Statistics>>,
        id: usize,
    ) {
        let rh_string = format!("[RequestHandler: {}]", id);

        logger.log_info(format!(
            "{} Connecting to airline extern web-service",
            rh_string
        ));
        loop {
            match connection.resolve_request() {
                Ok(_) => {
                    logger.log_info(format!("{} Airline request completed", rh_string));
                    break;
                }
                Err(_) => {
                    logger.log_warning(format!(
                        "{} Airline request could not be done. Retrying in a moment",
                        rh_string
                    ));
                }
            }

            thread::sleep(Duration::from_millis(envs.sleeping_retry_time));
        }

        barrier.wait();
        logger.log_info(format!("{} Request completed", rh_string));

        match request.write() {
            Ok(mut req) => {
                req.finish();
                match stats.lock() {
                    Ok(mut statistics) => statistics.update(InfoRequest::new(
                        req.get_route(),
                        *req.get_completion_time(),
                    )),
                    Err(_) => {
                        logger.log_error(format!("{} Fatal error: Poisoned Lock", rh_string));
                    }
                }
            }
            Err(_) => {
                logger.log_error(format!("{} Fatal error: Poisoned Lock", rh_string));
            }
        }
    }

    /// Procesa una request de hotel, recibe una conexiona al webservice donde debe derivar la misma.
    fn process_hotel(
        connection: WebServiceConnection,
        logger: Logger,
        barrier: Arc<Barrier>,
        id: usize,
    ) {
        let rh_string = format!("[RequestHandler: {}]", id);

        logger.log_info(format!(
            "{} Trying to connect to hotel extern web-service",
            rh_string
        ));
        logger.log_info(format!("{} Hotel request completed", rh_string));
        let _ = connection.resolve_request(); // Las request al hotel no fallan
        barrier.wait();
    }

    /// Consulta el estado interno, devuelve true si la request ya fue resuelta.
    /// False caso contrario.
    pub fn has_finished(&self) -> bool {
        match self.request.read() {
            Ok(req) => req.has_finished(),
            Err(_) => {
                let rh_string = format!("[RequestHandler: {}]", self.id.get());
                self.logger
                    .log_error(format!("{} Fatal error: Poisoned Lock", rh_string));
                false
            }
        }
    }

    /// Metodo bloqueante, bloquea la ejecucion hasta que la request fue resuelta.
    pub fn join(self) {
        if let Some(airline) = self.airline {
            let _ = airline.join();
        }
        if let Some(hotel) = self.hotel {
            let _ = hotel.join();
        }
    }
}
