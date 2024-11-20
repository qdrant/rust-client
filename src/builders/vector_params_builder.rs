use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct VectorParamsBuilder {
    /// Size of the vectors
    pub(crate) size: Option<u64>,
    /// Distance function used for comparing vectors
    pub(crate) distance: Option<i32>,
    /// Configuration of vector HNSW graph. If omitted - the collection configuration will be used
    pub(crate) hnsw_config: Option<Option<HnswConfigDiff>>,
    /// Configuration of vector quantization config. If omitted - the collection configuration will be used
    quantization_config: Option<quantization_config::Quantization>,
    /// If true - serve vectors from disk. If set to false, the vectors will be loaded in RAM.
    pub(crate) on_disk: Option<Option<bool>>,
    /// Data type of the vectors
    pub(crate) datatype: Option<Option<i32>>,
    /// Configuration for multi-vector search
    pub(crate) multivector_config: Option<Option<MultiVectorConfig>>,
}

impl VectorParamsBuilder {
    /// Size of the vectors
    #[allow(unused_mut)]
    pub fn size(self, value: u64) -> Self {
        let mut new = self;
        new.size = Option::Some(value);
        new
    }
    /// Distance function used for comparing vectors
    #[allow(unused_mut)]
    pub fn distance<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.distance = Option::Some(value.into());
        new
    }
    /// Configuration of vector HNSW graph. If omitted - the collection configuration will be used
    #[allow(unused_mut)]
    pub fn hnsw_config<VALUE: core::convert::Into<HnswConfigDiff>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.hnsw_config = Option::Some(Option::Some(value.into()));
        new
    }
    /// Configuration of vector quantization config. If omitted - the collection configuration will be used
    #[allow(unused_mut)]
    pub fn quantization_config<VALUE: core::convert::Into<quantization_config::Quantization>>(
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
    /// Data type of the vectors
    #[allow(unused_mut)]
    pub fn datatype<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.datatype = Option::Some(Option::Some(value.into()));
        new
    }
    /// Configuration for multi-vector search
    #[allow(unused_mut)]
    pub fn multivector_config<VALUE: core::convert::Into<MultiVectorConfig>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.multivector_config = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `VectorParams`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<VectorParams, VectorParamsBuilderError> {
        Ok(VectorParams {
            size: match self.size {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            distance: match self.distance {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            hnsw_config: match self.hnsw_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            quantization_config: { convert_option(&self.quantization_config) },
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            datatype: match self.datatype {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            multivector_config: match self.multivector_config {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            size: core::default::Default::default(),
            distance: core::default::Default::default(),
            hnsw_config: core::default::Default::default(),
            quantization_config: core::default::Default::default(),
            on_disk: core::default::Default::default(),
            datatype: core::default::Default::default(),
            multivector_config: core::default::Default::default(),
        }
    }
}

impl SparseVectorParamsBuilder {
    /// Configuration of sparse index
    #[allow(unused_mut)]
    pub fn index<VALUE: core::convert::Into<SparseIndexConfig>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.index = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set - apply modifier to the vector values
    #[allow(unused_mut)]
    pub fn modifier<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.modifier = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `SparseVectorParams`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<SparseVectorParams, std::convert::Infallible> {
        Ok(SparseVectorParams {
            index: match self.index {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            modifier: match self.modifier {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            index: core::default::Default::default(),
            modifier: core::default::Default::default(),
        }
    }
}

impl From<VectorParamsBuilder> for VectorParams {
    fn from(value: VectorParamsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "VectorParamsBuilder", "VectorParams",
        ))
    }
}

impl VectorParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> VectorParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "VectorParamsBuilder", "VectorParams",
        ))
    }
}

impl VectorParamsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

impl From<SparseVectorParamsBuilder> for SparseVectorParams {
    fn from(value: SparseVectorParamsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "SparseVectorParamsBuilder", "SparseVectorParams",
        ))
    }
}

impl SparseVectorParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SparseVectorParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "SparseVectorParamsBuilder", "SparseVectorParams",
        ))
    }
}
