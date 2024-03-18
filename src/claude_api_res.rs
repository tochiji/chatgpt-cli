use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum ClaudeEvent {
    MessageStart {
        message: Message,
    },
    ContentBlockStart {
        index: u32,
        content_block: ContentBlock,
    },
    Ping,
    ContentBlockDelta {
        index: u32,
        delta: ContentBlockDelta,
    },
    ContentBlockStop {
        index: u32,
    },
    MessageDelta {
        delta: MessageDelta,
    },
    MessageStop,
}

#[derive(Serialize, Deserialize, Debug)]
struct Usage {
    input_tokens: Option<u32>,
    output_tokens: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    id: String,
    #[serde(rename = "type")]
    message_type: String,
    role: String,
    content: Vec<String>,
    model: String,
    stop_reason: Option<String>,
    stop_sequence: Option<String>,
    usage: Option<Usage>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentBlock {
    #[serde(rename = "type")]
    block_type: String,
    text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ContentBlockDelta {
    #[serde(rename = "type")]
    pub delta_type: String,
    pub text: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MessageDelta {
    stop_reason: String,
    stop_sequence: Option<String>,
    usage: Option<Usage>,
}
