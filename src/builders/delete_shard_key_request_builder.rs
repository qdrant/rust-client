use crate::qdrant::*;

#[derive(Clone)]
pub struct DeleteShardKeyRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Request to delete shard key
    pub(crate) request: Option<Option<DeleteShardKey>>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
}

impl DeleteShardKeyRequestBuilder {
    /// Name of the collection
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<DeleteShardKeyRequest, DeleteShardKeyRequestBuilderError> {
        Ok(DeleteShardKeyRequest {
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

impl From<DeleteShardKeyRequestBuilder> for DeleteShardKeyRequest {
    fn from(value: DeleteShardKeyRequestBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DeleteShardKeyRequestBuilder", "DeleteShardKeyRequest"
            )
        })
    }
}

impl DeleteShardKeyRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteShardKeyRequest {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DeleteShardKeyRequestBuilder", "DeleteShardKeyRequest"
            )
        })
    }
}

impl DeleteShardKeyRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DeleteShardKeyRequestBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DeleteShardKeyRequestBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DeleteShardKeyRequestBuilderError {
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
impl std::error::Error for DeleteShardKeyRequestBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DeleteShardKeyRequestBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DeleteShardKeyRequestBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
