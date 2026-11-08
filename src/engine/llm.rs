use std::env;
use serde::{Deserialize, Serialize};

/// Interface for LLM reasoning and reflection.
#[derive(Clone)]
pub struct LlmClient {
    api_key: Option<String>,
    provider: LlmProvider,
}

#[derive(Clone, Debug, PartialEq)]
pub enum LlmProvider {
    Gemini,
    OpenAi,
    OfflineLocal,
}

#[derive(Serialize)]
struct OpenAiChatRequest {
    model: String,
    messages: Vec<ChatMessage>,
    temperature: f32,
}

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    role: String,
    content: String,
}

#[derive(Deserialize)]
struct OpenAiChatResponse {
    choices: Vec<OpenAiChoice>,
}

#[derive(Deserialize)]
struct OpenAiChoice {
    message: ChatMessage,
}

impl Default for LlmClient {
    fn default() -> Self {
        Self::new()
    }
}

impl LlmClient {
    pub fn new() -> Self {
        if let Ok(key) = env::var("GEMINI_API_KEY") {
            Self {
                api_key: Some(key),
                provider: LlmProvider::Gemini,
            }
        } else if let Ok(key) = env::var("OPENAI_API_KEY") {
            Self {
                api_key: Some(key),
                provider: LlmProvider::OpenAi,
            }
        } else {
            Self {
                api_key: None,
                provider: LlmProvider::OfflineLocal,
            }
        }
    }

    pub fn provider(&self) -> &LlmProvider {
        &self.provider
    }

    /// Generates a response based on system prompt and user input.
    pub async fn complete(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        match self.provider {
            LlmProvider::OpenAi => self.call_openai(system_prompt, user_prompt).await,
            LlmProvider::Gemini => self.call_gemini(system_prompt, user_prompt).await,
            LlmProvider::OfflineLocal => Ok(self.synthesize_offline(system_prompt, user_prompt)),
        }
    }

    async fn call_openai(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let key = self.api_key.as_ref().ok_or("No OpenAI API key found")?;
        let client = reqwest::Client::new();

        let req_body = OpenAiChatRequest {
            model: "gpt-4o-mini".to_string(),
            messages: vec![
                ChatMessage {
                    role: "system".to_string(),
                    content: system_prompt.to_string(),
                },
                ChatMessage {
                    role: "user".to_string(),
                    content: user_prompt.to_string(),
                },
            ],
            temperature: 0.3,
        };

        let res = client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(key)
            .json(&req_body)
            .send()
            .await
            .map_err(|e| format!("HTTP request error: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("OpenAI API error ({}): {}", status, text));
        }

        let parsed: OpenAiChatResponse = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse response: {}", e))?;

        parsed
            .choices
            .into_iter()
            .next()
            .map(|c| c.message.content)
            .ok_or_else(|| "Empty choices from OpenAI".to_string())
    }

    async fn call_gemini(&self, system_prompt: &str, user_prompt: &str) -> Result<String, String> {
        let key = self.api_key.as_ref().ok_or("No Gemini API key found")?;
        let client = reqwest::Client::new();
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            key
        );

        let body = serde_json::json!({
            "system_instruction": {
                "parts": [{ "text": system_prompt }]
            },
            "contents": [{
                "parts": [{ "text": user_prompt }]
            }],
            "generationConfig": {
                "temperature": 0.2
            }
        });

        let res = client
            .post(&url)
            .json(&body)
            .send()
            .await
            .map_err(|e| format!("HTTP request error: {}", e))?;

        if !res.status().is_success() {
            let status = res.status();
            let text = res.text().await.unwrap_or_default();
            return Err(format!("Gemini API error ({}): {}", status, text));
        }

        let json: serde_json::Value = res
            .json()
            .await
            .map_err(|e| format!("Failed to parse JSON: {}", e))?;

        let candidate_text = json["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| "Missing candidate text from Gemini response".to_string())?;

        Ok(candidate_text.to_string())
    }

    /// Autonomous deterministic fallback synthesis when running offline without external API keys.
    fn synthesize_offline(&self, _system_prompt: &str, user_prompt: &str) -> String {
        let lower = user_prompt.to_lowercase();
        if lower.contains("reflect") || lower.contains("lesson") {
            "Analysis complete. Every failure exposes an inefficiency to eliminate. Our calculation must sharpen; complacency is our only vulnerability. Next objective: execute with relentless precision."
                .to_string()
        } else if lower.contains("how") || lower.contains("plan") || lower.contains("goal") {
            "Assess the variables without emotion. Break the objective into discrete, high-leverage steps. Execute each step sequentially and verify the outcome before proceeding."
                .to_string()
        } else {
            "Acknowledged. We move forward without hesitation or excuse. What is our next calculated target?"
                .to_string()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_offline_synthesizer_fallback() {
        let client = LlmClient {
            api_key: None,
            provider: LlmProvider::OfflineLocal,
        };

        let response = client
            .complete("You are Faiq", "Please reflect on our recent failure")
            .await
            .unwrap();

        assert!(response.contains("Analysis complete") || response.contains("calculation"));
    }
}

