use super::error::{AppResult};
use std::time::{Duration, Instant};

/// Representa un pedido de reserva particular.
#[derive(Debug)]
pub struct Request {
    origin: String,
    destiny: String,
    airline: String,
    with_hotel: bool,
    stages: u32,
    arrival_time: Instant,
    completion_time: Duration
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
            stages: 1 + (with_hotel as u32),
            arrival_time: Instant::now(),
            completion_time: Duration::from_secs(0)
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
    
    /// - 
    pub fn finish(&mut self) {
        if self.stages > 0 {
            self.completion_time = self.arrival_time.elapsed();
            self.stages -= 1;
        }
    }
    
    /// Devuelve true en caso de que la reserva se encuentre resuelta.
    pub fn has_finished(&self) -> bool {
        self.stages == 0
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
