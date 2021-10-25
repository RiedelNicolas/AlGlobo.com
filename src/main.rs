use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
use model::request_handler::RequestHandler;
use model::web_service_provider::WebServiceProvider;
use model::statistics::Statistics;
use model::env;
use model::log_handler::LogHandler;
use std::sync::{Arc, Mutex};

fn process_requests(csv_path: &str, json_path: &str, log_path : &str) -> AppResult<()>{

    let log = LogHandler::new(log_path);
    let envs = env::get_envs(json_path, log.get_transmitter() );
    let mut parser = Parser::open(std::path::Path::new(csv_path), log.get_transmitter() )?;
    let mut web_provider = WebServiceProvider::new(envs.airline_limit, envs.hotel_limit, log.get_transmitter() );
    let statistics = Arc::new(Mutex::new(Statistics::new( log.get_transmitter() )));
    let mut handlers: Vec<RequestHandler> = Vec::new();
    
    let logger = log.get_transmitter();
    
    logger.log_info(String::from("[Main] Server up") );

    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos
            Some(request) => {
                let stats_clone = statistics.clone();
                //Levantar thread
                match RequestHandler::spawn(request, &mut web_provider, envs,
                                        logger.clone(), stats_clone) {
                    Ok(handler) => handlers.push(handler),
                    Err(error) => {
                        logger.log_error(format!("{:?}", error));
                    }
                };
            }
        }
       clean_finished(&mut handlers);
    }

    for handler in handlers {
        let _ = handler.join();
    }

    match statistics.lock() {
        Ok(stats) => {
            stats.log_data();
        },
        Err(_) => {
            logger.log_error(String::from("Fatal error: Poisoned Lock"));
        }
    }

    logger.log_info(String::from("[Main] Server down") );
    logger.close();

    log.join();
    Ok(())
}

fn clean_finished(handlers: &mut Vec<RequestHandler>) {
    let mut i = 0;
    while i < handlers.len() {
        if handlers[i].has_finished() {
            let req = handlers.remove(i);
            let _ = req.join();
        } else {
            i += 1;
        }
    }
}


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

    process_requests(&csv_path[..], &json_path[..], &log_path[..])
}