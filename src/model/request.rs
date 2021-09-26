use super::error::AppResult;

#[derive(Debug)]
pub struct Request {
    origin: String,
    destiny: String,
    airline: String,
    with_hotel: bool
}

impl Request {
    pub fn new(origin: &str, destiny: &str, airline: &str, with_hotel: bool) -> AppResult<Self> {

        let request = Request {
            origin: origin.to_string(),
            destiny: destiny.to_string(),
            airline: airline.to_string(),
            with_hotel
        };

        Ok(request)
    }

    pub fn get_airline(&self) -> &str {
        &self.airline[..]
    }

    pub fn is_package(&self) -> bool {
        self.with_hotel
    }
}
