use crate::qdrant::*;

pub struct RecommendInputBuilder {
    /// Look for vectors closest to the vectors from these points
    pub(crate) positive: Option<Vec<VectorInput>>,
    /// Try to avoid vectors like the vector from these points
    pub(crate) negative: Option<Vec<VectorInput>>,
    /// How to use the provided vectors to find the results
    pub(crate) strategy: Option<Option<i32>>,
}

impl RecommendInputBuilder {
    /// Look for vectors closest to the vectors from these points
    #[allow(unused_mut)]
    pub fn positive<VALUE: core::convert::Into<Vec<VectorInput>>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.positive = Option::Some(value.into());
        new
    }
    /// Try to avoid vectors like the vector from these points
    #[allow(unused_mut)]
    pub fn negative<VALUE: core::convert::Into<Vec<VectorInput>>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.negative = Option::Some(value.into());
        new
    }
    /// How to use the provided vectors to find the results
    #[allow(unused_mut)]
    pub fn strategy<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.strategy = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<RecommendInput, std::convert::Infallible> {
        Ok(RecommendInput {
            positive: match self.positive {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            negative: match self.negative {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            strategy: match self.strategy {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            positive: core::default::Default::default(),
            negative: core::default::Default::default(),
            strategy: core::default::Default::default(),
        }
    }
}

impl From<RecommendInputBuilder> for RecommendInput {
    fn from(value: RecommendInputBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "RecommendInputBuilder", "RecommendInput",
        ))
    }
}

impl RecommendInputBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> RecommendInput {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "RecommendInputBuilder", "RecommendInput",
        ))
    }
}

impl Default for RecommendInputBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
