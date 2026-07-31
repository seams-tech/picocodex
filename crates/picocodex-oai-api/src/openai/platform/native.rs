use std::{num::NonZeroU32, sync::Arc};

use crate::{DefaultResponsesService, ModelConfig, OpenAiAuthMode, OpenAiError, ResponsesService};

#[derive(Clone, Default)]
pub(crate) struct FactoryPlatform {
    http_client: Option<reqwest::Client>,
}

impl FactoryPlatform {
    pub(crate) const fn new() -> Self {
        Self { http_client: None }
    }

    pub(crate) fn set_http_client(&mut self, client: reqwest::Client) {
        self.http_client = Some(client);
    }

    pub(crate) const fn validate_config(&self, config: &ModelConfig) -> Result<(), OpenAiError> {
        if matches!(config.auth.mode(), OpenAiAuthMode::HostManaged) {
            Err(OpenAiError::InvalidConfiguration {
                detail: "host-managed authentication requires an embedding host transport",
            })
        } else {
            Ok(())
        }
    }

    pub(crate) fn make(
        &self,
        config: Arc<ModelConfig>,
        max_attempts: NonZeroU32,
    ) -> DefaultResponsesService {
        if let Some(client) = &self.http_client {
            ResponsesService::standard_with_http_client_and_max_attempts(
                config,
                client.clone(),
                max_attempts,
            )
        } else {
            ResponsesService::standard_with_max_attempts(config, max_attempts)
        }
    }
}
