use crate::qdrant::vectors_config::Config;
use crate::qdrant::{VectorParams, VectorParamsBuilder, VectorsConfig};

impl From<VectorParams> for VectorsConfig {
    fn from(value: VectorParams) -> Self {
        VectorsConfig {
            config: Some(Config::from(value)),
        }
    }
}

impl From<VectorParamsBuilder> for VectorsConfig {
    fn from(builder: VectorParamsBuilder) -> Self {
        VectorsConfig::from(builder.build())
    }
}
