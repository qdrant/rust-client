use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct DeletePayloadPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// List of keys to delete
    pub(crate) keys: Option<Vec<String>>,
    /// Affected points
    points_selector: Option<points_selector::PointsSelectorOneOf>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl DeletePayloadPointsBuilder {
    /// name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait until the changes have been applied?
    #[allow(unused_mut)]
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }
    /// List of keys to delete
    #[allow(unused_mut)]
    pub fn keys(self, value: Vec<String>) -> Self {
        let mut new = self;
        new.keys = Option::Some(value);
        new
    }
    /// Affected points
    #[allow(unused_mut)]
    pub fn points_selector<VALUE: core::convert::Into<points_selector::PointsSelectorOneOf>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.points_selector = Option::Some(value.into());
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /// Option for custom sharding to specify used shard keys
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<DeletePayloadPoints, DeletePayloadPointsBuilderError> {
        Ok(DeletePayloadPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: match self.wait {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            keys: match self.keys {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("keys"),
                    ));
                }
            },
            points_selector: { convert_option(&self.points_selector) },
            ordering: match self.ordering {
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
            wait: core::default::Default::default(),
            keys: core::default::Default::default(),
            points_selector: core::default::Default::default(),
            ordering: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<DeletePayloadPointsBuilder> for DeletePayloadPoints {
    fn from(value: DeletePayloadPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DeletePayloadPointsBuilder", "DeletePayloadPoints",
        ))
    }
}

impl DeletePayloadPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeletePayloadPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DeletePayloadPointsBuilder", "DeletePayloadPoints",
        ))
    }
}

impl DeletePayloadPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DeletePayloadPointsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DeletePayloadPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DeletePayloadPointsBuilderError {
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
impl std::error::Error for DeletePayloadPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DeletePayloadPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DeletePayloadPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
