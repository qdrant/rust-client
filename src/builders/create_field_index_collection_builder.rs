use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct CreateFieldIndexCollectionBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Field name to index
    pub(crate) field_name: Option<String>,
    /// Field type.
    pub(crate) field_type: Option<Option<i32>>,
    /// Payload index params.
    field_index_params: Option<payload_index_params::IndexParams>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
}

impl CreateFieldIndexCollectionBuilder {
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
    /// Field name to index
    #[allow(unused_mut)]
    pub fn field_name(self, value: String) -> Self {
        let mut new = self;
        new.field_name = Option::Some(value);
        new
    }
    /// Field type.
    #[allow(unused_mut)]
    pub fn field_type<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.field_type = Option::Some(Option::Some(value.into()));
        new
    }
    /// Payload index params.
    #[allow(unused_mut)]
    pub fn field_index_params<VALUE: core::convert::Into<payload_index_params::IndexParams>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.field_index_params = Option::Some(value.into());
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(
        self,
    ) -> Result<CreateFieldIndexCollection, CreateFieldIndexCollectionBuilderError> {
        Ok(CreateFieldIndexCollection {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: match self.wait {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            field_name: match self.field_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("field_name"),
                    ));
                }
            },
            field_type: match self.field_type {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            field_index_params: { convert_option(&self.field_index_params) },
            ordering: match self.ordering {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            field_name: core::default::Default::default(),
            field_type: core::default::Default::default(),
            field_index_params: core::default::Default::default(),
            ordering: core::default::Default::default(),
        }
    }
}

impl From<CreateFieldIndexCollectionBuilder> for CreateFieldIndexCollection {
    fn from(value: CreateFieldIndexCollectionBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateFieldIndexCollectionBuilder", "CreateFieldIndexCollection",
        ))
    }
}

impl CreateFieldIndexCollectionBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateFieldIndexCollection {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateFieldIndexCollectionBuilder", "CreateFieldIndexCollection",
        ))
    }
}

impl CreateFieldIndexCollectionBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

/// Error type for CreateFieldIndexCollectionBuilder
#[non_exhaustive]
#[derive(Debug)]
pub enum CreateFieldIndexCollectionBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for CreateFieldIndexCollectionBuilderError {
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
impl std::error::Error for CreateFieldIndexCollectionBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for CreateFieldIndexCollectionBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for CreateFieldIndexCollectionBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
