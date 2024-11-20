use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct DiscoverBatchPointsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    pub(crate) discover_points: Option<Vec<DiscoverPoints>>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl DiscoverBatchPointsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn discover_points(self, value: Vec<DiscoverPoints>) -> Self {
        let mut new = self;
        new.discover_points = Option::Some(value);
        new
    }
    /// Options for specifying read consistency guarantees
    #[allow(unused_mut)]
    pub fn read_consistency<VALUE: core::convert::Into<read_consistency::Value>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(value.into());
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<DiscoverBatchPoints, DiscoverBatchPointsBuilderError> {
        Ok(DiscoverBatchPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            discover_points: match self.discover_points {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("discover_points"),
                    ));
                }
            },
            read_consistency: { convert_option(&self.read_consistency) },
            timeout: match self.timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            discover_points: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<DiscoverBatchPointsBuilder> for DiscoverBatchPoints {
    fn from(value: DiscoverBatchPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DiscoverBatchPointsBuilder", "DiscoverBatchPoints",
        ))
    }
}

impl DiscoverBatchPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DiscoverBatchPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DiscoverBatchPointsBuilder", "DiscoverBatchPoints",
        ))
    }
}

impl DiscoverBatchPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

// src/builders/discover_batch_points_builder.rs

#[non_exhaustive]
#[derive(Debug)]
pub enum DiscoverBatchPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for DiscoverBatchPointsBuilderError {
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
impl std::error::Error for DiscoverBatchPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for DiscoverBatchPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for DiscoverBatchPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
