use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct UpdateCollectionBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// New configuration parameters for the collection. This operation is blocking, it will only proceed once all current optimizations are complete
    pub(crate) optimizers_config: Option<Option<OptimizersConfigDiff>>,
    /// Wait timeout for operation commit in seconds if blocking, if not specified - default value will be supplied
    pub(crate) timeout: Option<Option<u64>>,
    /// New configuration parameters for the collection
    pub(crate) params: Option<Option<CollectionParamsDiff>>,
    /// New HNSW parameters for the collection index
    pub(crate) hnsw_config: Option<Option<HnswConfigDiff>>,
    /// New vector parameters
    pub(crate) vectors_config: Option<Option<VectorsConfigDiff>>,
    /// Quantization configuration of vector
    quantization_config: Option<quantization_config_diff::Quantization>,
    /// New sparse vector parameters
    pub(crate) sparse_vectors_config: Option<Option<SparseVectorConfig>>,
    /// New strict mode configuration
    pub(crate) strict_mode_config: Option<Option<StrictModeConfig>>,
}

impl UpdateCollectionBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// New configuration parameters for the collection. This operation is blocking, it will only proceed once all current optimizations are complete
    #[allow(unused_mut)]
    pub fn optimizers_config<VALUE: core::convert::Into<OptimizersConfigDiff>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.optimizers_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Wait timeout for operation commit in seconds if blocking, if not specified - default value will be supplied
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /// New configuration parameters for the collection
    #[allow(unused_mut)]
    pub fn params<VALUE: core::convert::Into<CollectionParamsDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.params = Option::Some(Option::Some(value.into()));
        new
    }
    /// New HNSW parameters for the collection index
    #[allow(unused_mut)]
    pub fn hnsw_config<VALUE: core::convert::Into<HnswConfigDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.hnsw_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// New vector parameters
    #[allow(unused_mut)]
    pub fn vectors_config<VALUE: core::convert::Into<VectorsConfigDiff>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.vectors_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Quantization configuration of vector
    #[allow(unused_mut)]
    pub fn quantization_config<
        VALUE: core::convert::Into<quantization_config_diff::Quantization>,
    >(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.quantization_config = Option::Some(value.into());
        new
    }
    /// New sparse vector parameters
    #[allow(unused_mut)]
    pub fn sparse_vectors_config<VALUE: core::convert::Into<SparseVectorConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.sparse_vectors_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// New strict mode configuration
    #[allow(unused_mut)]
    pub fn strict_mode_config<VALUE: core::convert::Into<StrictModeConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.strict_mode_config = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<UpdateCollection, UpdateCollectionBuilderError> {
        Ok(UpdateCollection {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            optimizers_config: self.optimizers_config.unwrap_or_default(),
            timeout: self.timeout.unwrap_or_default(),
            params: self.params.unwrap_or_default(),
            hnsw_config: self.hnsw_config.unwrap_or_default(),
            vectors_config: self.vectors_config.unwrap_or_default(),
            quantization_config: { convert_option(&self.quantization_config) },
            sparse_vectors_config: self.sparse_vectors_config.unwrap_or_default(),
            strict_mode_config: self.strict_mode_config.unwrap_or_default(),
            metadata: Default::default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            optimizers_config: core::default::Default::default(),
            timeout: core::default::Default::default(),
            params: core::default::Default::default(),
            hnsw_config: core::default::Default::default(),
            vectors_config: core::default::Default::default(),
            quantization_config: core::default::Default::default(),
            sparse_vectors_config: core::default::Default::default(),
            strict_mode_config: core::default::Default::default(),
        }
    }
}

impl From<UpdateCollectionBuilder> for UpdateCollection {
    fn from(value: UpdateCollectionBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UpdateCollectionBuilder", "UpdateCollection"
            )
        })
    }
}

impl UpdateCollectionBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdateCollection {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UpdateCollectionBuilder", "UpdateCollection"
            )
        })
    }
}

impl UpdateCollectionBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum UpdateCollectionBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for UpdateCollectionBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(field) => {
                write!(f, "`{field}` must be initialized")
            }
            Self::ValidationError(error) => write!(f, "{error}"),
        }
    }
}

// Implementing the Error trait
impl std::error::Error for UpdateCollectionBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for UpdateCollectionBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for UpdateCollectionBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
