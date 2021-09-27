use std::error::Error;
mod model;
use model::parser::Parser;
use model::error::{AppResult, InternalError};
use model::request_handler::RequestHandler;
use std::collections::LinkedList;
use model::web_service_provider::WebServiceProvider;

fn process_requests(path: &str) -> AppResult<()>{

    let mut parser = Parser::open(std::path::Path::new(path))?;
    let mut handlers: LinkedList<RequestHandler> = LinkedList::new();
    //Valores hardcodeados, deberian levantarse de ENV o algo por el estilo.
    let mut web_provider = WebServiceProvider::new(1, 1);
    loop {
        match parser.parse_request()? {
            None => break,  //Finalizamos
            Some(request) => {
                //Levantar thread
                match RequestHandler::spawn(request, &mut web_provider) {
                    Ok(handler) => handlers.push_back(handler),
                    Err(error) => println!("{:?}", error) //Esto deberia ser un llamado a Logger.log_error
                };
            }
        }
    }

    for handler in handlers {
        if let Err(error) = handler.join() {
            println!("{:?}", error)
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


/*

SEMAFOROS



fn main() {
    // Crear un semaforo que represente 1 recurso
    let sem = Semaphore::new(0);

    // Adquirir el recurso
    sem.acquire();

    println!("Semaforo adquirido!");

    // Liberarlo
    sem.release();

    println!("Semaforo liberado!");
}


NOTA: Para implementar la logica has_finished() -> join() con los threads
se puede usar un atomic. Ej: 

use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

fn main() {
    let val = Arc::new(AtomicUsize::new(5));

    for _ in 0..10 {
        let val = Arc::clone(&val);

        thread::spawn(move || {
            let v = val.fetch_add(1, Ordering::SeqCst);
            println!("{:?}", v);
        });
    }
}


*/