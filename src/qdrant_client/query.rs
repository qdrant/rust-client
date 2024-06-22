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
    use crate::qdrant::{
        CreateCollectionBuilder, Datatype, Distance, Modifier, MultiVectorConfig, NamedVectors,
        PointStruct, QueryPointsBuilder, ScalarQuantizationBuilder, SparseIndexConfigBuilder,
        SparseVectorParamsBuilder, UpsertPointsBuilder, Vector, VectorParamsBuilder,
    };
    use crate::qdrant_client::builers::sparse_vectors_config::SparseVectorsConfigBuilder;
    use crate::qdrant_client::builers::vectors_config::VectorsConfigBuilder;
    use crate::Payload;
    use serde_json::json;

    use super::*;

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
                    vec![PointStruct::new(
                        0,
                        NamedVectors::default()
                            .add_vector("large_vector", vec![0.1; 8])
                            .add_vector("small_vector", vec![0.1; 4])
                            .add_vector("colbert_vector", vec![vec![0.1; 4], vec![0.1; 4]])
                            .add_vector(
                                "sparse_idf_vector",
                                Vector::new_sparse(vec![1, 2, 3], vec![0.1, 0.2, 0.3]),
                            ),
                        Payload::try_from(json!({"foo": "bar"})).unwrap(),
                    )],
                )
                .wait(true),
            )
            .await
            .unwrap();

        let request = QueryPointsBuilder::new(collection_name);
        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 1);
    }
}
