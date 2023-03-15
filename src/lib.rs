mod api;

use api::Api;
use reqwest::header::AUTHORIZATION;
use reqwest::header::CONTENT_TYPE;
use std::collections::HashMap;

pub async fn chat() -> Result<String, Box<dyn std::error::Error>> {
    let api = Api::new("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".to_string());

    let res = api
        .send_chat("What is the OpenAI mission?".to_string())
        .await?;

    Ok(res)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! aw {
        ($e:expr) => {
            tokio_test::block_on($e)
        };
    }

    #[test]
    fn it_works_async() {
        let ret = aw!(chat()).unwrap();
        assert_eq!(ret, "4");
    }
}
