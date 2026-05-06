use crate::qdrant::*;

/// Dense vector creation parameters.
/// Only includes immutable properties that define the vector space.
/// Storage type, index, and quantization are configured separately.
#[must_use]
#[derive(Clone)]
pub struct DenseVectorCreationConfigBuilder {
    /// Size/dimensionality of the vectors
    pub(crate) size: Option<u64>,
    /// Distance function used for comparing vectors
    pub(crate) distance: Option<i32>,
    /// Configuration for multi-vector search (e.g., ColBERT)
    pub(crate) multivector_config: Option<Option<MultiVectorConfig>>,
    /// Data type of the vectors (Float32, Float16, Uint8)
    pub(crate) datatype: Option<Option<i32>>,
}

impl DenseVectorCreationConfigBuilder {
    /// Size/dimensionality of the vectors
    pub fn size(self, value: u64) -> Self {
        let mut new = self;
        new.size = Option::Some(value);
        new
    }

    /// Distance function used for comparing vectors
    pub fn distance<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.distance = Option::Some(value.into());
        new
    }

    /// Configuration for multi-vector search (e.g., ColBERT)
    pub fn multivector_config<VALUE: core::convert::Into<MultiVectorConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.multivector_config = Option::Some(Option::Some(value.into()));
        new
    }

    /// Data type of the vectors (Float32, Float16, Uint8)
    pub fn datatype<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.datatype = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(
        self,
    ) -> Result<DenseVectorCreationConfig, DenseVectorCreationConfigBuilderError> {
        Ok(DenseVectorCreationConfig {
            size: match self.size {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("size"),
                    ));
                }
            },
            distance: match self.distance {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("distance"),
                    ));
                }
            },
            multivector_config: self.multivector_config.unwrap_or_default(),
            datatype: self.datatype.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            size: core::default::Default::default(),
            distance: core::default::Default::default(),
            multivector_config: core::default::Default::default(),
            datatype: core::default::Default::default(),
        }
    }
}

impl From<DenseVectorCreationConfigBuilder> for DenseVectorCreationConfig {
    fn from(value: DenseVectorCreationConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DenseVectorCreationConfigBuilder", "DenseVectorCreationConfig"
            )
        })
    }
}

impl DenseVectorCreationConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DenseVectorCreationConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DenseVectorCreationConfigBuilder", "DenseVectorCreationConfig"
            )
        })
    }
}

impl DenseVectorCreationConfigBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for DenseVectorCreationConfigBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum DenseVectorCreationConfigBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DenseVectorCreationConfigBuilderError {
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
impl std::error::Error for DenseVectorCreationConfigBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DenseVectorCreationConfigBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DenseVectorCreationConfigBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
