use crate::qdrant::*;

pub struct ScalarQuantizationBuilder {
    /// Type of quantization
    pub(crate) r#type: Option<i32>,
    /// Number of bits to use for quantization
    pub(crate) quantile: Option<Option<f32>>,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl ScalarQuantizationBuilder {
    /// Type of quantization
    #[allow(unused_mut)]
    pub fn r#type(self, value: i32) -> Self {
        let mut new = self;
        new.r#type = Option::Some(value);
        new
    }
    /// Number of bits to use for quantization
    #[allow(unused_mut)]
    pub fn quantile(self, value: f32) -> Self {
        let mut new = self;
        new.quantile = Option::Some(Option::Some(value));
        new
    }
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<ScalarQuantization, ScalarQuantizationBuilderError> {
        Ok(ScalarQuantization {
            r#type: match self.r#type {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("r#type"),
                    ));
                }
            },
            quantile: self.quantile.unwrap_or_default(),
            always_ram: self.always_ram.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            r#type: core::default::Default::default(),
            quantile: core::default::Default::default(),
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<ScalarQuantizationBuilder> for ScalarQuantization {
    fn from(value: ScalarQuantizationBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "ScalarQuantizationBuilder", "ScalarQuantization"
            )
        })
    }
}

impl ScalarQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ScalarQuantization {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "ScalarQuantizationBuilder", "ScalarQuantization"
            )
        })
    }
}

impl ScalarQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum ScalarQuantizationBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for ScalarQuantizationBuilderError {
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
impl std::error::Error for ScalarQuantizationBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for ScalarQuantizationBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for ScalarQuantizationBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
