use super::error::{AppResult, InternalError};
use super::logger::Logger;
use rand::Rng;
use std::ops::Range;
use std::sync::Arc;
use std::{thread, time};
use std_semaphore::Semaphore;

/// Clase que modela un webservice
pub struct WebServiceConnection {
    name: String,
    permission: Arc<Semaphore>,
    work_time_range: Range<u64>,
    failure_probability: f32,
    logger: Logger,
}

impl WebServiceConnection {
    /// Instancia un webservice
    /// Recibe un Semaphore para sincronismo.
    /// Una probabilidad de fallo (Para simular un webservice real).
    /// Y un rango que sera utilizado para simular (mediante un aleatorio) cuanto tarda la consulta.
    pub fn new(
        name: String,
        permission: Arc<Semaphore>,
        work_time_range: Range<u64>,
        failure_probability: f32,
        logger: Logger,
    ) -> Self {
        WebServiceConnection {
            name,
            permission,
            work_time_range,
            failure_probability,
            logger,
        }
    }

    /// Simula una conexion a un web service externo.
    /// Retorna Err en caso de fallo de conexion u Ok vacio en caso exitoso.
    pub fn resolve_request(&self, id: usize) -> AppResult<()> {
        self.permission.acquire();
        self.logger.log_info(format!(
            "[WebServiceConnection: {}]: Connection with {} web service established",
            id, self.name
        ));
        let mut rng = rand::thread_rng();
        let work_time = rng.gen_range(self.work_time_range.clone());
        let ok = rng.gen::<f32>() >= self.failure_probability;
        thread::sleep(time::Duration::from_millis(work_time));
        self.permission.release();

        if ok {
            Ok(())
        } else {
            Err(Box::new(InternalError::new("Request could not be done")))
        }
    }
}
