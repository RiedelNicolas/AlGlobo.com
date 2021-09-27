use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
use model::request_handler::RequestHandler;
use model::web_service_provider::WebServiceProvider;

fn process_requests(path: &str) -> AppResult<()>{

    let mut parser = Parser::open(std::path::Path::new(path))?;
    let mut handlers: Vec<RequestHandler> = Vec::new();
    //Valores hardcodeados, deberian levantarse de ENV o algo por el estilo.
    let mut web_provider = WebServiceProvider::new(1, 1);
    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos
            Some(request) => {
                //Levantar thread
                match RequestHandler::spawn(request, &mut web_provider) {
                    Ok(handler) => handlers.push(handler),
                    Err(error) => println!("{:?}", error) //Esto deberia ser un llamado a Logger.log_error
                };
            }
        }
        clean_finished(&mut handlers);
    }

    for handler in handlers {
        handler.join();
    }

    Ok(())
}

fn clean_finished(handlers: &mut Vec<RequestHandler>) {
    let mut i = 0;
    while i < handlers.len() {
        if handlers[i].has_finished() {
            let req = handlers.remove(i);
            req.join();
            //Aca se podria llamar a Statistics y pasarle la nueva info
        } else {
            i += 1;
        }
    }
}


fn main() -> Result<(), Box<dyn Error>> {
    let path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };

    process_requests(&path[..])
}
