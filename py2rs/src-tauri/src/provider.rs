use crate::anthropic;
use serde::Deserialize;
use serde_json::{json, Value};

#[derive(Deserialize, Clone, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Kind {
    OpenAiCompatible,
    Anthropic,
    LlamaCpp,
    OpenRouter,
    Groq,
    Azure,
    Custom,
}

impl Kind {
    pub fn to_string(&self) -> String {
        match self {
            Kind::OpenAiCompatible => "openaicompatible".to_string(),
            Kind::Anthropic => "anthropic".to_string(),
            Kind::LlamaCpp => "llamacpp".to_string(),
            Kind::OpenRouter => "openrouter".to_string(),
            Kind::Groq => "groq".to_string(),
            Kind::Azure => "azure".to_string(),
            Kind::Custom => "custom".to_string(),
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Provider {
    pub kind: Kind,
    pub base_url: String,
    pub api_key: Option<String>,
    pub model: String,
    pub custom_body: Option<String>,
    pub custom_response_path: Option<String>,
    pub custom_headers: Option<String>,
}

impl Provider {
    pub async fn chat(&self, system: &str, user: &str) -> anyhow::Result<String> {
        match self.kind {
            Kind::Anthropic => {
                let key = self.api_key.as_deref().unwrap_or("");
                anthropic::chat(key, &self.model, system, user).await
            }
            Kind::LlamaCpp => {
                let url = format!("{}/completion", self.base_url.trim_end_matches('/'));
                let prompt = format!("System: {}\n\nUser: {}\n\nAssistant:", system, user);
                let body = json!({
                    "prompt": prompt,
                    "temperature": 0.7,
                    "top_k": 40,
                    "top_p": 0.9,
                    "n_predict": 4096,
                    "stop": ["User:", "\n\n"]
                });
                let v: Value = reqwest::Client::new()
                    .post(url)
                    .json(&body)
                    .send()
                    .await?
                    .error_for_status()?
                    .json()
                    .await?;
                Ok(v["content"].as_str().unwrap_or("").to_string())
            }
            Kind::Custom => {
                let body_template = self.custom_body.as_deref().unwrap_or("");
                let response_path = self.custom_response_path.as_deref().unwrap_or("choices[0].message.content");

                let body_str = body_template
                    .replace("${system}", system)
                    .replace("${user}", user)
                    .replace("${model}", &self.model);

                let body: Value = serde_json::from_str(&body_str)
                    .unwrap_or_else(|_| json!({"prompt": format!("{}\n\n{}", system, user)}));

                let mut req = reqwest::Client::new()
                    .post(&self.base_url)
                    .header("Content-Type", "application/json")
                    .json(&body);

                if let Some(k) = self.api_key.as_ref().filter(|k| !k.is_empty()) {
                    req = req.bearer_auth(k);
                }

                if let Some(headers_str) = &self.custom_headers {
                    if let Ok(headers_json) = serde_json::from_str::<serde_json::Map<String, Value>>(headers_str) {
                        for (key, val) in headers_json {
                            if let Some(s) = val.as_str() {
                                req = req.header(&key, s);
                            }
                        }
                    }
                }

                let v: Value = req.send().await?.error_for_status()?.json().await?;

                let parts: Vec<&str> = response_path.split('.').collect();
                let mut current = &v;
                for part in parts {
                    if part.contains('[') && part.contains(']') {
                        let array_name = &part[..part.find('[').unwrap()];
                        let idx_str = &part[part.find('[').unwrap()+1..part.find(']').unwrap()];
                        let idx: usize = idx_str.parse().unwrap_or(0);
                        current = &current[array_name][idx];
                    } else {
                        current = &current[part];
                    }
                }

                Ok(current.as_str().unwrap_or("").to_string())
            }
            Kind::OpenAiCompatible | Kind::OpenRouter | Kind::Groq | Kind::Azure => {
                let url = format!("{}/chat/completions", self.base_url.trim_end_matches('/'));
                let mut req = reqwest::Client::new().post(url).json(&json!({
                    "model": self.model,
                    "messages": [
                        { "role": "system", "content": system },
                        { "role": "user", "content": user }
                    ]
                }));
                if let Some(k) = self.api_key.as_ref().filter(|k| !k.is_empty()) {
                    req = req.bearer_auth(k);
                }
                let v: Value = req.send().await?.error_for_status()?.json().await?;
                Ok(v["choices"][0]["message"]["content"].as_str().unwrap_or("").to_string())
            }
        }
    }
}
