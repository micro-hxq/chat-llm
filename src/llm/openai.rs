use derive_builder::Builder;
use reqwest::blocking;
use serde::{Deserialize, Serialize};
use tokio::runtime;
use tokio::sync::mpsc::Receiver;

use crate::config::APP_CONFIG;
use crate::llm::{Llm, LlmConfig, LlmGenerationConfig, LlmMessage};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ChatCompletionMessage {
    role: String,
    content: String,
}

impl ChatCompletionMessage {
    pub fn new(role: &str, content: &str) -> Self {
        Self {
            role: role.to_string(),
            content: content.to_string(),
        }
    }
}

#[derive(Debug, Clone, Builder, Serialize, Deserialize)]
struct ChatCompletionRequest {
    model: String,
    messages: Vec<ChatCompletionMessage>,
    frequency_penalty: Option<f32>,
    max_tokens: Option<i32>,
    n: Option<i32>,
    presence_penalty: Option<f32>,
    seed: Option<i32>,
    stop: Option<Vec<String>>,
    stream: Option<bool>,
    temperature: Option<f32>,
    top_p: Option<f32>,
}

impl ChatCompletionRequest {
    pub fn new(model: &str, messages: &[&dyn LlmMessage]) -> Self {
        Self {
            model: model.to_owned(),
            messages: messages.iter().map(|&m| ChatCompletionMessage::new(m.role(), m.content())).collect::<Vec<ChatCompletionMessage>>(),
            frequency_penalty: None,
            max_tokens: None,
            n: None,
            presence_penalty: None,
            seed: None,
            stop: None,
            stream: None,
            temperature: None,
            top_p: None,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    pub prompt_tokens: i64,
    pub completion_tokens: i64,
    pub total_tokens: i64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Choice {
    pub index: i64,
    pub message: ChatCompletionMessage,
    // pub logprobs: Option<_>,
    pub finish_reason: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ChatCompletionResponse {
    pub id: String,
    pub object: String,
    pub created: i64,
    pub model: String,
    pub system_fingerprint: String,
    pub choices: Vec<Choice>,
    pub usage: Usage,
}

#[derive(Debug)]
pub struct ChatOpenAI {
    config: Option<ChatOpenAIConfig>,
    rt: runtime::Runtime,
}

impl ChatOpenAI {
    pub fn new() -> Self {
        Self {
            config: APP_CONFIG.openai.clone(),
            rt: runtime::Runtime::new().unwrap(),
        }
    }
}

impl Llm for ChatOpenAI {
    fn generate(&self, messages: &[&dyn LlmMessage]) -> anyhow::Result<String> {
        let config = self.config.as_ref().unwrap();
        let request = ChatCompletionRequestBuilder::default()
            .model(config.model().to_owned())
            .messages(messages.iter()
                .map(|&m| ChatCompletionMessage::new(m.role(), m.content()))
                .collect::<Vec<ChatCompletionMessage>>())
            .frequency_penalty(config.generation_config().frequency_penalty())
            .max_tokens(config.generation_config().max_tokens())
            .presence_penalty(config.generation_config().presence_penalty())
            .temperature(config.generation_config().temperature())
            .top_p(config.generation_config().top_p())
            .build()?;
        let response = blocking::Client::new().post(config.url())
            .header("Authorization", format!("Bearer {}", config.api_key()))
            .json(&request)
            .send()?
            .json::<ChatCompletionResponse>()?;
        Ok(response.choices[0].message.content.clone())
    }

    fn stream_generate(&self, messages: &[&dyn LlmMessage]) -> Receiver<anyhow::Result<String>> {
        let config = self.config.as_ref().unwrap();
        let request = ChatCompletionRequestBuilder::default()
            .model(config.model().to_owned())
            .messages(messages.iter()
                .map(|&m| ChatCompletionMessage::new(m.role(), m.content()))
                .collect::<Vec<ChatCompletionMessage>>())
            .frequency_penalty(config.generation_config().frequency_penalty())
            .max_tokens(config.generation_config().max_tokens())
            .presence_penalty(config.generation_config().presence_penalty())
            .temperature(config.generation_config().temperature())
            .top_p(config.generation_config().top_p())
            .stream(Some(true))
            .build().unwrap();

        unimplemented!()
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChatOpenAIConfig {
    api_key: String,
    url: Option<String>,
    model: Option<String>,
    generation_config: Option<ChatOpenAIGenerationConfig>,
}

impl LlmConfig for ChatOpenAIConfig {
    type GenerationConfig = ChatOpenAIGenerationConfig;

    fn api_key(&self) -> &str {
        self.api_key.as_str()
    }

    fn url(&self) -> &str {
        self.url.as_deref().unwrap_or("https://api.openai.com/v1/chat/completions")
    }

    fn model(&self) -> &str {
        self.model.as_deref().unwrap_or("gpt-4-0125-preview")
    }

    fn generation_config(&self) -> &Self::GenerationConfig {
        self.generation_config.as_ref().unwrap()
    }
}


#[derive(Debug, Clone, Copy, Deserialize)]
pub struct ChatOpenAIGenerationConfig {
    do_sample: Option<bool>,
    temperature: Option<f32>,
    max_tokens: Option<i32>,
    top_p: Option<f32>,
    frequency_penalty: Option<f32>,
    presence_penalty: Option<f32>,
}

impl LlmGenerationConfig for ChatOpenAIGenerationConfig {
    fn do_sample(&self) -> Option<bool> {
        self.do_sample
    }

    fn temperature(&self) -> Option<f32> {
        self.temperature
    }

    fn max_tokens(&self) -> Option<i32> {
        self.max_tokens
    }

    fn top_p(&self) -> Option<f32> {
        self.top_p
    }

    fn frequency_penalty(&self) -> Option<f32> {
        self.frequency_penalty
    }

    fn presence_penalty(&self) -> Option<f32> {
        self.presence_penalty
    }
}
