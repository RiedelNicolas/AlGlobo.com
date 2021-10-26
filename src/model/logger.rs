use actix::prelude::*;
use std::io::Write;
use std::fs::File;
use std::fs;
use super::logger_message::LoggerMessage;

pub struct Logger {
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
                .expect("[Logger]: Log file could not be open")
        }

    }

}


impl Handler<LoggerMessage> for Logger {

    type Result = ();

    fn handle(&mut self, msg: LoggerMessage, _ctx: &mut Context <Self>) -> Self::Result {
        let message = msg.generate_string();
        println!("{}", message);
        match write!(self.log_file, "{}\n", message ) {
            Ok(v) => v,
            Err(e) => println!("[Logger]: Error found trying to write the logfile : {}", e)
        }
    }
}