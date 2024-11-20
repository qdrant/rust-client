use crate::qdrant::*;

pub struct CreateShardKeyRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Request to create shard key
    pub(crate) request: Option<Option<CreateShardKey>>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
}

impl CreateShardKeyRequestBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Request to create shard key
    #[allow(unused_mut)]
    pub fn request<VALUE: core::convert::Into<CreateShardKey>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.request = Option::Some(Option::Some(value.into()));
        new
    }
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `CreateShardKeyRequest`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<CreateShardKeyRequest, CreateShardKeyRequestBuilderError> {
        Ok(CreateShardKeyRequest {
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

impl From<CreateShardKeyRequestBuilder> for CreateShardKeyRequest {
    fn from(value: CreateShardKeyRequestBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateShardKeyRequestBuilder", "CreateShardKeyRequest",
        ))
    }
}

impl CreateShardKeyRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateShardKeyRequest {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateShardKeyRequestBuilder", "CreateShardKeyRequest",
        ))
    }
}

impl CreateShardKeyRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
