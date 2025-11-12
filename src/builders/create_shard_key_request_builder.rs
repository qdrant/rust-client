use crate::qdrant::*;

#[derive(Clone)]
pub struct CreateShardKeyRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Request to create shard key
    pub(crate) request: Option<Option<CreateShardKey>>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
}

impl CreateShardKeyRequestBuilder {
    /// Name of the collection
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Request to create shard key
    pub fn request<VALUE: core::convert::Into<CreateShardKey>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.request = Option::Some(Option::Some(value.into()));
        new
    }
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<CreateShardKeyRequest, CreateShardKeyRequestBuilderError> {
        Ok(CreateShardKeyRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            request: self.request.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            request: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<CreateShardKeyRequestBuilder> for CreateShardKeyRequest {
    fn from(value: CreateShardKeyRequestBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "CreateShardKeyRequestBuilder", "CreateShardKeyRequest"
            )
        })
    }
}

impl CreateShardKeyRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateShardKeyRequest {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "CreateShardKeyRequestBuilder", "CreateShardKeyRequest"
            )
        })
    }
}

impl CreateShardKeyRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for CreateShardKeyRequestBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum CreateShardKeyRequestBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for CreateShardKeyRequestBuilderError {
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
impl std::error::Error for CreateShardKeyRequestBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for CreateShardKeyRequestBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for CreateShardKeyRequestBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
