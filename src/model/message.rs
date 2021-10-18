use super::message_type::MessageType;
extern crate chrono;
use chrono::{DateTime, Utc};

pub struct Message {
    message_type : MessageType,
    text:String
}

impl Message {

    //creo los 3 constructores 
    pub fn new_error(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Error
        }
    }

    pub fn new_info(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Info
        }
    }

    pub fn new_warning(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Warning
        }
    }

    pub fn generate_string (&self)->String {
    
        let aux : String;
        let now: DateTime<Utc> = Utc::now();

        match self.message_type {
            MessageType::Info => aux = "INF".to_string(),
            MessageType::Error => aux = "ERR".to_string(),
            MessageType::Warning => aux = "WAR".to_string(),
        }
        format!("{}-{}-{}", aux, now.format("%e %B %Y %T") ,self.text)
    }
}