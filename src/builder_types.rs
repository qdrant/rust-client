use crate::qdrant::{PointId, Vector};

/// A recommendation example, being a [`PointId`] or a [`Vector`]
pub enum RecommendExample {
    PointId(PointId),
    Vector(Vector),
}

impl From<Vector> for RecommendExample {
    fn from(value: Vector) -> Self {
        Self::Vector(value)
    }
}

impl From<PointId> for RecommendExample {
    fn from(value: PointId) -> Self {
        Self::PointId(value)
    }
}
