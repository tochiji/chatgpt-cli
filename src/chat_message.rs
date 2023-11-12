use std::fmt;

#[derive(Debug, Default)]
/// APIに送るメッセージ履歴
///
/// ChatGPTへの初期プロンプトおよびユーザーからの質問、ChatGPTからの回答が、順次格納される。
/// この履歴を元に、ChatGPTは次の回答を生成する。
pub struct MessageHistory {
    pub messages: Vec<Message>,
}

impl MessageHistory {
    pub fn push(&mut self, role: Role, content: &str) {
        let m = Message::new(role, content);
        self.messages.push(m);
    }
}

#[derive(Debug, Clone)]
pub enum Role {
    User,
    System,
    Assistant,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Role::User => write!(f, "user"),
            Role::System => write!(f, "system"),
            Role::Assistant => write!(f, "assistant"),
        }
    }
}

#[derive(Debug)]
pub struct Message {
    pub role: Role,
    pub content: String,
}

impl Message {
    pub fn new(role: Role, content: &str) -> Self {
        Self {
            role,
            content: content.to_owned(),
        }
    }
}
