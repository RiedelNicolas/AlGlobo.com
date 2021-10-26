use super::error::AppResult;
use std::time::{Duration, Instant};

/// Representa un pedido de reserva particular.
#[derive(Debug)]
pub struct Request {
    origin: String,
    destiny: String,
    airline: String,
    with_hotel: bool,
    arrival_time: Instant,
    completion_time: Duration,
    finished: bool,
}

impl Request {
    /// Genera una instancia de una solicitud
    /// Debe recibir por parametros los datos necesarios para construirla.
    pub fn new(origin: &str, destiny: &str, airline: &str, with_hotel: bool) -> AppResult<Self> {
        let request = Request {
            origin: origin.to_string(),
            destiny: destiny.to_string(),
            airline: airline.to_string(),
            with_hotel,
            arrival_time: Instant::now(),
            completion_time: Duration::from_secs(0),
            finished: false,
        };

        Ok(request)
    }

    /// Getter de la aerolinea
    pub fn get_airline(&self) -> &str {
        &self.airline[..]
    }

    /// Responde a la pregunta -> ¿ La reserva es con hotel ?
    pub fn is_package(&self) -> bool {
        self.with_hotel
    }

    /// Marca request como finalizada y establece el tiempo de resolucion
    pub fn finish(&mut self) {
        self.finished = true;
        self.completion_time = self.arrival_time.elapsed();
    }

    /// Devuelve true en caso de que la reserva se encuentre resuelta
    pub fn has_finished(&self) -> bool {
        self.finished
    }

    /// Devuelve la ruta del vuelo.
    pub fn get_route(&self) -> String {
        format!("{}->{}", self.origin, self.destiny)
    }

    /// Tiempo total desde que se hizo la consulta hasta que fue resuelta  
    pub fn get_completion_time(&self) -> &Duration {
        &self.completion_time
    }
}
