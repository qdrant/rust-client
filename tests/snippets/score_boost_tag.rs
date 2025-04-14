use qdrant_client::qdrant::{
    Condition, Expression, FormulaBuilder, PrefetchQueryBuilder, QueryPointsBuilder,
};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

client.query(
    QueryPointsBuilder::new("{collection_name}")
        .add_prefetch(PrefetchQueryBuilder::default()
            .query(vec![0.01, 0.45, 0.67])
            .limit(100u64)
        )
        .query(FormulaBuilder::new(Expression::sum_with([
            Expression::variable("$score"),
            Expression::mult_with([
                Expression::constant(0.5),
                Expression::condition(Condition::matches("tag", ["h1", "h2", "h3", "h4"])),
            ]),
            Expression::mult_with([
                Expression::constant(0.25),
                Expression::condition(Condition::matches("tag", ["p", "li"])),
            ]),
        ])))
        .limit(10)
).await?;
