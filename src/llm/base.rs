use tokio::sync::mpsc::Receiver;

pub trait LlmMessage {
    fn role(&self) -> &str;

    fn content(&self) -> &str;
}

pub trait LlmConfig {
    type GenerationConfig: LlmGenerationConfig;

    fn api_key(&self) -> &str;

    fn url(&self) -> &str;

    fn model(&self) -> &str;

    fn generation_config(&self) -> &Self::GenerationConfig;
}

pub trait LlmGenerationConfig {
    fn do_sample(&self) -> Option<bool>;

    fn temperature(&self) -> Option<f32>;

    fn max_tokens(&self) -> Option<i32>;

    fn top_p(&self) -> Option<f32>;

    fn frequency_penalty(&self) -> Option<f32>;

    fn presence_penalty(&self) -> Option<f32>;
}

pub trait Llm {
    fn generate(&self, messages: &[&dyn LlmMessage]) -> anyhow::Result<String>;

    fn stream_generate(&self, messages: &[&dyn LlmMessage]) -> Receiver<anyhow::Result<String>>;
}
