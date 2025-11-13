use crate::qdrant::AcornSearchParams;

#[derive(Clone)]
pub struct AcornSearchParamsBuilder {
    /// If true, then ACORN may be used for the HNSW search based on filters selectivity.
    ///
    /// Improves search recall for searches with multiple low-selectivity
    /// payload filters, at cost of performance.
    pub(crate) enable: Option<Option<bool>>,
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
    /// Create a new AcornSearchParamsBuilder with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::AcornSearchParamsBuilder;
    ///
    /// let acorn = AcornSearchParamsBuilder::new().build();
    /// ```
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Create a new AcornSearchParamsBuilder with enabled flag set.
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
    /// let acorn = AcornSearchParamsBuilder::with_enable(true).build();
    /// ```
    pub fn with_enable(enable: bool) -> Self {
        Self::new().enable(enable)
    }

    /// Create a new AcornSearchParamsBuilder with both enable and max_selectivity.
    ///
    /// # Arguments
    ///
    /// * `enable` - If true, ACORN may be used for HNSW search based on filter selectivity
    /// * `max_selectivity` - Value between 0.0 (never) and 1.0 (always). Default is 0.4.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::AcornSearchParamsBuilder;
    ///
    /// let acorn = AcornSearchParamsBuilder::with_params(true, 0.5).build();
    /// ```
    pub fn with_params(enable: bool, max_selectivity: f64) -> Self {
        Self::new().enable(enable).max_selectivity(max_selectivity)
    }

    /// Set if ACORN may be used for the HNSW search based on filters selectivity.
    ///
    /// Improves search recall for searches with multiple low-selectivity
    /// payload filters, at cost of performance.
    pub fn enable(self, value: bool) -> Self {
        let mut new = self;
        new.enable = Option::Some(Option::Some(value));
        new
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
            enable: self.enable.unwrap_or_default(),
            max_selectivity: self.max_selectivity.unwrap_or_default(),
        }
    }

    /// Create an empty builder, with all fields set to `None`.
    fn create_empty() -> Self {
        Self {
            enable: core::default::Default::default(),
            max_selectivity: core::default::Default::default(),
        }
    }
}

impl From<AcornSearchParamsBuilder> for AcornSearchParams {
    fn from(value: AcornSearchParamsBuilder) -> Self {
        value.build()
    }
}

impl Default for AcornSearchParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
