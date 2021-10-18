use super::message;
use std::io::Write;
use std::fs::File;
use std::fs;


pub struct Logger {
    rx: std::sync::mpsc::Receiver<message::Message>,
    tx: std::sync::mpsc::Sender<message::Message>,
    log_file:File
}

impl Logger {
    pub fn new() -> Logger {
        let (sender, receiver) = std::sync::mpsc::channel::<message::Message>();
        Logger {
            rx: receiver,
            tx: sender,
            log_file : fs::OpenOptions::new()
                .write(true)
                .append(true)
                .create(true)
                .open("files/log_file.txt")
                .unwrap()
        }
    }
    pub fn print_received(&mut self) {
        let received = self.rx.recv().unwrap();
        println!("{}", received.generate_string() );
        let r = write!(self.log_file, "{}\n", &received.generate_string() );
        match r {
            Ok(v) => v,
            Err(e) => println!("Error found trying to write the logfile : {}",e)
        }
    }
    // returns an instances of a channel sender.
    pub fn get_transmitter(&self) -> std::sync::mpsc::Sender<message::Message> {
        self.tx.clone()
    }
}