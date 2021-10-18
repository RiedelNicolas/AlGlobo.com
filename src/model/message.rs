pub struct Message {
    type:MessageType,
    text:String
}

enum MessageType {
    Error,
    Warning,
    Info
}
