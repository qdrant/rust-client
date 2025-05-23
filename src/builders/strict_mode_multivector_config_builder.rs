use std::collections::HashMap;

use crate::qdrant::{StrictModeMultivector, StrictModeMultivectorConfig};

/// Builder for StrictModeMultivectorConfig, which defines multivector configuration for strict mode.
#[derive(Clone)]
pub struct StrictModeMultivectorConfigBuilder {
    /// The multivector configuration map, where keys are vector names and values are their configurations.
    pub(crate) multivector_config: HashMap<String, StrictModeMultivector>,
}

impl StrictModeMultivectorConfigBuilder {
    /// Create a new builder with an empty multivector configuration map.
    pub fn new() -> Self {
        Self {
            multivector_config: HashMap::new(),
        }
    }

    /// Add a configuration for a named vector, specifying its maximum number of vectors.
    pub fn add_vector_config<S: Into<String>>(
        self,
        name: S,
        strict_mode_multivector: StrictModeMultivector,
    ) -> Self {
        let mut new = self;
        let mut config = new.multivector_config;

        config.insert(name.into(), strict_mode_multivector);

        new.multivector_config = config;
        new
    }

    /// Set the entire multivector configuration map at once.
    pub fn multivector_config<M: Into<HashMap<String, StrictModeMultivector>>>(
        self,
        config: M,
    ) -> Self {
        let mut new = self;
        new.multivector_config = config.into();
        new
    }

    fn build_inner(self) -> Result<StrictModeMultivectorConfig, std::convert::Infallible> {
        Ok(StrictModeMultivectorConfig {
            multivector_config: self.multivector_config,
        })
    }

    /// Create an empty builder, with all fields set to `None` or default values.
    fn create_empty() -> Self {
        Self {
            multivector_config: core::default::Default::default(),
        }
    }
}

impl From<StrictModeMultivectorConfigBuilder> for StrictModeMultivectorConfig {
    fn from(value: StrictModeMultivectorConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "StrictModeMultivectorConfigBuilder", "StrictModeMultivectorConfig"
            )
        })
    }
}

impl StrictModeMultivectorConfigBuilder {
    /// Builds the desired StrictModeMultivectorConfig type.
    pub fn build(self) -> StrictModeMultivectorConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "StrictModeMultivectorConfigBuilder", "StrictModeMultivectorConfig"
            )
        })
    }
}

impl Default for StrictModeMultivectorConfigBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
