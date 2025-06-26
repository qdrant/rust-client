use crate::qdrant::{vector_input, Document, Image, InferenceObject, PointId, VectorInput};

impl<T: Into<PointId>> From<T> for VectorInput {
    fn from(value: T) -> Self {
        Self::new_id(value)
    }
}

impl From<Vec<f32>> for VectorInput {
    fn from(value: Vec<f32>) -> Self {
        Self::new_dense(value)
    }
}

impl From<&[(u32, f32)]> for VectorInput {
    fn from(value: &[(u32, f32)]) -> Self {
        let (indices, values): (Vec<_>, Vec<_>) = value.iter().copied().unzip();
        Self::new_sparse(indices, values)
    }
}

impl From<Vec<Vec<f32>>> for VectorInput {
    fn from(value: Vec<Vec<f32>>) -> Self {
        Self::new_multi(value)
    }
}

impl From<Document> for VectorInput {
    fn from(value: Document) -> Self {
        VectorInput {
            variant: Some(vector_input::Variant::Document(value)),
        }
    }
}

impl From<Image> for VectorInput {
    fn from(value: Image) -> Self {
        VectorInput {
            variant: Some(vector_input::Variant::Image(value)),
        }
    }
}

impl From<InferenceObject> for VectorInput {
    fn from(value: InferenceObject) -> Self {
        VectorInput {
            variant: Some(vector_input::Variant::Object(value)),
        }
    }
}
