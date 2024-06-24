use crate::qdrant::payload_index_params::IndexParams;
use crate::qdrant::{IntegerIndexParams, PayloadIndexParams, TextIndexParams};

impl From<IndexParams> for PayloadIndexParams {
    fn from(value: IndexParams) -> Self {
        Self {
            index_params: Some(value),
        }
    }
}

impl From<TextIndexParams> for IndexParams {
    fn from(value: TextIndexParams) -> Self {
        Self::TextIndexParams(value)
    }
}

impl From<IntegerIndexParams> for IndexParams {
    fn from(value: IntegerIndexParams) -> Self {
        Self::IntegerIndexParams(value)
    }
}

impl From<IntegerIndexParams> for PayloadIndexParams {
    fn from(value: IntegerIndexParams) -> Self {
        Self {
            index_params: Some(IndexParams::from(value)),
        }
    }
}

impl From<TextIndexParams> for PayloadIndexParams {
    fn from(value: TextIndexParams) -> Self {
        Self {
            index_params: Some(IndexParams::from(value)),
        }
    }
}
