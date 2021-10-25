use std::error::Error;
use actix::{System, Actor};
mod model;
use model::configuration::{Configuration, get_envs};
use model::error::{InternalError};
use model::administrator::{Administrator};
use model::parser::{Parser, ReadNextLine};
use model::logger::Logger;
use model::logger_message::LoggerMessage;


fn main() -> Result<(), Box<dyn Error>> {

    let csv_path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };
    
    let json_path = match std::env::args().nth(2) {
        Some(r) => r,
        None => String::from("files/env.json")
    };

    let configuration = get_envs(json_path);

    let system = System::new();
    system.block_on(async {
        let addr = Administrator::new(configuration.clone()).start();
        let addr_logger = Logger::new("files/log_file.txt").start();
        addr_logger.do_send(LoggerMessage::new_info(String::from("Arrancamos caravana..")));
        Parser::open(csv_path, addr).unwrap()   //ESTO ESTA MAL, MUY MAL, PERO NO SE COMO ATAJAR EL ERROR
            .start()
            .do_send(ReadNextLine);
    });

    if system.run().is_err() {
        return Err(Box::new(InternalError::new("System error")))    //RRAAAAROOO
    };

    Ok(())
}