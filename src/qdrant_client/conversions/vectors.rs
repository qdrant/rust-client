use std::collections::HashMap;

use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::{vector, Document, Image, InferenceObject, NamedVectors, Vector, Vectors};

impl From<Vec<f32>> for Vector {
    fn from(vector: Vec<f32>) -> Self {
        Vector::new_dense(vector)
    }
}

/// Create dense vector from borrowed slice.
///
/// Useful for bulk uploads from contiguous memory (Arrow, NumPy) to reduce upfront allocations.
/// The copy happens during serialization rather than per-vector allocation.
impl From<&[f32]> for Vector {
    fn from(slice: &[f32]) -> Self {
        Vector::new_dense(slice.to_vec())
    }
}

impl From<Vec<(u32, f32)>> for Vector {
    fn from(tuples: Vec<(u32, f32)>) -> Self {
        Self::from(tuples.as_slice())
    }
}

impl From<Vec<Vec<f32>>> for Vector {
    fn from(vectors: Vec<Vec<f32>>) -> Self {
        Vector::new_multi(vectors)
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<&[(u32, f32)]> for Vector {
    fn from(tuples: &[(u32, f32)]) -> Self {
        let (indices, values): (Vec<_>, Vec<_>) = tuples.iter().cloned().unzip();
        Self::new_sparse(indices, values)
    }
}

impl From<NamedVectors> for Vectors {
    fn from(named_vectors: NamedVectors) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(named_vectors)),
        }
    }
}

impl From<Vector> for Vectors {
    fn from(vector: Vector) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(vector)),
        }
    }
}

impl From<HashMap<String, Vec<f32>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<f32>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

/// Create named vectors from borrowed slices.
///
/// Optimized for bulk uploads with multiple vector fields (e.g., text_embedding, image_embedding).
/// Useful when processing multiple Arrow columns or numpy arrays simultaneously.
impl From<HashMap<String, &[f32]>> for Vectors {
    fn from(named_vectors: HashMap<String, &[f32]>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vector>> for Vectors {
    fn from(named_vectors: HashMap<String, Vector>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors.into_iter().collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vec<(u32, f32)>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<(u32, f32)>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<HashMap<String, &[(u32, f32)]>> for Vectors {
    fn from(named_vectors: HashMap<String, &[(u32, f32)]>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<Vec<f32>> for Vectors {
    fn from(vector: Vec<f32>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(vector.into())),
        }
    }
}

/// Create single vector from borrowed slice.
///
/// Useful for bulk uploads from contiguous memory to reduce upfront allocations.
impl From<&[f32]> for Vectors {
    fn from(slice: &[f32]) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(slice.into())),
        }
    }
}

impl From<HashMap<String, Vector>> for NamedVectors {
    fn from(value: HashMap<String, Vector>) -> Self {
        Self { vectors: value }
    }
}

impl<T: Into<String>, H: Into<Vector>> From<Vec<(T, H)>> for NamedVectors {
    fn from(value: Vec<(T, H)>) -> Self {
        Self {
            vectors: value
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
        }
    }
}

impl From<VectorsOptions> for Vectors {
    fn from(value: VectorsOptions) -> Self {
        Self {
            vectors_options: Some(value),
        }
    }
}

impl From<Vector> for VectorsOptions {
    fn from(value: Vector) -> Self {
        Self::Vector(value)
    }
}

impl From<NamedVectors> for VectorsOptions {
    fn from(value: NamedVectors) -> Self {
        Self::Vectors(value)
    }
}

impl From<Document> for Vector {
    fn from(value: Document) -> Self {
        Vector {
            vector: Some(vector::Vector::Document(value)),
            ..Default::default()
        }
    }
}

impl From<Document> for Vectors {
    fn from(value: Document) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(Vector::from(value))),
        }
    }
}

impl From<HashMap<String, Document>> for Vectors {
    fn from(value: HashMap<String, Document>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: value
                    .into_iter()
                    .map(|(k, v)| (k, Vector::from(v)))
                    .collect(),
            })),
        }
    }
}

impl From<Image> for Vector {
    fn from(value: Image) -> Self {
        Vector {
            vector: Some(vector::Vector::Image(value)),
            ..Default::default()
        }
    }
}

impl From<Image> for Vectors {
    fn from(value: Image) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(Vector::from(value))),
        }
    }
}

impl From<HashMap<String, Image>> for Vectors {
    fn from(value: HashMap<String, Image>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: value
                    .into_iter()
                    .map(|(k, v)| (k, Vector::from(v)))
                    .collect(),
            })),
        }
    }
}

impl From<InferenceObject> for Vector {
    fn from(value: InferenceObject) -> Self {
        Vector {
            vector: Some(vector::Vector::Object(value)),
            ..Default::default()
        }
    }
}

impl From<InferenceObject> for Vectors {
    fn from(value: InferenceObject) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(Vector::from(value))),
        }
    }
}

impl From<HashMap<String, InferenceObject>> for Vectors {
    fn from(value: HashMap<String, InferenceObject>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: value
                    .into_iter()
                    .map(|(k, v)| (k, Vector::from(v)))
                    .collect(),
            })),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_from_slice() {
        let data: Vec<f32> = vec![1.0, 2.0, 3.0];
        let slice: &[f32] = &data;

        let vector = Vector::from(slice);

        if let Some(vector::Vector::Dense(dense)) = &vector.vector {
            assert_eq!(dense.data, vec![1.0, 2.0, 3.0]);
        } else {
            panic!("Expected dense vector");
        }
    }

    #[test]
    fn test_vectors_from_slice() {
        let data: Vec<f32> = vec![1.0, 2.0, 3.0];
        let slice: &[f32] = &data;

        let vectors = Vectors::from(slice);

        assert!(vectors.vectors_options.is_some());
        if let Some(VectorsOptions::Vector(vector)) = vectors.vectors_options {
            if let Some(vector::Vector::Dense(dense)) = vector.vector {
                assert_eq!(dense.data, vec![1.0, 2.0, 3.0]);
            } else {
                panic!("Expected dense vector");
            }
        } else {
            panic!("Expected VectorsOptions::Vector");
        }
    }

    #[test]
    fn test_named_vectors_from_slice_hashmap() {
        let data1: Vec<f32> = vec![1.0, 2.0, 3.0];
        let data2: Vec<f32> = vec![4.0, 5.0, 6.0];
        let slice1: &[f32] = &data1;
        let slice2: &[f32] = &data2;

        let mut named_vectors = HashMap::new();
        named_vectors.insert("vector1".to_string(), slice1);
        named_vectors.insert("vector2".to_string(), slice2);

        let vectors = Vectors::from(named_vectors);

        assert!(vectors.vectors_options.is_some());
        if let Some(VectorsOptions::Vectors(named)) = vectors.vectors_options {
            assert_eq!(named.vectors.len(), 2);
            assert!(named.vectors.contains_key("vector1"));
            assert!(named.vectors.contains_key("vector2"));

            if let Some(vector::Vector::Dense(dense)) = &named.vectors["vector1"].vector {
                assert_eq!(dense.data, vec![1.0, 2.0, 3.0]);
            } else {
                panic!("Expected dense vector for vector1");
            }

            if let Some(vector::Vector::Dense(dense)) = &named.vectors["vector2"].vector {
                assert_eq!(dense.data, vec![4.0, 5.0, 6.0]);
            } else {
                panic!("Expected dense vector for vector2");
            }
        } else {
            panic!("Expected VectorsOptions::Vectors");
        }
    }

    #[test]
    fn test_vector_from_slice_backwards_compatible() {
        let owned: Vec<f32> = vec![1.0, 2.0, 3.0];
        let borrowed: &[f32] = &[1.0, 2.0, 3.0];

        let vector_owned = Vector::from(owned);
        let vector_borrowed = Vector::from(borrowed);

        if let (
            Some(vector::Vector::Dense(dense_owned)),
            Some(vector::Vector::Dense(dense_borrowed)),
        ) = (&vector_owned.vector, &vector_borrowed.vector)
        {
            assert_eq!(dense_owned.data, dense_borrowed.data);
        } else {
            panic!("Expected dense vectors");
        }
    }
}
