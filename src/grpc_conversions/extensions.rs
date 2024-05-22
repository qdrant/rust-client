use crate::client::Payload;
use crate::prelude::PointStruct;
use crate::qdrant::{PointId, Vectors};

impl PointStruct {
    pub fn new(
        id: impl Into<PointId>,
        vectors: impl Into<Vectors>,
        payload: impl Into<Payload>,
    ) -> Self {
        Self {
            id: Some(id.into()),
            payload: payload.into().into(),
            vectors: Some(vectors.into()),
        }
    }
}
