//! Provides utility constructors and functions for creating Qdrant Expression instances.
//!
//! This module offers a more ergonomic way to create Expression instances for use in
//! scoring formulas and other Qdrant operations.

use crate::qdrant::*;

impl Expression {
    /// Creates a new Expression with a constant value.
    pub fn constant(value: f32) -> Self {
        Self {
            variant: Some(expression::Variant::Constant(value)),
        }
    }

    /// Creates a new Expression with a variable (payload key or reference to score).
    pub fn variable<S: Into<String>>(name: S) -> Self {
        Self {
            variant: Some(expression::Variant::Variable(name.into())),
        }
    }

    /// Creates a new Expression with a condition. If true, becomes 1.0; otherwise 0.0.
    pub fn condition<C: Into<Condition>>(condition: C) -> Self {
        Self {
            variant: Some(expression::Variant::Condition(condition.into())),
        }
    }

    /// Creates a new Expression with a geographic distance in meters.
    pub fn geo_distance<G: Into<GeoDistance>>(geo_distance: G) -> Self {
        Self {
            variant: Some(expression::Variant::GeoDistance(geo_distance.into())),
        }
    }

    /// Creates a new Expression with a date-time constant.
    pub fn datetime<S: Into<String>>(datetime: S) -> Self {
        Self {
            variant: Some(expression::Variant::Datetime(datetime.into())),
        }
    }

    /// Creates a new Expression with a payload key with date-time values.
    pub fn datetime_key<S: Into<String>>(key: S) -> Self {
        Self {
            variant: Some(expression::Variant::DatetimeKey(key.into())),
        }
    }

    /// Creates a new Expression with a multiplication expression.
    pub fn mult<M: Into<MultExpression>>(mult: M) -> Self {
        Self {
            variant: Some(expression::Variant::Mult(mult.into())),
        }
    }

    /// Creates a new Expression with a sum expression.
    pub fn sum<S: Into<SumExpression>>(sum: S) -> Self {
        Self {
            variant: Some(expression::Variant::Sum(sum.into())),
        }
    }

    /// Creates a new Expression with a division expression.
    pub fn div<D: Into<DivExpression>>(div: D) -> Self {
        Self {
            variant: Some(expression::Variant::Div(Box::new(div.into()))),
        }
    }

    /// Creates a new Expression with a negation expression.
    pub fn neg<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Neg(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with an absolute value expression.
    pub fn abs<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Abs(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with a square root expression.
    pub fn sqrt<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Sqrt(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with a power expression.
    pub fn pow<P: Into<PowExpression>>(pow: P) -> Self {
        Self {
            variant: Some(expression::Variant::Pow(Box::new(pow.into()))),
        }
    }

    /// Creates a new Expression with an exponential expression.
    pub fn exp<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Exp(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with a log10 expression.
    pub fn log10<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Log10(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with a natural logarithm expression.
    pub fn ln<E: Into<Expression>>(expr: E) -> Self {
        Self {
            variant: Some(expression::Variant::Ln(Box::new(expr.into()))),
        }
    }

    /// Creates a new Expression with an exponential decay expression.
    pub fn exp_decay<D: Into<DecayParamsExpression>>(decay: D) -> Self {
        Self {
            variant: Some(expression::Variant::ExpDecay(Box::new(decay.into()))),
        }
    }

    /// Creates a new Expression with a Gaussian decay expression.
    pub fn gauss_decay<D: Into<DecayParamsExpression>>(decay: D) -> Self {
        Self {
            variant: Some(expression::Variant::GaussDecay(Box::new(decay.into()))),
        }
    }

    /// Creates a new Expression with a linear decay expression.
    pub fn lin_decay<D: Into<DecayParamsExpression>>(decay: D) -> Self {
        Self {
            variant: Some(expression::Variant::LinDecay(Box::new(decay.into()))),
        }
    }

    /// Helper method to create a multiplication expression with multiple sub-expressions.
    pub fn mult_with<E: Into<Expression>, I: IntoIterator<Item = E>>(expressions: I) -> Self {
        let exprs: Vec<Expression> = expressions.into_iter().map(|e| e.into()).collect();
        Self::mult(MultExpression { mult: exprs })
    }

    /// Helper method to create a sum expression with multiple sub-expressions.
    pub fn sum_with<E: Into<Expression>, I: IntoIterator<Item = E>>(expressions: I) -> Self {
        let exprs: Vec<Expression> = expressions.into_iter().map(|e| e.into()).collect();
        Self::sum(SumExpression { sum: exprs })
    }

    /// Helper method to create a division expression with left and right operands.
    pub fn div_with<L: Into<Expression>, R: Into<Expression>>(
        left: L,
        right: R,
        by_zero_default: Option<f32>,
    ) -> Self {
        Self::div(DivExpression {
            left: Some(Box::new(left.into())),
            right: Some(Box::new(right.into())),
            by_zero_default,
        })
    }

    /// Helper method to create a power expression with base and exponent.
    pub fn pow_with<B: Into<Expression>, E: Into<Expression>>(base: B, exponent: E) -> Self {
        Self::pow(PowExpression {
            base: Some(Box::new(base.into())),
            exponent: Some(Box::new(exponent.into())),
        })
    }
}

impl From<String> for Expression {
    fn from(value: String) -> Self {
        Self::variable(value)
    }
}

impl From<f32> for Expression {
    fn from(value: f32) -> Self {
        Self::constant(value)
    }
}
