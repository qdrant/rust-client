use crate::qdrant::*;

#[derive(Clone)]
pub struct RelevanceFeedbackInputBuilder {
    /// The original query vector
    pub(crate) target: VectorInput,
    /// Previous results scored by the feedback provider
    pub(crate) feedback: Vec<FeedbackItem>,
    /// Formula and trained coefficients to use
    pub(crate) strategy: Option<FeedbackStrategy>,
}

impl RelevanceFeedbackInputBuilder {
    /// Create a new builder with a target vector.
    ///
    /// # Arguments
    ///
    /// * `target` - The original query vector to search around.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{RelevanceFeedbackInputBuilder, VectorInput};
    ///
    /// let builder = RelevanceFeedbackInputBuilder::new(VectorInput::new_dense(vec![0.1, 0.2, 0.3]));
    /// ```
    pub fn new(target: impl Into<VectorInput>) -> Self {
        Self {
            target: target.into(),
            feedback: Vec::new(),
            strategy: None,
        }
    }

    /// Add a single feedback item.
    pub fn add_feedback(mut self, item: impl Into<FeedbackItem>) -> Self {
        self.feedback.push(item.into());
        self
    }

    /// Set the feedback strategy.
    pub fn strategy(mut self, value: impl Into<FeedbackStrategy>) -> Self {
        self.strategy = Some(value.into());
        self
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> RelevanceFeedbackInput {
        RelevanceFeedbackInput {
            target: Some(self.target),
            feedback: self.feedback,
            strategy: self.strategy,
        }
    }
}

impl From<RelevanceFeedbackInputBuilder> for RelevanceFeedbackInput {
    fn from(value: RelevanceFeedbackInputBuilder) -> Self {
        value.build()
    }
}
