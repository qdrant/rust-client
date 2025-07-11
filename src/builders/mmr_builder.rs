use crate::qdrant::Mmr;

#[derive(Clone)]
pub struct MmrBuilder {
    /// Tunable parameter for the MMR algorithm.
    /// Determines the balance between diversity and relevance.
    ///
    /// A higher value favors diversity (dissimilarity to selected results),
    /// while a lower value favors relevance (similarity to the query vector).
    ///
    /// Must be in the range [0, 1].
    /// Default value is 0.5.
    pub(crate) diversity: Option<Option<f32>>,
    /// The maximum number of candidates to consider for re-ranking.
    ///
    /// If not specified, the `limit` value is used.
    pub(crate) candidates_limit: Option<Option<u32>>,
}

impl MmrBuilder {
    /// Create a new MmrBuilder with default values.
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Create a new MmrBuilder with both diversity and candidates limit.
    ///
    /// # Arguments
    ///
    /// * `diversity` - Must be in the range [0, 1]. Higher values favor diversity.
    /// * `candidates_limit` - Maximum number of candidates to consider for re-ranking.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::MmrBuilder;
    ///
    /// let mmr = MmrBuilder::with_params(0.6, 100).build();
    /// ```
    pub fn with_params(diversity: f32, candidates_limit: u32) -> Self {
        Self::new()
            .diversity(diversity)
            .candidates_limit(candidates_limit)
    }

    /// Tunable parameter for the MMR algorithm.
    /// Determines the balance between diversity and relevance.
    ///
    /// A higher value favors diversity (dissimilarity to selected results),
    /// while a lower value favors relevance (similarity to the query vector).
    ///
    /// Must be in the range [0, 1].
    /// Default value is 0.5.
    pub fn diversity(self, value: f32) -> Self {
        let mut new = self;
        new.diversity = Option::Some(Option::Some(value));
        new
    }

    /// The maximum number of candidates to consider for re-ranking.
    ///
    /// If not specified, the `limit` value is used.
    pub fn candidates_limit(self, value: u32) -> Self {
        let mut new = self;
        new.candidates_limit = Option::Some(Option::Some(value));
        new
    }

    fn build(self) -> Mmr {
        Mmr {
            diversity: self.diversity.unwrap_or_default(),
            candidates_limit: self.candidates_limit.unwrap_or_default(),
        }
    }

    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            diversity: core::default::Default::default(),
            candidates_limit: core::default::Default::default(),
        }
    }
}

impl From<MmrBuilder> for Mmr {
    fn from(value: MmrBuilder) -> Self {
        value.build()
    }
}

impl Default for MmrBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
