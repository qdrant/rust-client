use crate::qdrant::*;

pub struct CreateAliasBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// New name of the alias
    pub(crate) alias_name: Option<String>,
}

impl CreateAliasBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// New name of the alias
    #[allow(unused_mut)]
    pub fn alias_name(self, value: String) -> Self {
        let mut new = self;
        new.alias_name = Option::Some(value);
        new
    }

    fn build_inner(self) -> Result<CreateAlias, CreateAliasBuilderError> {
        Ok(CreateAlias {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            alias_name: match self.alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("alias_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            alias_name: core::default::Default::default(),
        }
    }
}

impl From<CreateAliasBuilder> for CreateAlias {
    fn from(value: CreateAliasBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateAliasBuilder", "CreateAlias",
        ))
    }
}

impl CreateAliasBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateAlias {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateAliasBuilder", "CreateAlias",
        ))
    }
}

impl CreateAliasBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for CreateAliasBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum CreateAliasBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for CreateAliasBuilderError {
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
impl std::error::Error for CreateAliasBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for CreateAliasBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for CreateAliasBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
