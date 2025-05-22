use crate::qdrant::*;

#[derive(Clone, Default)]
pub struct SparseVectorBuilder {
    pub(crate) indices: Vec<u32>,
    pub(crate) values: Vec<f32>,
}

impl SparseVectorBuilder {
    pub fn new(indices: impl Into<Vec<u32>>, values: impl Into<Vec<f32>>) -> Self {
        Self {
            indices: indices.into(),
            values: values.into(),
        }
    }

    #[allow(unused_mut)]
    pub fn indices(mut self, indices: impl Into<Vec<u32>>) -> Self {
        self.indices = indices.into();
        self
    }

    #[allow(unused_mut)]
    pub fn values(mut self, values: impl Into<Vec<f32>>) -> Self {
        self.values = values.into();
        self
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SparseVector {
        SparseVector {
            indices: self.indices,
            values: self.values,
        }
    }
}

impl From<(Vec<u32>, Vec<f32>)> for SparseVector {
    fn from((indices, values): (Vec<u32>, Vec<f32>)) -> Self {
        SparseVectorBuilder::new(indices, values).build()
    }
}

impl From<SparseVector> for Vector {
    fn from(dense_vector: SparseVector) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector).into()
    }
}

impl From<SparseVectorBuilder> for Vector {
    fn from(dense_vector: SparseVectorBuilder) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector.build()).into()
    }
}

impl From<SparseVector> for crate::qdrant::vector::Vector {
    fn from(dense_vector: SparseVector) -> Self {
        Self::Sparse(dense_vector)
    }
}
