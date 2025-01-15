use crate::qdrant::*;

#[derive(Default)]
pub struct MultiDenseVectorBuilder {
    pub(crate) vectors: Vec<DenseVector>,
}

impl MultiDenseVectorBuilder {
    pub fn new(vectors: impl Into<Vec<DenseVector>>) -> Self {
        Self {
            vectors: vectors.into(),
        }
    }

    pub fn single(vector: impl Into<DenseVector>) -> Self {
        Self::new(vec![vector.into()])
    }

    #[allow(unused_mut)]
    pub fn vectors(mut self, vectors: impl Into<Vec<DenseVector>>) -> Self {
        self.vectors = vectors.into();
        self
    }

    #[allow(unused_mut)]
    pub fn add_vector(mut self, vector: impl Into<DenseVector>) -> Self {
        self.vectors.push(vector.into());
        self
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> MultiDenseVector {
        MultiDenseVector {
            vectors: self.vectors,
        }
    }
}

impl From<Vec<Vec<f32>>> for MultiDenseVector {
    fn from(vectors: Vec<Vec<f32>>) -> Self {
        Self::from(
            vectors
                .into_iter()
                .map(DenseVector::from)
                .collect::<Vec<_>>(),
        )
    }
}

impl From<Vec<DenseVector>> for MultiDenseVector {
    fn from(vectors: Vec<DenseVector>) -> Self {
        MultiDenseVectorBuilder::new(vectors).build()
    }
}

impl From<MultiDenseVector> for Vector {
    fn from(dense_vector: MultiDenseVector) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector).into()
    }
}

impl From<MultiDenseVectorBuilder> for Vector {
    fn from(dense_vector: MultiDenseVectorBuilder) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector.build()).into()
    }
}

impl From<MultiDenseVector> for crate::qdrant::vector::Vector {
    fn from(dense_vector: MultiDenseVector) -> Self {
        Self::MultiDense(dense_vector)
    }
}
