pub mod client;
pub mod prelude;
pub mod qdrant;

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::qdrant::{CreateFieldIndexCollection, FieldType, Value, VectorParams, VectorsConfig};
    use crate::qdrant::vectors_config::Config;
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
                vectors_config: Some(
                    VectorsConfig {
                        config: Some(
                            Config::Params(
                                VectorParams {
                                    size: 10,
                                    distance: Distance::Cosine.into(),
                                }
                            )
                        )
                    }
                ),
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
                with_payload: Some(true.into()),
                params: None,
                score_threshold: None,
                offset: None,
                vector_name: None,
                with_vectors: None,
            })
            .await?;

        eprintln!("search_result = {:#?}", search_result);

        // Override payload of the existing point
        let new_payload: Payload = vec![
            ("foo", "BAZ".into()),
        ].into_iter().collect::<HashMap<_, Value>>().into();
        client.set_payload(collection_name, vec![0.into()], new_payload).await?;


        // Delete some payload fields
        client.delete_payload_blocking(collection_name, vec![0.into()], vec!["sub_payload".to_string()]).await?;

        // retrieve points
        let points = client.get_points(collection_name, vec![0.into()], Some(true), Some(true)).await?;

        assert_eq!(points.result.len(), 1);
        let point = points.result[0].clone();
        assert!(point.payload.contains_key("foo"));
        assert!(!point.payload.contains_key("sub_payload"));

        client.delete_points(collection_name, vec![0.into()].into()).await?;

        // Access raw point api with client
        client.points_api().create_field_index(CreateFieldIndexCollection {
            collection_name: collection_name.to_string(),
            wait: None,
            field_name: "foo".to_string(),
            field_type: Some(FieldType::Keyword as i32),
            field_index_params: None,
        }).await?;


        client.create_snapshot(collection_name).await?;
        client
            .download_snapshot("test.tar", collection_name, None, None)
            .await?;

        Ok(())
    }
}
