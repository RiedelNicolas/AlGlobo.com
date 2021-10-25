use std::error::Error;
use std::fmt;


pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Clase utilizada para manejar errores.
#[derive(Debug)]
pub struct InternalError {
    message: String
}

impl InternalError {
    /// Devuelve una instancia de un error interno,
    /// El string recibido es utilizado como descripcion del mensaje.
    pub fn new(msg: &str) -> InternalError {
        InternalError{message: msg.to_string()}
    }
}

/// Imprime la descripcion interna del mensaje.
impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.message)
    }
}

impl Error for InternalError {
    fn description(&self) -> &str {
        &self.message
    }
}