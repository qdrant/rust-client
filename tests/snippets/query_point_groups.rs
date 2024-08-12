use qdrant_client::qdrant::{PointId, PrefetchQueryBuilder, Query, QueryPointGroupsBuilder, RecommendInputBuilder};
use qdrant_client::Qdrant;

let client = Qdrant::from_url("http://localhost:6334").build()?;

// Query nearest by ID and grouped by 'document_id' payload key
client
    .query_groups(
        QueryPointGroupsBuilder::new(
            "{collection_name}",
            "document_id",
        )
        .query(PointId::from("43cf51e2-8777-4f52-bc74-c2cbde0c8b04"))
    )
    .await?;

// Recommend on the average of these vectors and grouped by 'document_id' payload key
client
    .query_groups(
        QueryPointGroupsBuilder::new(
            "{collection_name}",
            "document_id",
        )
        .query(
            Query::new_recommend(
                RecommendInputBuilder::default()
                    .add_positive(vec![0.1; 8])
                    .add_negative(PointId::from(0))
            )
        )
    )
    .await?;

// 2-stage query and grouped by 'document_id' payload key
client
    .query_groups(
        QueryPointGroupsBuilder::new(
            "{collection_name}",
            "document_id",
        )
        .add_prefetch(
            PrefetchQueryBuilder::default()
                .query(vec![0.01, 0.45, 0.67])
                .limit(100u64)
        )
        .query(vec![
            vec![0.1, 0.2],
            vec![0.2, 0.1],
            vec![0.8, 0.9],
        ])
        .using("colbert")
        .limit(10u64)
    ).await?;