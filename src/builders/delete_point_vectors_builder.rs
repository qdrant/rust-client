use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct DeletePointVectorsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Affected points
    points_selector: Option<points_selector::PointsSelectorOneOf>,
    /// List of vector names to delete
    pub(crate) vectors: Option<Option<VectorsSelector>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl DeletePointVectorsBuilder {
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
    /// Affected points
    pub fn points_selector<VALUE: core::convert::Into<points_selector::PointsSelectorOneOf>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.points_selector = Option::Some(value.into());
        new
    }
    /// List of vector names to delete
    pub fn vectors<VALUE: core::convert::Into<VectorsSelector>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.vectors = Option::Some(Option::Some(value.into()));
        new
    }
    /// Write ordering guarantees
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /// Option for custom sharding to specify used shard keys
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<DeletePointVectors, DeletePointVectorsBuilderError> {
        Ok(DeletePointVectors {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            points_selector: { convert_option(&self.points_selector) },
            vectors: self.vectors.unwrap_or_default(),
            ordering: self.ordering.unwrap_or_default(),
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            points_selector: core::default::Default::default(),
            vectors: core::default::Default::default(),
            ordering: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<DeletePointVectorsBuilder> for DeletePointVectors {
    fn from(value: DeletePointVectorsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DeletePointVectorsBuilder", "DeletePointVectors"
            )
        })
    }
}

impl DeletePointVectorsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeletePointVectors {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DeletePointVectorsBuilder", "DeletePointVectors"
            )
        })
    }
}

impl DeletePointVectorsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DeletePointVectorsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DeletePointVectorsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DeletePointVectorsBuilderError {
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
impl std::error::Error for DeletePointVectorsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DeletePointVectorsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DeletePointVectorsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
