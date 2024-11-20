use crate::qdrant::*;

pub struct CreateShardKeyBuilder {
    /// User-defined shard key
    pub(crate) shard_key: Option<Option<ShardKey>>,
    /// Number of shards to create per shard key
    pub(crate) shards_number: Option<Option<u32>>,
    /// Number of replicas of each shard to create
    pub(crate) replication_factor: Option<Option<u32>>,
    /// List of peer ids, allowed to create shards. If empty - all peers are allowed
    pub(crate) placement: Option<Vec<u64>>,
}

impl CreateShardKeyBuilder {
    /// User-defined shard key
    #[allow(unused_mut)]
    pub fn shard_key<VALUE: core::convert::Into<ShardKey>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.shard_key = Option::Some(Option::Some(value.into()));
        new
    }
    /// Number of shards to create per shard key
    #[allow(unused_mut)]
    pub fn shards_number(self, value: u32) -> Self {
        let mut new = self;
        new.shards_number = Option::Some(Option::Some(value));
        new
    }
    /// Number of replicas of each shard to create
    #[allow(unused_mut)]
    pub fn replication_factor(self, value: u32) -> Self {
        let mut new = self;
        new.replication_factor = Option::Some(Option::Some(value));
        new
    }
    /// List of peer ids, allowed to create shards. If empty - all peers are allowed
    #[allow(unused_mut)]
    pub fn placement(self, value: Vec<u64>) -> Self {
        let mut new = self;
        new.placement = Option::Some(value);
        new
    }
    /**Builds a new `CreateShardKey`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<CreateShardKey, std::convert::Infallible> {
        Ok(CreateShardKey {
            shard_key: match self.shard_key {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            shards_number: match self.shards_number {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            replication_factor: match self.replication_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            placement: match self.placement {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_key: core::default::Default::default(),
            shards_number: core::default::Default::default(),
            replication_factor: core::default::Default::default(),
            placement: core::default::Default::default(),
        }
    }
}

impl From<CreateShardKeyBuilder> for CreateShardKey {
    fn from(value: CreateShardKeyBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateShardKeyBuilder", "CreateShardKey",
        ))
    }
}

impl CreateShardKeyBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateShardKey {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateShardKeyBuilder", "CreateShardKey",
        ))
    }
}
