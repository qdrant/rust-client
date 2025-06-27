use crate::qdrant::*;

#[derive(Clone)]
pub struct IntegerIndexParamsBuilder {
    /// If true - support direct lookups.
    pub(crate) lookup: Option<Option<bool>>,
    /// If true - support ranges filters.
    pub(crate) range: Option<Option<bool>>,
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    pub(crate) is_principal: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl IntegerIndexParamsBuilder {
    pub fn new(lookup: bool, range: bool) -> Self {
        Self::create_empty().lookup(lookup).range(range)
    }

    /// If true - support direct lookups.
    #[allow(unused_mut)]
    pub fn lookup<VALUE: core::convert::Into<bool>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup = Option::Some(Option::Some(value.into()));
        new
    }
    /// If true - support ranges filters.
    #[allow(unused_mut)]
    pub fn range<VALUE: core::convert::Into<bool>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.range = Option::Some(Option::Some(value.into()));
        new
    }
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    #[allow(unused_mut)]
    pub fn is_principal(self, value: bool) -> Self {
        let mut new = self;
        new.is_principal = Option::Some(Option::Some(value));
        new
    }
    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<IntegerIndexParams, IntegerIndexParamsBuilderError> {
        Ok(IntegerIndexParams {
            lookup: self.lookup.unwrap_or_default(),
            range: self.range.unwrap_or_default(),
            is_principal: self.is_principal.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            lookup: core::default::Default::default(),
            range: core::default::Default::default(),
            is_principal: core::default::Default::default(),
            on_disk: core::default::Default::default(),
        }
    }
}

impl From<IntegerIndexParamsBuilder> for IntegerIndexParams {
    fn from(value: IntegerIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "IntegerIndexParamsBuilder", "IntegerIndexParams"
            )
        })
    }
}

impl IntegerIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> IntegerIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "IntegerIndexParamsBuilder", "IntegerIndexParams"
            )
        })
    }
}

impl Default for IntegerIndexParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

/// Error type for IntegerIndexParamsBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum IntegerIndexParamsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for IntegerIndexParamsBuilderError {
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
impl std::error::Error for IntegerIndexParamsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for IntegerIndexParamsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for IntegerIndexParamsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
