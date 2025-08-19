use crate::qdrant::*;

#[derive(Clone)]
pub struct DisabledQuantizationBuilder {}

impl DisabledQuantizationBuilder {
    pub fn empty() -> Self {
        Self {}
    }

    fn build_inner(self) -> Disabled {
        Disabled {}
    }
}

impl From<DisabledQuantizationBuilder> for Disabled {
    fn from(value: DisabledQuantizationBuilder) -> Self {
        value.build_inner()
    }
}

impl DisabledQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> Disabled {
        self.build_inner()
    }
}
