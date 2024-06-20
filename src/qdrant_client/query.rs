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
        CreateCollectionBuilder, Distance, QueryPointsBuilder, VectorParamsBuilder,
    };
    use crate::qdrant_client::builers::vectors_config::VectorsConfigBuilder;

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

        let create_collection =
            CreateCollectionBuilder::new(collection_name).vectors_config(vector_config);

        client.create_collection(create_collection).await.unwrap();

        let request = QueryPointsBuilder::new(collection_name);
        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 0);
    }
}
