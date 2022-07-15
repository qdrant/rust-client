use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::value::Kind;
use crate::qdrant::{
    CollectionOperationResponse, CountPoints, CountResponse, CreateCollection,
    CreateSnapshotRequest, CreateSnapshotResponse, DeleteCollection, GetCollectionInfoRequest,
    GetCollectionInfoResponse, ListCollectionsRequest, ListCollectionsResponse,
    ListSnapshotsRequest, ListSnapshotsResponse, ListValue, OptimizersConfigDiff, PointId,
    PointStruct, PointsOperationResponse, RecommendPoints, RecommendResponse, ScrollPoints,
    ScrollResponse, SearchPoints, SearchResponse, Struct, UpdateCollection, UpsertPoints, Value,
};
use anyhow::{bail, Result};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tonic::transport::Channel;

pub struct QdrantClientConfig {
    pub uri: String,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub keep_alive_while_idle: bool,
}

impl Default for QdrantClientConfig {
    fn default() -> Self {
        Self {
            uri: String::from("http://[::1]:6334"),
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(5),
            keep_alive_while_idle: true,
        }
    }
}

pub struct QdrantClient {
    pub collection_api: CollectionsClient<Channel>,
    pub points_api: PointsClient<Channel>,
    pub snapshots_api: SnapshotsClient<Channel>,
}

impl QdrantClient {
    pub async fn new(cfg: Option<QdrantClientConfig>) -> Result<Self> {
        let cfg = cfg.unwrap_or_default();

        let endpoint = Channel::builder(cfg.uri.parse().unwrap())
            .timeout(cfg.timeout)
            .connect_timeout(cfg.connect_timeout)
            .keep_alive_while_idle(cfg.keep_alive_while_idle);
        let channel = endpoint.connect().await?;

        let collection_api = CollectionsClient::new(channel.clone());
        let points_api = PointsClient::new(channel.clone());
        let snapshots_api = SnapshotsClient::new(channel);

        let client = Self {
            collection_api,
            points_api,
            snapshots_api,
        };

        Ok(client)
    }

    pub async fn list_collections(&mut self) -> Result<ListCollectionsResponse> {
        let result = self.collection_api.list(ListCollectionsRequest {}).await?;
        Ok(result.into_inner())
    }

    pub async fn has_collection(&mut self, collection_name: impl ToString) -> Result<bool> {
        let collection_name = collection_name.to_string();
        let response = self.list_collections().await?;
        let result = response
            .collections
            .into_iter()
            .find(|c| c.name == collection_name)
            .is_some();

        Ok(result)
    }

    pub async fn create_collection(
        &mut self,
        details: CreateCollection,
    ) -> Result<CollectionOperationResponse> {
        let result = self.collection_api.create(details).await?;
        Ok(result.into_inner())
    }

    pub async fn update_collection(
        &mut self,
        collection_name: impl ToString,
        optimizers_config: OptimizersConfigDiff,
    ) -> Result<CollectionOperationResponse> {
        let result = self
            .collection_api
            .update(UpdateCollection {
                collection_name: collection_name.to_string(),
                optimizers_config: Some(optimizers_config),
                timeout: None,
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn delete_collection(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let result = self
            .collection_api
            .delete(DeleteCollection {
                collection_name: collection_name.to_string(),
                ..Default::default()
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn collection_info(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<GetCollectionInfoResponse> {
        let result = self
            .collection_api
            .get(GetCollectionInfoRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn upsert(
        &mut self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
    ) -> Result<PointsOperationResponse> {
        self._upsert(collection_name, points, false).await
    }

    pub async fn upsert_blocking(
        &mut self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
    ) -> Result<PointsOperationResponse> {
        self._upsert(collection_name, points, true).await
    }

    #[inline]
    async fn _upsert(
        &mut self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
        block: bool,
    ) -> Result<PointsOperationResponse> {
        let result = self
            .points_api
            .upsert(UpsertPoints {
                collection_name: collection_name.to_string(),
                wait: Some(block),
                points: points.into_iter().map(|p| p.into()).collect(),
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn search(&mut self, points: SearchPoints) -> Result<SearchResponse> {
        let result = self.points_api.search(points).await?;
        Ok(result.into_inner())
    }

    pub async fn scroll(&mut self, points: ScrollPoints) -> Result<ScrollResponse> {
        let result = self.points_api.scroll(points).await?;
        Ok(result.into_inner())
    }

    pub async fn recommend(&mut self, points: RecommendPoints) -> Result<RecommendResponse> {
        let result = self.points_api.recommend(points).await?;
        Ok(result.into_inner())
    }

    pub async fn count(&mut self, points: CountPoints) -> Result<CountResponse> {
        let result = self.points_api.count(points).await?;
        Ok(result.into_inner())
    }

    pub async fn create_snapshot(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<CreateSnapshotResponse> {
        let result = self
            .snapshots_api
            .create(CreateSnapshotRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn list_snapshots(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<ListSnapshotsResponse> {
        let result = self
            .snapshots_api
            .list(ListSnapshotsRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;
        Ok(result.into_inner())
    }

    #[cfg(feature = "download_snapshots")]
    pub async fn download_snapshot<T>(
        &mut self,
        out_path: impl Into<PathBuf>,
        collection_name: T,
        snapshot_name: Option<T>,
        rest_api_uri: Option<T>,
    ) -> Result<()>
    where
        T: ToString + Clone,
    {
        let snapshot_name = match snapshot_name {
            Some(sn) => sn.to_string(),
            _ => match self
                .list_snapshots(collection_name.clone())
                .await?
                .snapshot_descriptions
                .first()
            {
                Some(sn) => sn.name.clone(),
                _ => bail!(
                    "No snapshots found for collection {}",
                    collection_name.to_string()
                ),
            },
        };

        let file = reqwest::get(format!(
            "{}/collections/{}/snapshots/{}",
            rest_api_uri
                .map(|uri| uri.to_string())
                .unwrap_or(String::from("http://[::1]:6333")),
            collection_name.to_string(),
            snapshot_name
        ))
        .await?
        .bytes()
        .await?;

        let _ = std::fs::write(out_path.into(), file);

        Ok(())
    }
}

impl PointStruct {
    pub fn new(id: impl Into<PointId>, vector: Vec<f32>, payload: Payload) -> Self {
        Self {
            id: Some(id.into()),
            vector,
            payload: payload.into(),
        }
    }
}

impl From<String> for PointId {
    fn from(val: String) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Uuid(val)),
        }
    }
}
impl From<u64> for PointId {
    fn from(val: u64) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Num(val)),
        }
    }
}

pub struct Payload(HashMap<String, Value>);

impl From<Payload> for HashMap<String, Value> {
    #[inline]
    fn from(payload: Payload) -> Self {
        payload.0
    }
}

impl Payload {
    pub fn new() -> Self {
        Self(HashMap::new())
    }

    pub fn insert(&mut self, key: impl ToString, val: impl Into<Value>) {
        self.0.insert(key.to_string(), val.into());
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Self {
            kind: Some(Kind::DoubleValue(val)),
        }
    }
}
impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Self {
            kind: Some(Kind::IntegerValue(val)),
        }
    }
}
impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Self {
            kind: Some(Kind::BoolValue(val)),
        }
    }
}
impl From<String> for Value {
    fn from(val: String) -> Self {
        Self {
            kind: Some(Kind::StringValue(val)),
        }
    }
}
impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self {
            kind: Some(Kind::StringValue(val.into())),
        }
    }
}
impl From<Payload> for Value {
    fn from(val: Payload) -> Self {
        Self {
            kind: Some(Kind::StructValue(Struct { fields: val.0 })),
        }
    }
}
impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<T>) -> Self {
        Self {
            kind: Some(Kind::ListValue(ListValue {
                values: val.into_iter().map(|v| v.into()).collect(),
            })),
        }
    }
}
