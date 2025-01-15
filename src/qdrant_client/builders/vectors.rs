use crate::qdrant::{DenseVector, MultiDenseVector, NamedVectors, SparseVector, Vector};
use crate::QdrantError;

impl Vector {
    pub fn new(values: Vec<f32>) -> Self {
        Self::new_dense(values)
    }

    pub fn new_dense(values: Vec<f32>) -> Self {
        Vector {
            vector: Some(crate::qdrant::vector::Vector::Dense(DenseVector {
                data: values,
            })),
            // Deprecated
            data: vec![],
            indices: None,
            vectors_count: None,
        }
    }

    pub fn new_sparse(indices: impl Into<Vec<u32>>, values: impl Into<Vec<f32>>) -> Self {
        Vector {
            vector: Some(crate::qdrant::vector::Vector::Sparse(SparseVector {
                values: values.into(),
                indices: indices.into(),
            })),
            // Deprecated
            data: vec![],
            indices: None,
            vectors_count: None,
        }
    }

    pub fn new_multi(values: Vec<Vec<f32>>) -> Self {
        let vectors = values
            .into_iter()
            .map(|data| DenseVector { data })
            .collect();
        Vector {
            vector: Some(crate::qdrant::vector::Vector::MultiDense(
                MultiDenseVector { vectors },
            )),
            // Deprecated
            data: vec![],
            indices: None,
            vectors_count: None,
        }
    }

    pub fn try_into_dense(self) -> Result<Vec<f32>, QdrantError> {
        if self.indices.is_some() {
            return Err(QdrantError::ConversionError(
                "Cannot convert sparse vector to dense".to_string(),
            ));
        }

        if self.vectors_count.is_some() && self.vectors_count.unwrap() > 1 {
            return Err(QdrantError::ConversionError(
                "Cannot convert multi vector to dense".to_string(),
            ));
        }

        Ok(self.data)
    }

    pub fn try_into_sparse(self) -> Result<(Vec<u32>, Vec<f32>), QdrantError> {
        if self.indices.is_none() {
            return Err(QdrantError::ConversionError(
                "Cannot convert dense vector to sparse".to_string(),
            ));
        }

        if self.vectors_count.is_some() && self.vectors_count.unwrap() > 1 {
            return Err(QdrantError::ConversionError(
                "Cannot convert multi vector to sparse".to_string(),
            ));
        }

        let indices = self.indices.unwrap().data;

        if indices.len() != self.data.len() {
            return Err(QdrantError::ConversionError(format!(
                "Malformed sparse vector: indices length {} is not equal to data length {}",
                indices.len(),
                self.data.len()
            )));
        }

        Ok((indices, self.data))
    }

    pub fn try_into_multi(self) -> Result<Vec<Vec<f32>>, QdrantError> {
        if self.vectors_count.is_none() {
            return Err(QdrantError::ConversionError(
                "Cannot convert single vector to multi".to_string(),
            ));
        }

        let vectors_count = self.vectors_count.unwrap();

        if self.data.len() % vectors_count as usize != 0 {
            return Err(QdrantError::ConversionError(format!(
                "Malformed multi vector: data length {} is not divisible by vectors count {}",
                self.data.len(),
                vectors_count
            )));
        }

        Ok(self
            .data
            .chunks(self.data.len() / self.vectors_count.unwrap() as usize)
            .map(|v| v.to_vec())
            .collect())
    }
}

impl NamedVectors {
    pub fn add_vector(mut self, name: impl Into<String>, vector: impl Into<Vector>) -> Self {
        self.vectors.insert(name.into(), vector.into());
        self
    }
}
