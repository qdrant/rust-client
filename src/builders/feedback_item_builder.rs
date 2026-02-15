use crate::qdrant::*;

#[derive(Clone)]
pub struct FeedbackItemBuilder {
    /// The id or vector from the original model
    pub(crate) example: VectorInput,
    /// Score for this vector as determined by the feedback provider
    pub(crate) score: f32,
}

impl FeedbackItemBuilder {
    /// Create a new builder with an example and its score.
    ///
    /// # Arguments
    ///
    /// * `example` - The id or vector from the original model.
    /// * `score` - Score for this vector as determined by the feedback provider.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::{FeedbackItemBuilder, PointId, VectorInput};
    ///
    /// let item = FeedbackItemBuilder::new(VectorInput::new_id(PointId::from(42)), 0.9);
    /// ```
    pub fn new(example: impl Into<VectorInput>, score: f32) -> Self {
        Self {
            example: example.into(),
            score,
        }
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> FeedbackItem {
        FeedbackItem {
            example: Some(self.example),
            score: self.score,
        }
    }
}

impl From<FeedbackItemBuilder> for FeedbackItem {
    fn from(value: FeedbackItemBuilder) -> Self {
        value.build()
    }
}
