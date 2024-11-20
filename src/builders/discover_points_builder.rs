use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct DiscoverPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Use this as the primary search objective
    pub(crate) target: Option<Option<TargetVector>>,
    /// Search will be constrained by these pairs of examples
    pub(crate) context: Option<Vec<ContextExamplePair>>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    pub(crate) filter: Option<Option<Filter>>,
    /// Max number of result
    pub(crate) limit: Option<u64>,
    /// Options for specifying which payload to include or not
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Search config
    pub(crate) params: Option<Option<SearchParams>>,
    /// Offset of the result
    pub(crate) offset: Option<Option<u64>>,
    /// Define which vector to use for recommendation, if not specified - default vector
    pub(crate) using: Option<Option<String>>,
    /// Options for specifying which vectors to include into response
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
    /// Name of the collection to use for points lookup, if not specified - use current collection
    pub(crate) lookup_from: Option<Option<LookupLocation>>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl DiscoverPointsBuilder {
    /// name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Use this as the primary search objective
    #[allow(unused_mut)]
    pub fn target<VALUE: core::convert::Into<TargetVector>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.target = Option::Some(Option::Some(value.into()));
        new
    }
    /// Search will be constrained by these pairs of examples
    #[allow(unused_mut)]
    pub fn context(self, value: Vec<ContextExamplePair>) -> Self {
        let mut new = self;
        new.context = Option::Some(value);
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of result
    #[allow(unused_mut)]
    pub fn limit(self, value: u64) -> Self {
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
    /// Offset of the result
    #[allow(unused_mut)]
    pub fn offset(self, value: u64) -> Self {
        let mut new = self;
        new.offset = Option::Some(Option::Some(value));
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

    fn build_inner(self) -> Result<DiscoverPoints, DiscoverPointsBuilderError> {
        Ok(DiscoverPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            target: match self.target {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            context: match self.context {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("context"),
                    ));
                }
            },
            filter: match self.filter {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            limit: match self.limit {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("limit"),
                    ));
                }
            },
            with_payload: { convert_option(&self.with_payload) },
            params: match self.params {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            offset: match self.offset {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            using: match self.using {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            with_vectors: { convert_option(&self.with_vectors) },
            lookup_from: match self.lookup_from {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            read_consistency: { convert_option(&self.read_consistency) },
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
            target: core::default::Default::default(),
            context: core::default::Default::default(),
            filter: core::default::Default::default(),
            limit: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            params: core::default::Default::default(),
            offset: core::default::Default::default(),
            using: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            lookup_from: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            timeout: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<DiscoverPointsBuilder> for DiscoverPoints {
    fn from(value: DiscoverPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DiscoverPointsBuilder", "DiscoverPoints",
        ))
    }
}

impl DiscoverPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DiscoverPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DiscoverPointsBuilder", "DiscoverPoints",
        ))
    }
}

impl DiscoverPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DiscoverPointsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DiscoverPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DiscoverPointsBuilderError {
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
impl std::error::Error for DiscoverPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DiscoverPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DiscoverPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
