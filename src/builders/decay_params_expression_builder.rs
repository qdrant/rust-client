use crate::qdrant::*;

/// Builder for the DecayParamsExpression struct, which represents parameters for decay functions.
///
/// Decay functions (exponential, Gaussian, linear) are used in scoring to create a decay effect
/// based on distance from a target value.
pub struct DecayParamsExpressionBuilder {
    /// The variable to decay
    pub(crate) x: Expression,
    /// The target value to start decaying from. Defaults to 0.
    pub(crate) target: Option<Expression>,
    /// The scale factor of the decay, in terms of `x`. Defaults to 1.0. Must be a non-zero positive number.
    pub(crate) scale: Option<f32>,
    /// The midpoint of the decay. Defaults to 0.5. Output will be this value when `|x - target| == scale`.
    pub(crate) midpoint: Option<f32>,
}

impl DecayParamsExpressionBuilder {
    /// Creates a new DecayParamsExpressionBuilder with the variable to decay.
    /// This is the only required parameter.
    pub fn new<E: Into<Expression>>(x: E) -> Self {
        Self {
            x: x.into(),
            target: None,
            scale: None,
            midpoint: None,
        }
    }

    /// Sets the variable to decay. This is the value that will be compared to the target.
    pub fn x<E: Into<Expression>>(self, x: E) -> Self {
        let mut new = self;
        new.x = x.into();
        new
    }

    /// Sets the target value to start decaying from. Defaults to 0 if not specified.
    /// The decay is at its maximum (1.0) when x equals the target.
    pub fn target<E: Into<Expression>>(self, target: E) -> Self {
        let mut new = self;
        new.target = Some(target.into());
        new
    }

    /// Sets the scale factor of the decay, in terms of `x`. Defaults to 1.0 if not specified.
    /// Must be a non-zero positive number.
    /// This controls how quickly the function decays away from the target value.
    pub fn scale(self, scale: f32) -> Self {
        let mut new = self;
        new.scale = Some(scale);
        new
    }

    /// Sets the midpoint of the decay. Defaults to 0.5 if not specified.
    /// The output will be this value when the distance between x and target equals the scale.
    pub fn midpoint(self, midpoint: f32) -> Self {
        let mut new = self;
        new.midpoint = Some(midpoint);
        new
    }

    fn build_inner(self) -> Result<DecayParamsExpression, std::convert::Infallible> {
        Ok(DecayParamsExpression {
            x: Some(Box::new(self.x)),
            target: self.target.map(Box::new),
            scale: self.scale,
            midpoint: self.midpoint,
        })
    }
}

impl From<DecayParamsExpressionBuilder> for DecayParamsExpression {
    fn from(value: DecayParamsExpressionBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "DecayParamsExpressionBuilder", "DecayParamsExpression"
            )
        })
    }
}

impl DecayParamsExpressionBuilder {
    /// Builds the DecayParamsExpression from this builder.
    pub fn build(self) -> DecayParamsExpression {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "DecayParamsExpressionBuilder", "DecayParamsExpression"
            )
        })
    }
}
