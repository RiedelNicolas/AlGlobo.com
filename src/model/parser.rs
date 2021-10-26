use super::administrator::Administrator;
use super::error::AppResult;
use super::logger::Logger;
use super::request::Request;
use crate::model::{
    administrator::{EndOfRequests, NewRequest},
    logger_message::LoggerMessage,
};
use actix::prelude::*;
use regex::Regex;
use std::{
    fs::File,
    io::{self, prelude::*},
};

#[derive(Message)]
#[rtype(result = "")]
pub struct ReadNextLine;

pub struct Parser {
    reader: io::BufReader<File>,
    matcher: Regex,
    admin: Addr<Administrator>,
    logger: Addr<Logger>,
}

impl Parser {
    pub fn open(
        path: impl AsRef<std::path::Path>,
        admin: Addr<Administrator>,
        logger: Addr<Logger>,
    ) -> AppResult<Self> {
        let file = File::open(path)?;

        let parser = Parser {
            reader: io::BufReader::new(file),
            matcher: Regex::new(r"^([A-Z]{3}),([A-Z]{3}),([A-z]+),([PV])$")?,
            admin,
            logger,
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

            let bytes = self
                .reader
                .read_until(b'\n', &mut buffer)
                .expect("[Parser]: Error trying to read input file");

            if bytes == 0 {
                self.admin.do_send(EndOfRequests);
                break;
            }

            let buffer = String::from_utf8(buffer)
                .expect("[Parser]: Error trying to parse string")
                .replace("\n", "");

            let cap = match self.matcher.captures(&buffer) {
                None => {
                    self.logger.do_send(LoggerMessage::new_warning(
                        "[Parser]: Bad format on input line found".to_string(),
                    ));
                    continue;
                }
                Some(value) => value,
            };

            let request = Request::new(&cap[1], &cap[2], &cap[3], &cap[4] == "P");

            self.admin.do_send(NewRequest(request));

            ctx.address().do_send(ReadNextLine);
        }
    }
}
