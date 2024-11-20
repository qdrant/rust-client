use crate::qdrant::*;

pub struct LookupLocationBuilder {
    pub(crate) collection_name: Option<String>,
    /// Which vector to use for search, if not specified - use default vector
    pub(crate) vector_name: Option<Option<String>>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl LookupLocationBuilder {
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Which vector to use for search, if not specified - use default vector
    #[allow(unused_mut)]
    pub fn vector_name<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.vector_name = Option::Some(Option::Some(value.into()));
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `LookupLocation`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<LookupLocation, LookupLocationBuilderError> {
        Ok(LookupLocation {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            vector_name: match self.vector_name {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            shard_key_selector: match self.shard_key_selector {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            vector_name: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<LookupLocationBuilder> for LookupLocation {
    fn from(value: LookupLocationBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "LookupLocationBuilder", "LookupLocation",
        ))
    }
}

impl LookupLocationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> LookupLocation {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "LookupLocationBuilder", "LookupLocation",
        ))
    }
}

impl LookupLocationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
