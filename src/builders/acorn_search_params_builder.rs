use crate::qdrant::*;

impl AcornSearchParams {
    /// Create new ACORN search parameters
    ///
    /// # Arguments
    /// * `enable` - If true, ACORN may be used for HNSW search based on filter selectivity
    ///
    /// # Example
    /// ```
    /// use qdrant_client::qdrant::AcornSearchParams;
    ///
    /// let acorn = AcornSearchParams::new(true);
    /// let acorn_with_selectivity = AcornSearchParams::new(true)
    ///     .with_max_selectivity(0.5);
    /// ```
    pub fn new(enable: bool) -> Self {
        Self {
            enable: Some(enable),
            max_selectivity: None,
        }
    }

    /// Set maximum selectivity threshold for enabling ACORN
    ///
    /// If estimated filter selectivity is higher than this value, ACORN will not be used.
    /// Selectivity is estimated as: `estimated number of points satisfying the filters / total number of points`
    ///
    /// # Arguments
    /// * `max_selectivity` - Value between 0.0 (never) and 1.0 (always). Default is 0.4.
    pub fn with_max_selectivity(mut self, max_selectivity: f64) -> Self {
        self.max_selectivity = Some(max_selectivity);
        self
    }
}
