use std::collections::HashMap;

use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::{vector, Document, Image, InferenceObject, NamedVectors, Vector, Vectors};

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
                vectors: value.into_iter().map(|(k, v)| (k, Vector::from(v))).collect(),
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
                vectors: value.into_iter().map(|(k, v)| (k, Vector::from(v))).collect(),
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
                vectors: value.into_iter().map(|(k, v)| (k, Vector::from(v))).collect(),
            })),
        }
    }
}
