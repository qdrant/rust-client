use std::collections::HashMap;

use crate::qdrant::{StrictModeSparse, StrictModeSparseConfig};

/// Builder for StrictModeSparseConfig, which defines sparse vector configuration for strict mode.
pub struct StrictModeSparseConfigBuilder {
    /// The sparse vectors configuration map, where keys are vector names and values are their configurations.
    pub(crate) sparse_config: HashMap<String, StrictModeSparse>,
}

impl StrictModeSparseConfigBuilder {
    /// Create a new builder with an empty sparse vectors configuration map.
    pub fn new() -> Self {
        Self {
            sparse_config: HashMap::new(),
        }
    }

    /// Add a configuration for a named vector, specifying its maximum number of vectors.
    pub fn add_vector_config<S: Into<String>>(
        self,
        name: S,
        strict_mode_multivector: StrictModeSparse,
    ) -> Self {
        let mut new = self;
        new.sparse_config
            .insert(name.into(), strict_mode_multivector);

        new
    }

    /// Set the entire sparse vectors configuration map at once.
    pub fn sparse_config<M: Into<HashMap<String, StrictModeSparse>>>(self, config: M) -> Self {
        let mut new = self;
        new.sparse_config = config.into();
        new
    }

    fn build_inner(self) -> Result<StrictModeSparseConfig, std::convert::Infallible> {
        Ok(StrictModeSparseConfig {
            sparse_config: self.sparse_config,
        })
    }

    /// Create an empty builder, with all fields set to `None` or default values.
    fn create_empty() -> Self {
        Self {
            sparse_config: core::default::Default::default(),
        }
    }
}

impl From<StrictModeSparseConfigBuilder> for StrictModeSparseConfig {
    fn from(value: StrictModeSparseConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "StrictModeSparseConfigBuilder", "StrictModeSparseConfig"
            )
        })
    }
}

impl StrictModeSparseConfigBuilder {
    /// Builds the desired StrictModeSparseConfig type.
    pub fn build(self) -> StrictModeSparseConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "StrictModeSparseConfigBuilder", "StrictModeSparseConfig"
            )
        })
    }
}

impl Default for StrictModeSparseConfigBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
