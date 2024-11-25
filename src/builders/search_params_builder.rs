use crate::qdrant::*;

pub struct SearchParamsBuilder {
    ///
    /// Params relevant to HNSW index. Size of the beam in a beam-search.
    /// Larger the value - more accurate the result, more time required for search.
    pub(crate) hnsw_ef: Option<Option<u64>>,
    ///
    /// Search without approximation. If set to true, search may run long but with exact results.
    pub(crate) exact: Option<Option<bool>>,
    ///
    /// If set to true, search will ignore quantized vector data
    pub(crate) quantization: Option<Option<QuantizationSearchParams>>,
    ///
    /// If enabled, the engine will only perform search among indexed or small segments.
    /// Using this option prevents slow searches in case of delayed index, but does not
    /// guarantee that all uploaded vectors will be included in search results
    pub(crate) indexed_only: Option<Option<bool>>,
}

impl SearchParamsBuilder {
    ///
    /// Params relevant to HNSW index. Size of the beam in a beam-search.
    /// Larger the value - more accurate the result, more time required for search.
    #[allow(unused_mut)]
    pub fn hnsw_ef(self, value: u64) -> Self {
        let mut new = self;
        new.hnsw_ef = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Search without approximation. If set to true, search may run long but with exact results.
    #[allow(unused_mut)]
    pub fn exact(self, value: bool) -> Self {
        let mut new = self;
        new.exact = Option::Some(Option::Some(value));
        new
    }
    ///
    /// If set to true, search will ignore quantized vector data
    #[allow(unused_mut)]
    pub fn quantization<VALUE: core::convert::Into<QuantizationSearchParams>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.quantization = Option::Some(Option::Some(value.into()));
        new
    }
    ///
    /// If enabled, the engine will only perform search among indexed or small segments.
    /// Using this option prevents slow searches in case of delayed index, but does not
    /// guarantee that all uploaded vectors will be included in search results
    #[allow(unused_mut)]
    pub fn indexed_only(self, value: bool) -> Self {
        let mut new = self;
        new.indexed_only = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<SearchParams, std::convert::Infallible> {
        Ok(SearchParams {
            hnsw_ef: self.hnsw_ef.unwrap_or_default(),
            exact: self.exact.unwrap_or_default(),
            quantization: self.quantization.unwrap_or_default(),
            indexed_only: self.indexed_only.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            hnsw_ef: core::default::Default::default(),
            exact: core::default::Default::default(),
            quantization: core::default::Default::default(),
            indexed_only: core::default::Default::default(),
        }
    }
}

impl From<SearchParamsBuilder> for SearchParams {
    fn from(value: SearchParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "SearchParamsBuilder", "SearchParams"
            )
        })
    }
}

impl SearchParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SearchParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "SearchParamsBuilder", "SearchParams"
            )
        })
    }
}

impl Default for SearchParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
