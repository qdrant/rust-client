use std::collections::HashMap;

use crate::qdrant::*;

/// Builder for the Formula struct, which represents a scoring formula for points.
///
/// The Formula struct is used to define custom scoring expressions and default values.
pub struct FormulaBuilder {
    /// The expression that defines how to score points.
    pub(crate) expression: Expression,
    /// Default values to use for undefined variables in the expression.
    pub(crate) defaults: HashMap<String, Value>,
}

impl FormulaBuilder {
    /// Sets the expression for the formula.
    pub fn new<E: Into<Expression>>(expression: E) -> Self {
        Self {
            expression: expression.into(),
            defaults: Default::default(),
        }
    }

    pub fn expression<E: Into<Expression>>(self, value: E) -> Self {
        let mut new = self;
        new.expression = value.into();
        new
    }

    /// Sets all default values for the formula.
    pub fn defaults(mut self, defaults: HashMap<String, Value>) -> Self {
        self.defaults = defaults;
        self
    }

    /// Adds a single default value to the formula's defaults.
    pub fn add_default<K: Into<String>, V: Into<Value>>(self, key: K, value: V) -> Self {
        let mut new = self;
        new.defaults.insert(key.into(), value.into());
        new
    }

    fn build_inner(self) -> Result<Formula, std::convert::Infallible> {
        Ok(Formula {
            expression: Some(self.expression),
            defaults: self.defaults,
        })
    }
}

impl From<FormulaBuilder> for Formula {
    fn from(value: FormulaBuilder) -> Self {
        value
            .build_inner()
            .unwrap_or_else(|_| panic!("Failed to convert {0} to {1}", "FormulaBuilder", "Formula"))
    }
}

impl FormulaBuilder {
    /// Builds the desired Formula type.
    pub fn build(self) -> Formula {
        self.build_inner()
            .unwrap_or_else(|_| panic!("Failed to build {0} into {1}", "FormulaBuilder", "Formula"))
    }
}
