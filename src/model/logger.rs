use std::sync::mpsc::Sender;
use super::message::Message;

#[derive(Debug)]
#[derive(Clone)]
pub struct Logger {
    tx: Sender<Message> 
}

impl Logger {
    
    pub fn new (in_tx : &Sender<Message>) ->Logger {
        Logger {
            tx:in_tx.clone()
        }
    }


    // For con x intentos.

    pub fn log_error ( &self, text: String) {
        let _ = self.tx.send(Message::new_error(text));
    }

    pub fn log_warning (&self, text: String) {
        let _ = self.tx.send(Message::new_warning(text));
    }

    pub fn log_info (&self, text: String) {
        let _ = self.tx.send(Message::new_info(text ));
    }

    pub fn close (&self) {
        let _ = self.tx.send(Message::close());
    }
}