use crate::qdrant::vectors_config::Config;
use crate::qdrant::{VectorParams, VectorParamsMap, VectorsConfig};
use std::collections::HashMap;

const DEFAULT_VECTOR_NAME: &str = "";

#[derive(Debug, Clone, Default)]
pub struct VectorsConfigBuilder {
    params: HashMap<String, VectorParams>,
}

impl VectorsConfigBuilder {
    /// Add a named vector with the given parameters
    pub fn add_named_vector_params(
        &mut self,
        name: impl Into<String>,
        params: impl Into<VectorParams>,
    ) -> &mut Self {
        self.params.insert(name.into(), params.into());
        self
    }

    /// Add the default vector with the given parameters
    pub fn add_vector_params(&mut self, params: impl Into<VectorParams>) -> &mut Self {
        self.params
            .insert(DEFAULT_VECTOR_NAME.to_string(), params.into());
        self
    }
}

impl From<VectorsConfigBuilder> for VectorsConfig {
    fn from(mut builder: VectorsConfigBuilder) -> Self {
        if builder.params.is_empty() {
            return VectorsConfig::default();
        }

        if builder.params.len() == 1 && builder.params.contains_key(DEFAULT_VECTOR_NAME) {
            return VectorsConfig {
                config: Some(Config::from(
                    builder.params.remove(DEFAULT_VECTOR_NAME).unwrap(),
                )),
            };
        }

        VectorsConfig {
            config: Some(Config::from(VectorParamsMap::from(builder.params))),
        }
    }
}
