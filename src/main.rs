use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
//use model::request::Request;


fn procesar(path: &str) -> AppResult<()>{

    let mut parser = Parser::open(std::path::Path::new(path))?;

    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos

            Some(request) => {
                //Levantar thread
                println!("{:?}", request)
            }
        }
    }

    Ok(())
}


fn main() -> Result<(), Box<dyn Error>> {
    let path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };

    procesar(&path[..])
}
