use std::{sync::Arc};
use config::Config;
use tracing::{info};

#[derive(Debug, Default, serde::Deserialize, PartialEq)]
pub struct AppConfig {
    pub user_agent: String,
    pub orionoid_rate_limit: String,
}

#[derive(Debug)]
pub struct ConfigurationProvider;

impl ConfigurationProvider {
    pub fn load_config() -> anyhow::Result<Arc<AppConfig>> {
        let config = Config::builder()
            .set_default("user_agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 Chrome/124.0.0.0")?
            .set_default("orionoid_rate_limit", "10/second")?
            .add_source(
                config::Environment::with_prefix("ORIONZNAB")
            )
            .build()?;

        let config: AppConfig = config.try_deserialize()?;

        if config.user_agent.trim().is_empty() {
            return Err(anyhow::anyhow!("ORIONZNAB_USER_AGENT must be set and cannot be empty"));
        }

        if config.orionoid_rate_limit.trim().is_empty() {
            return Err(anyhow::anyhow!("ORIONZNAB_RATE_LIMIT must be set and cannot be empty"));
        }

        info!("Loaded configuration: {:?}", config);

        Ok(Arc::new(config))
    }
}