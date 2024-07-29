use crate::qdrant::payload_index_params::IndexParams;
use crate::qdrant::{
    DatetimeIndexParams, DatetimeIndexParamsBuilder, FloatIndexParams, FloatIndexParamsBuilder,
    IntegerIndexParams, IntegerIndexParamsBuilder, KeywordIndexParams, KeywordIndexParamsBuilder,
    PayloadIndexParams, TextIndexParams, TextIndexParamsBuilder,
};

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

impl From<TextIndexParamsBuilder> for IndexParams {
    fn from(value: TextIndexParamsBuilder) -> Self {
        Self::TextIndexParams(value.build())
    }
}

impl From<TextIndexParams> for PayloadIndexParams {
    fn from(value: TextIndexParams) -> Self {
        Self {
            index_params: Some(IndexParams::from(value)),
        }
    }
}

impl From<IntegerIndexParams> for IndexParams {
    fn from(value: IntegerIndexParams) -> Self {
        Self::IntegerIndexParams(value)
    }
}

impl From<IntegerIndexParamsBuilder> for IndexParams {
    fn from(value: IntegerIndexParamsBuilder) -> Self {
        Self::IntegerIndexParams(value.build())
    }
}

impl From<IntegerIndexParams> for PayloadIndexParams {
    fn from(value: IntegerIndexParams) -> Self {
        Self {
            index_params: Some(IndexParams::from(value)),
        }
    }
}

impl From<KeywordIndexParams> for IndexParams {
    fn from(value: KeywordIndexParams) -> Self {
        Self::KeywordIndexParams(value)
    }
}

impl From<KeywordIndexParamsBuilder> for IndexParams {
    fn from(value: KeywordIndexParamsBuilder) -> Self {
        Self::KeywordIndexParams(value.build())
    }
}

impl From<FloatIndexParams> for IndexParams {
    fn from(value: FloatIndexParams) -> Self {
        Self::FloatIndexParams(value)
    }
}

impl From<FloatIndexParamsBuilder> for IndexParams {
    fn from(value: FloatIndexParamsBuilder) -> Self {
        Self::FloatIndexParams(value.build())
    }
}

impl From<DatetimeIndexParams> for IndexParams {
    fn from(value: DatetimeIndexParams) -> Self {
        Self::DatetimeIndexParams(value)
    }
}

impl From<DatetimeIndexParamsBuilder> for IndexParams {
    fn from(value: DatetimeIndexParamsBuilder) -> Self {
        Self::DatetimeIndexParams(value.build())
    }
}
