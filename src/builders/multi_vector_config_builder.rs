use crate::qdrant::*;

#[derive(Clone)]
pub struct MultiVectorConfigBuilder {
    /// Comparator for multi-vector search
    pub(crate) comparator: Option<i32>,
}

impl MultiVectorConfigBuilder {
    /// Comparator for multi-vector search
    #[allow(unused_mut)]
    pub fn comparator<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.comparator = Option::Some(value.into());
        new
    }

    fn build_inner(self) -> Result<MultiVectorConfig, MultiVectorConfigBuilderError> {
        Ok(MultiVectorConfig {
            comparator: self.comparator.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            comparator: core::default::Default::default(),
        }
    }
}

impl From<MultiVectorConfigBuilder> for MultiVectorConfig {
    fn from(value: MultiVectorConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "MultiVectorConfigBuilder", "MultiVectorConfig"
            )
        })
    }
}

impl MultiVectorConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> MultiVectorConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "MultiVectorConfigBuilder", "MultiVectorConfig"
            )
        })
    }
}

impl MultiVectorConfigBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for MultiVectorConfigBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum MultiVectorConfigBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for MultiVectorConfigBuilderError {
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
impl std::error::Error for MultiVectorConfigBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for MultiVectorConfigBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for MultiVectorConfigBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
