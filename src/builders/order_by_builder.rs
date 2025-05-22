use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct OrderByBuilder {
    /// Payload key to order by
    pub(crate) key: Option<String>,
    /// Ascending or descending order
    pub(crate) direction: Option<Option<i32>>,
    /// Start from this value
    start_from: Option<start_from::Value>,
}

impl OrderByBuilder {
    /// Payload key to order by
    #[allow(unused_mut)]
    pub fn key(self, value: String) -> Self {
        let mut new = self;
        new.key = Option::Some(value);
        new
    }
    /// Ascending or descending order
    #[allow(unused_mut)]
    pub fn direction(self, value: i32) -> Self {
        let mut new = self;
        new.direction = Option::Some(Option::Some(value));
        new
    }
    /// Start from this value
    #[allow(unused_mut)]
    pub fn start_from<VALUE: core::convert::Into<start_from::Value>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.start_from = Option::Some(value.into());
        new
    }

    fn build_inner(self) -> Result<OrderBy, OrderByBuilderError> {
        Ok(OrderBy {
            key: match self.key {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("key"),
                    ));
                }
            },
            direction: self.direction.unwrap_or_default(),
            start_from: { convert_option(&self.start_from) },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            key: core::default::Default::default(),
            direction: core::default::Default::default(),
            start_from: core::default::Default::default(),
        }
    }
}

impl From<OrderByBuilder> for OrderBy {
    fn from(value: OrderByBuilder) -> Self {
        value
            .build_inner()
            .unwrap_or_else(|_| panic!("Failed to convert {0} to {1}", "OrderByBuilder", "OrderBy"))
    }
}

impl OrderByBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> OrderBy {
        self.build_inner()
            .unwrap_or_else(|_| panic!("Failed to build {0} into {1}", "OrderByBuilder", "OrderBy"))
    }
}

impl OrderByBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for OrderByBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum OrderByBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for OrderByBuilderError {
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
impl std::error::Error for OrderByBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for OrderByBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for OrderByBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
