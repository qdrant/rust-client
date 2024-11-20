use crate::qdrant::*;

pub struct ReplicaBuilder {
    pub(crate) shard_id: Option<u32>,
    pub(crate) peer_id: Option<u64>,
}

impl ReplicaBuilder {
    #[allow(unused_mut)]
    pub fn shard_id(self, value: u32) -> Self {
        let mut new = self;
        new.shard_id = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn peer_id(self, value: u64) -> Self {
        let mut new = self;
        new.peer_id = Option::Some(value);
        new
    }
    /**Builds a new `Replica`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<Replica, ReplicaBuilderError> {
        Ok(Replica {
            shard_id: match self.shard_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("shard_id"),
                    ));
                }
            },
            peer_id: match self.peer_id {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("peer_id"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_id: core::default::Default::default(),
            peer_id: core::default::Default::default(),
        }
    }
}

impl From<ReplicaBuilder> for Replica {
    fn from(value: ReplicaBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ReplicaBuilder", "Replica",
        ))
    }
}

impl ReplicaBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> Replica {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ReplicaBuilder", "Replica",
        ))
    }
}

impl ReplicaBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
