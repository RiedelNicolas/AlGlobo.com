use super::logger_message::LoggerMessage;
use actix::prelude::*;
use std::fs;
use std::fs::File;
use std::io::Write;

///Clase Logger
pub struct Logger {
    log_file: File,
}

impl Actor for Logger {
    type Context = Context<Self>;
}

/// Constructor del logger, devuelve una instancia.
/// El string recibido por parametros es la ruta al archivo donde se escriben los mensajes
impl Logger {
    pub fn new(path: &str) -> Logger {
        Logger {
            log_file: fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .expect("[Logger]: Log file could not be open"),
        }
    }
}

impl Handler<LoggerMessage> for Logger {
    type Result = ();

    fn handle(&mut self, msg: LoggerMessage, _ctx: &mut Context<Self>) -> Self::Result {
        let message = msg.generate_string();
        println!("{}", message);
        match writeln!(self.log_file, "{}", message) {
            Ok(v) => v,
            Err(e) => println!("[Logger]: Error found trying to write the logfile : {}", e),
        }
    }
}
