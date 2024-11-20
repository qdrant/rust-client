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

    fn build_inner(self) -> Result<StrictModeConfig, std::convert::Infallible> {
        Ok(StrictModeConfig {
            enabled: match self.enabled {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_query_limit: match self.max_query_limit {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_timeout: match self.max_timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            unindexed_filtering_retrieve: match self.unindexed_filtering_retrieve {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            unindexed_filtering_update: match self.unindexed_filtering_update {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            search_max_hnsw_ef: match self.search_max_hnsw_ef {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            search_allow_exact: match self.search_allow_exact {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            search_max_oversampling: match self.search_max_oversampling {
                Some(value) => value,
                None => core::default::Default::default(),
            },
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
        }
    }
}

impl From<StrictModeConfigBuilder> for StrictModeConfig {
    fn from(value: StrictModeConfigBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "StrictModeConfigBuilder", "StrictModeConfig",
        ))
    }
}

impl StrictModeConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> StrictModeConfig {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "StrictModeConfigBuilder", "StrictModeConfig",
        ))
    }
}
