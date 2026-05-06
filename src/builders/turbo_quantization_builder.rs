use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct TurboQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
    pub(crate) bits: Option<Option<i32>>,
    pub(crate) data_fit: Option<Option<bool>>,
}

impl TurboQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Some(Some(value));
        new
    }

    /// Number of bits used to encode each component of the quantized vector.
    pub fn bits(self, value: impl Into<TurboQuantBitSize>) -> Self {
        let mut new = self;
        let bits: TurboQuantBitSize = value.into();
        new.bits = Some(Some(bits.into()));
        new
    }

    #[doc(hidden)]
    /// Disable data-fit which is usually enabled.
    pub fn data_fit_disabled(self) -> Self {
        let mut new = self;
        new.data_fit = Some(Some(false));
        new
    }

    fn build_inner(self) -> Result<TurboQuantization, TurboQuantizationBuilderError> {
        Ok(TurboQuantization {
            always_ram: self.always_ram.unwrap_or_default(),
            bits: self.bits.unwrap_or_default(),
            data_fit: self.data_fit.unwrap_or_default(),
        })
    }

    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            always_ram: Default::default(),
            bits: Default::default(),
            data_fit: Default::default(),
        }
    }
}

impl From<TurboQuantizationBuilder> for TurboQuantization {
    fn from(value: TurboQuantizationBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "TurboQuantizationBuilder", "TurboQuantization"
            )
        })
    }
}

impl TurboQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> TurboQuantization {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "TurboQuantizationBuilder", "TurboQuantization"
            )
        })
    }
}

impl TurboQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

impl Default for TurboQuantizationBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TurboQuantizationBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for TurboQuantizationBuilderError {
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
impl std::error::Error for TurboQuantizationBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for TurboQuantizationBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for TurboQuantizationBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
