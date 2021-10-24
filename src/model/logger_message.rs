use super::message_type::MessageType;
extern crate chrono;
use chrono::{DateTime, Local};
use actix::prelude::*;

#[derive(Message)]
#[rtype(result = "")]
pub struct LoggerMessage {
    message_type : MessageType,
    text:String
}

impl LoggerMessage {

    //creo los 3 constructores 
    pub fn new_error(in_text:String) -> LoggerMessage {
        Message {
            text: in_text,
            message_type: MessageType::Error
        }
    }

    pub fn new_info(in_text:String) -> LoggerMessage {
        Message {
            text: in_text,
            message_type: MessageType::Info
        }
    }

    pub fn new_warning(in_text:String) -> LoggerMessage {
        Message {
            text: in_text,
            message_type: MessageType::Warning
        }
    }

    pub fn generate_string (&self)->String {
    
        let aux : String;
        let now: DateTime<Local> = Local::now();

        match self.message_type {
            MessageType::Info => aux = "INF".to_string(),
            MessageType::Error => aux = "ERR".to_string(),
            MessageType::Warning => aux = "WAR".to_string(),
        }
        format!("{} - {} - {}", aux, now.format("%e %b %Y %T") ,self.text)
    }
}