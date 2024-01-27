use config::{Config, FileFormat};
use once_cell::sync::Lazy;
use serde::Deserialize;

pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| AppConfig::new());


#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub api_key: String,
    pub url: String,
    pub model: String,
}

impl AppConfig {
    pub fn new() -> Self {
        let config = Config::builder()
            .add_source(config::File::with_name("./config/config").format(FileFormat::Yaml))
            .build()
            .unwrap();
        config.try_deserialize().unwrap()
    }
}


mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn test_config() {
        let config = AppConfig::new();
        println!("{:?}", config);
    }

    #[test]
    fn test_app_config() {
        println!("{:?}", *APP_CONFIG);
    }
}
