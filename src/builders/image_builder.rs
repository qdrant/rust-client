use std::collections::HashMap;

use crate::qdrant::{value, Image, Value};

impl Image {
    pub fn new_from_url(url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            image: Some(Value {
                kind: Some(value::Kind::StringValue(url.into())),
            }),
            model: model.into(),
            options: HashMap::new(),
        }
    }

    pub fn new_from_base64(base64: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            image: Some(Value {
                kind: Some(value::Kind::StringValue(base64.into())),
            }),
            model: model.into(),
            options: HashMap::new(),
        }
    }
}

pub struct ImageBuilder {
    image: Option<Value>,
    model: String,
    options: HashMap<String, Value>,
}

impl ImageBuilder {
    pub fn new_from_url(url: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            image: Some(Value {
                kind: Some(value::Kind::StringValue(url.into())),
            }),
            model: model.into(),
            options: HashMap::new(),
        }
    }

    pub fn new_from_base64(base64: impl Into<String>, model: impl Into<String>) -> Self {
        Self {
            image: Some(Value {
                kind: Some(value::Kind::StringValue(base64.into())),
            }),
            model: model.into(),
            options: HashMap::new(),
        }
    }

    pub fn image(mut self, image: Value) -> Self {
        self.image = Some(image);
        self
    }

    pub fn model(mut self, model: String) -> Self {
        self.model = model;
        self
    }

    pub fn options(mut self, options: HashMap<String, Value>) -> Self {
        self.options = options;
        self
    }

    pub fn build(self) -> Image {
        Image {
            image: self.image,
            model: self.model,
            options: self.options,
        }
    }
}
