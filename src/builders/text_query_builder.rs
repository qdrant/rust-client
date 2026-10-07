use crate::qdrant::TextQuery;

/// Builder for [`TextQuery`], ranking points by BM25 over the text index of a payload field.
///
/// The payload field is selected with `using` on the query request, and its text index must
/// have scoring enabled, see [`TextIndexParamsBuilder::bm25_scoring`](crate::qdrant::TextIndexParamsBuilder::bm25_scoring).
#[must_use]
#[derive(Clone)]
pub struct TextQueryBuilder {
    /// Text to search for, tokenized by the field's text index.
    pub(crate) query: String,
    /// Term frequency saturation. Default is 1.2.
    pub(crate) k: Option<f32>,
    /// Document length normalization, from 0 (none) to 1 (full). Default is 0.75.
    pub(crate) b: Option<f32>,
}

impl TextQueryBuilder {
    /// Create a new TextQueryBuilder for the given text.
    ///
    /// # Examples
    ///
    /// ```
    /// use qdrant_client::qdrant::TextQueryBuilder;
    ///
    /// let text_query = TextQueryBuilder::new("vector database").k(1.5).b(0.5).build();
    /// ```
    pub fn new(query: impl Into<String>) -> Self {
        Self {
            query: query.into(),
            k: None,
            b: None,
        }
    }

    /// Term frequency saturation. Default is 1.2.
    pub fn k(self, value: f32) -> Self {
        let mut new = self;
        new.k = Some(value);
        new
    }

    /// Document length normalization, from 0 (none) to 1 (full). Default is 0.75.
    pub fn b(self, value: f32) -> Self {
        let mut new = self;
        new.b = Some(value);
        new
    }

    pub fn build(self) -> TextQuery {
        TextQuery {
            query: self.query,
            k: self.k,
            b: self.b,
        }
    }
}

impl From<TextQueryBuilder> for TextQuery {
    fn from(value: TextQueryBuilder) -> Self {
        value.build()
    }
}
