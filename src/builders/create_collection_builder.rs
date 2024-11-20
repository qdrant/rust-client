use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Default, Debug)]
pub struct CreateCollectionBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Configuration of vector index
    pub(crate) hnsw_config: Option<Option<HnswConfigDiff>>,
    /// Configuration of the Write-Ahead-Log
    pub(crate) wal_config: Option<Option<WalConfigDiff>>,
    /// Configuration of the optimizers
    pub(crate) optimizers_config: Option<Option<OptimizersConfigDiff>>,
    /// Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1
    pub(crate) shard_number: Option<Option<u32>>,
    /// If true - point's payload will not be stored in memory
    pub(crate) on_disk_payload: Option<Option<bool>>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
    /// Configuration for vectors
    pub(crate) vectors_config: Option<Option<VectorsConfig>>,
    /// Number of replicas of each shard that network tries to maintain, default = 1
    pub(crate) replication_factor: Option<Option<u32>>,
    /// How many replicas should apply the operation for us to consider it successful, default = 1
    pub(crate) write_consistency_factor: Option<Option<u32>>,
    /// Specify name of the other collection to copy data from
    pub(crate) init_from_collection: Option<Option<String>>,
    /// Quantization configuration of vector
    quantization_config: Option<quantization_config::Quantization>,
    /// Sharding method
    pub(crate) sharding_method: Option<Option<i32>>,
    /// Configuration for sparse vectors
    pub(crate) sparse_vectors_config: Option<Option<SparseVectorConfig>>,
    /// Configuration for strict mode
    pub(crate) strict_mode_config: Option<Option<StrictModeConfig>>,
}
#[allow(clippy::all)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[allow(dead_code)]
impl CreateCollectionBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value.into());
        new
    }
    /// Configuration of vector index
    #[allow(unused_mut)]
    pub fn hnsw_config<VALUE: core::convert::Into<HnswConfigDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.hnsw_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Configuration of the Write-Ahead-Log
    #[allow(unused_mut)]
    pub fn wal_config<VALUE: core::convert::Into<WalConfigDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.wal_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Configuration of the optimizers
    #[allow(unused_mut)]
    pub fn optimizers_config<VALUE: core::convert::Into<OptimizersConfigDiff>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.optimizers_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Number of shards in the collection, default is 1 for standalone, otherwise equal to the number of nodes. Minimum is 1
    #[allow(unused_mut)]
    pub fn shard_number(self, value: u32) -> Self {
        let mut new = self;
        new.shard_number = Option::Some(Option::Some(value));
        new
    }
    /// If true - point's payload will not be stored in memory
    #[allow(unused_mut)]
    pub fn on_disk_payload(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk_payload = Option::Some(Option::Some(value));
        new
    }
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /// Configuration for vectors
    #[allow(unused_mut)]
    pub fn vectors_config<VALUE: core::convert::Into<VectorsConfig>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.vectors_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Number of replicas of each shard that network tries to maintain, default = 1
    #[allow(unused_mut)]
    pub fn replication_factor(self, value: u32) -> Self {
        let mut new = self;
        new.replication_factor = Option::Some(Option::Some(value));
        new
    }
    /// How many replicas should apply the operation for us to consider it successful, default = 1
    #[allow(unused_mut)]
    pub fn write_consistency_factor(self, value: u32) -> Self {
        let mut new = self;
        new.write_consistency_factor = Option::Some(Option::Some(value));
        new
    }
    /// Specify name of the other collection to copy data from
    #[allow(unused_mut)]
    pub fn init_from_collection<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.init_from_collection = Option::Some(Option::Some(value.into()));
        new
    }
    /// Quantization configuration of vector
    #[allow(unused_mut)]
    pub fn quantization_config<VALUE: core::convert::Into<quantization_config::Quantization>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.quantization_config = Option::Some(value.into());
        new
    }
    /// Sharding method
    #[allow(unused_mut)]
    pub fn sharding_method(self, value: i32) -> Self {
        let mut new = self;
        new.sharding_method = Option::Some(Option::Some(value));
        new
    }
    /// Configuration for sparse vectors
    #[allow(unused_mut)]
    pub fn sparse_vectors_config<VALUE: core::convert::Into<SparseVectorConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.sparse_vectors_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Configuration for strict mode
    #[allow(unused_mut)]
    pub fn strict_mode_config<VALUE: core::convert::Into<StrictModeConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.strict_mode_config = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `CreateCollection`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<CreateCollection, std::convert::Infallible> {
        Ok(CreateCollection {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            hnsw_config: match self.hnsw_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            wal_config: match self.wal_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            optimizers_config: match self.optimizers_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            shard_number: match self.shard_number {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk_payload: match self.on_disk_payload {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            timeout: match self.timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            vectors_config: match self.vectors_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            replication_factor: match self.replication_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            write_consistency_factor: match self.write_consistency_factor {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            init_from_collection: match self.init_from_collection {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            quantization_config: { convert_option(&self.quantization_config) },
            sharding_method: match self.sharding_method {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            sparse_vectors_config: match self.sparse_vectors_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            strict_mode_config: match self.strict_mode_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            hnsw_config: core::default::Default::default(),
            wal_config: core::default::Default::default(),
            optimizers_config: core::default::Default::default(),
            shard_number: core::default::Default::default(),
            on_disk_payload: core::default::Default::default(),
            timeout: core::default::Default::default(),
            vectors_config: core::default::Default::default(),
            replication_factor: core::default::Default::default(),
            write_consistency_factor: core::default::Default::default(),
            init_from_collection: core::default::Default::default(),
            quantization_config: core::default::Default::default(),
            sharding_method: core::default::Default::default(),
            sparse_vectors_config: core::default::Default::default(),
            strict_mode_config: core::default::Default::default(),
        }
    }
}

impl From<CreateCollectionBuilder> for CreateCollection {
    fn from(value: CreateCollectionBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateCollectionBuilder", "CreateCollection",
        ))
    }
}

impl CreateCollectionBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateCollection {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateCollectionBuilder", "CreateCollection",
        ))
    }
}
