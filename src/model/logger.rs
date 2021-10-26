use super::message::Message;
use std::sync::mpsc::Sender;

/// Instancia de un logger, es utilizado para guardar mensajes en el logFile.
#[derive(Debug, Clone)]
pub struct Logger {
    tx: Sender<Message>,
}

/// Instancia de logger, la misma debe recibir por parametro el trasmiter
/// por el cual debe escribir.
impl Logger {
    pub fn new(in_tx: &Sender<Message>) -> Logger {
        Logger { tx: in_tx.clone() }
    }

    /// Envia un mensaje de Error al logHandler con la descripcion del string recibido por parametros.
    pub fn log_error(&self, text: String) {
        let _ = self.tx.send(Message::new_error(text));
    }

    /// Envia un mensaje de Warning al logHandler con la descripcion del string recibido por parametros.
    pub fn log_warning(&self, text: String) {
        let _ = self.tx.send(Message::new_warning(text));
    }

    /// Envia un mensaje de Info al logHandler con la descripcion del string recibido por parametros.
    pub fn log_info(&self, text: String) {
        let _ = self.tx.send(Message::new_info(text));
    }

    // Finaliza la comunicacion.
    pub fn close(&self) {
        let _ = self.tx.send(Message::close());
    }
}
