use crate::qdrant::*;

pub struct DiscoverInputBuilder {
    /// Use this as the primary search objective
    pub(crate) target: Option<Option<VectorInput>>,
    /// Search space will be constrained by these pairs of vectors
    pub(crate) context: Option<Option<ContextInput>>,
}

impl DiscoverInputBuilder {
    /// Use this as the primary search objective
    #[allow(unused_mut)]
    pub fn target<VALUE: core::convert::Into<VectorInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.target = Option::Some(Option::Some(value.into()));
        new
    }
    /// Search space will be constrained by these pairs of vectors
    #[allow(unused_mut)]
    pub fn context<VALUE: core::convert::Into<ContextInput>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.context = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `DiscoverInput`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<DiscoverInput, DiscoverInputBuilderError> {
        Ok(DiscoverInput {
            target: match self.target {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            context: match self.context {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            target: core::default::Default::default(),
            context: core::default::Default::default(),
        }
    }
}

impl From<DiscoverInputBuilder> for DiscoverInput {
    fn from(value: DiscoverInputBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DiscoverInputBuilder", "DiscoverInput",
        ))
    }
}

impl DiscoverInputBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DiscoverInput {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DiscoverInputBuilder", "DiscoverInput",
        ))
    }
}

impl DiscoverInputBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
