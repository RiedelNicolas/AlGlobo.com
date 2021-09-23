use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
use model::request_handler::RequestHandler;
use std::collections::LinkedList;

fn process_requests(path: &str) -> AppResult<()>{

    let mut parser = Parser::open(std::path::Path::new(path))?;
    let mut handlers: LinkedList<RequestHandler> = LinkedList::new();
    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos
            Some(request) => {
                //Levantar thread
                match RequestHandler::spawn(request) {
                    Ok(handler) => handlers.push_back(handler),
                    Err(error) => println!("{:?}", error) //Esto deberia ser un llamado a Logger.log_error
                };
            }
        }
    }

    for handler in handlers{
        match handler.join() {
            Err(error) => println!("{:?}", error),
            Ok(_) => {}
        }
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };

    process_requests(&path[..])
}
