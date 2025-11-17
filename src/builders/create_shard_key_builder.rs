use crate::qdrant::*;

#[derive(Clone)]
pub struct CreateShardKeyBuilder {
    /// User-defined shard key
    pub(crate) shard_key: Option<Option<ShardKey>>,
    /// Number of shards to create per shard key
    pub(crate) shards_number: Option<Option<u32>>,
    /// Number of replicas of each shard to create
    pub(crate) replication_factor: Option<Option<u32>>,
    /// List of peer ids, allowed to create shards. If empty - all peers are allowed
    pub(crate) placement: Option<Vec<u64>>,
    /// Initial replica state for newly created shards. If empty - use Active state.
    ///
    /// # Warning
    ///
    /// Use with caution! Setting arbirray replica states here may break your Qdrant cluster.
    pub(crate) initial_state: Option<Option<i32>>,
}

impl CreateShardKeyBuilder {
    /// User-defined shard key
    pub fn shard_key<VALUE: core::convert::Into<ShardKey>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.shard_key = Option::Some(Option::Some(value.into()));
        new
    }
    /// Number of shards to create per shard key
    pub fn shards_number(self, value: u32) -> Self {
        let mut new = self;
        new.shards_number = Option::Some(Option::Some(value));
        new
    }
    /// Number of replicas of each shard to create
    pub fn replication_factor(self, value: u32) -> Self {
        let mut new = self;
        new.replication_factor = Option::Some(Option::Some(value));
        new
    }
    /// List of peer ids, allowed to create shards. If empty - all peers are allowed
    pub fn placement(self, value: Vec<u64>) -> Self {
        let mut new = self;
        new.placement = Option::Some(value);
        new
    }
    /// Initial replica state for newly created shards.
    ///
    /// Uses Active state by default.
    ///
    /// # Warning
    ///
    /// Use with caution! Setting arbirray replica states here may break your Qdrant cluster.
    #[allow(unused_mut)]
    pub fn initial_state(self, value: ReplicaState) -> Self {
        let mut new = self;
        new.initial_state = Option::Some(Option::Some(value as i32));
        new
    }

    fn build_inner(self) -> Result<CreateShardKey, std::convert::Infallible> {
        Ok(CreateShardKey {
            shard_key: self.shard_key.unwrap_or_default(),
            shards_number: self.shards_number.unwrap_or_default(),
            replication_factor: self.replication_factor.unwrap_or_default(),
            placement: self.placement.unwrap_or_default(),
            initial_state: self.initial_state.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            shard_key: core::default::Default::default(),
            shards_number: core::default::Default::default(),
            replication_factor: core::default::Default::default(),
            placement: core::default::Default::default(),
            initial_state: core::default::Default::default(),
        }
    }
}

impl From<CreateShardKeyBuilder> for CreateShardKey {
    fn from(value: CreateShardKeyBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "CreateShardKeyBuilder", "CreateShardKey"
            )
        })
    }
}

impl CreateShardKeyBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateShardKey {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "CreateShardKeyBuilder", "CreateShardKey"
            )
        })
    }
}

impl Default for CreateShardKeyBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
