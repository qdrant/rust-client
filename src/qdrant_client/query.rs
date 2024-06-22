use crate::qdrant::{QueryPoints, QueryResponse};
use crate::qdrant_client::Qdrant;

impl Qdrant {
    pub async fn query(
        &self,
        request: impl Into<QueryPoints>,
    ) -> crate::qdrant_client::Result<QueryResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.query(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::*;
    use crate::qdrant::{
        ContextInputBuilder, CreateCollectionBuilder, CreateFieldIndexCollectionBuilder, Datatype,
        DiscoverInputBuilder, Distance, FieldType, Fusion, IntegerIndexParamsBuilder, Modifier,
        MultiVectorConfig, NamedVectors, PointId, PointStruct, PrefetchQueryBuilder, Query,
        QueryPointsBuilder, RecommendInputBuilder, ScalarQuantizationBuilder,
        SparseIndexConfigBuilder, SparseVectorParamsBuilder, UpsertPointsBuilder, Vector,
        VectorInput, VectorParamsBuilder,
    };
    use crate::qdrant_client::builers::sparse_vectors_config::SparseVectorsConfigBuilder;
    use crate::qdrant_client::builers::vectors_config::VectorsConfigBuilder;
    use crate::Payload;

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
