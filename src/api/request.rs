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
    pub id: String,
    pub choices: Vec<ChatChoice>,
}

#[derive(Deserialize, Debug)]
pub struct ChatChoice {
    pub message: ChatMessage,
}

#[derive(Deserialize, Debug)]
pub struct ChatMessage {
    pub content: String,
}

#[derive(Serialize, Default, Debug)]
pub enum Model {
    #[default]
    #[serde(rename = "gpt-3.5-turbo")]
    GPT35,
    #[serde(rename = "gpt-4")]
    GPT4,
}
