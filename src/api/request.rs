use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<MessageRequest>,
}

#[derive(Serialize, Debug)]
pub struct MessageRequest {
    pub role: String,
    pub content: String,
}

#[derive(Deserialize, Debug)]
pub struct ChatResponse {
    id: String,
    choices: Vec<ChatMessage>,
}

#[derive(Deserialize, Debug)]
pub struct ChatMessage {
    content: String,
}
