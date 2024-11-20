use crate::qdrant::*;

pub struct TextIndexParamsBuilder {
    /// Tokenizer type
    pub(crate) tokenizer: Option<i32>,
    /// If true - all tokens will be lowercase
    pub(crate) lowercase: Option<Option<bool>>,
    /// Minimal token length
    pub(crate) min_token_len: Option<Option<u64>>,
    /// Maximal token length
    pub(crate) max_token_len: Option<Option<u64>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl TextIndexParamsBuilder {
    /// Tokenizer type
    #[allow(unused_mut)]
    pub fn tokenizer(self, value: i32) -> Self {
        let mut new = self;
        new.tokenizer = Option::Some(value);
        new
    }
    /// If true - all tokens will be lowercase
    #[allow(unused_mut)]
    pub fn lowercase(self, value: bool) -> Self {
        let mut new = self;
        new.lowercase = Option::Some(Option::Some(value));
        new
    }
    /// Minimal token length
    #[allow(unused_mut)]
    pub fn min_token_len(self, value: u64) -> Self {
        let mut new = self;
        new.min_token_len = Option::Some(Option::Some(value));
        new
    }
    /// Maximal token length
    #[allow(unused_mut)]
    pub fn max_token_len(self, value: u64) -> Self {
        let mut new = self;
        new.max_token_len = Option::Some(Option::Some(value));
        new
    }
    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<TextIndexParams, TextIndexParamsBuilderError> {
        Ok(TextIndexParams {
            tokenizer: match self.tokenizer {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("tokenizer"),
                    ));
                }
            },
            lowercase: match self.lowercase {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            min_token_len: match self.min_token_len {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_token_len: match self.max_token_len {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            tokenizer: core::default::Default::default(),
            lowercase: core::default::Default::default(),
            min_token_len: core::default::Default::default(),
            max_token_len: core::default::Default::default(),
            on_disk: core::default::Default::default(),
        }
    }
}

impl From<TextIndexParamsBuilder> for TextIndexParams {
    fn from(value: TextIndexParamsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "TextIndexParamsBuilder", "TextIndexParams",
        ))
    }
}

impl TextIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> TextIndexParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "TextIndexParamsBuilder", "TextIndexParams",
        ))
    }
}

impl TextIndexParamsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum TextIndexParamsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for TextIndexParamsBuilderError {
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
impl std::error::Error for TextIndexParamsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for TextIndexParamsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for TextIndexParamsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
