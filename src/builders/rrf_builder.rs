use crate::qdrant::Rrf;

#[derive(Clone)]
pub struct RrfBuilder {
    /// K parameter for reciprocal rank fusion.
    ///
    /// Controls how quickly the weights decrease as rank increases.
    /// Higher values make the weighting more uniform across ranks.
    ///
    /// Default value is 60.
    pub(crate) k: Option<Option<u32>>,
    /// Weights for each prefetch source.
    /// Higher weight gives more influence on the final ranking.
    /// If not specified, all prefetches are weighted equally.
    pub(crate) weights: Option<Vec<f32>>,
}

impl RrfBuilder {
    /// Create a new RrfBuilder with default values.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::RrfBuilder;
    ///
    /// let rrf = RrfBuilder::new().build();
    /// ```
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Create a new RrfBuilder with a specific k parameter.
    ///
    /// # Arguments
    ///
    /// * `k` - K parameter for reciprocal rank fusion. Default is 60.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::RrfBuilder;
    ///
    /// let rrf = RrfBuilder::with_k(100).build();
    /// ```
    pub fn with_k(k: u32) -> Self {
        Self::new().k(k)
    }

    /// Set the K parameter for reciprocal rank fusion.
    ///
    /// Controls how quickly the weights decrease as rank increases.
    /// Higher values make the weighting more uniform across ranks.
    ///
    /// Default value is 60.
    pub fn k(self, value: u32) -> Self {
        let mut new = self;
        new.k = Option::Some(Option::Some(value));
        new
    }

    /// Weights for each prefetch source.
    /// Higher weight gives more influence on the final ranking.
    /// If not specified, all prefetches are weighted equally.
    pub fn weights(self, value: Vec<f32>) -> Self {
        let mut new = self;
        new.weights = Option::Some(value);
        new
    }

    pub fn build(self) -> Rrf {
        Rrf {
            k: self.k.unwrap_or_default(),
            weights: self.weights.unwrap_or_default(),
        }
    }

    /// Create an empty builder, with all fields set to `None`.
    fn create_empty() -> Self {
        Self {
            k: core::default::Default::default(),
            weights: core::default::Default::default(),
        }
    }
}

impl From<RrfBuilder> for Rrf {
    fn from(value: RrfBuilder) -> Self {
        value.build()
    }
}

impl Default for RrfBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
