pub enum IncompleteMessageType {
    Text,
    Binary,
}

pub struct IncompleteMessage {
    message_type: IncompleteMessageType,
    payload: Vec<u8>,
}

impl IncompleteMessage {
    pub fn new(message_type: IncompleteMessageType) -> Self {
        IncompleteMessage {
            message_type,
            payload: vec![],
        }
    }
}

impl IncompleteMessage {
    pub fn extend<T: AsRef<[u8]>>(&mut self, payload: T) {
        self.payload.extend(payload.as_ref());
    }

    pub fn complete(self) -> Message {
        match self.message_type {
            IncompleteMessageType::Text => Message::Text(String::from_utf8(self.payload).unwrap()),
            IncompleteMessageType::Binary => Message::Binary(self.payload),
        }
    }
}

pub enum Message {
    Text(String),
    Binary(Vec<u8>),
    Ping(Vec<u8>),
    Pong(Vec<u8>),
    Close,
}
