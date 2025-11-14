use crate::qdrant::{Filter, ReplicatePoints, ShardKey};

#[derive(Clone)]
pub struct ReplicatePointsBuilder {
    /// Source shard key
    pub(crate) from_shard_key: ShardKey,
    /// Target shard key
    pub(crate) to_shard_key: ShardKey,
    /// If set - only points matching the filter will be replicated
    pub(crate) filter: Option<Option<Filter>>,
}

impl ReplicatePointsBuilder {
    /// Create a new ReplicatePointsBuilder with required shard keys.
    ///
    /// # Arguments
    ///
    /// * `from_shard_key` - Source shard key to replicate points from
    /// * `to_shard_key` - Target shard key to replicate points to
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{ReplicatePointsBuilder, ShardKey};
    ///
    /// let replicate = ReplicatePointsBuilder::new(
    ///     ShardKey::from("shard_1".to_string()),
    ///     ShardKey::from("shard_2".to_string())
    /// ).build();
    /// ```
    pub fn new(from_shard_key: impl Into<ShardKey>, to_shard_key: impl Into<ShardKey>) -> Self {
        Self {
            from_shard_key: from_shard_key.into(),
            to_shard_key: to_shard_key.into(),
            filter: None,
        }
    }

    /// Set a filter to replicate only points matching the filter.
    ///
    /// # Arguments
    ///
    /// * `filter` - Filter condition - only points matching this filter will be replicated
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{Condition, Filter, ReplicatePointsBuilder, ShardKey};
    ///
    /// let replicate = ReplicatePointsBuilder::new(
    ///     ShardKey::from("shard_1".to_string()),
    ///     ShardKey::from("shard_2".to_string())
    /// )
    /// .filter(Filter::must([Condition::matches("status", "active".to_string())]))
    /// .build();
    /// ```
    pub fn filter(self, value: impl Into<Filter>) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }

    pub fn build(self) -> ReplicatePoints {
        ReplicatePoints {
            from_shard_key: Some(self.from_shard_key),
            to_shard_key: Some(self.to_shard_key),
            filter: self.filter.unwrap_or_default(),
        }
    }
}

impl From<ReplicatePointsBuilder> for ReplicatePoints {
    fn from(value: ReplicatePointsBuilder) -> Self {
        value.build()
    }
}
