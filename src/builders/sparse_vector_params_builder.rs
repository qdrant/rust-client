use crate::qdrant::*;

pub struct SparseVectorParamsBuilder {
    /// Configuration of sparse index
    pub(crate) index: Option<Option<SparseIndexConfig>>,
    /// If set - apply modifier to the vector values
    pub(crate) modifier: Option<Option<i32>>,
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

impl Default for SparseVectorParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
