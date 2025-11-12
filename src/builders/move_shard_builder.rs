use crate::qdrant::*;

#[derive(Clone)]
pub struct MoveShardBuilder {
    /// Local shard id
    pub(crate) shard_id: Option<u32>,
    pub(crate) to_shard_id: Option<Option<u32>>,
    pub(crate) from_peer_id: Option<u64>,
    pub(crate) to_peer_id: Option<u64>,
    pub(crate) method: Option<Option<i32>>,
}

impl MoveShardBuilder {
    /// Local shard id
    pub fn shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.shard_id = Option::Some(value);
        new
    }
    pub fn to_shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.to_shard_id = Option::Some(Option::Some(value));
        new
    }
    pub fn from_peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.from_peer_id = Option::Some(value);
        new
    }
    pub fn to_peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.to_peer_id = Option::Some(value);
        new
    }
    pub fn method<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.method = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<MoveShard, MoveShardBuilderError> {
        Ok(MoveShard {
            shard_id: match self.shard_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("shard_id"),
                    ));
                }
            },
            to_shard_id: self.to_shard_id.unwrap_or_default(),
            from_peer_id: match self.from_peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("from_peer_id"),
                    ));
                }
            },
            to_peer_id: match self.to_peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("to_peer_id"),
                    ));
                }
            },
            method: self.method.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_id: core::default::Default::default(),
            to_shard_id: core::default::Default::default(),
            from_peer_id: core::default::Default::default(),
            to_peer_id: core::default::Default::default(),
            method: core::default::Default::default(),
        }
    }
}

impl From<MoveShardBuilder> for MoveShard {
    fn from(value: MoveShardBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "MoveShardBuilder", "MoveShard"
            )
        })
    }
}

impl MoveShardBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> MoveShard {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "MoveShardBuilder", "MoveShard"
            )
        })
    }
}

impl MoveShardBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for MoveShardBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum MoveShardBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for MoveShardBuilderError {
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
impl std::error::Error for MoveShardBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for MoveShardBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for MoveShardBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
