use crate::qdrant::{
    vector_input, DenseVector, MultiDenseVector, PointId, SparseVector, VectorInput,
};

impl VectorInput {
    pub fn new_id(point_id: impl Into<PointId>) -> Self {
        Self {
            variant: Some(vector_input::Variant::Id(point_id.into())),
        }
    }

    pub fn new_dense(vector: impl Into<Vec<f32>>) -> Self {
        let vector = vector.into();
        Self {
            variant: Some(vector_input::Variant::Dense(DenseVector { data: vector })),
        }
    }

    pub fn new_sparse(indices: impl Into<Vec<u32>>, values: impl Into<Vec<f32>>) -> Self {
        Self {
            variant: Some(vector_input::Variant::Sparse(SparseVector {
                values: values.into(),
                indices: indices.into(),
            })),
        }
    }

    pub fn new_multi(vectors: impl Into<Vec<Vec<f32>>>) -> Self {
        let vectors = vectors.into();

        Self {
            variant: Some(vector_input::Variant::MultiDense(MultiDenseVector {
                vectors: vectors
                    .into_iter()
                    .map(|v| DenseVector { data: v })
                    .collect(),
            })),
        }
    }
}
