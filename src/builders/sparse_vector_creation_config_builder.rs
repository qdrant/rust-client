use crate::qdrant::*;

/// Sparse vector creation parameters.
/// Only includes immutable properties that define the vector space.
#[must_use]
#[derive(Clone)]
pub struct SparseVectorCreationConfigBuilder {
    /// If set - apply modifier to the vector values (e.g., IDF)
    pub(crate) modifier: Option<Option<i32>>,
    /// Data type used to store weights in the index
    pub(crate) datatype: Option<Option<i32>>,
}

impl SparseVectorCreationConfigBuilder {
    /// If set - apply modifier to the vector values (e.g., IDF)
    pub fn modifier<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.modifier = Option::Some(Option::Some(value.into()));
        new
    }

    /// Data type used to store weights in the index
    pub fn datatype<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.datatype = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<SparseVectorCreationConfig, std::convert::Infallible> {
        Ok(SparseVectorCreationConfig {
            modifier: self.modifier.unwrap_or_default(),
            datatype: self.datatype.unwrap_or_default(),
        })
    }

    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            modifier: core::default::Default::default(),
            datatype: core::default::Default::default(),
        }
    }
}

impl From<SparseVectorCreationConfigBuilder> for SparseVectorCreationConfig {
    fn from(value: SparseVectorCreationConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "SparseVectorCreationConfigBuilder", "SparseVectorCreationConfig"
            )
        })
    }
}

impl SparseVectorCreationConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SparseVectorCreationConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "SparseVectorCreationConfigBuilder", "SparseVectorCreationConfig"
            )
        })
    }
}

impl SparseVectorCreationConfigBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}

impl Default for SparseVectorCreationConfigBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
