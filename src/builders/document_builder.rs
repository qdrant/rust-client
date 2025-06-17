use std::collections::HashMap;

use crate::qdrant::{Document, Value};

impl Document {
    pub fn new(text: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            model: model.into(),
            options: HashMap::new(),
        }
    }
}

pub struct DocumentBuilder {
    text: String,
    model: String,
    options: HashMap<String, Value>,
}

impl DocumentBuilder {
    pub fn new(text: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            model: model.into(),
            options: HashMap::new(),
        }
    }

    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.text = text.into();
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.model = model.into();
        self
    }

    pub fn options(mut self, options: HashMap<String, Value>) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> Document {
        Document {
            text: self.text,
            model: self.model,
            options: self.options,
        }
    }
}
