use crate::qdrant::*;

pub struct ContextInputBuilder {
    /// Search space will be constrained by these pairs of vectors
    pub(crate) pairs: Option<Vec<ContextInputPair>>,
}

impl ContextInputBuilder {
    /// Search space will be constrained by these pairs of vectors
    #[allow(unused_mut)]
    pub fn pairs<VALUE: core::convert::Into<Vec<ContextInputPair>>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.pairs = Option::Some(value.into());
        new
    }

    fn build_inner(self) -> Result<ContextInput, std::convert::Infallible> {
        Ok(ContextInput {
            pairs: match self.pairs {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            pairs: core::default::Default::default(),
        }
    }
}

impl From<ContextInputBuilder> for ContextInput {
    fn from(value: ContextInputBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ContextInputBuilder", "ContextInput",
        ))
    }
}

impl ContextInputBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ContextInput {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ContextInputBuilder", "ContextInput",
        ))
    }
}

impl Default for ContextInputBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
