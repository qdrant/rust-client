use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::{PointId, Vector};

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

impl From<u64> for RecommendExample {
    fn from(value: u64) -> Self {
        Self::PointId(PointId {
            point_id_options: Some(PointIdOptions::Num(value)),
        })
    }
}

impl From<&str> for RecommendExample {
    fn from(value: &str) -> Self {
        Self::PointId(PointId {
            point_id_options: Some(PointIdOptions::Uuid(value.to_string())),
        })
    }
}

impl From<String> for RecommendExample {
    fn from(value: String) -> Self {
        Self::PointId(PointId {
            point_id_options: Some(PointIdOptions::Uuid(value)),
        })
    }
}

impl From<Vec<f32>> for RecommendExample {
    fn from(value: Vec<f32>) -> Self {
        Self::Vector(value.into())
    }
}
