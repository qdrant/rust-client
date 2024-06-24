use std::collections::HashMap;

use crate::qdrant::{SparseVectorConfig, SparseVectorParams};

#[derive(Debug, Clone, Default)]
pub struct SparseVectorsConfigBuilder {
    params: HashMap<String, SparseVectorParams>,
}

impl SparseVectorsConfigBuilder {
    /// Add a named vector with the given parameters
    pub fn add_named_vector_params(
        &mut self,
        name: impl Into<String>,
        params: impl Into<SparseVectorParams>,
    ) -> &mut Self {
        self.params.insert(name.into(), params.into());
        self
    }
}

impl From<SparseVectorsConfigBuilder> for SparseVectorConfig {
    fn from(builder: SparseVectorsConfigBuilder) -> Self {
        if builder.params.is_empty() {
            return SparseVectorConfig::default();
        }

        SparseVectorConfig {
            map: builder.params,
        }
    }
}
