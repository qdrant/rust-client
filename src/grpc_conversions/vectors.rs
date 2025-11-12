use crate::qdrant::vector_output::Vector;
use crate::qdrant::vectors_output::VectorsOptions;
use crate::qdrant::{MultiDenseVector, SparseVector, VectorOutput, VectorsOutput};

impl VectorOutput {
    #[allow(deprecated)]
    pub fn into_vector(self) -> Vector {
        let VectorOutput {
            data,
            indices,
            vectors_count,
            vector,
        } = self;

        if let Some(v) = vector {
            return v;
        }

        if let Some(indices) = indices {
            return Vector::Sparse(SparseVector::from((indices.data, data)));
        }

        if let Some(vectors_count) = vectors_count {
            let vectors: Vec<_> = data
                .chunks(data.len() / vectors_count as usize)
                .collect::<Vec<_>>();

            return Vector::MultiDense(MultiDenseVector::from(vectors));
        }

        Vector::Dense(crate::qdrant::DenseVector { data })
    }
}

impl VectorsOutput {
    /// Get default (unnamed) vector from VectorsOutput.
    ///
    /// Return `None` if the default vector is not present.
    ///
    /// Use `Self::get_vector_by_name` to get named vectors from VectorsOutput.
    pub fn get_vector(&self) -> Option<Vector> {
        self.vectors_options
            .as_ref()
            .and_then(|option| match option {
                VectorsOptions::Vector(vector) => Some(vector.clone().into_vector()),
                VectorsOptions::Vectors(_) => None,
            })
    }

    /// Get vector by name from VectorsOutput.
    ///
    /// Return `None` if the named vector is not present or is a default (unnamed) vector.
    ///
    /// Use `Self::get_vector` to get the default (unnamed) vector from VectorsOutput.
    pub fn get_vector_by_name(&self, name: &str) -> Option<Vector> {
        self.vectors_options
            .as_ref()
            .and_then(|option| match option {
                VectorsOptions::Vector(vector) => {
                    if name.is_empty() {
                        Some(vector.clone().into_vector())
                    } else {
                        None
                    }
                }
                VectorsOptions::Vectors(vectors) => vectors
                    .vectors
                    .get(name)
                    .cloned()
                    .map(|vector| vector.into_vector()),
            })
    }
}
