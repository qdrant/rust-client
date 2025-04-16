use crate::qdrant::{
    Condition, DivExpression, Expression, GeoDistance, MultExpression, PowExpression, SumExpression,
};

impl From<SumExpression> for Expression {
    fn from(value: SumExpression) -> Self {
        Expression::sum(value)
    }
}

impl From<MultExpression> for Expression {
    fn from(value: MultExpression) -> Self {
        Expression::mult(value)
    }
}

impl From<DivExpression> for Expression {
    fn from(value: DivExpression) -> Self {
        Expression::div(value)
    }
}

impl From<PowExpression> for Expression {
    fn from(value: PowExpression) -> Self {
        Expression::pow(value)
    }
}

impl From<Condition> for Expression {
    fn from(value: Condition) -> Self {
        Expression::condition(value)
    }
}

impl From<GeoDistance> for Expression {
    fn from(value: GeoDistance) -> Self {
        Expression::geo_distance(value)
    }
}
