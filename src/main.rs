use std::error::Error;
use actix::{System, Actor};
mod model;
use model::error::{AppResult, InternalError};
use model::administrator::{Administrator, NewRequest};
use model::request::Request;
use model::parser::{Parser, ReadNextLine};

fn main() -> Result<(), Box<dyn Error>> {

    let csv_path = match std::env::args().nth(1) {
        Some(r) => r,
        None => return Err(Box::new(InternalError::new("Usage: cargo run <path-to-input-file>")))
    };
    let system = System::new();
    system.block_on(async {
        let addr = Administrator::new().start();
        Parser::open(csv_path, addr).unwrap()   //ESTO ESTA MAL, MUY MAL, PERO NO SE COMO ATAJAR EL ERROR
            .start()
            .do_send(ReadNextLine);
    });

    if let Err(_) = system.run() {
        return Err(Box::new(InternalError::new("System error")))    //RRAAAAROOO
    };

    Ok(())
}