use std::error::Error;
mod model;
use model::env;
use model::error::{AppResult, InternalError};
use model::log_handler::LogHandler;
use model::parser::Parser;
use model::request_handler::RequestHandler;
use model::statistics::Statistics;
use model::web_service_provider::WebServiceProvider;
use std::sync::{Arc, Mutex};

/// Encapsula el flujo principal del programa.
/// Encargada de instanciar las distintas entidades que componen el programa y relacionarlas.
/// Recibe los argumentos necesarios para iniciar el mismo  :
/// * Ruta valida a un archivo csv de donde debe leer las solicitudes. (El archivo debe existir y estar correctamente cargado)
/// * Ruta valida a archivo .json con la configuracion a utilizar. (En caso de que este no exista se utilizara la configuracion por defecto)
/// * Ruta valida a un log_file, en caso de que este archivo no exista se creara uno en el path aportado.  
fn process_requests(csv_path: &str, json_path: &str, log_path: &str) -> AppResult<()> {
    let log = LogHandler::new(log_path);
    let envs = env::get_envs(json_path, log.get_transmitter());
    let statistics = Arc::new(Mutex::new(Statistics::new(log.get_transmitter(), envs)));
    let mut parser = Parser::open(std::path::Path::new(csv_path), log.get_transmitter())?;
    let mut web_provider =
        WebServiceProvider::new(envs.airline_limit, envs.hotel_limit, log.get_transmitter());
    let mut handlers: Vec<RequestHandler> = Vec::new();

    let logger = log.get_transmitter();

    logger.log_info(String::from("[Main] Server up"));

    loop {
        match parser.parse_request()? {
            None => break, // Finalizamos
            Some(request) => {
                let stats_clone = statistics.clone();

                //Levantar thread
                match RequestHandler::spawn(
                    request,
                    &mut web_provider,
                    envs,
                    logger.clone(),
                    stats_clone,
                ) {
                    Ok(handler) => handlers.push(handler),
                    Err(error) => {
                        logger.log_error(format!("[Main] {:?}", error));
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
        }
        Err(_) => {
            logger.log_error(String::from(
                "[Main] Fatal error at statistics log_data: Poisoned Lock",
            ));
        }
    }

    logger.log_info(String::from("[Main] Server down"));
    logger.close();

    log.join();
    Ok(())
}

/// Sincroniza la ejecucion del programa con los hilos generados.
/// Para poder finalizar la ejecucion de forma ordenada
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

    process_requests(&csv_path[..], &json_path[..], &log_path[..])
}
