pub mod client;
pub mod prelude;
pub mod qdrant;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::qdrant::Value;
    use super::prelude::*;

    #[tokio::test]
    async fn test_qdrant_queries() -> anyhow::Result<()> {
        let config = QdrantClientConfig::from_url("http://localhost:6334");
        let client = QdrantClient::new(Some(config)).await?;

        let collections_list = client.list_collections().await?;
        println!("{:?}", collections_list);

        let collection_name = "test";
        client.delete_collection(collection_name).await?;

        client
            .create_collection(CreateCollection {
                collection_name: collection_name.into(),
                vector_size: 10,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })
            .await?;

        let collection_info = client.collection_info(collection_name).await?;
        println!("{:#?}", collection_info);

        let mut sub_payload = Payload::new();
        sub_payload.insert("foo", "Not bar");

        let payload: Payload = vec![
            ("foo", "Bar".into()),
            ("bar", 12.into()),
            ("sub_payload", sub_payload.into()),
        ].into_iter().collect::<HashMap<_, Value>>().into();

        let points = vec![PointStruct::new(0, vec![12.; 10], payload)];
        client.upsert_points_blocking(collection_name, points).await?;

        let search_result = client
            .search_points(SearchPoints {
                collection_name: collection_name.into(),
                vector: vec![11.; 10],
                filter: None,
                limit: 10,
                with_vector: None,
                with_payload: None,
                params: None,
                score_threshold: None,
                offset: None,
            })
            .await?;

        eprintln!("search_result = {:#?}", search_result);

        client.create_snapshot(collection_name).await?;
        client
            .download_snapshot("test.tar", collection_name, None, None)
            .await?;

        Ok(())
    }
}
