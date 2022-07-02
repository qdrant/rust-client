use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::points_client::PointsClient;
use tonic::transport::Channel;

pub mod client;
pub mod qdrant;

pub struct QdrantClient {
    pub collection_api: CollectionsClient<Channel>,
    pub points_api: PointsClient<Channel>,
}

impl QdrantClient {
    pub fn new(channel: Channel) -> Self {
        let collection_api = CollectionsClient::new(channel.clone());
        let points_api = PointsClient::new(channel.clone());
        Self {
            collection_api,
            points_api,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::qdrant::{
        CreateCollection, DeleteCollection, Distance, GetCollectionInfoRequest,
        ListCollectionsRequest,
    };
    use std::time::Duration;

    #[tokio::test]
    async fn test_qdrant_queries() {
        let uri = "http://localhost:6334".parse().unwrap();
        let endpoint = Channel::builder(uri)
            .timeout(Duration::from_secs(5))
            .connect_timeout(Duration::from_secs(5))
            .keep_alive_while_idle(true);
        // `connect` is using the `Reconnect` network service internally to handle dropped connections
        let channel = endpoint.connect().await.unwrap(); // Do not unwrap, this is a test
        let mut client = QdrantClient::new(channel);
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
            .get(GetCollectionInfoRequest { collection_name })
            .await
            .unwrap();
        println!("{:#?}", collection_info.into_inner());
    }
}
