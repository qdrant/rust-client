use crate::qdrant::*;

impl ShardKeySelector {
    /// Create a new ShardKeySelector with the given shard keys
    pub fn new(shard_keys: impl Into<Vec<ShardKey>>) -> Self {
        Self {
            shard_keys: shard_keys.into(),
            fallback: None,
        }
    }

    /// Set a fallback shard key to use if the primary shard keys are not available
    pub fn with_fallback(mut self, fallback: impl Into<ShardKey>) -> Self {
        self.fallback = Some(fallback.into());
        self
    }
}
