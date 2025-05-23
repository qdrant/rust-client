use crate::qdrant::*;

#[derive(Clone)]
pub struct QuantizationSearchParamsBuilder {
    ///
    /// If set to true, search will ignore quantized vector data
    pub(crate) ignore: Option<Option<bool>>,
    ///
    /// If true, use original vectors to re-score top-k results. If ignored, qdrant decides automatically does rescore enabled or not.
    pub(crate) rescore: Option<Option<bool>>,
    ///
    /// Oversampling factor for quantization.
    ///
    /// Defines how many extra vectors should be pre-selected using quantized index,
    /// and then re-scored using original vectors.
    ///
    /// For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index,
    /// and then top-100 will be returned after re-scoring.
    pub(crate) oversampling: Option<Option<f64>>,
}

impl QuantizationSearchParamsBuilder {
    ///
    /// If set to true, search will ignore quantized vector data
    #[allow(unused_mut)]
    pub fn ignore(self, value: bool) -> Self {
        let mut new = self;
        new.ignore = Option::Some(Option::Some(value));
        new
    }
    ///
    /// If true, use original vectors to re-score top-k results. If ignored, qdrant decides automatically does rescore enabled or not.
    #[allow(unused_mut)]
    pub fn rescore(self, value: bool) -> Self {
        let mut new = self;
        new.rescore = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Oversampling factor for quantization.
    ///
    /// Defines how many extra vectors should be pre-selected using quantized index,
    /// and then re-scored using original vectors.
    ///
    /// For example, if `oversampling` is 2.4 and `limit` is 100, then 240 vectors will be pre-selected using quantized index,
    /// and then top-100 will be returned after re-scoring.
    #[allow(unused_mut)]
    pub fn oversampling(self, value: f64) -> Self {
        let mut new = self;
        new.oversampling = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<QuantizationSearchParams, std::convert::Infallible> {
        Ok(QuantizationSearchParams {
            ignore: self.ignore.unwrap_or_default(),
            rescore: self.rescore.unwrap_or_default(),
            oversampling: self.oversampling.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            ignore: core::default::Default::default(),
            rescore: core::default::Default::default(),
            oversampling: core::default::Default::default(),
        }
    }
}

impl Default for QuantizationSearchParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl From<QuantizationSearchParamsBuilder> for QuantizationSearchParams {
    fn from(value: QuantizationSearchParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "QuantizationSearchParamsBuilder", "QuantizationSearchParams"
            )
        })
    }
}

impl QuantizationSearchParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> QuantizationSearchParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "QuantizationSearchParamsBuilder", "QuantizationSearchParams"
            )
        })
    }
}
