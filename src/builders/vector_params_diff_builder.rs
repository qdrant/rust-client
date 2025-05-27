use crate::grpc_macros::convert_option;
use crate::qdrant::*;

#[derive(Clone)]
pub struct VectorParamsDiffBuilder {
    /// Update params for HNSW index. If empty object - it will be unset
    pub(crate) hnsw_config: Option<Option<HnswConfigDiff>>,
    /// Update quantization params. If none - it is left unchanged.
    quantization_config: Option<quantization_config_diff::Quantization>,
    /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl VectorParamsDiffBuilder {
    /// Update params for HNSW index. If empty object - it will be unset
    #[allow(unused_mut)]
    pub fn hnsw_config<VALUE: core::convert::Into<HnswConfigDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.hnsw_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Update quantization params. If none - it is left unchanged.
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
    /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<VectorParamsDiff, std::convert::Infallible> {
        Ok(VectorParamsDiff {
            hnsw_config: self.hnsw_config.unwrap_or_default(),
            quantization_config: { convert_option(&self.quantization_config) },
            on_disk: self.on_disk.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            hnsw_config: core::default::Default::default(),
            quantization_config: core::default::Default::default(),
            on_disk: core::default::Default::default(),
        }
    }
}

impl Default for VectorParamsDiffBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl From<VectorParamsDiffBuilder> for VectorParamsDiff {
    fn from(value: VectorParamsDiffBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "VectorParamsDiffBuilder", "VectorParamsDiff"
            )
        })
    }
}

impl VectorParamsDiffBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> VectorParamsDiff {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "VectorParamsDiffBuilder", "VectorParamsDiff"
            )
        })
    }
}
