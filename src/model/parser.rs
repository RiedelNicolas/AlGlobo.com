use crate::model::administrator::NewRequest;

use super::request::Request;
use super::administrator::Administrator;
use super::error::AppResult;
use actix::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::{self, prelude::*}
};

#[derive(Message)]
#[rtype(result = "")]
pub struct ReadNextLine;

pub struct Parser {
    reader: io::BufReader<File>,
    matcher: Regex,
    admin: Addr<Administrator>
}

impl Parser {
    
    pub fn open(path: impl AsRef<std::path::Path>, admin: Addr<Administrator>) -> AppResult<Self> {
        let file = File::open(path)?;

        let parser = Parser {
            reader: io::BufReader::new(file),
            matcher: Regex::new(r"^([A-Z]{3}),([A-Z]{3}),([A-z]+),([PV])$")?,
            admin
        };

        Ok(parser)
    }
}

impl Actor for Parser {
    type Context = Context<Self>;
}

impl Handler<ReadNextLine> for Parser {
    type Result = ();

    fn handle(&mut self, _msg: ReadNextLine, ctx: &mut Context<Self>) -> Self::Result {

        loop {
            let mut buffer = vec![];
            //CAMBIAR ESTE UNWRAP, ESTA MAAAAL
            let bytes = self.reader.read_until(b'\n', &mut buffer).unwrap();
   
            if bytes == 0 {
                return
            }
            //CAMBIAR ESTE UNWRAP
            let buffer = String::from_utf8(buffer).unwrap().replace("\n", "");

            let cap = match self.matcher.captures(&buffer) {
                None => {continue}, //Si no matchea se ignora el pedido
                Some(value) => value
            };

            let request = Request::new(&cap[1],&cap[2], &cap[3], &cap[4] == "P");

            if self.admin.try_send(NewRequest(request)).is_err(){
                println!("Failing to send a new request to administrator");
            }

            if ctx.address().try_send(ReadNextLine).is_err(){
                println!("Failing to send a next line message to parser");
            }
        }
    }
}