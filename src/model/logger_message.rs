use super::message_type::MessageType;
extern crate chrono;
use actix::prelude::*;
use chrono::{DateTime, Local};

/// Mensaje utilizado para enviar los mensajes que debe imprimir el logger.
#[derive(Message)]
#[rtype(result = "")]
pub struct LoggerMessage {
    message_type: MessageType,
    text: String,
}

impl LoggerMessage {
    /// Constructor que genera un mensaje de tipo *"error"*
    /// El string recibido por parametro es la descripcion del mismo
    pub fn new_error(in_text: String) -> LoggerMessage {
        LoggerMessage {
            text: in_text,
            message_type: MessageType::Error,
        }
    }
    /// Constructor que genera un mensaje de tipo *"info"*
    /// El string recibido por parametro es la descripcion del mismo
    pub fn new_info(in_text: String) -> LoggerMessage {
        LoggerMessage {
            text: in_text,
            message_type: MessageType::Info,
        }
    }

    /// Constructor que genera un mensaje de tipo *"warning"*
    /// El string recibido por parametro es la descripcion del mismo
    pub fn new_warning(in_text: String) -> LoggerMessage {
        LoggerMessage {
            text: in_text,
            message_type: MessageType::Warning,
        }
    }

    /// Devuelve el mensaje formateado como string con su timestamp correspondiente
    pub fn generate_string(&self) -> String {
        let aux: String;
        let now: DateTime<Local> = Local::now();

        match self.message_type {
            MessageType::Info => aux = "INF".to_string(),
            MessageType::Error => aux = "ERR".to_string(),
            MessageType::Warning => aux = "WAR".to_string(),
        }
        format!("{} - {} - {}", aux, now.format("%e %b %Y %T"), self.text)
    }
}
