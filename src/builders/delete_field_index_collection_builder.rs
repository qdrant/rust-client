use crate::qdrant::*;

#[derive(Clone)]
pub struct DeleteFieldIndexCollectionBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Field name to delete
    pub(crate) field_name: Option<String>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Timeout for the request in seconds
    pub(crate) timeout: Option<Option<u64>>,
}

impl DeleteFieldIndexCollectionBuilder {
    /// name of the collection
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait until the changes have been applied?
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }
    /// Field name to delete
    pub fn field_name(self, value: String) -> Self {
        let mut new = self;
        new.field_name = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    pub fn ordering(self, value: WriteOrdering) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value));
        new
    }
    /// Timeout for the request in seconds
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(
        self,
    ) -> Result<DeleteFieldIndexCollection, DeleteFieldIndexCollectionBuilderError> {
        Ok(DeleteFieldIndexCollection {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            field_name: match self.field_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("field_name"),
                    ));
                }
            },
            ordering: self.ordering.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            field_name: core::default::Default::default(),
            ordering: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<DeleteFieldIndexCollectionBuilder> for DeleteFieldIndexCollection {
    fn from(value: DeleteFieldIndexCollectionBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DeleteFieldIndexCollectionBuilder", "DeleteFieldIndexCollection"
            )
        })
    }
}

impl DeleteFieldIndexCollectionBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteFieldIndexCollection {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DeleteFieldIndexCollectionBuilder", "DeleteFieldIndexCollection"
            )
        })
    }
}

impl DeleteFieldIndexCollectionBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DeleteFieldIndexCollectionBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DeleteFieldIndexCollectionBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DeleteFieldIndexCollectionBuilderError {
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
impl std::error::Error for DeleteFieldIndexCollectionBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DeleteFieldIndexCollectionBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DeleteFieldIndexCollectionBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
