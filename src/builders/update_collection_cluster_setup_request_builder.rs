use crate::qdrant::*;

#[derive(Clone)]
pub struct UpdateCollectionClusterSetupRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
    pub(crate) operation: Option<Option<update_collection_cluster_setup_request::Operation>>,
}

impl UpdateCollectionClusterSetupRequestBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn operation(
        self,
        value: Option<update_collection_cluster_setup_request::Operation>,
    ) -> Self {
        let mut new = self;
        new.operation = Option::Some(value);
        new
    }

    fn build_inner(
        self,
    ) -> Result<UpdateCollectionClusterSetupRequest, UpdateCollectionClusterSetupRequestBuilderError>
    {
        Ok(UpdateCollectionClusterSetupRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            timeout: self.timeout.unwrap_or_default(),
            operation: match self.operation {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("operation"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            timeout: core::default::Default::default(),
            operation: core::default::Default::default(),
        }
    }
}

impl From<UpdateCollectionClusterSetupRequestBuilder> for UpdateCollectionClusterSetupRequest {
    fn from(value: UpdateCollectionClusterSetupRequestBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UpdateCollectionClusterSetupRequestBuilder", "UpdateCollectionClusterSetupRequest"
            )
        })
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdateCollectionClusterSetupRequest {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UpdateCollectionClusterSetupRequestBuilder", "UpdateCollectionClusterSetupRequest"
            )
        })
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum UpdateCollectionClusterSetupRequestBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for UpdateCollectionClusterSetupRequestBuilderError {
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
impl std::error::Error for UpdateCollectionClusterSetupRequestBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError>
    for UpdateCollectionClusterSetupRequestBuilderError
{
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for UpdateCollectionClusterSetupRequestBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
