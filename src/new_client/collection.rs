#![allow(dead_code)]

use crate::new_client::QdrantClient;
use crate::prelude::Distance;
use crate::qdrant::{vectors_config, HnswConfigDiff, VectorParams, VectorParamsMap};
use derive_builder::Builder;
use std::collections::HashMap;

pub const DEFAULT_VECTOR_CONFIG_NAME: &str = "default";

#[derive(Builder)]
#[builder(build_fn(skip))]
pub struct NewCollection {
    client: QdrantClient,

    /// Name of the new collection.
    name: String,

    /// Configuration for vectors.
    #[builder(default, setter(custom))]
    vectors: Option<vectors_config::Config>,

    /// Custom params for HNSW index. If not set values from service configuration file are used
    #[builder(default, setter(strip_option, into))]
    hnsw_config: Option<HnswConfig>,

    /// If true - point's payload will not be stored in memory. It will be read from the disk every time it is requested.
    /// This setting saves RAM by (slightly) increasing the response time.
    /// Note: those payload values that are involved in filtering and are indexed - remain in RAM.
    #[builder(default, setter(strip_option))]
    on_disk_payload: Option<bool>,
}

impl NewCollectionBuilder {
    pub(crate) fn new(client: QdrantClient, name: impl ToString) -> Self {
        NewCollectionBuilder {
            client: Some(client),
            name: Some(name.to_string()),
            vectors: None,
            hnsw_config: None,
            on_disk_payload: None,
        }
    }

    /// Sets the collections default vector configuration.
    /// Calling this function after `add_vectors_config` will remove all existing configs and only use
    /// the config provided to this function.
    pub fn vectors_config(&mut self, config: impl Into<VectorsConfig>) -> &mut Self {
        self.vectors = Some(Some(vectors_config::Config::Params(config.into().into())));
        self
    }

    /// Adds another vector config to the collection. This allows a collection to have multiple vectors
    /// per record with each different configurations ([See also](https://qdrant.tech/documentation/concepts/collections/#collection-with-multiple-vectors)).
    ///
    /// If there has already been a vector config added using `vectors_config` this config gets assigned "default"
    /// as name.
    /// Vector configs with the same name will result in the old config being replaced.
    pub fn add_vectors_config(
        &mut self,
        name: impl ToString,
        config: impl Into<VectorsConfig>,
    ) -> &mut Self {
        let new_map = match self.vectors.take().flatten() {
            Some(vectors_config::Config::Params(single_params)) => {
                let mut params: HashMap<String, VectorParams> = HashMap::with_capacity(2);
                params.insert(DEFAULT_VECTOR_CONFIG_NAME.to_string(), single_params);
                params.insert(name.to_string(), config.into().into());
                VectorParamsMap { map: params }
            }
            Some(vectors_config::Config::ParamsMap(mut params)) => {
                params.map.insert(name.to_string(), config.into().into());
                params
            }
            None => {
                let mut params: HashMap<String, VectorParams> = HashMap::with_capacity(1);
                params.insert(name.to_string(), config.into().into());
                VectorParamsMap { map: params }
            }
        };

        self.vectors = Some(Some(vectors_config::Config::ParamsMap(new_map)));

        self
    }
}

#[derive(Builder)]
#[builder(build_fn(private, error = "std::convert::Infallible", name = "build_inner"))]
pub struct VectorsConfig {
    /// Dimension/Size of the vectors.
    #[builder(default)]
    size: u64,

    /// The distance to compare vectors.
    #[builder(default)]
    distance: Distance,

    /// Whether vectors should be served from disk, improving RAM usage at the cost of latency.
    #[builder(default, setter(strip_option))]
    on_disk: Option<bool>,

    /// Custom params for HNSW index. If not set, values from collection configuration are used.
    #[builder(default, setter(strip_option, into))]
    hnsw_config: Option<HnswConfig>,
}

impl From<VectorsConfig> for VectorParams {
    fn from(value: VectorsConfig) -> Self {
        Self {
            size: value.size,
            distance: value.distance.into(),
            on_disk: value.on_disk,
            hnsw_config: value.hnsw_config.map(|i| i.into()),
            // Todo remove this when all fields were implemented
            ..Default::default()
        }
    }
}

impl VectorsConfigBuilder {
    pub fn new(size: u64, distance: Distance) -> Self {
        Self {
            size: Some(size),
            distance: Some(distance),
            ..Default::default()
        }
    }
}

impl From<VectorsConfigBuilder> for VectorsConfig {
    fn from(value: VectorsConfigBuilder) -> Self {
        value.build_inner().unwrap()
    }
}

impl From<&mut VectorsConfigBuilder> for VectorsConfig {
    fn from(value: &mut VectorsConfigBuilder) -> Self {
        value.clone().build_inner().unwrap()
    }
}

#[derive(Builder, Clone)]
#[builder(build_fn(private, error = "std::convert::Infallible", name = "build_inner"))]
pub struct HnswConfig {
    /// Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
    #[builder(default, setter(strip_option))]
    m: Option<u64>,

    /// Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index.
    #[builder(default, setter(strip_option))]
    ef_construct: Option<u64>,

    /// Minimal size (in kilobytes) of vectors for additional payload-based indexing. If payload chunk is
    /// smaller than `full_scan_threshold_kb` additional indexing won't be used - in this case full-scan search
    /// should be preferred by query planner and additional indexing is not required. Note: 1Kb = 1 vector of size 256.
    #[builder(default, setter(strip_option))]
    full_scan_threshold: Option<u64>,

    /// Number of parallel threads used for background index building.
    /// If 0 - automatically select from 8 to 16. Best to keep between 8 and 16 to prevent likelihood
    /// of building broken/inefficient HNSW graphs. On small CPUs, less threads are used.
    #[builder(default, setter(strip_option))]
    max_indexing_threads: Option<u64>,

    /// Store HNSW index on disk. If set to false, the index will be stored in RAM.
    #[builder(default, setter(strip_option))]
    on_disk: Option<bool>,

    /// Custom M param for additional payload-aware HNSW links. If not set, default M will be used.
    #[builder(default, setter(strip_option))]
    payload_m: Option<u64>,
}

impl HnswConfigBuilder {
    pub fn new() -> Self {
        Self::default()
    }
}

impl From<HnswConfigBuilder> for HnswConfig {
    fn from(value: HnswConfigBuilder) -> Self {
        value.build_inner().unwrap()
    }
}

impl From<&mut HnswConfigBuilder> for HnswConfig {
    fn from(value: &mut HnswConfigBuilder) -> Self {
        value.clone().build_inner().unwrap()
    }
}

impl From<HnswConfig> for HnswConfigDiff {
    fn from(value: HnswConfig) -> Self {
        HnswConfigDiff {
            m: value.m,
            ef_construct: value.ef_construct,
            full_scan_threshold: value.full_scan_threshold,
            max_indexing_threads: value.max_indexing_threads,
            on_disk: value.on_disk,
            payload_m: value.payload_m,
        }
    }
}
