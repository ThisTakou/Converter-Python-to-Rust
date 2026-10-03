use serde_json::{json, Value};

pub async fn chat(api_key: &str, model: &str, system: &str, user: &str) -> anyhow::Result<String> {
    let v: Value = reqwest::Client::new()
        .post("https://api.anthropic.com/v1/messages")
        .header("x-api-key", api_key)
        .header("anthropic-version", "2023-06-01")
        .json(&json!({
            "model": model,
            "max_tokens": 4096,
            "system": system,
            "messages": [{ "role": "user", "content": user }]
        }))
        .send()
        .await?
        .error_for_status()?
        .json()
        .await?;
    Ok(v["content"][0]["text"].as_str().unwrap_or("").to_string())
}
