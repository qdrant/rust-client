use crate::qdrant::*;

#[derive(Clone)]
pub struct BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<BinaryQuantization, BinaryQuantizationBuilderError> {
        Ok(BinaryQuantization {
            always_ram: self.always_ram.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<BinaryQuantizationBuilder> for BinaryQuantization {
    fn from(value: BinaryQuantizationBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "BinaryQuantizationBuilder", "BinaryQuantization"
            )
        })
    }
}

impl BinaryQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> BinaryQuantization {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "BinaryQuantizationBuilder", "BinaryQuantization"
            )
        })
    }
}

impl BinaryQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

// src/builders/binary_quantization_builder.rs

#[non_exhaustive]
#[derive(Debug)]
pub enum BinaryQuantizationBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for BinaryQuantizationBuilderError {
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
impl std::error::Error for BinaryQuantizationBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for BinaryQuantizationBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for BinaryQuantizationBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
