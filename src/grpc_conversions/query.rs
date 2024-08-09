use crate::qdrant::{query, Query, VectorInput};

impl From<Vec<f32>> for Query {
    fn from(value: Vec<f32>) -> Self {
        Query {
            variant: Some(query::Variant::Nearest(VectorInput::new_dense(value))),
        }
    }
}

impl From<Vec<(u32, f32)>> for Query {
    fn from(value: Vec<(u32, f32)>) -> Self {
        let (indices, values): (Vec<_>, Vec<_>) = value.iter().copied().unzip();
        Query {
            variant: Some(query::Variant::Nearest(VectorInput::new_sparse(
                indices, values,
            ))),
        }
    }
}

impl From<Vec<Vec<f32>>> for Query {
    fn from(value: Vec<Vec<f32>>) -> Self {
        Query {
            variant: Some(query::Variant::Nearest(VectorInput::new_multi(value))),
        }
    }
}
