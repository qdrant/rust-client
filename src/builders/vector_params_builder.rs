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

    fn build_inner(self) -> Result<VectorParams, VectorParamsBuilderError> {
        Ok(VectorParams {
            size: self.size.unwrap_or_default(),
            distance: self.distance.unwrap_or_default(),
            hnsw_config: self.hnsw_config.unwrap_or_default(),
            quantization_config: { convert_option(&self.quantization_config) },
            on_disk: self.on_disk.unwrap_or_default(),
            datatype: self.datatype.unwrap_or_default(),
            multivector_config: self.multivector_config.unwrap_or_default(),
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

impl From<VectorParamsBuilder> for VectorParams {
    fn from(value: VectorParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "VectorParamsBuilder", "VectorParams"
            )
        })
    }
}

impl VectorParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> VectorParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "VectorParamsBuilder", "VectorParams"
            )
        })
    }
}

impl VectorParamsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub enum VectorParamsBuilderError {
    /// Uninitialized field
    UninitializedField(&'static str),
    /// Custom validation error
    ValidationError(String),
}

// Implementing the Display trait for better error messages
impl std::fmt::Display for VectorParamsBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::UninitializedField(field) => {
                write!(f, "`{}` must be initialized", field)
            }
            Self::ValidationError(error) => write!(f, "{}", error),
        }
    }
}

// Implementing the Error trait
impl std::error::Error for VectorParamsBuilderError {}

// Implementing From trait for conversion from UninitializedFieldError
impl From<derive_builder::UninitializedFieldError> for VectorParamsBuilderError {
    fn from(error: derive_builder::UninitializedFieldError) -> Self {
        Self::UninitializedField(error.field_name())
    }
}

// Implementing From trait for conversion from String
impl From<String> for VectorParamsBuilderError {
    fn from(error: String) -> Self {
        Self::ValidationError(error)
    }
}
