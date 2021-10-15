use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
use model::request_handler::RequestHandler;
use model::web_service_provider::WebServiceProvider;
use model::statistics::Statistics;
use model::env;

fn process_requests(csv_path: &str, json_path: &str) -> AppResult<()>{

    let envs = env::get_envs(json_path);
    let mut parser = Parser::open(std::path::Path::new(csv_path))?;
    let mut handlers: Vec<RequestHandler> = Vec::new();
    let mut web_provider = WebServiceProvider::new(envs.airline_limit, envs.hotel_limit);
    let mut statistics = Statistics::new();

    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos
            Some(request) => {
                //Levantar thread
                match RequestHandler::spawn(request, &mut web_provider, &envs) {
                    Ok(handler) => handlers.push(handler),
                    Err(error) => println!("{:?}", error) //Esto deberia ser un llamado a Logger.log_error
                };
            }
        }
       clean_finished(&mut handlers, &mut statistics);
    }

    for handler in handlers {
        let datos = handler.join();
        statistics.update(datos);
    }

    statistics.log_data();
    Ok(())
}

fn clean_finished(handlers: &mut Vec<RequestHandler>, statistics: &mut Statistics) {
    let mut i = 0;
    while i < handlers.len() {
        if handlers[i].has_finished() {
            let req = handlers.remove(i);
            let datos = req.join();
            statistics.update(datos);
            statistics.log_data();
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

    process_requests(&csv_path[..], &json_path[..])
}
