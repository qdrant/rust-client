use crate::qdrant::*;

pub struct QueryPointGroupsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Sub-requests to perform first. If present, the query will be performed on the results of the prefetches.
    pub(crate) prefetch: Option<Vec<PrefetchQuery>>,
    /// Query to perform. If missing, returns points ordered by their IDs.
    pub(crate) query: Option<Option<Query>>,
    /// Define which vector to use for querying. If missing, the default vector is used.
    pub(crate) using: Option<Option<String>>,
    /// Filter conditions - return only those points that satisfy the specified conditions.
    pub(crate) filter: Option<Option<Filter>>,
    /// Search params for when there is no prefetch.
    pub(crate) params: Option<Option<SearchParams>>,
    /// Return points with scores better than this threshold.
    pub(crate) score_threshold: Option<Option<f32>>,
    /// Options for specifying which payload to include or not
    pub(crate) with_payload: Option<Option<WithPayloadSelector>>,
    /// Options for specifying which vectors to include into response
    pub(crate) with_vectors: Option<Option<WithVectorsSelector>>,
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    pub(crate) lookup_from: Option<Option<LookupLocation>>,
    /// Max number of points. Default is 3.
    pub(crate) limit: Option<Option<u64>>,
    /// Maximum amount of points to return per group. Default to 10.
    pub(crate) group_size: Option<Option<u64>>,
    /// Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups.
    pub(crate) group_by: Option<String>,
    /// Options for specifying read consistency guarantees
    pub(crate) read_consistency: Option<Option<ReadConsistency>>,
    /// Options for specifying how to use the group id to lookup points in another collection
    pub(crate) with_lookup: Option<Option<WithLookup>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl QueryPointGroupsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Sub-requests to perform first. If present, the query will be performed on the results of the prefetches.
    #[allow(unused_mut)]
    pub fn prefetch<VALUE: core::convert::Into<Vec<PrefetchQuery>>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.prefetch = Option::Some(value.into());
        new
    }
    /// Query to perform. If missing, returns points ordered by their IDs.
    #[allow(unused_mut)]
    pub fn query<VALUE: core::convert::Into<Query>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.query = Option::Some(Option::Some(value.into()));
        new
    }
    /// Define which vector to use for querying. If missing, the default vector is used.
    #[allow(unused_mut)]
    pub fn using<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.using = Option::Some(Option::Some(value.into()));
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions.
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Search params for when there is no prefetch.
    #[allow(unused_mut)]
    pub fn params<VALUE: core::convert::Into<SearchParams>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.params = Option::Some(Option::Some(value.into()));
        new
    }
    /// Return points with scores better than this threshold.
    #[allow(unused_mut)]
    pub fn score_threshold<VALUE: core::convert::Into<f32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.score_threshold = Option::Some(Option::Some(value.into()));
        new
    }
    /// Options for specifying which payload to include or not
    #[allow(unused_mut)]
    pub fn with_payload<VALUE: core::convert::Into<WithPayloadSelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(Option::Some(value.into()));
        new
    }
    /// Options for specifying which vectors to include into response
    #[allow(unused_mut)]
    pub fn with_vectors<VALUE: core::convert::Into<WithVectorsSelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(Option::Some(value.into()));
        new
    }
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    #[allow(unused_mut)]
    pub fn lookup_from<VALUE: core::convert::Into<LookupLocation>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup_from = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of points. Default is 3.
    #[allow(unused_mut)]
    pub fn limit<VALUE: core::convert::Into<u64>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.limit = Option::Some(Option::Some(value.into()));
        new
    }
    /// Maximum amount of points to return per group. Default to 10.
    #[allow(unused_mut)]
    pub fn group_size<VALUE: core::convert::Into<u64>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.group_size = Option::Some(Option::Some(value.into()));
        new
    }
    /// Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups.
    #[allow(unused_mut)]
    pub fn group_by(self, value: String) -> Self {
        let mut new = self;
        new.group_by = Option::Some(value);
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
    /// Options for specifying how to use the group id to lookup points in another collection
    #[allow(unused_mut)]
    pub fn with_lookup<VALUE: core::convert::Into<WithLookup>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.with_lookup = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout<VALUE: core::convert::Into<u64>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value.into()));
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
    /**Builds a new `QueryPointGroups`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<QueryPointGroups, QueryPointGroupsBuilderError> {
        Ok(QueryPointGroups {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            prefetch: match self.prefetch {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            query: match self.query {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            using: match self.using {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            filter: match self.filter {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            params: match self.params {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            score_threshold: match self.score_threshold {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            with_payload: match self.with_payload {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            with_vectors: match self.with_vectors {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            lookup_from: match self.lookup_from {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            limit: match self.limit {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            group_size: match self.group_size {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            group_by: match self.group_by {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("group_by"),
                    ));
                }
            },
            read_consistency: match self.read_consistency {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            with_lookup: match self.with_lookup {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            timeout: match self.timeout {
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
            prefetch: core::default::Default::default(),
            query: core::default::Default::default(),
            using: core::default::Default::default(),
            filter: core::default::Default::default(),
            params: core::default::Default::default(),
            score_threshold: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            lookup_from: core::default::Default::default(),
            limit: core::default::Default::default(),
            group_size: core::default::Default::default(),
            group_by: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            with_lookup: core::default::Default::default(),
            timeout: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<QueryPointGroupsBuilder> for QueryPointGroups {
    fn from(value: QueryPointGroupsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "QueryPointGroupsBuilder", "QueryPointGroups",
        ))
    }
}

impl QueryPointGroupsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> QueryPointGroups {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "QueryPointGroupsBuilder", "QueryPointGroups",
        ))
    }
}

impl QueryPointGroupsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
