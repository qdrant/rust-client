use crate::qdrant::*;

#[derive(Clone)]
pub struct ReplicaBuilder {
    pub(crate) shard_id: Option<u32>,
    pub(crate) peer_id: Option<u64>,
}

impl ReplicaBuilder {
    #[allow(unused_mut)]
    pub fn shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.shard_id = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.peer_id = Option::Some(value);
        new
    }

    fn build_inner(self) -> Result<Replica, ReplicaBuilderError> {
        Ok(Replica {
            shard_id: match self.shard_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("shard_id"),
                    ));
                }
            },
            peer_id: match self.peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("peer_id"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_id: core::default::Default::default(),
            peer_id: core::default::Default::default(),
        }
    }
}

impl From<ReplicaBuilder> for Replica {
    fn from(value: ReplicaBuilder) -> Self {
        value
            .build_inner()
            .unwrap_or_else(|_| panic!("Failed to convert {0} to {1}", "ReplicaBuilder", "Replica"))
    }
}

impl ReplicaBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> Replica {
        self.build_inner()
            .unwrap_or_else(|_| panic!("Failed to build {0} into {1}", "ReplicaBuilder", "Replica"))
    }
}

impl ReplicaBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for ReplicaBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum ReplicaBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for ReplicaBuilderError {
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
impl std::error::Error for ReplicaBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for ReplicaBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for ReplicaBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
