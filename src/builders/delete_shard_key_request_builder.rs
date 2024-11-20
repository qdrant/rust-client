use crate::qdrant::*;

pub struct DeleteShardKeyRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Request to delete shard key
    pub(crate) request: Option<Option<DeleteShardKey>>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
}

impl DeleteShardKeyRequestBuilder {
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
    /**Builds a new `DeleteShardKeyRequest`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<DeleteShardKeyRequest, DeleteShardKeyRequestBuilderError> {
        Ok(DeleteShardKeyRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            request: match self.request {
                Some(value) => value,
                None => core::default::Default::default(),
            },
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
            request: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<DeleteShardKeyRequestBuilder> for DeleteShardKeyRequest {
    fn from(value: DeleteShardKeyRequestBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DeleteShardKeyRequestBuilder", "DeleteShardKeyRequest",
        ))
    }
}

impl DeleteShardKeyRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteShardKeyRequest {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DeleteShardKeyRequestBuilder", "DeleteShardKeyRequest",
        ))
    }
}

impl DeleteShardKeyRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
