use super::message;
use super::logger::Logger;
use super::message_type::MessageType;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::thread::{self, JoinHandle};

/// Representa un log system.
pub struct LogHandler {
    tx: std::sync::mpsc::Sender<message::Message>,
    logger: Option<JoinHandle<()>>,
}

impl LogHandler {
    /// Genera una instancia de la clase.
    /// Recibe un path donde dicho archivo debe ser construido.
    pub fn new( path : &str ) -> LogHandler {
        let (sender, receiver) = std::sync::mpsc::channel::<message::Message>();
        let log_file = fs::OpenOptions::new()
                                            .write(true)
                                            .append(true)
                                            .create(true)
                                            .open(path)
                                            .expect("No se pudo abrir el log_file");
        
        LogHandler {
            tx: sender,
            logger: Some(thread::spawn( move || LogHandler::print_received(receiver, log_file)))
        }
    }
    /// Imprime todos los mensajes que estan esperando en la cola.
    /// **Peligro** : Es un metodo bloqueante
    fn print_received(rx: std::sync::mpsc::Receiver<message::Message>, mut log_file: File) {
        loop {
            match rx.recv() {
                Ok(r) => {
                    match *r.message_type() {
                        MessageType::Close => return,
                        _ => {
                            match writeln!(log_file, "{}", r.generate_string() ) {
                                Ok(v) => v,
                                Err(e) => println!("Error found trying to write the logfile : {}", e)
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("{}",e);
                    return; //posiblemente cerraron el channel, cortamos el loop.
                }
            }
        }
    }

    /// Devuelve una instancia de un Logger.
    /// Este es usado para enviar los mensajes que se desean imprimir en el log_file
    pub fn get_transmitter(&self) -> Logger {
        Logger::new( &self.tx.clone() )
    }

    /// Bloqueante, espera a que se impriman todos los mensajes recibidos. 
    pub fn join(self) {
        if let Some(logger) = self.logger { let _ = logger.join(); }
    }
    
}