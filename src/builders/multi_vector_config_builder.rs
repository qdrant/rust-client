use crate::qdrant::*;

pub struct MultiVectorConfigBuilder {
    /// Comparator for multi-vector search
    pub(crate) comparator: Option<i32>,
}

impl MultiVectorConfigBuilder {
    /// Comparator for multi-vector search
    #[allow(unused_mut)]
    pub fn comparator<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.comparator = Option::Some(value.into());
        new
    }
    /**Builds a new `MultiVectorConfig`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<MultiVectorConfig, MultiVectorConfigBuilderError> {
        Ok(MultiVectorConfig {
            comparator: match self.comparator {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            comparator: core::default::Default::default(),
        }
    }
}

impl From<MultiVectorConfigBuilder> for MultiVectorConfig {
    fn from(value: MultiVectorConfigBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "MultiVectorConfigBuilder", "MultiVectorConfig",
        ))
    }
}

impl MultiVectorConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> MultiVectorConfig {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "MultiVectorConfigBuilder", "MultiVectorConfig",
        ))
    }
}

impl MultiVectorConfigBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
