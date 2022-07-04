pub mod client;
pub mod prelude;
pub mod qdrant;

#[cfg(test)]
mod tests {
    use super::client::*;

    use crate::qdrant::{
        CreateCollection, DeleteCollection, Distance, GetCollectionInfoRequest,
        ListCollectionsRequest, PointStruct,
    };

    #[tokio::test]
    async fn test_qdrant_queries() -> anyhow::Result<()> {
        let mut client = QdrantClient::new(None).await?;
        let collections_list = client
            .collection_api
            .list(ListCollectionsRequest {})
            .await
            .unwrap();
        println!("{:?}", collections_list.into_inner());
        let collection_name = "test".to_string();
        client
            .collection_api
            .delete(DeleteCollection {
                collection_name: collection_name.clone(),
                ..Default::default()
            })
            .await
            .unwrap();
        client
            .collection_api
            .create(CreateCollection {
                collection_name: collection_name.clone(),
                vector_size: 10,
                distance: Distance::Cosine.into(),
                ..Default::default()
            })
            .await
            .unwrap();
        let collection_info = client
            .collection_api
            .get(GetCollectionInfoRequest {
                collection_name: collection_name.clone(),
            })
            .await
            .unwrap();
        println!("{:#?}", collection_info.into_inner());

        let mut points = Vec::new();
        let mut payload = Payload::new();
        payload.insert("foo", "Bar");
        payload.insert("foo2", 12);
        let mut sub_payload = Payload::new();
        sub_payload.insert("foo", "Not bar");
        payload.insert("sub_payload", sub_payload);

        points.push(PointStruct::new(0, vec![12.; 10], payload));
        client.upsert(collection_name, points).await?;

        Ok(())
    }
}
