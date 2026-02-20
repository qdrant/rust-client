use tonic::service::Interceptor;

use super::QdrantResult;
use crate::qdrant::{
    QueryBatchPoints, QueryBatchResponse, QueryGroupsResponse, QueryPointGroups, QueryPoints,
    QueryResponse,
};
use crate::qdrant_client::GenericQdrant;

/// # Query operations
///
/// Query points using the universal search API.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/search/#query-api>
impl<I: Send + Sync + 'static + Clone + Interceptor> GenericQdrant<I> {
    /// Query points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, QueryPointsBuilder};
    ///
    ///# async fn query(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .query(
    ///         QueryPointsBuilder::new("my_collection")
    ///             .filter(Filter::must([Condition::matches(
    ///                 "city",
    ///                 "London".to_string(),
    ///             )]))
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#query-api>
    pub async fn query(&self, request: impl Into<QueryPoints>) -> QdrantResult<QueryResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.query(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Batch multiple point queries in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, QueryPointsBuilder, QueryBatchPointsBuilder};
    ///
    ///# async fn query_batch(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .query_batch(
    ///         QueryBatchPointsBuilder::new("my_collection", vec![
    ///             QueryPointsBuilder::new("my_collection")
    ///                 .filter(Filter::must([Condition::matches(
    ///                     "city",
    ///                     "London".to_string(),
    ///                 )]))
    ///                 .build(),
    ///             QueryPointsBuilder::new("my_collection")
    ///                 .filter(Filter::must([Condition::matches(
    ///                     "city",
    ///                     "Berlin".to_string(),
    ///                 )]))
    ///                 .build(),
    ///         ])
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#query-api>
    pub async fn query_batch(
        &self,
        request: impl Into<QueryBatchPoints>,
    ) -> QdrantResult<QueryBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.query_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Query points in a collection and group results by a payload field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{PrefetchQueryBuilder, QueryPointGroupsBuilder};
    ///
    ///# async fn query_groups(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .query_groups(
    ///         QueryPointGroupsBuilder::new(
    ///             "my_collection", // Collection name
    ///             "city",          // Group by field
    ///          )
    ///          .add_prefetch(
    ///              PrefetchQueryBuilder::default()
    ///                  .query(vec![0.01, 0.45, 0.67])
    ///                  .limit(100u64)
    ///          )
    ///          .query(vec![0.1, 0.2, 0.3, 0.4]) // Query vector
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#query-api>
    pub async fn query_groups(
        &self,
        request: impl Into<QueryPointGroups>,
    ) -> QdrantResult<QueryGroupsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.query_groups(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use crate::builders::CreateCollectionBuilder;
    use crate::qdrant::{
        ContextInputBuilder, CreateFieldIndexCollectionBuilder, Datatype, DiscoverInputBuilder,
        Distance, FieldType, Fusion, IntegerIndexParamsBuilder, Modifier, MultiVectorConfig,
        NamedVectors, PointId, PointStruct, PrefetchQueryBuilder, Query, QueryPointsBuilder,
        RecommendInputBuilder, ScalarQuantizationBuilder, SparseIndexConfigBuilder,
        SparseVectorParamsBuilder, SparseVectorsConfigBuilder, UpsertPointsBuilder, Vector,
        VectorInput, VectorParamsBuilder, VectorsConfigBuilder,
    };
    use crate::{Payload, Qdrant};

    #[tokio::test]
    async fn test_query() {
        let client = Qdrant::from_url("http://localhost:6334").build().unwrap();
        let collection_name = "test_collection_query";

        client.delete_collection(collection_name).await.unwrap();

        let mut vector_config = VectorsConfigBuilder::default();

        vector_config.add_named_vector_params(
            "large_vector",
            VectorParamsBuilder::new(8, Distance::Cosine),
        );
        vector_config.add_named_vector_params(
            "small_vector",
            VectorParamsBuilder::new(4, Distance::Euclid),
        );

        vector_config.add_named_vector_params(
            "colbert_vector",
            VectorParamsBuilder::new(4, Distance::Dot)
                .multivector_config(MultiVectorConfig::default()),
        );

        let mut sparse_vector_config = SparseVectorsConfigBuilder::default();

        sparse_vector_config.add_named_vector_params(
            "sparse_idf_vector",
            SparseVectorParamsBuilder::default()
                .modifier(Modifier::Idf)
                .index(SparseIndexConfigBuilder::default().datatype(Datatype::Float32)),
        );

        let create_collection = CreateCollectionBuilder::new(collection_name)
            .vectors_config(vector_config)
            .sparse_vectors_config(sparse_vector_config)
            .quantization_config(ScalarQuantizationBuilder::default());

        client.create_collection(create_collection).await.unwrap();

        client
            .upsert_points(
                UpsertPointsBuilder::new(
                    collection_name,
                    vec![
                        PointStruct::new(
                            0,
                            NamedVectors::default()
                                .add_vector("large_vector", vec![0.1; 8])
                                .add_vector("small_vector", vec![0.1; 4])
                                .add_vector(
                                    "colbert_vector",
                                    vec![vec![0.1, 0.2, 0.3, 0.4], vec![0.4, 0.2, 0.3, 0.1]],
                                )
                                .add_vector(
                                    "sparse_idf_vector",
                                    Vector::new_sparse(vec![1, 2, 3], vec![0.1, 0.2, 0.3]),
                                ),
                            Payload::try_from(json!({"foo": "bar", "num": 1})).unwrap(),
                        ),
                        PointStruct::new(
                            1,
                            NamedVectors::default()
                                .add_vector("large_vector", vec![1.1; 8])
                                .add_vector("small_vector", vec![1.1; 4])
                                .add_vector(
                                    "colbert_vector",
                                    vec![vec![1.1, 1.2, 1.3, 1.4], vec![1.4, 1.2, 1.3, 1.1]],
                                )
                                .add_vector(
                                    "sparse_idf_vector",
                                    Vector::new_sparse(vec![1, 2, 3], vec![1.1, 1.2, 1.3]),
                                ),
                            Payload::try_from(json!({"foo": "bar", "num": 2})).unwrap(),
                        ),
                    ],
                )
                .wait(true),
            )
            .await
            .unwrap();

        client
            .create_field_index(
                CreateFieldIndexCollectionBuilder::new(collection_name, "num", FieldType::Integer)
                    .wait(true)
                    .field_index_params(IntegerIndexParamsBuilder::new(false, true).build()),
            )
            .await
            .unwrap();

        // Let's build a hierarchical query.
        //
        // fusion
        // ├── colbert
        // │   ├── sparse
        // │   └── dense large
        // │       └── dense small
        // └── order_by
        let request = QueryPointsBuilder::new(collection_name)
            .limit(1)
            .query(Query::new_fusion(Fusion::Rrf))
            .add_prefetch(
                PrefetchQueryBuilder::default()
                    .using("colbert_vector")
                    .query(Query::new_nearest(vec![
                        vec![0.1, 0.2, 0.3, 0.4],
                        vec![1.1, 1.2, 1.3, 1.4],
                    ]))
                    .add_prefetch(
                        PrefetchQueryBuilder::default()
                            .using("sparse_idf_vector")
                            .query(VectorInput::new_sparse(vec![1, 2, 3], vec![0.1, 0.2, 0.3])),
                    )
                    .add_prefetch(
                        PrefetchQueryBuilder::default()
                            .using("large_vector")
                            .query(Query::new_nearest(vec![0.1; 8]))
                            .add_prefetch(
                                PrefetchQueryBuilder::default()
                                    .using("small_vector")
                                    .query(Query::new_nearest(vec![0.1; 4])),
                            ),
                    ),
            )
            .add_prefetch(PrefetchQueryBuilder::default().query(Query::new_order_by("num")));

        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 1);

        // Let's build a recommendation query.

        let request = QueryPointsBuilder::new(collection_name)
            .limit(1)
            .using("large_vector")
            .query(Query::new_recommend(
                RecommendInputBuilder::default()
                    .add_positive(vec![0.1; 8])
                    .add_negative(PointId::from(0)),
            ));

        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 1);

        // Let's build a discover query.

        let request = QueryPointsBuilder::new(collection_name)
            .limit(1)
            .using("large_vector")
            .query(Query::new_discover(DiscoverInputBuilder::new(
                vec![0.1; 8],
                ContextInputBuilder::default().add_pair(PointId::from(0), vec![0.2; 8]),
            )));

        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 1);
    }
}
