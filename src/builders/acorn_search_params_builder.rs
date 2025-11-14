use crate::qdrant::AcornSearchParams;

#[derive(Clone)]
pub struct AcornSearchParamsBuilder {
    /// If true, then ACORN may be used for the HNSW search based on filters selectivity.
    ///
    /// Improves search recall for searches with multiple low-selectivity
    /// payload filters, at cost of performance.
    pub(crate) enable: bool,
    /// Maximum selectivity of filters to enable ACORN.
    ///
    /// If estimated filters selectivity is higher than this value,
    /// ACORN will not be used. Selectivity is estimated as:
    /// `estimated number of points satisfying the filters / total number of points`.
    ///
    /// 0.0 for never, 1.0 for always. Default is 0.4.
    pub(crate) max_selectivity: Option<Option<f64>>,
}

impl AcornSearchParamsBuilder {
    /// Create a new AcornSearchParamsBuilder with required enable parameter.
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, ACORN may be used for HNSW search based on filter selectivity
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::AcornSearchParamsBuilder;
    ///
    /// let acorn = AcornSearchParamsBuilder::new(true).build();
    /// let acorn_with_selectivity = AcornSearchParamsBuilder::new(true)
    ///     .max_selectivity(0.5)
    ///     .build();
    /// ```
    pub fn new(enable: bool) -> Self {
        Self {
            enable,
            max_selectivity: None,
        }
    }

    /// Set maximum selectivity threshold for enabling ACORN.
    ///
    /// If estimated filter selectivity is higher than this value, ACORN will not be used.
    /// Selectivity is estimated as: `estimated number of points satisfying the filters / total number of points`
    ///
    /// Value between 0.0 (never) and 1.0 (always). Default is 0.4.
    pub fn max_selectivity(self, value: f64) -> Self {
        let mut new = self;
        new.max_selectivity = Option::Some(Option::Some(value));
        new
    }

    pub fn build(self) -> AcornSearchParams {
        AcornSearchParams {
            enable: Some(self.enable),
            max_selectivity: self.max_selectivity.unwrap_or_default(),
        }
    }
}

impl From<AcornSearchParamsBuilder> for AcornSearchParams {
    fn from(value: AcornSearchParamsBuilder) -> Self {
        value.build()
    }
}
