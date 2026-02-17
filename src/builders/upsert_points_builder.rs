use crate::qdrant::*;

#[derive(Clone)]
pub struct UpsertPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    pub(crate) points: Option<Vec<PointStruct>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// Optional filter to apply to the upsert operation. If set, only points matching the filter will be updated, others will be inserted.
    pub(crate) update_filter: Option<Option<Filter>>,
    /// Timeout for the request in seconds
    pub(crate) timeout: Option<Option<u64>>,
    /// Mode of the upsert operation: insert_only, upsert (default), update_only
    pub(crate) update_mode: Option<Option<i32>>,
}

impl UpsertPointsBuilder {
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
    pub fn points(self, value: Vec<PointStruct>) -> Self {
        let mut new = self;
        new.points = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /// Option for custom sharding to specify used shard keys
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /// Optional filter to apply to the upsert operation. If set, only points matching the filter will be updated, others will be inserted.
    pub fn update_filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.update_filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Timeout for the request in seconds
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /// Mode of the upsert operation: insert_only, upsert (default), update_only
    pub fn update_mode(self, value: UpdateMode) -> Self {
        let mut new = self;
        new.update_mode = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<UpsertPoints, UpsertPointsBuilderError> {
        Ok(UpsertPoints {
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
            timeout: self.timeout.unwrap_or_default(),
            update_mode: self.update_mode.unwrap_or_default(),
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
            timeout: core::default::Default::default(),
            update_mode: core::default::Default::default(),
        }
    }
}

impl From<UpsertPointsBuilder> for UpsertPoints {
    fn from(value: UpsertPointsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UpsertPointsBuilder", "UpsertPoints"
            )
        })
    }
}

impl UpsertPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpsertPoints {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UpsertPointsBuilder", "UpsertPoints"
            )
        })
    }
}

impl UpsertPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum UpsertPointsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for UpsertPointsBuilderError {
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
impl std::error::Error for UpsertPointsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for UpsertPointsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for UpsertPointsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
