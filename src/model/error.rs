use std::error::Error;
use std::fmt;


pub type AppResult<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct InternalError {
    message: String
}

impl InternalError {
    pub fn new(msg: &str) -> InternalError {
        InternalError{message: msg.to_string()}
    }
}

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