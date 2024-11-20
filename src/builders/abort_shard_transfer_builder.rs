use crate::qdrant::*;

pub struct AbortShardTransferBuilder {
    /// Local shard id
    pub(crate) shard_id: Option<u32>,
    pub(crate) to_shard_id: Option<Option<u32>>,
    pub(crate) from_peer_id: Option<u64>,
    pub(crate) to_peer_id: Option<u64>,
}

impl AbortShardTransferBuilder {
    /// Local shard id
    #[allow(unused_mut)]
    pub fn shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.shard_id = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn to_shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.to_shard_id = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn from_peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.from_peer_id = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn to_peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.to_peer_id = Option::Some(value);
        new
    }
    /**Builds a new `AbortShardTransfer`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<AbortShardTransfer, AbortShardTransferBuilderError> {
        Ok(AbortShardTransfer {
            shard_id: match self.shard_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("shard_id"),
                    ));
                }
            },
            to_shard_id: match self.to_shard_id {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            from_peer_id: match self.from_peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("from_peer_id"),
                    ));
                }
            },
            to_peer_id: match self.to_peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("to_peer_id"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_id: core::default::Default::default(),
            to_shard_id: core::default::Default::default(),
            from_peer_id: core::default::Default::default(),
            to_peer_id: core::default::Default::default(),
        }
    }
}

impl From<AbortShardTransferBuilder> for AbortShardTransfer {
    fn from(value: AbortShardTransferBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "AbortShardTransferBuilder", "AbortShardTransfer",
        ))
    }
}

impl AbortShardTransferBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> AbortShardTransfer {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "AbortShardTransferBuilder", "AbortShardTransfer",
        ))
    }
}

impl AbortShardTransferBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
