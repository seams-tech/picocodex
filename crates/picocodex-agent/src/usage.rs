use picocodex_oai_api::Model;
pub use picocodex_oai_api::ServiceTier;
use serde::{Deserialize, Serialize};

/// Exact token accounting for every Responses call in one logical agent turn.
///
/// Cache-read and cache-write tokens are subsets of input tokens. Reasoning
/// tokens are a subset of output tokens. The values are summed from provider
/// usage records across warmup, generation, tool continuation, steering, and
/// compaction calls made before the turn reaches its terminal boundary.
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
#[allow(clippy::struct_field_names)]
pub struct TurnUsage {
    model: Model,
    service_tier: ServiceTier,
    reported: bool,
    input_tokens: u64,
    cached_input_tokens: u64,
    cache_write_input_tokens: u64,
    output_tokens: u64,
    reasoning_output_tokens: u64,
    total_tokens: u64,
}

#[allow(clippy::struct_field_names)]
#[derive(Clone, Copy)]
pub(crate) struct TurnUsageCounts {
    pub(crate) input_tokens: u64,
    pub(crate) cached_input_tokens: u64,
    pub(crate) cache_write_input_tokens: u64,
    pub(crate) output_tokens: u64,
    pub(crate) reasoning_output_tokens: u64,
    pub(crate) total_tokens: u64,
    pub(crate) reported: bool,
}

impl TurnUsage {
    pub(crate) const fn from_counts(
        counts: TurnUsageCounts,
        model: picocodex_oai_api::Model,
        fast_mode: bool,
    ) -> Self {
        Self {
            model,
            service_tier: if fast_mode {
                ServiceTier::Priority
            } else {
                ServiceTier::Standard
            },
            reported: counts.reported,
            input_tokens: counts.input_tokens,
            cached_input_tokens: counts.cached_input_tokens,
            cache_write_input_tokens: counts.cache_write_input_tokens,
            output_tokens: counts.output_tokens,
            reasoning_output_tokens: counts.reasoning_output_tokens,
            total_tokens: counts.total_tokens,
        }
    }

    /// Returns the model fixed for the complete turn lineage.
    #[must_use]
    pub const fn model(&self) -> Model {
        self.model
    }

    /// Returns the provider service tier requested for the turn.
    #[must_use]
    pub const fn service_tier(&self) -> ServiceTier {
        self.service_tier
    }

    /// Returns whether the provider reported usage for any model call.
    #[must_use]
    pub const fn reported(&self) -> bool {
        self.reported
    }

    /// Returns all input tokens billed or reported by the provider.
    #[must_use]
    pub const fn input_tokens(&self) -> u64 {
        self.input_tokens
    }

    /// Returns input tokens served from the provider's prompt cache.
    #[must_use]
    pub const fn cached_input_tokens(&self) -> u64 {
        self.cached_input_tokens
    }

    /// Returns input tokens newly written into the provider's prompt cache.
    #[must_use]
    pub const fn cache_write_input_tokens(&self) -> u64 {
        self.cache_write_input_tokens
    }

    /// Returns all output tokens billed or reported by the provider.
    #[must_use]
    pub const fn output_tokens(&self) -> u64 {
        self.output_tokens
    }

    /// Returns reasoning tokens included within [`Self::output_tokens`].
    #[must_use]
    pub const fn reasoning_output_tokens(&self) -> u64 {
        self.reasoning_output_tokens
    }

    /// Returns the provider-reported total token count.
    #[must_use]
    pub const fn total_tokens(&self) -> u64 {
        self.total_tokens
    }
}
