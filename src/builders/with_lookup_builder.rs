use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct WithLookupBuilder {
    /// Name of the collection to use for points lookup
    pub(crate) collection: Option<String>,
    /// Options for specifying which payload to include (or not)
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Options for specifying which vectors to include (or not)
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
}

impl WithLookupBuilder {
    /// Name of the collection to use for points lookup
    #[allow(unused_mut)]
    pub fn collection(self, value: String) -> Self {
        let mut new = self;
        new.collection = Option::Some(value);
        new
    }
    /// Options for specifying which payload to include (or not)
    #[allow(unused_mut)]
    pub fn with_payload<VALUE: core::convert::Into<with_payload_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(value.into());
        new
    }
    /// Options for specifying which vectors to include (or not)
    #[allow(unused_mut)]
    pub fn with_vectors<VALUE: core::convert::Into<with_vectors_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(value.into());
        new
    }

    fn build_inner(self) -> Result<WithLookup, WithLookupBuilderError> {
        Ok(WithLookup {
            collection: match self.collection {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection"),
                    ));
                }
            },
            with_payload: { convert_option(&self.with_payload) },
            with_vectors: { convert_option(&self.with_vectors) },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
        }
    }
}

impl From<WithLookupBuilder> for WithLookup {
    fn from(value: WithLookupBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "WithLookupBuilder", "WithLookup"
            )
        })
    }
}

impl WithLookupBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> WithLookup {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "WithLookupBuilder", "WithLookup"
            )
        })
    }
}

impl WithLookupBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum WithLookupBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for WithLookupBuilderError {
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
impl std::error::Error for WithLookupBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for WithLookupBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for WithLookupBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
