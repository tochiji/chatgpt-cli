pub struct MessageHistory {
    pub messages: Vec<Message>,
}

impl MessageHistory {
    pub fn new() -> Self {
        Self { messages: vec![] }
    }

    pub fn push(&mut self, role: &str, content: &str) {
        self.messages.push(Message::new(role, content));
    }
}

#[derive(Debug)]
pub struct Message {
    pub role: String,
    pub content: String,
}

impl Message {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}
