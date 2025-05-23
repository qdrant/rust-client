use crate::qdrant::*;

#[derive(Clone)]
pub struct DenseVectorBuilder {
    pub(crate) values: Vec<f32>,
}

impl DenseVectorBuilder {
    pub fn new(values: impl Into<Vec<f32>>) -> Self {
        Self {
            values: values.into(),
        }
    }

    #[allow(unused_mut)]
    pub fn values(mut self, values: impl Into<Vec<f32>>) -> Self {
        self.values = values.into();
        self
    }

    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DenseVector {
        DenseVector { data: self.values }
    }
}

impl From<Vec<f32>> for DenseVector {
    fn from(values: Vec<f32>) -> Self {
        DenseVectorBuilder::new(values).build()
    }
}

impl From<DenseVector> for Vector {
    fn from(dense_vector: DenseVector) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector).into()
    }
}

impl From<DenseVectorBuilder> for Vector {
    fn from(dense_vector: DenseVectorBuilder) -> Self {
        crate::qdrant::vector::Vector::from(dense_vector.build()).into()
    }
}

impl From<DenseVector> for crate::qdrant::vector::Vector {
    fn from(dense_vector: DenseVector) -> Self {
        Self::Dense(dense_vector)
    }
}
