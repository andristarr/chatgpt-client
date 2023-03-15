use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug)]
pub struct ChatRequest {
    pub model: Model,
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

#[derive(Serialize, Default, Debug)]
pub enum Model {
    #[default]
    #[serde(rename = "gpt-3.5-turbo")]
    GTP35,
    #[serde(rename = "gpt-4")]
    GTP4,
}
