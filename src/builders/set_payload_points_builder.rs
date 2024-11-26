use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct SetPayloadPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// New payload values
    pub(crate) payload: Option<::std::collections::HashMap<String, Value>>,
    /// Affected points
    points_selector: Option<points_selector::PointsSelectorOneOf>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// Option for indicate property of payload
    pub(crate) key: Option<Option<String>>,
}

impl SetPayloadPointsBuilder {
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
    /// New payload values
    #[allow(unused_mut)]
    pub fn payload(self, value: ::std::collections::HashMap<String, Value>) -> Self {
        let mut new = self;
        new.payload = Option::Some(value);
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
    /// Option for indicate property of payload
    #[allow(unused_mut)]
    pub fn key<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.key = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<SetPayloadPoints, SetPayloadPointsBuilderError> {
        Ok(SetPayloadPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            payload: match self.payload {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("payload"),
                    ));
                }
            },
            points_selector: { convert_option(&self.points_selector) },
            ordering: self.ordering.unwrap_or_default(),
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
            key: self.key.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            payload: core::default::Default::default(),
            points_selector: core::default::Default::default(),
            ordering: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            key: core::default::Default::default(),
        }
    }
}

impl From<SetPayloadPointsBuilder> for SetPayloadPoints {
    fn from(value: SetPayloadPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "SetPayloadPointsBuilder", "SetPayloadPoints"
            )
        })
    }
}

impl SetPayloadPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SetPayloadPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "SetPayloadPointsBuilder", "SetPayloadPoints"
            )
        })
    }
}

impl SetPayloadPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum SetPayloadPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for SetPayloadPointsBuilderError {
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
impl std::error::Error for SetPayloadPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for SetPayloadPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for SetPayloadPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
