use crate::qdrant::*;

#[derive(Clone)]
pub struct ProductQuantizationBuilder {
    /// Compression ratio
    pub(crate) compression: Option<i32>,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl ProductQuantizationBuilder {
    /// Compression ratio
    #[allow(unused_mut)]
    pub fn compression(self, value: i32) -> Self {
        let mut new = self;
        new.compression = Option::Some(value);
        new
    }
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<ProductQuantization, ProductQuantizationBuilderError> {
        Ok(ProductQuantization {
            compression: match self.compression {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("compression"),
                    ));
                }
            },
            always_ram: self.always_ram.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            compression: core::default::Default::default(),
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<ProductQuantizationBuilder> for ProductQuantization {
    fn from(value: ProductQuantizationBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "ProductQuantizationBuilder", "ProductQuantization"
            )
        })
    }
}

impl ProductQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ProductQuantization {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "ProductQuantizationBuilder", "ProductQuantization"
            )
        })
    }
}

impl ProductQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for ProductQuantizationBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum ProductQuantizationBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for ProductQuantizationBuilderError {
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
impl std::error::Error for ProductQuantizationBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for ProductQuantizationBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for ProductQuantizationBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
