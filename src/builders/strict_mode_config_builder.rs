use crate::qdrant::*;

pub struct StrictModeConfigBuilder {
    pub(crate) enabled: Option<Option<bool>>,
    pub(crate) max_query_limit: Option<Option<u32>>,
    pub(crate) max_timeout: Option<Option<u32>>,
    pub(crate) unindexed_filtering_retrieve: Option<Option<bool>>,
    pub(crate) unindexed_filtering_update: Option<Option<bool>>,
    pub(crate) search_max_hnsw_ef: Option<Option<u32>>,
    pub(crate) search_allow_exact: Option<Option<bool>>,
    pub(crate) search_max_oversampling: Option<Option<f32>>,
    pub(crate) upsert_max_batchsize: Option<Option<u64>>,
    pub(crate) max_collection_vector_size_bytes: Option<Option<u64>>,
    pub(crate) read_rate_limit: Option<Option<u32>>,
    pub(crate) write_rate_limit: Option<Option<u32>>,
    pub(crate) max_collection_payload_size_bytes: Option<Option<u64>>,
    pub(crate) filter_max_conditions: Option<Option<u64>>,
    pub(crate) condition_max_size: Option<Option<u64>>,
}

impl StrictModeConfigBuilder {
    #[allow(unused_mut)]
    pub fn enabled(self, value: bool) -> Self {
        let mut new = self;
        new.enabled = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn max_query_limit(self, value: u32) -> Self {
        let mut new = self;
        new.max_query_limit = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn max_timeout(self, value: u32) -> Self {
        let mut new = self;
        new.max_timeout = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn unindexed_filtering_retrieve(self, value: bool) -> Self {
        let mut new = self;
        new.unindexed_filtering_retrieve = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn unindexed_filtering_update(self, value: bool) -> Self {
        let mut new = self;
        new.unindexed_filtering_update = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn search_max_hnsw_ef(self, value: u32) -> Self {
        let mut new = self;
        new.search_max_hnsw_ef = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn search_allow_exact(self, value: bool) -> Self {
        let mut new = self;
        new.search_allow_exact = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn search_max_oversampling(self, value: f32) -> Self {
        let mut new = self;
        new.search_max_oversampling = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn upsert_max_batchsize(self, value: u64) -> Self {
        let mut new = self;
        new.upsert_max_batchsize = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn max_collection_vector_size_bytes(self, value: u64) -> Self {
        let mut new = self;
        new.max_collection_vector_size_bytes = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn read_rate_limit(self, value: u32) -> Self {
        let mut new = self;
        new.read_rate_limit = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn write_rate_limit(self, value: u32) -> Self {
        let mut new = self;
        new.write_rate_limit = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn max_collection_payload_size_bytes(self, value: u64) -> Self {
        let mut new = self;
        new.max_collection_payload_size_bytes = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn filter_max_conditions(self, value: u64) -> Self {
        let mut new = self;
        new.filter_max_conditions = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn condition_max_size(self, value: u64) -> Self {
        let mut new = self;
        new.condition_max_size = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<StrictModeConfig, std::convert::Infallible> {
        Ok(StrictModeConfig {
            enabled: self.enabled.unwrap_or_default(),
            max_query_limit: self.max_query_limit.unwrap_or_default(),
            max_timeout: self.max_timeout.unwrap_or_default(),
            unindexed_filtering_retrieve: self.unindexed_filtering_retrieve.unwrap_or_default(),
            unindexed_filtering_update: self.unindexed_filtering_update.unwrap_or_default(),
            search_max_hnsw_ef: self.search_max_hnsw_ef.unwrap_or_default(),
            search_allow_exact: self.search_allow_exact.unwrap_or_default(),
            search_max_oversampling: self.search_max_oversampling.unwrap_or_default(),
            upsert_max_batchsize: self.upsert_max_batchsize.unwrap_or_default(),
            max_collection_vector_size_bytes: self
                .max_collection_vector_size_bytes
                .unwrap_or_default(),
            read_rate_limit: self.read_rate_limit.unwrap_or_default(),
            write_rate_limit: self.write_rate_limit.unwrap_or_default(),
            max_collection_payload_size_bytes: self
                .max_collection_payload_size_bytes
                .unwrap_or_default(),
            filter_max_conditions: self.filter_max_conditions.unwrap_or_default(),
            condition_max_size: self.condition_max_size.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            enabled: core::default::Default::default(),
            max_query_limit: core::default::Default::default(),
            max_timeout: core::default::Default::default(),
            unindexed_filtering_retrieve: core::default::Default::default(),
            unindexed_filtering_update: core::default::Default::default(),
            search_max_hnsw_ef: core::default::Default::default(),
            search_allow_exact: core::default::Default::default(),
            search_max_oversampling: core::default::Default::default(),
            upsert_max_batchsize: core::default::Default::default(),
            max_collection_vector_size_bytes: core::default::Default::default(),
            read_rate_limit: core::default::Default::default(),
            write_rate_limit: core::default::Default::default(),
            max_collection_payload_size_bytes: core::default::Default::default(),
            filter_max_conditions: core::default::Default::default(),
            condition_max_size: core::default::Default::default(),
        }
    }
}

impl Default for StrictModeConfigBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl From<StrictModeConfigBuilder> for StrictModeConfig {
    fn from(value: StrictModeConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "StrictModeConfigBuilder", "StrictModeConfig"
            )
        })
    }
}

impl StrictModeConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> StrictModeConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "StrictModeConfigBuilder", "StrictModeConfig"
            )
        })
    }
}
