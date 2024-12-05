use crate::qdrant::*;

pub struct CollectionParamsDiffBuilder {
    /// Number of replicas of each shard that network tries to maintain
    pub(crate) replication_factor: Option<Option<u32>>,
    /// How many replicas should apply the operation for us to consider it successful
    pub(crate) write_consistency_factor: Option<Option<u32>>,
    /// If true - point's payload will not be stored in memory
    pub(crate) on_disk_payload: Option<Option<bool>>,
    /// Fan-out every read request to these many additional remote nodes (and return first available response)
    pub(crate) read_fan_out_factor: Option<Option<u32>>,
}
#[allow(clippy::all)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl CollectionParamsDiffBuilder {
    /// Number of replicas of each shard that network tries to maintain
    #[allow(unused_mut)]
    pub fn replication_factor(self, value: u32) -> Self {
        let mut new = self;
        new.replication_factor = Option::Some(Option::Some(value));
        new
    }
    /// How many replicas should apply the operation for us to consider it successful
    #[allow(unused_mut)]
    pub fn write_consistency_factor(self, value: u32) -> Self {
        let mut new = self;
        new.write_consistency_factor = Option::Some(Option::Some(value));
        new
    }
    /// If true - point's payload will not be stored in memory
    #[allow(unused_mut)]
    pub fn on_disk_payload(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk_payload = Option::Some(Option::Some(value));
        new
    }
    /// Fan-out every read request to these many additional remote nodes (and return first available response)
    #[allow(unused_mut)]
    pub fn read_fan_out_factor(self, value: u32) -> Self {
        let mut new = self;
        new.read_fan_out_factor = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<CollectionParamsDiff, std::convert::Infallible> {
        Ok(CollectionParamsDiff {
            replication_factor: match self.replication_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            write_consistency_factor: match self.write_consistency_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk_payload: match self.on_disk_payload {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            read_fan_out_factor: match self.read_fan_out_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            replication_factor: core::default::Default::default(),
            write_consistency_factor: core::default::Default::default(),
            on_disk_payload: core::default::Default::default(),
            read_fan_out_factor: core::default::Default::default(),
        }
    }
}

impl From<CollectionParamsDiffBuilder> for CollectionParamsDiff {
    fn from(value: CollectionParamsDiffBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "CollectionParamsDiffBuilder", "CollectionParamsDiff"
            )
        })
    }
}

impl CollectionParamsDiffBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CollectionParamsDiff {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "CollectionParamsDiffBuilder", "CollectionParamsDiff"
            )
        })
    }
}

impl Default for CollectionParamsDiffBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
