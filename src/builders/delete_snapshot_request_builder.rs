use crate::qdrant::*;

#[derive(Clone)]
pub struct DeleteSnapshotRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Name of the collection snapshot
    pub(crate) snapshot_name: Option<String>,
}

impl DeleteSnapshotRequestBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Name of the collection snapshot
    #[allow(unused_mut)]
    pub fn snapshot_name(self, value: String) -> Self {
        let mut new = self;
        new.snapshot_name = Option::Some(value);
        new
    }

    fn build_inner(self) -> Result<DeleteSnapshotRequest, DeleteSnapshotRequestBuilderError> {
        Ok(DeleteSnapshotRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            snapshot_name: match self.snapshot_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("snapshot_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            snapshot_name: core::default::Default::default(),
        }
    }
}

impl From<DeleteSnapshotRequestBuilder> for DeleteSnapshotRequest {
    fn from(value: DeleteSnapshotRequestBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DeleteSnapshotRequestBuilder", "DeleteSnapshotRequest"
            )
        })
    }
}

impl DeleteSnapshotRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteSnapshotRequest {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DeleteSnapshotRequestBuilder", "DeleteSnapshotRequest"
            )
        })
    }
}

impl DeleteSnapshotRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DeleteSnapshotRequestBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DeleteSnapshotRequestBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DeleteSnapshotRequestBuilderError {
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
impl std::error::Error for DeleteSnapshotRequestBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DeleteSnapshotRequestBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DeleteSnapshotRequestBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
