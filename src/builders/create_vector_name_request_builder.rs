use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct CreateVectorNameRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Name of the new vector
    pub(crate) vector_name: Option<String>,
    /// Configuration for the new vector - either dense or sparse
    vector_config: Option<create_vector_name_request::VectorConfig>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
}

impl CreateVectorNameRequestBuilder {
    /// Name of the collection
    pub fn collection_name(self, value: impl Into<String>) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value.into());
        new
    }

    /// Wait until the changes have been applied?
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }

    /// Name of the new vector
    pub fn vector_name(self, value: impl Into<String>) -> Self {
        let mut new = self;
        new.vector_name = Option::Some(value.into());
        new
    }

    /// Configuration for the new vector - either dense or sparse.
    ///
    /// Accepts either a [`DenseVectorCreationConfig`] (or its builder)
    /// or a [`SparseVectorCreationConfig`] (or its builder).
    pub fn vector_config<VALUE: core::convert::Into<create_vector_name_request::VectorConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.vector_config = Option::Some(value.into());
        new
    }

    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    /// Write ordering guarantees
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<CreateVectorNameRequest, CreateVectorNameRequestBuilderError> {
        Ok(CreateVectorNameRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            vector_name: match self.vector_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("vector_name"),
                    ));
                }
            },
            timeout: self.timeout.unwrap_or_default(),
            ordering: self.ordering.unwrap_or_default(),
            vector_config: { convert_option(&self.vector_config) },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            vector_name: core::default::Default::default(),
            vector_config: core::default::Default::default(),
            timeout: core::default::Default::default(),
            ordering: core::default::Default::default(),
        }
    }
}

impl From<CreateVectorNameRequestBuilder> for CreateVectorNameRequest {
    fn from(value: CreateVectorNameRequestBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "CreateVectorNameRequestBuilder", "CreateVectorNameRequest"
            )
        })
    }
}

impl CreateVectorNameRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateVectorNameRequest {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "CreateVectorNameRequestBuilder", "CreateVectorNameRequest"
            )
        })
    }
}

impl CreateVectorNameRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for CreateVectorNameRequestBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum CreateVectorNameRequestBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for CreateVectorNameRequestBuilderError {
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
impl std::error::Error for CreateVectorNameRequestBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for CreateVectorNameRequestBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for CreateVectorNameRequestBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
