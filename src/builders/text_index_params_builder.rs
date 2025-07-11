use crate::qdrant::*;

#[derive(Clone)]
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
    pub(crate) stopwords: Option<Option<StopwordsSet>>,
    /// If true - support phrase matching.
    pub(crate) phrase_matching: Option<Option<bool>>,
    /// Set an algorithm for stemming.
    pub(crate) stemmer: Option<Option<StemmingAlgorithm>>,
}

impl TextIndexParamsBuilder {
    pub fn new(tokenizer: TokenizerType) -> Self {
        let mut builder = Self::create_empty();
        builder.tokenizer = Some(tokenizer.into());
        builder
    }

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

    /// Stopwords for a single language for the text index
    pub fn stopwords_language(self, language: String) -> Self {
        let mut new = self;
        let stopwords_set = StopwordsSet {
            languages: vec![language],
            custom: vec![],
        };
        new.stopwords = Some(Some(stopwords_set));
        new
    }

    /// Stopwords for the text index
    pub fn stopwords(self, stopwords_set: StopwordsSet) -> Self {
        let mut new = self;
        new.stopwords = Some(Some(stopwords_set));
        new
    }

    /// If true - support phrase matching. Default is false.
    pub fn phrase_matching(self, phrase_matching: bool) -> Self {
        let mut new = self;
        new.phrase_matching = Some(Some(phrase_matching));
        new
    }

    /// Set snowball stemmer with the provided language
    pub fn stemmer_language(self, language: String) -> Self {
        let mut new = self;
        let stemmer = StemmingAlgorithm {
            stemming_params: Some(stemming_algorithm::StemmingParams::Snowball(
                SnowballParams { language },
            )),
        };
        new.stemmer = Some(Some(stemmer));
        new
    }

    /// Set an algorithm for stemming.
    pub fn stemmer(self, stemming_params: stemming_algorithm::StemmingParams) -> Self {
        let mut new = self;
        let stemmer = StemmingAlgorithm {
            stemming_params: Some(stemming_params),
        };
        new.stemmer = Some(Some(stemmer));
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
            lowercase: self.lowercase.unwrap_or_default(),
            min_token_len: self.min_token_len.unwrap_or_default(),
            max_token_len: self.max_token_len.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
            stopwords: self.stopwords.unwrap_or_default(),
            phrase_matching: self.phrase_matching.unwrap_or_default(),
            stemmer: self.stemmer.unwrap_or_default(),
        })
    }

    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            tokenizer: Default::default(),
            lowercase: Default::default(),
            min_token_len: Default::default(),
            max_token_len: Default::default(),
            on_disk: Default::default(),
            stopwords: Default::default(),
            phrase_matching: Default::default(),
            stemmer: Default::default(),
        }
    }
}

impl From<TextIndexParamsBuilder> for TextIndexParams {
    fn from(value: TextIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "TextIndexParamsBuilder", "TextIndexParams"
            )
        })
    }
}

impl TextIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> TextIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "TextIndexParamsBuilder", "TextIndexParams"
            )
        })
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
                write!(f, "`{field}` must be initialized")
            }
            Self::ValidationError(error) => write!(f, "{error}"),
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
