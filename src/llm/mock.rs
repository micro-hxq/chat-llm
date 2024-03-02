use tokio::sync::mpsc;

use crate::llm::{Llm, LlmConfig, LlmGenerationConfig, LlmMessage};

#[derive(Debug, Default)]
pub struct MockLlm {}


impl LlmGenerationConfig for MockLlm {
    fn do_sample(&self) -> bool {
        true
    }

    fn temperature(&self) -> f32 {
        0.98
    }

    fn max_tokens(&self) -> i32 {
        1024
    }

    fn top_p(&self) -> f32 {
        0.95
    }

    fn frequency_penalty(&self) -> f32 {
        1.0
    }

    fn presence_penalty(&self) -> f32 {
        1.0
    }
}

impl LlmConfig for MockLlm {
    fn api_key(&self) -> &str {
        "mock"
    }

    fn url(&self) -> &str {
        "mock"
    }

    fn model(&self) -> &str {
        "mock-model"
    }

    fn generation_config(&self) -> &dyn LlmGenerationConfig {
        self
    }
}

impl Llm for MockLlm {
    fn generate(&self, _messages: &[&dyn LlmMessage]) -> String {
        "This is a mock response".to_string()
    }

    fn stream_generate(&self, _messages: &[&dyn LlmMessage]) -> mpsc::Receiver<anyhow::Result<String>> {
        let (tx, rx) = mpsc::channel(1);
        tokio::spawn(async move {
            for i in 0..10 {
                tx.send(Ok(format!("This is No.{i} message"))).await.unwrap();
            }
        });
        rx
    }
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_mock_llm_config() {
        let llm = MockLlm::default();
        assert_eq!(llm.api_key(), "mock");
        assert_eq!(llm.url(), "mock");
        assert_eq!(llm.model(), "mock-model");
    }

    #[test]
    fn test_mock_llm_generate() {
        let llm = MockLlm::default();
        assert_eq!(llm.generate(&vec![]), "This is a mock response");
    }

    #[tokio::test]
    async fn test_mock_llm_stream_generate() {
        let llm = MockLlm::default();
        let mut rx = llm.stream_generate(&vec![]);
        for i in 0..10 {
            assert_eq!(rx.recv().await.unwrap().unwrap(), format!("This is No.{i} message"));
        }
    }
}