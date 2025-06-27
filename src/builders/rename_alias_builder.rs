use crate::qdrant::*;

#[derive(Clone)]
pub struct RenameAliasBuilder {
    /// Name of the alias to rename
    pub(crate) old_alias_name: Option<String>,
    /// Name of the alias
    pub(crate) new_alias_name: Option<String>,
}

impl RenameAliasBuilder {
    /// Name of the alias to rename
    #[allow(unused_mut)]
    pub fn old_alias_name(self, value: String) -> Self {
        let mut new = self;
        new.old_alias_name = Option::Some(value);
        new
    }
    /// Name of the alias
    #[allow(unused_mut)]
    pub fn new_alias_name(self, value: String) -> Self {
        let mut new = self;
        new.new_alias_name = Option::Some(value);
        new
    }

    fn build_inner(self) -> Result<RenameAlias, RenameAliasBuilderError> {
        Ok(RenameAlias {
            old_alias_name: match self.old_alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("old_alias_name"),
                    ));
                }
            },
            new_alias_name: match self.new_alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("new_alias_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            old_alias_name: core::default::Default::default(),
            new_alias_name: core::default::Default::default(),
        }
    }
}

impl From<RenameAliasBuilder> for RenameAlias {
    fn from(value: RenameAliasBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "RenameAliasBuilder", "RenameAlias"
            )
        })
    }
}

impl RenameAliasBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> RenameAlias {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "RenameAliasBuilder", "RenameAlias"
            )
        })
    }
}

impl RenameAliasBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for RenameAliasBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum RenameAliasBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for RenameAliasBuilderError {
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
impl std::error::Error for RenameAliasBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for RenameAliasBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for RenameAliasBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
