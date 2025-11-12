use crate::qdrant::*;

#[derive(Clone)]
pub struct LookupLocationBuilder {
    pub(crate) collection_name: Option<String>,
    /// Which vector to use for search, if not specified - use default vector
    pub(crate) vector_name: Option<Option<String>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl LookupLocationBuilder {
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Which vector to use for search, if not specified - use default vector
    pub fn vector_name<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.vector_name = Option::Some(Option::Some(value.into()));
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<LookupLocation, LookupLocationBuilderError> {
        Ok(LookupLocation {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            vector_name: self.vector_name.unwrap_or_default(),
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            vector_name: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<LookupLocationBuilder> for LookupLocation {
    fn from(value: LookupLocationBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "LookupLocationBuilder", "LookupLocation"
            )
        })
    }
}

impl LookupLocationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> LookupLocation {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "LookupLocationBuilder", "LookupLocation"
            )
        })
    }
}

impl LookupLocationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for LookupLocationBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum LookupLocationBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for LookupLocationBuilderError {
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
impl std::error::Error for LookupLocationBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for LookupLocationBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for LookupLocationBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
