use super::error::{AppResult};
use regex::Regex;
use super::request::Request;

#[derive(Debug)]
pub struct Parser {
    reader: io::BufReader<File>,
    matcher: Regex
}

use std::{
    fs::File,
    io::{self, prelude::*}
};

impl Parser {
    pub fn open(path: impl AsRef<std::path::Path>) -> AppResult<Self> {
        let file = File::open(path)?;

        let parser = Parser {
            reader: io::BufReader::new(file),
            matcher: Regex::new(r"^([A-Z]{3}),([A-Z]{3}),([A-z]+),([PV])$")?
        };

        Ok(parser)
    }

    pub fn parse_request(&mut self) -> AppResult<Option<Request>> {

        loop {
            let mut buffer = vec![];

            let bytes = self.reader.read_until(b'\n', &mut buffer)?;
   
            if bytes == 0 {
                return Ok(None)
            }

            let buffer = String::from_utf8(buffer)?.replace("\n", "");

            let cap = match self.matcher.captures(&buffer) {
                None => {continue}, //Si no matchea se ignora el pedido
                Some(value) => value
            };

            let request = Request::new(&cap[1],&cap[2], &cap[3], &cap[4] == "P")?;

            return Ok(Some(request))
        }
    }
}