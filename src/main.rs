use actix::{Actor, System};
use std::error::Error;
mod model;
use model::administrator::Administrator;
use model::configuration::get_envs;
use model::error::InternalError;
use model::logger::Logger;
use model::logger_message::LoggerMessage;
use model::parser::{Parser, ReadNextLine};
use model::statistics::Statistics;

fn main() -> Result<(), Box<dyn Error>> {
    let csv_path = match std::env::args().nth(1) {
        Some(r) => r,
        None => {
            return Err(Box::new(InternalError::new(
                "Usage: cargo run <path-to-input-file>",
            )))
        }
    };

    let json_path = match std::env::args().nth(2) {
        Some(r) => r,
        None => String::from("files/env.json"),
    };

    let log_path = match std::env::args().nth(3) {
        Some(r) => r,
        None => String::from("files/log_file.txt"),
    };

    let system = System::new();

    system.block_on(async {
        let logger_addr = Logger::new(&log_path).start();
        let configuration = get_envs(json_path, logger_addr.clone());
        let stats_addr = Statistics::new(configuration, logger_addr.clone()).start();
        let admin_addr = Administrator::new(stats_addr, configuration, logger_addr.clone()).start();

        match Parser::open(csv_path, admin_addr, logger_addr.clone()) {
            Ok(parser) => {
                parser.start().do_send(ReadNextLine);
                logger_addr.do_send(LoggerMessage::new_info(String::from("[Main]: Server up")));
            }
            Err(_) => {
                logger_addr.do_send(LoggerMessage::new_error(
                    "[Main]: Could not open input file".to_string(),
                ));
                System::current().stop();
            }
        }
    });

    if system.run().is_err() {
        return Err(Box::new(InternalError::new("[Main]: System could not run")));
    };

    Ok(())
}