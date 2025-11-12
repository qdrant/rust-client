use crate::qdrant::*;

#[derive(Clone)]
pub struct UpdateBatchPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    pub(crate) operations: Option<Vec<PointsUpdateOperation>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
}

impl UpdateBatchPointsBuilder {
    /// name of the collection
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait until the changes have been applied?
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }
    pub fn operations(self, value: Vec<PointsUpdateOperation>) -> Self {
        let mut new = self;
        new.operations = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<UpdateBatchPoints, UpdateBatchPointsBuilderError> {
        Ok(UpdateBatchPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            operations: match self.operations {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("operations"),
                    ));
                }
            },
            ordering: self.ordering.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            operations: core::default::Default::default(),
            ordering: core::default::Default::default(),
        }
    }
}

impl From<UpdateBatchPointsBuilder> for UpdateBatchPoints {
    fn from(value: UpdateBatchPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UpdateBatchPointsBuilder", "UpdateBatchPoints"
            )
        })
    }
}

impl UpdateBatchPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdateBatchPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UpdateBatchPointsBuilder", "UpdateBatchPoints"
            )
        })
    }
}

impl UpdateBatchPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum UpdateBatchPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for UpdateBatchPointsBuilderError {
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
impl std::error::Error for UpdateBatchPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for UpdateBatchPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for UpdateBatchPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
