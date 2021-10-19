use super::message;
use super::logger::Logger;
use std::io::Write;
use std::fs::File;
use std::fs;


pub struct LogHandler {
    rx: std::sync::mpsc::Receiver<message::Message>,
    tx: std::sync::mpsc::Sender<message::Message>,
    log_file:File
}

impl LogHandler {
    pub fn new( path : &str ) -> LogHandler {
        let (sender, receiver) = std::sync::mpsc::channel::<message::Message>();
        LogHandler {
            rx: receiver,
            tx: sender,
            log_file : fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open(path)
                .unwrap()
        }
    }
    
    pub fn print_received(&mut self) {
        loop {
            match self.rx.recv() {
                Ok(r) => {
                    match write!(self.log_file, "{}\n", r.generate_string() ) {
                        Ok(v) => v,
                        Err(e) => println!("Error found trying to write the logfile : {}", e)
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
}