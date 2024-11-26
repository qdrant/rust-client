use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct QueryPointsBuilder {
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
    /// Max number of points. Default is 10.
    pub(crate) limit: Option<Option<u64>>,
    /// Offset of the result. Skip this many points. Default is 0.
    pub(crate) offset: Option<Option<u64>>,
    /// Options for specifying which vectors to include into the response.
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
    /// Options for specifying which payload to include or not.
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Options for specifying read consistency guarantees.
    read_consistency: Option<read_consistency::Value>,
    /// Specify in which shards to look for the points, if not specified - look in all shards.
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    pub(crate) lookup_from: Option<Option<LookupLocation>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl QueryPointsBuilder {
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
    /// Max number of points. Default is 10.
    #[allow(unused_mut)]
    pub fn limit(self, value: u64) -> Self {
        let mut new = self;
        new.limit = Option::Some(Option::Some(value));
        new
    }
    /// Offset of the result. Skip this many points. Default is 0.
    #[allow(unused_mut)]
    pub fn offset(self, value: u64) -> Self {
        let mut new = self;
        new.offset = Option::Some(Option::Some(value));
        new
    }
    /// Options for specifying which vectors to include into the response.
    #[allow(unused_mut)]
    pub fn with_vectors<VALUE: core::convert::Into<with_vectors_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(value.into());
        new
    }
    /// Options for specifying which payload to include or not.
    #[allow(unused_mut)]
    pub fn with_payload<VALUE: core::convert::Into<with_payload_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(value.into());
        new
    }
    /// Options for specifying read consistency guarantees.
    #[allow(unused_mut)]
    pub fn read_consistency<VALUE: core::convert::Into<read_consistency::Value>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(value.into());
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards.
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    #[allow(unused_mut)]
    pub fn lookup_from<VALUE: core::convert::Into<LookupLocation>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup_from = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<QueryPoints, QueryPointsBuilderError> {
        Ok(QueryPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            prefetch: self.prefetch.unwrap_or_default(),
            query: self.query.unwrap_or_default(),
            using: self.using.unwrap_or_default(),
            filter: self.filter.unwrap_or_default(),
            params: self.params.unwrap_or_default(),
            score_threshold: self.score_threshold.unwrap_or_default(),
            limit: self.limit.unwrap_or_default(),
            offset: self.offset.unwrap_or_default(),
            with_vectors: { convert_option(&self.with_vectors) },
            with_payload: { convert_option(&self.with_payload) },
            read_consistency: { convert_option(&self.read_consistency) },
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
            lookup_from: self.lookup_from.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
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
            limit: core::default::Default::default(),
            offset: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            lookup_from: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<QueryPointsBuilder> for QueryPoints {
    fn from(value: QueryPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "QueryPointsBuilder", "QueryPoints"
            )
        })
    }
}

impl QueryPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> QueryPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "QueryPointsBuilder", "QueryPoints"
            )
        })
    }
}

impl QueryPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for QueryPointsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum QueryPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for QueryPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing the From trait for conversion from String
impl From<String> for QueryPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for QueryPointsBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(field) => {
                write!(f, "`{}` must be initialized", field)
            }
            Self::ValidationError(error) => write!(f, "{}", error),
        }
    }
}

// Implementing the Error trait
impl std::error::Error for QueryPointsBuilderError {}
