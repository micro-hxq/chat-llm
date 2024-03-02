use tokio::sync::mpsc::Receiver;

pub trait LlmMessage {
    fn role(&self) -> String;

    fn content(&self) -> String;
}

pub trait LlmConfig {
    fn api_key(&self) -> &str;

    fn url(&self) -> &str;

    fn model(&self) -> &str;

    fn generation_config(&self) -> &dyn LlmGenerationConfig;
}

pub trait LlmGenerationConfig {
    fn do_sample(&self) -> bool;

    fn temperature(&self) -> f32;

    fn max_tokens(&self) -> i32;

    fn top_p(&self) -> f32;

    fn frequency_penalty(&self) -> f32;

    fn presence_penalty(&self) -> f32;
}

pub trait Llm {
    fn generate(&self, messages: &[&dyn LlmMessage]) -> String;

    fn stream_generate(&self, messages: &[&dyn LlmMessage]) -> Receiver<anyhow::Result<String>>;
}
