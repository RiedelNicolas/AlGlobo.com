use std::error::Error;
use std::fmt;

pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

/// Clase utilizada para manejar error internos del sistema
#[derive(Debug)]
pub struct InternalError {
    message: String,
}

impl InternalError {
    /// Genera una intancia de InternalError, el string recibido es utilizado para identificar el error.
    pub fn new(msg: &str) -> InternalError {
        InternalError {
            message: msg.to_string(),
        }
    }
}

impl fmt::Display for InternalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for InternalError {
    fn description(&self) -> &str {
        &self.message
    }
}
