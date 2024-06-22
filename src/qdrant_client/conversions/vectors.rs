use std::collections::HashMap;

use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::{NamedVectors, SparseIndices, Vector, Vectors};

impl From<Vec<f32>> for Vector {
    fn from(vector: Vec<f32>) -> Self {
        Vector::new_dense(vector)
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
        let mut indices = Vec::with_capacity(tuples.len());
        let mut values = Vec::with_capacity(tuples.len());
        for (i, w) in tuples {
            indices.push(*i);
            values.push(*w);
        }
        Vector {
            data: values,
            indices: Some(SparseIndices { data: indices }),
            vectors_count: None,
        }
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
