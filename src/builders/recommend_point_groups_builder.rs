use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct RecommendPointGroupsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Look for vectors closest to the vectors from these points
    pub(crate) positive: Option<Vec<PointId>>,
    /// Try to avoid vectors like the vector from these points
    pub(crate) negative: Option<Vec<PointId>>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    pub(crate) filter: Option<Option<Filter>>,
    /// Max number of groups in result
    pub(crate) limit: Option<u32>,
    /// Options for specifying which payload to include or not
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Search config
    pub(crate) params: Option<Option<SearchParams>>,
    /// If provided - cut off results with worse scores
    pub(crate) score_threshold: Option<Option<f32>>,
    /// Define which vector to use for recommendation, if not specified - default vector
    pub(crate) using: Option<Option<String>>,
    /// Options for specifying which vectors to include into response
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
    /// Name of the collection to use for points lookup, if not specified - use current collection
    pub(crate) lookup_from: Option<Option<LookupLocation>>,
    /// Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups.
    pub(crate) group_by: Option<String>,
    /// Maximum amount of points to return per group
    pub(crate) group_size: Option<u32>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// Options for specifying how to use the group id to lookup points in another collection
    pub(crate) with_lookup: Option<Option<WithLookup>>,
    /// How to use the example vectors to find the results
    pub(crate) strategy: Option<Option<i32>>,
    /// Look for vectors closest to those
    pub(crate) positive_vectors: Option<Vec<Vector>>,
    /// Try to avoid vectors like this
    pub(crate) negative_vectors: Option<Vec<Vector>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl RecommendPointGroupsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of groups in result
    #[allow(unused_mut)]
    pub fn limit(self, value: u32) -> Self {
        let mut new = self;
        new.limit = Option::Some(value);
        new
    }
    /// Options for specifying which payload to include or not
    #[allow(unused_mut)]
    pub fn with_payload<VALUE: core::convert::Into<with_payload_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(value.into());
        new
    }
    /// Search config
    #[allow(unused_mut)]
    pub fn params<VALUE: core::convert::Into<SearchParams>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.params = Option::Some(Option::Some(value.into()));
        new
    }
    /// If provided - cut off results with worse scores
    #[allow(unused_mut)]
    pub fn score_threshold(self, value: f32) -> Self {
        let mut new = self;
        new.score_threshold = Option::Some(Option::Some(value));
        new
    }
    /// Define which vector to use for recommendation, if not specified - default vector
    #[allow(unused_mut)]
    pub fn using<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.using = Option::Some(Option::Some(value.into()));
        new
    }
    /// Options for specifying which vectors to include into response
    #[allow(unused_mut)]
    pub fn with_vectors<VALUE: core::convert::Into<with_vectors_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(value.into());
        new
    }
    /// Name of the collection to use for points lookup, if not specified - use current collection
    #[allow(unused_mut)]
    pub fn lookup_from<VALUE: core::convert::Into<LookupLocation>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup_from = Option::Some(Option::Some(value.into()));
        new
    }
    /// Payload field to group by, must be a string or number field. If there are multiple values for the field, all of them will be used. One point can be in multiple groups.
    #[allow(unused_mut)]
    pub fn group_by(self, value: String) -> Self {
        let mut new = self;
        new.group_by = Option::Some(value);
        new
    }
    /// Maximum amount of points to return per group
    #[allow(unused_mut)]
    pub fn group_size(self, value: u32) -> Self {
        let mut new = self;
        new.group_size = Option::Some(value);
        new
    }
    /// Options for specifying read consistency guarantees
    #[allow(unused_mut)]
    pub fn read_consistency<VALUE: core::convert::Into<read_consistency::Value>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(value.into());
        new
    }
    /// Options for specifying how to use the group id to lookup points in another collection
    #[allow(unused_mut)]
    pub fn with_lookup<VALUE: core::convert::Into<WithLookup>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.with_lookup = Option::Some(Option::Some(value.into()));
        new
    }
    /// How to use the example vectors to find the results
    #[allow(unused_mut)]
    pub fn strategy<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.strategy = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
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

    fn build_inner(self) -> Result<RecommendPointGroups, RecommendPointGroupsBuilderError> {
        Ok(RecommendPointGroups {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            positive: self.positive.unwrap_or_default(),
            negative: self.negative.unwrap_or_default(),
            filter: self.filter.unwrap_or_default(),
            limit: match self.limit {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("limit"),
                    ));
                }
            },
            with_payload: { convert_option(&self.with_payload) },
            params: self.params.unwrap_or_default(),
            score_threshold: self.score_threshold.unwrap_or_default(),
            using: self.using.unwrap_or_default(),
            with_vectors: { convert_option(&self.with_vectors) },
            lookup_from: self.lookup_from.unwrap_or_default(),
            group_by: match self.group_by {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("group_by"),
                    ));
                }
            },
            group_size: match self.group_size {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("group_size"),
                    ));
                }
            },
            read_consistency: { convert_option(&self.read_consistency) },
            with_lookup: self.with_lookup.unwrap_or_default(),
            strategy: self.strategy.unwrap_or_default(),
            positive_vectors: self.positive_vectors.unwrap_or_default(),
            negative_vectors: self.negative_vectors.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            positive: core::default::Default::default(),
            negative: core::default::Default::default(),
            filter: core::default::Default::default(),
            limit: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            params: core::default::Default::default(),
            score_threshold: core::default::Default::default(),
            using: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            lookup_from: core::default::Default::default(),
            group_by: core::default::Default::default(),
            group_size: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            with_lookup: core::default::Default::default(),
            strategy: core::default::Default::default(),
            positive_vectors: core::default::Default::default(),
            negative_vectors: core::default::Default::default(),
            timeout: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<RecommendPointGroupsBuilder> for RecommendPointGroups {
    fn from(value: RecommendPointGroupsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "RecommendPointGroupsBuilder", "RecommendPointGroups"
            )
        })
    }
}

impl RecommendPointGroupsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> RecommendPointGroups {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "RecommendPointGroupsBuilder", "RecommendPointGroups"
            )
        })
    }
}

impl RecommendPointGroupsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for RecommendPointGroupsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum RecommendPointGroupsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for RecommendPointGroupsBuilderError {
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
impl std::error::Error for RecommendPointGroupsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for RecommendPointGroupsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for RecommendPointGroupsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
