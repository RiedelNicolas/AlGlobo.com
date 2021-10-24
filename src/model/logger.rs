use actix::prelude::*;
use std::io::Write;
use std::fs::File;
use std::fs;
use super::logger_message::LoggerMessage;

struct Logger {
    log_file : File
}

impl Actor for Logger {
    type Context = Context<Self>;
}

// Constructor del logger, 
impl Logger {
    pub fn new( path : &str ) -> Logger {
        Logger {
            log_file : fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .unwrap()
        }
    }
}


impl Handler<LoggerMessage> for Logger {

    type Result = ();

    fn handle(&mut self, msg: LoggerMessage, ctx: &mut Context <Self>) -> Self::Result {
        match write!(self.log_file, "{}\n", msg.generate_string() ) {
            Ok(v) => v,
            Err(e) => println!("Error found trying to write the logfile : {}", e)
        }
    }
}