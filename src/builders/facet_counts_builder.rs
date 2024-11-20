use crate::qdrant::*;

pub struct FacetCountsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Payload key of the facet
    pub(crate) key: Option<String>,
    /// Filter conditions - return only those points that satisfy the specified conditions.
    pub(crate) filter: Option<Option<Filter>>,
    /// Max number of facets. Default is 10.
    pub(crate) limit: Option<Option<u64>>,
    /// If true, return exact counts, slower but useful for debugging purposes. Default is false.
    pub(crate) exact: Option<Option<bool>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
    /// Options for specifying read consistency guarantees
    pub(crate) read_consistency: Option<Option<ReadConsistency>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl FacetCountsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Payload key of the facet
    #[allow(unused_mut)]
    pub fn key(self, value: String) -> Self {
        let mut new = self;
        new.key = Option::Some(value);
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions.
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of facets. Default is 10.
    #[allow(unused_mut)]
    pub fn limit(self, value: u64) -> Self {
        let mut new = self;
        new.limit = Option::Some(Option::Some(value));
        new
    }
    /// If true, return exact counts, slower but useful for debugging purposes. Default is false.
    #[allow(unused_mut)]
    pub fn exact(self, value: bool) -> Self {
        let mut new = self;
        new.exact = Option::Some(Option::Some(value));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /// Options for specifying read consistency guarantees
    #[allow(unused_mut)]
    pub fn read_consistency<VALUE: core::convert::Into<ReadConsistency>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(Option::Some(value.into()));
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `FacetCounts`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<FacetCounts, FacetCountsBuilderError> {
        Ok(FacetCounts {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            key: match self.key {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("key"),
                    ));
                }
            },
            filter: match self.filter {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            limit: match self.limit {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            exact: match self.exact {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            timeout: match self.timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            read_consistency: match self.read_consistency {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            shard_key_selector: match self.shard_key_selector {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            key: core::default::Default::default(),
            filter: core::default::Default::default(),
            limit: core::default::Default::default(),
            exact: core::default::Default::default(),
            timeout: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<FacetCountsBuilder> for FacetCounts {
    fn from(value: FacetCountsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "FacetCountsBuilder", "FacetCounts",
        ))
    }
}

impl FacetCountsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> FacetCounts {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "FacetCountsBuilder", "FacetCounts",
        ))
    }
}

impl FacetCountsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
