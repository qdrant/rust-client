use crate::qdrant::*;

#[derive(Clone)]
pub struct BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
    pub(crate) encoding: Option<Option<i32>>,
    pub(crate) query_encoding: Option<Option<BinaryQuantizationQueryEncoding>>,
}

impl BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Some(Some(value));
        new
    }

    /// Binary quantization encoding method
    pub fn encoding(self, value: impl Into<BinaryQuantizationEncoding>) -> Self {
        let mut new = self;
        let encoding: BinaryQuantizationEncoding = value.into();
        new.encoding = Some(Some(encoding.into()));
        new
    }

    /// Asymmetric quantization configuration allows a query to have different quantization than stored vectors.
    /// It can increase the accuracy of search at the cost of performance.
    pub fn query_encoding(self, value: impl Into<BinaryQuantizationQueryEncoding>) -> Self {
        let mut new = self;
        new.query_encoding = Some(Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<BinaryQuantization, BinaryQuantizationBuilderError> {
        Ok(BinaryQuantization {
            always_ram: self.always_ram.unwrap_or_default(),
            encoding: self.encoding.unwrap_or_default(),
            query_encoding: self.query_encoding.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            always_ram: Default::default(),
            encoding: Default::default(),
            query_encoding: Default::default(),
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
                write!(f, "`{field}` must be initialized")
            }
            Self::ValidationError(error) => write!(f, "{error}"),
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

impl BinaryQuantizationQueryEncoding {
    pub fn new_setting(setting: binary_quantization_query_encoding::Setting) -> Self {
        Self {
            variant: Some(binary_quantization_query_encoding::Variant::Setting(setting.into())),
        }
    }
}
