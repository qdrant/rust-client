use crate::qdrant::*;

#[derive(Clone)]
pub struct ContextExamplePairBuilder {
    pub(crate) positive: Option<Option<VectorExample>>,
    pub(crate) negative: Option<Option<VectorExample>>,
}

impl ContextExamplePairBuilder {
    #[allow(unused_mut)]
    pub fn positive<VALUE: core::convert::Into<VectorExample>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.positive = Option::Some(Option::Some(value.into()));
        new
    }
    #[allow(unused_mut)]
    pub fn negative<VALUE: core::convert::Into<VectorExample>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.negative = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<ContextExamplePair, std::convert::Infallible> {
        Ok(ContextExamplePair {
            positive: self.positive.unwrap_or_default(),
            negative: self.negative.unwrap_or_default(),
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

impl From<ContextExamplePairBuilder> for ContextExamplePair {
    fn from(value: ContextExamplePairBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "ContextExamplePairBuilder", "ContextExamplePair"
            )
        })
    }
}

impl ContextExamplePairBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ContextExamplePair {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "ContextExamplePairBuilder", "ContextExamplePair"
            )
        })
    }
}

impl Default for ContextExamplePairBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
