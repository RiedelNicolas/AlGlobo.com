use super::message_type::MessageType;
extern crate chrono;
use chrono::{DateTime, Local};

/// Tipo de dato utilizado para encapsular los mensajes que envia la clase logger.
pub struct Message {
    message_type : MessageType,
    text:String
}

impl Message {

    /// Instancia un mensaje de tipo error
    pub fn new_error(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Error
        }
    }

    /// Instancia un mensaje de tipo Info
    pub fn new_info(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Info
        }
    }

    /// Instancia un mensaje de tipo Alerta
    pub fn new_warning(in_text:String) -> Message {
        Message {
            text: in_text,
            message_type: MessageType::Warning
        }
    }

    /// Mensaje utilizado para cerrar la comunicación.
    pub fn close() -> Message {
        Message {
            text: String::from("__CloseChannelSignal__"),
            message_type: MessageType::Close
        }
    }

    /// Devuelve el estado interno del String formateado como string.
    pub fn generate_string (&self)->String {
    
        let aux : String;
        let now: DateTime<Local> = Local::now();

        match self.message_type {
            MessageType::Info => aux = "INF".to_string(),
            MessageType::Error => aux = "ERR".to_string(),
            MessageType::Warning => aux = "WAR".to_string(),
            MessageType::Close => aux = "".to_string(),
        }
        format!("{} - {} - {}", aux, now.format("%e %b %Y %T") ,self.text)
    }

    /// Get a reference to the message's message type.
    pub fn message_type(&self) -> &MessageType {
        &self.message_type
    }

}