use crate::qdrant::{ShardKey, ShardKeySelector};

#[derive(Clone)]
pub struct ShardKeySelectorBuilder {
    /// List of shard keys which should be used in the request
    pub(crate) shard_keys: Option<Vec<ShardKey>>,
    /// Fallback shard key to use if the primary shard keys are not available
    pub(crate) fallback: Option<Option<ShardKey>>,
}

impl ShardKeySelectorBuilder {
    /// Create a new ShardKeySelectorBuilder with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{ShardKey, ShardKeySelectorBuilder};
    ///
    /// let selector = ShardKeySelectorBuilder::new()
    ///     .add_shard_key(ShardKey::from("key1".to_string()))
    ///     .build();
    /// ```
    pub fn new() -> Self {
        Self::create_empty()
    }


    pub fn with_shard_key(shard_key: impl Into<ShardKey>) -> Self {
        Self::new().add_shard_key(shard_key)
    }

    /// Create a new ShardKeySelectorBuilder with the given shard keys.
    ///
    /// # Arguments
    ///
    /// * `shard_keys` - List of shard keys which should be used in the request
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{ShardKey, ShardKeySelectorBuilder};
    ///
    /// let selector = ShardKeySelectorBuilder::with_shard_keys(
    ///     vec![ShardKey::from("key1".to_string())]
    /// ).build();
    /// ```
    pub fn with_shard_keys(shard_keys: impl Into<Vec<ShardKey>>) -> Self {
        Self::new().shard_keys(shard_keys)
    }

    /// Set the shard keys which should be used in the request.
    ///
    /// # Arguments
    ///
    /// * `shard_keys` - List of shard keys which should be used in the request
    pub fn shard_keys(self, value: impl Into<Vec<ShardKey>>) -> Self {
        let mut new = self;
        new.shard_keys = Option::Some(value.into());
        new
    }

    /// Add a shard key to the list of shard keys.
    ///
    /// # Arguments
    ///
    /// * `shard_key` - Shard key to add to the list
    pub fn add_shard_key(mut self, shard_key: impl Into<ShardKey>) -> Self {
        match self.shard_keys {
            Some(ref mut keys) => keys.push(shard_key.into()),
            None => self.shard_keys = Some(vec![shard_key.into()]),
        }
        self
    }

    /// Set a fallback shard key to use if the primary shard keys are not available.
    ///
    /// # Arguments
    ///
    /// * `fallback` - Fallback shard key
    pub fn fallback(self, value: impl Into<ShardKey>) -> Self {
        let mut new = self;
        new.fallback = Option::Some(Option::Some(value.into()));
        new
    }

    pub fn build(self) -> ShardKeySelector {
        ShardKeySelector {
            shard_keys: self.shard_keys.unwrap_or_default(),
            fallback: self.fallback.unwrap_or_default(),
        }
    }

    /// Create an empty builder, with all fields set to `None`.
    fn create_empty() -> Self {
        Self {
            shard_keys: core::default::Default::default(),
            fallback: core::default::Default::default(),
        }
    }
}

impl From<ShardKeySelectorBuilder> for ShardKeySelector {
    fn from(value: ShardKeySelectorBuilder) -> Self {
        value.build()
    }
}

impl Default for ShardKeySelectorBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
