use crate::qdrant::*;

#[derive(Clone)]
pub struct ContextInputPairBuilder {
    /// A positive vector
    pub(crate) positive: Option<Option<VectorInput>>,
    /// Repel from this vector
    pub(crate) negative: Option<Option<VectorInput>>,
}

impl ContextInputPairBuilder {
    /// A positive vector
    #[allow(unused_mut)]
    pub fn positive<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.positive = Option::Some(Option::Some(value.into()));
        new
    }
    /// Repel from this vector
    #[allow(unused_mut)]
    pub fn negative<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.negative = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<ContextInputPair, ContextInputPairBuilderError> {
        Ok(ContextInputPair {
            positive: self.positive.unwrap_or_default(),
            negative: self.negative.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            positive: core::default::Default::default(),
            negative: core::default::Default::default(),
        }
    }
}

impl From<ContextInputPairBuilder> for ContextInputPair {
    fn from(value: ContextInputPairBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "ContextInputPairBuilder", "ContextInputPair"
            )
        })
    }
}

impl ContextInputPairBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ContextInputPair {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "ContextInputPairBuilder", "ContextInputPair"
            )
        })
    }
}

impl ContextInputPairBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for ContextInputPairBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum ContextInputPairBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for ContextInputPairBuilderError {
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
impl std::error::Error for ContextInputPairBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for ContextInputPairBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for ContextInputPairBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
