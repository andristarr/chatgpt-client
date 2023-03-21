pub mod request;

use request::ChatRequest;
use request::ChatResponse;
use request::MessageRequest;

use reqwest::header::{AUTHORIZATION, CONTENT_TYPE};

const API_URL: &'static str = "https://api.openai.com/v1/chat/completions";

pub struct Api {
    client: reqwest::Client,
    api_key: String,
}

impl Api {
    pub fn new(api_key: String) -> Api {
        Api {
            client: reqwest::Client::new(),
            api_key: api_key,
        }
    }

    pub async fn send_chat(
        &self,
        model: request::Model,
        message: String,
    ) -> Result<ChatResponse, Box<dyn std::error::Error>> {
        let chat_message: ChatRequest = ChatRequest {
            model: model,
            messages: vec![MessageRequest {
                role: "user".to_string(),
                content: message,
            }],
        };

        let response = self
            .client
            .post(API_URL)
            .body(serde_json::to_string(&chat_message)?)
            .header(AUTHORIZATION, format!("Bearer {}", self.api_key))
            .header(CONTENT_TYPE, "application/json")
            .send()
            .await;

        let res = response?.text().await;

        let qwe = res?;

        let ret: ChatResponse = serde_json::from_str(&qwe)?;

        Ok(ret)
    }
}
