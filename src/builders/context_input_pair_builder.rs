use crate::qdrant::*;

pub struct ContextInputPairBuilder {
    /// A positive vector
    pub(crate) positive: Option<Option<VectorInput>>,
    /// Repel from this vector
    pub(crate) negative: Option<Option<VectorInput>>,
}

impl ContextInputPairBuilder {
    /// A positive vector
    #[allow(unused_mut)]
    pub fn positive<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.positive = Option::Some(Option::Some(value.into()));
        new
    }
    /// Repel from this vector
    #[allow(unused_mut)]
    pub fn negative<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.negative = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `ContextInputPair`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<ContextInputPair, ContextInputPairBuilderError> {
        Ok(ContextInputPair {
            positive: match self.positive {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            negative: match self.negative {
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
        }
    }
}

impl From<ContextInputPairBuilder> for ContextInputPair {
    fn from(value: ContextInputPairBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ContextInputPairBuilder", "ContextInputPair",
        ))
    }
}

impl ContextInputPairBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ContextInputPair {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ContextInputPairBuilder", "ContextInputPair",
        ))
    }
}

impl ContextInputPairBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
