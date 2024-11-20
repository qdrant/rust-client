use crate::qdrant::*;

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
    /**Builds a new `UpdateCollectionClusterSetupRequest`.

    # Errors

    If a required field has not been initialized.
    */
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
            timeout: match self.timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
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
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "UpdateCollectionClusterSetupRequestBuilder", "UpdateCollectionClusterSetupRequest",
        ))
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdateCollectionClusterSetupRequest {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "UpdateCollectionClusterSetupRequestBuilder", "UpdateCollectionClusterSetupRequest",
        ))
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
