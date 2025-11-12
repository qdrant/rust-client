use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct ScrollPointsBuilder {
    pub(crate) collection_name: Option<String>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    pub(crate) filter: Option<Option<Filter>>,
    /// Start with this ID
    pub(crate) offset: Option<Option<PointId>>,
    /// Max number of result
    pub(crate) limit: Option<Option<u32>>,
    /// Options for specifying which payload to include or not
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Options for specifying which vectors to include into response
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// Order the records by a payload field
    pub(crate) order_by: Option<Option<OrderBy>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl ScrollPointsBuilder {
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Start with this ID
    pub fn offset<VALUE: core::convert::Into<PointId>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.offset = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of result
    pub fn limit(self, value: u32) -> Self {
        let mut new = self;
        new.limit = Option::Some(Option::Some(value));
        new
    }
    /// Options for specifying which payload to include or not
    pub fn with_payload<VALUE: core::convert::Into<with_payload_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(value.into());
        new
    }
    /// Options for specifying which vectors to include into response
    pub fn with_vectors<VALUE: core::convert::Into<with_vectors_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(value.into());
        new
    }
    /// Options for specifying read consistency guarantees
    pub fn read_consistency<VALUE: core::convert::Into<read_consistency::Value>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(value.into());
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /// Order the records by a payload field
    pub fn order_by<VALUE: core::convert::Into<OrderBy>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.order_by = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<ScrollPoints, ScrollPointsBuilderError> {
        Ok(ScrollPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            filter: self.filter.unwrap_or_default(),
            offset: self.offset.unwrap_or_default(),
            limit: self.limit.unwrap_or_default(),
            with_payload: { convert_option(&self.with_payload) },
            with_vectors: { convert_option(&self.with_vectors) },
            read_consistency: { convert_option(&self.read_consistency) },
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
            order_by: self.order_by.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            filter: core::default::Default::default(),
            offset: core::default::Default::default(),
            limit: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            order_by: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<ScrollPointsBuilder> for ScrollPoints {
    fn from(value: ScrollPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "ScrollPointsBuilder", "ScrollPoints"
            )
        })
    }
}

impl ScrollPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ScrollPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "ScrollPointsBuilder", "ScrollPoints"
            )
        })
    }
}

impl ScrollPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ScrollPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for ScrollPointsBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(field) => {
                write!(f, "`{field}` must be initialized")
            }
            Self::ValidationError(error) => write!(f, "{error}"),
        }
    }
}

// Implementing the Error trait
impl std::error::Error for ScrollPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for ScrollPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for ScrollPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
