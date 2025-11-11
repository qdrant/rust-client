use crate::qdrant::*;

#[derive(Clone)]
pub struct UpdatePointVectorsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// List of points and vectors to update
    pub(crate) points: Option<Vec<PointVectors>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// Optional filter to apply to the update operation. If set, only points matching the filter will be updated.
    pub(crate) update_filter: Option<Option<Filter>>,
}

impl UpdatePointVectorsBuilder {
    /// name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait until the changes have been applied?
    #[allow(unused_mut)]
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }
    /// List of points and vectors to update
    #[allow(unused_mut)]
    pub fn points(self, value: Vec<PointVectors>) -> Self {
        let mut new = self;
        new.points = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /// Option for custom sharding to specify used shard keys
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /// Optional filter to apply to the update operation. If set, only points matching the filter will be updated.
    #[allow(unused_mut)]
    pub fn update_filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.update_filter = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<UpdatePointVectors, UpdatePointVectorsBuilderError> {
        Ok(UpdatePointVectors {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: self.wait.unwrap_or_default(),
            points: match self.points {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("points"),
                    ));
                }
            },
            ordering: self.ordering.unwrap_or_default(),
            shard_key_selector: self.shard_key_selector.unwrap_or_default(),
            update_filter: self.update_filter.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            points: core::default::Default::default(),
            ordering: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            update_filter: core::default::Default::default(),
        }
    }
}

impl From<UpdatePointVectorsBuilder> for UpdatePointVectors {
    fn from(value: UpdatePointVectorsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UpdatePointVectorsBuilder", "UpdatePointVectors"
            )
        })
    }
}

impl UpdatePointVectorsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdatePointVectors {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UpdatePointVectorsBuilder", "UpdatePointVectors"
            )
        })
    }
}

impl UpdatePointVectorsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum UpdatePointVectorsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for UpdatePointVectorsBuilderError {
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
impl std::error::Error for UpdatePointVectorsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for UpdatePointVectorsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for UpdatePointVectorsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
