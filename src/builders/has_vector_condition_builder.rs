use crate::qdrant::*;

pub struct HasVectorConditionBuilder {
    pub(crate) vector_name: String,
}

impl HasVectorConditionBuilder {
    pub fn new(vector_name: impl Into<String>) -> Self {
        Self {
            vector_name: vector_name.into(),
        }
    }

    /// Builds the desired type. Can often be omitted.
    fn build(self) -> HasVectorCondition {
        HasVectorCondition {
            has_vector: self.vector_name,
        }
    }
}

impl From<String> for HasVectorConditionBuilder {
    fn from(vector_name: String) -> Self {
        Self::new(vector_name)
    }
}

impl From<HasVectorConditionBuilder> for HasVectorCondition {
    fn from(value: HasVectorConditionBuilder) -> Self {
        value.build()
    }
}

impl From<String> for HasVectorCondition {
    fn from(vector_name: String) -> Self {
        HasVectorConditionBuilder::from(vector_name).into()
    }
}
