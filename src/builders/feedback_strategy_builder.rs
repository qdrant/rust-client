use crate::qdrant::*;

#[derive(Clone)]
pub struct FeedbackStrategyBuilder {
    pub(crate) variant: feedback_strategy::Variant,
}

impl FeedbackStrategyBuilder {
    /// Create a naive feedback strategy with specified coefficients.
    ///
    /// The naive strategy uses the formula: `a * score + sim(confidence^b * c * delta)`
    ///
    /// # Arguments
    ///
    /// * `a` - Coefficient for the original score component.
    /// * `b` - Exponent for confidence in the feedback component.
    /// * `c` - Coefficient for the delta in the feedback component.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::FeedbackStrategyBuilder;
    ///
    /// let strategy = FeedbackStrategyBuilder::naive(1.0, 1.0, 1.0);
    /// ```
    pub fn naive(a: f32, b: f32, c: f32) -> Self {
        Self {
            variant: feedback_strategy::Variant::Naive(NaiveFeedbackStrategy { a, b, c }),
        }
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> FeedbackStrategy {
        FeedbackStrategy {
            variant: Some(self.variant),
        }
    }
}

impl From<FeedbackStrategyBuilder> for FeedbackStrategy {
    fn from(value: FeedbackStrategyBuilder) -> Self {
        value.build()
    }
}
