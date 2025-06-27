use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct GetPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// List of points to retrieve
    pub(crate) ids: Option<Vec<PointId>>,
    /// Options for specifying which payload to include or not
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Options for specifying which vectors to include into response
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl GetPointsBuilder {
    /// name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// List of points to retrieve
    #[allow(unused_mut)]
    pub fn ids(self, value: Vec<PointId>) -> Self {
        let mut new = self;
        new.ids = Option::Some(value);
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
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<GetPoints, GetPointsBuilderError> {
        Ok(GetPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            ids: match self.ids {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("ids"),
                    ));
                }
            },
            with_payload: { convert_option(&self.with_payload) },
            with_vectors: { convert_option(&self.with_vectors) },
            read_consistency: { convert_option(&self.read_consistency) },
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            ids: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<GetPointsBuilder> for GetPoints {
    fn from(value: GetPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "GetPointsBuilder", "GetPoints"
            )
        })
    }
}

impl GetPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> GetPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "GetPointsBuilder", "GetPoints"
            )
        })
    }
}

impl GetPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for GetPointsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum GetPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for GetPointsBuilderError {
    fn from(s: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(s.field_name())
    }
}

// Implementing the From trait for conversion from String
impl From<String> for GetPointsBuilderError {
    fn from(s: String) -> Self {
        Self::ValidationError(s)
    }
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for GetPointsBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(ref field) => {
                write!(f, "`{field}` must be initialized")
            }
            Self::ValidationError(ref error) => {
                write!(f, "{error}")
            }
        }
    }
}

// Implementing the Error trait
impl std::error::Error for GetPointsBuilderError {}
