use super::message;
use super::logger::Logger;
use super::message_type::MessageType;
use std::io::Write;
use std::fs::File;
use std::fs;
use std::thread::{self, JoinHandle};

pub struct LogHandler {
    tx: std::sync::mpsc::Sender<message::Message>,
    logger: Option<JoinHandle<()>>,
}

impl LogHandler {
    pub fn new( path : &str ) -> LogHandler {
        let (sender, receiver) = std::sync::mpsc::channel::<message::Message>();
        let log_file = fs::OpenOptions::new()
                                            .write(true)
                                            .append(true)
                                            .create(true)
                                            .open(path)
                                            .unwrap();
        
        LogHandler {
            tx: sender,
            logger: Some(thread::spawn( move || LogHandler::print_received(receiver, log_file)))
        }
    }
    
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

    // returns an instances of a sender.
    pub fn get_transmitter(&self) -> Logger {
        Logger::new( &self.tx.clone() )
    }

    pub fn join(self) {
        if let Some(logger) = self.logger { let _ = logger.join(); }
    }
}