mod api;

use api::Api;

pub async fn chat(
    prompt: String,
    model: api::request::Model,
    api_key: String,
) -> Result<String, Box<dyn std::error::Error>> {
    let api = Api::new(api_key);

    let res = api.send_chat(model, prompt).await?;

    Ok(res
        .choices
        .get(0)
        .unwrap()
        .message
        .content
        .trim()
        .to_string())
}
