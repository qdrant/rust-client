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
    use super::*;
    use crate::qdrant::{CreateCollectionBuilder, QueryPointsBuilder};

    #[tokio::test]
    async fn test_query() {
        let client = Qdrant::from_url("http://localhost:6334").build().unwrap();
        let collection_name = "test_collection";

        client.delete_collection(collection_name).await.unwrap();

        let create_collection = CreateCollectionBuilder::new(collection_name);
        client.create_collection(create_collection).await.unwrap();

        let request = QueryPointsBuilder::new(collection_name);
        let response = client.query(request).await.unwrap();
        assert_eq!(response.result.len(), 0);
    }
}
