pub mod client;
pub mod prelude;
pub mod qdrant;

#[cfg(test)]
mod tests {
    use super::prelude::*;

    #[tokio::test]
    async fn test_qdrant_queries() -> anyhow::Result<()> {
        let mut client = QdrantClient::new(None).await?;
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

        let mut points = Vec::new();
        let mut payload = Payload::new();
        payload.insert("foo", "Bar");
        payload.insert("foo2", 12);
        let mut sub_payload = Payload::new();
        sub_payload.insert("foo", "Not bar");
        payload.insert("sub_payload", sub_payload);

        points.push(PointStruct::new(0, vec![12.; 10], payload));
        client.upsert_points(collection_name, points).await?;

        client.create_snapshot(collection_name).await?;
        client
            .download_snapshot("test.tar", collection_name, None, None)
            .await?;

        Ok(())
    }
}
