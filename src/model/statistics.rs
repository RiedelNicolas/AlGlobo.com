use std::collections::HashMap;
use std::time::Duration;
use super::env::Configuration;
use super::logger::Logger;

/// Clase auxiliar utilizada para contener la informacion de un request
#[derive(Debug)]
pub struct InfoRequest {
  route: String,
  time: Duration
}

impl InfoRequest {
    pub fn new(route: String, time: Duration) -> Self { Self { route, time } }

    /// getter de ruta
    pub fn route(&self) -> String {
        String::from(&self.route)
    }
    /// getter de cuanto tiempo demoro en resolverse la consulta
    pub fn time(&self) -> &Duration {
        &self.time
    }
}

/// Clase que representa las estadisticas del sistema
#[derive(Debug)]
pub struct Statistics {
  routes_requested: HashMap<String, u32>,
  total_time: Duration,
  requests_amount: u64,
  logger: Logger,
  log_rate: u64,
  top_req_amount: usize
}


impl Statistics {
    /// Genera una instancia de la clase.
    /// Recibe una referencia al logger donde debe enviar los distintos errores que se le presentan
    pub fn new( in_logger : Logger, envs: Configuration ) -> Self { 
        Self { 
            routes_requested: HashMap::new(),
            total_time: Duration::from_secs(0),
            requests_amount: 0,
            logger: in_logger,
            log_rate: envs.stats_log_rate,
            top_req_amount: envs.stats_top_req_amount,
        }
    }

    /// Actualiza la estadisticas
    pub fn update(&mut self, req: InfoRequest) {
        let route = req.route();
        let time = *req.time();

        self.requests_amount += 1;
        self.total_time += time;
        
        *self.routes_requested.entry(route).or_insert(0) += 1;

        if self.requests_amount % self.log_rate == 0 {
            self.log_data();
        }
    }

    /// Imprime las estadisticas generadas en el log file
    pub fn log_data(&self) {
        let mut top_requested: Vec<(&String, &u32)> = Vec::new();
        for (key,value) in self.routes_requested.iter() {
            top_requested.push((key, value));
        }
        top_requested.sort_by_key(|k| k.1);
        
        let mut top_req_str: String = String::new();
        let index = if top_requested.len() > self.top_req_amount { self.top_req_amount } else { top_requested.len() };
        
        for (i, item) in top_requested.iter().enumerate().take(index) {
            top_req_str.push_str(item.0);
            if i != index-1 { top_req_str.push_str(" // "); };
        }

        self.logger.log_info(format!("[Statistics] Requests Amount: {}", self.requests_amount ));
        self.logger.log_info(format!("[Statistics] Average Request Time: {}s", (self.total_time.as_secs() / self.requests_amount) ));
        self.logger.log_info(format!("[Statistics] Top 10 Requested Routes: {}", top_req_str ));
    }
}
