enum MessageType {
    Error,
    Warning,
    Info
}

pub struct Message {
    type:MessageType,
    text:String
}