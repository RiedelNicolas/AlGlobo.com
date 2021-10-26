use std::error::Error;
use actix::{System, Actor};
mod model;
use model::configuration::{get_envs};
use model::error::{InternalError};
use model::administrator::{Administrator};
use model::parser::{Parser, ReadNextLine};
use model::logger::Logger;
use model::logger_message::LoggerMessage;
use model::statistics::Statistics;


fn main() -> Result<(), Box<dyn Error>> {

    let csv_path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };

    let json_path = match std::env::args().nth(2) {
        Some(r) => r,
        None => String::from("files/env.json")
    };

    let log_path = match std::env::args().nth(3) {
        Some(r) =>r,
        None => String::from("files/log_file.txt")
    };

    let configuration = get_envs(json_path);

    let system = System::new();

    system.block_on(async {
        let logger_addr = Logger::new(&log_path).start();
        let stats_addr = Statistics::new(configuration.clone(), 
                                                        logger_addr.clone()).start();
        let admin_addr = Administrator::new(
            stats_addr,
                    configuration.clone(), 
                    logger_addr.clone()).start();

        match Parser::open(csv_path, admin_addr) {
            Ok(parser) => {
                parser.start().do_send(ReadNextLine);
                logger_addr.do_send(LoggerMessage::new_info(String::from("[Main]: Server up")));
            },
            Err(_) => {
                logger_addr.do_send(LoggerMessage::new_error("[Main]: Could not open input file".to_string()));
                System::current().stop();
            }
        }       
    });

    if system.run().is_err() {
        return Err(Box::new(InternalError::new("[Main]: System could not run")))
    };

    Ok(())
}