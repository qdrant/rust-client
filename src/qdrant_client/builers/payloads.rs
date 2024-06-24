use crate::qdrant::{IntegerIndexParamsBuilder, TextIndexParamsBuilder, TokenizerType};

impl TextIndexParamsBuilder {
    pub fn new(tokenizer: TokenizerType) -> Self {
        let mut builder = Self::empty();
        builder.tokenizer = Some(tokenizer.into());
        builder
    }
}

impl IntegerIndexParamsBuilder {
    pub fn new(lookup: bool, range: bool) -> Self {
        Self::empty().lookup(lookup).range(range)
    }
}
