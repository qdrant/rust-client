use crate::qdrant::*;

#[derive(Clone)]
pub struct DiscoverInputBuilder {
    /// Use this as the primary search objective
    pub(crate) target: Option<Option<VectorInput>>,
    /// Search space will be constrained by these pairs of vectors
    pub(crate) context: Option<Option<ContextInput>>,
}

impl DiscoverInputBuilder {
    /// Use this as the primary search objective
    #[allow(unused_mut)]
    pub fn target<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.target = Option::Some(Option::Some(value.into()));
        new
    }
    /// Search space will be constrained by these pairs of vectors
    #[allow(unused_mut)]
    pub fn context<VALUE: core::convert::Into<ContextInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.context = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<DiscoverInput, DiscoverInputBuilderError> {
        Ok(DiscoverInput {
            target: self.target.unwrap_or_default(),
            context: self.context.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            target: core::default::Default::default(),
            context: core::default::Default::default(),
        }
    }
}

impl From<DiscoverInputBuilder> for DiscoverInput {
    fn from(value: DiscoverInputBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DiscoverInputBuilder", "DiscoverInput"
            )
        })
    }
}

impl DiscoverInputBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DiscoverInput {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DiscoverInputBuilder", "DiscoverInput"
            )
        })
    }
}

impl DiscoverInputBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DiscoverInputBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DiscoverInputBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DiscoverInputBuilderError {
    fn from(s: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(s.field_name())
    }
}

// Implementing the From trait for conversion from String
impl From<String> for DiscoverInputBuilderError {
    fn from(s: String) -> Self {
        Self::ValidationError(s)
    }
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DiscoverInputBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(ref field) => {
                write!(f, "`{}` must be initialized", field)
            }
            Self::ValidationError(ref error) => write!(f, "{}", error),
        }
    }
}

// Implementing the Error trait
impl std::error::Error for DiscoverInputBuilderError {}
