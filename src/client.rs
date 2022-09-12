use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::value::Kind;
use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::with_payload_selector::SelectorOptions;
use crate::qdrant::{ClearPayloadPoints, CollectionOperationResponse, CountPoints, CountResponse, CreateCollection, CreateFullSnapshotRequest, CreateSnapshotRequest, CreateSnapshotResponse, DeleteCollection, DeletePayloadPoints, DeletePoints, Filter, GetCollectionInfoRequest, GetCollectionInfoResponse, GetPoints, GetResponse, ListCollectionsRequest, ListCollectionsResponse, ListFullSnapshotsRequest, ListSnapshotsRequest, ListSnapshotsResponse, ListValue, NamedVectors, OptimizersConfigDiff, PayloadIncludeSelector, PointId, PointStruct, PointsIdsList, PointsOperationResponse, PointsSelector, RecommendBatchPoints, RecommendBatchResponse, RecommendPoints, RecommendResponse, ScrollPoints, ScrollResponse, SearchBatchPoints, SearchBatchResponse, SearchPoints, SearchResponse, SetPayloadPoints, Struct, UpdateCollection, UpsertPoints, Value, Vector, Vectors, WithPayloadSelector, WithVectorsSelector, with_vectors_selector, VectorsSelector};
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

impl QdrantClientConfig {
    pub fn from_url(url: &str) -> Self {
        let mut default = Self::default();
        default.uri = url.to_string();
        default
    }
}

impl From<bool> for WithPayloadSelector {
    fn from(flag: bool) -> Self {
        WithPayloadSelector {
            selector_options: Some(SelectorOptions::Enable(flag)),
        }
    }
}

impl From<Vec<&str>> for WithPayloadSelector {
    fn from(fields: Vec<&str>) -> Self {
        WithPayloadSelector {
            selector_options: Some(SelectorOptions::Include(PayloadIncludeSelector {
                fields: fields.into_iter().map(|f| f.to_string()).collect(),
            })),
        }
    }
}

impl From<Vec<PointId>> for PointsSelector {
    fn from(point_ids: Vec<PointId>) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: point_ids,
            })),
        }
    }
}

impl From<Filter> for PointsSelector {
    fn from(filter: Filter) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Filter(filter)),
        }
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vector: Vec<f32>) -> Self {
        Vector { data: vector }
    }
}

impl From<HashMap<String, Vec<f32>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<f32>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<Vec<f32>> for Vectors {
    fn from(vector: Vec<f32>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(vector.into())),
        }
    }
}

impl From<Vec<&str>> for WithVectorsSelector {
    fn from(names: Vec<&str>) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Include(VectorsSelector{
                names: names.into_iter().map(|name| name.to_string()).collect()
            })),
        }
    }
}

impl From<bool> for WithVectorsSelector {
    fn from(flag: bool) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Enable(flag)),
        }
    }
}

impl Default for QdrantClientConfig {
    fn default() -> Self {
        Self {
            uri: String::from("http://localhost:6334"),
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(5),
            keep_alive_while_idle: true,
        }
    }
}

pub struct QdrantClient {
    pub channel: Channel,
    pub cfg: QdrantClientConfig,
}

impl QdrantClient {
    // Access to raw collection API
    pub fn collection_api(&self) -> CollectionsClient<Channel> {
        CollectionsClient::new(self.channel.clone())
    }

    // Access to raw points API
    pub fn points_api(&self) -> PointsClient<Channel> {
        PointsClient::new(self.channel.clone())
    }

    pub async fn new(cfg: Option<QdrantClientConfig>) -> Result<Self> {
        let cfg = cfg.unwrap_or_default();

        let endpoint = Channel::builder(cfg.uri.parse().unwrap())
            .timeout(cfg.timeout)
            .connect_timeout(cfg.connect_timeout)
            .keep_alive_while_idle(cfg.keep_alive_while_idle);

        let channel = endpoint.connect().await?;

        let client = Self { channel, cfg };

        Ok(client)
    }

    pub async fn reconnect(&mut self) -> Result<()> {
        let channel = Channel::builder(self.cfg.uri.parse().unwrap())
            .timeout(self.cfg.timeout)
            .connect_timeout(self.cfg.connect_timeout)
            .keep_alive_while_idle(self.cfg.keep_alive_while_idle);

        let channel = channel.connect().await?;
        self.channel = channel;

        Ok(())
    }

    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        let mut collection_api = self.collection_api();
        let result = collection_api.list(ListCollectionsRequest {}).await?;
        Ok(result.into_inner())
    }

    pub async fn has_collection(&self, collection_name: impl ToString) -> Result<bool> {
        let collection_name = collection_name.to_string();
        let response = self.list_collections().await?;
        let result = response
            .collections
            .into_iter()
            .any(|c| c.name == collection_name);

        Ok(result)
    }

    pub async fn create_collection(
        &self,
        details: CreateCollection,
    ) -> Result<CollectionOperationResponse> {
        let mut collection_api = self.collection_api();
        let result = collection_api.create(details).await?;
        Ok(result.into_inner())
    }

    pub async fn update_collection(
        &self,
        collection_name: impl ToString,
        optimizers_config: OptimizersConfigDiff,
    ) -> Result<CollectionOperationResponse> {
        let mut collection_api = self.collection_api();
        let result = collection_api
            .update(UpdateCollection {
                collection_name: collection_name.to_string(),
                optimizers_config: Some(optimizers_config),
                timeout: None,
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn delete_collection(
        &self,
        collection_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let mut collection_api = self.collection_api();
        let result = collection_api
            .delete(DeleteCollection {
                collection_name: collection_name.to_string(),
                ..Default::default()
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn collection_info(
        &self,
        collection_name: impl ToString,
    ) -> Result<GetCollectionInfoResponse> {
        let mut collection_api = self.collection_api();
        let result = collection_api
            .get(GetCollectionInfoRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn upsert_points(
        &self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(collection_name, points, false).await
    }

    pub async fn upsert_points_blocking(
        &self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(collection_name, points, true).await
    }

    #[inline]
    async fn _upsert_points(
        &self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
        block: bool,
    ) -> Result<PointsOperationResponse> {
        let mut points_api = self.points_api();

        let result = points_api
            .upsert(UpsertPoints {
                collection_name: collection_name.to_string(),
                wait: Some(block),
                points,
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn set_payload(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        payload: Payload,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(collection_name, points, payload, false)
            .await
    }

    pub async fn set_payload_blocking(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        payload: Payload,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(collection_name, points, payload, true)
            .await
    }

    #[inline]
    async fn _set_payload(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        payload: Payload,
        block: bool,
    ) -> Result<PointsOperationResponse> {
        let mut points_api = self.points_api();
        let result = points_api
            .set_payload(SetPayloadPoints {
                collection_name: collection_name.to_string(),
                wait: Some(block),
                payload: payload.0,
                points,
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn delete_payload(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        keys: Vec<String>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(collection_name, points, keys, false)
            .await
    }

    pub async fn delete_payload_blocking(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        keys: Vec<String>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(collection_name, points, keys, true)
            .await
    }

    #[inline]
    async fn _delete_payload(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        keys: Vec<String>,
        block: bool,
    ) -> Result<PointsOperationResponse> {
        let mut points_api = self.points_api();
        let result = points_api
            .delete_payload(DeletePayloadPoints {
                collection_name: collection_name.to_string(),
                wait: Some(block),
                keys,
                points,
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn clear_payload(
        &self,
        collection_name: impl ToString,
        points_selector: Option<PointsSelector>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(collection_name, points_selector, false)
            .await
    }

    pub async fn clear_payload_blocking(
        &self,
        collection_name: impl ToString,
        points_selector: Option<PointsSelector>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(collection_name, points_selector, true)
            .await
    }

    #[inline]
    async fn _clear_payload(
        &self,
        collection_name: impl ToString,
        points_selector: Option<PointsSelector>,
        block: bool,
    ) -> Result<PointsOperationResponse> {
        let mut points_api = self.points_api();
        let result = points_api
            .clear_payload(ClearPayloadPoints {
                collection_name: collection_name.to_string(),
                wait: Some(block),
                points: points_selector,
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn get_points(
        &self,
        collection_name: impl ToString,
        points: Vec<PointId>,
        with_vectors: Option<impl Into<WithVectorsSelector>>,
        with_payload: Option<impl Into<WithPayloadSelector>>,
    ) -> Result<GetResponse> {
        let mut points_api = self.points_api();
        let result = points_api
            .get(GetPoints {
                collection_name: collection_name.to_string(),
                ids: points,
                with_vector: None,
                with_payload: with_payload.map(|x| x.into()),
                with_vectors: with_vectors.map(|x| x.into()),
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn search_points(&self, request: SearchPoints) -> Result<SearchResponse> {
        let mut points_api = self.points_api();
        let result = points_api.search(request).await?;
        Ok(result.into_inner())
    }

    pub async fn search_batch_points(
        &self,
        request: SearchBatchPoints,
    ) -> Result<SearchBatchResponse> {
        let mut points_api = self.points_api();
        let result = points_api.search_batch(request).await?;
        Ok(result.into_inner())
    }

    pub async fn delete_points(
        &self,
        collection_name: impl ToString,
        points: PointsSelector,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, false, Some(points))
            .await
    }

    pub async fn delete_points_blocking(
        &self,
        collection_name: impl ToString,
        points: PointsSelector,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, true, Some(points))
            .await
    }

    pub async fn delete_all_points(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, false, None).await
    }

    pub async fn delete_all_points_blocking(
        &mut self,
        collection_name: impl ToString,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, true, None).await
    }

    async fn _delete_points(
        &self,
        collection_name: impl ToString,
        blocking: bool,
        points: Option<PointsSelector>,
    ) -> Result<PointsOperationResponse> {
        let mut points_api = self.points_api();
        let result = points_api
            .delete(DeletePoints {
                collection_name: collection_name.to_string(),
                wait: Some(blocking),
                points,
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn scroll(&self, request: ScrollPoints) -> Result<ScrollResponse> {
        let mut points_api = self.points_api();
        let result = points_api.scroll(request).await?;
        Ok(result.into_inner())
    }

    pub async fn recommend(&self, request: RecommendPoints) -> Result<RecommendResponse> {
        let mut points_api = self.points_api();
        let result = points_api.recommend(request).await?;
        Ok(result.into_inner())
    }

    pub async fn recommend_batch(
        &self,
        request: RecommendBatchPoints,
    ) -> Result<RecommendBatchResponse> {
        let mut points_api = self.points_api();
        let result = points_api.recommend_batch(request).await?;
        Ok(result.into_inner())
    }

    pub async fn count(&self, request: CountPoints) -> Result<CountResponse> {
        let mut points_api = self.points_api();
        let result = points_api.count(request).await?;
        Ok(result.into_inner())
    }

    pub async fn create_snapshot(
        &self,
        collection_name: impl ToString,
    ) -> Result<CreateSnapshotResponse> {
        let mut snapshots_api = SnapshotsClient::new(self.channel.clone());
        let result = snapshots_api
            .create(CreateSnapshotRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn list_snapshots(
        &self,
        collection_name: impl ToString,
    ) -> Result<ListSnapshotsResponse> {
        let mut snapshots_api = SnapshotsClient::new(self.channel.clone());
        let result = snapshots_api
            .list(ListSnapshotsRequest {
                collection_name: collection_name.to_string(),
            })
            .await?;
        Ok(result.into_inner())
    }

    pub async fn create_full_snapshot(&self) -> Result<CreateSnapshotResponse> {
        let mut snapshots_api = SnapshotsClient::new(self.channel.clone());
        let result = snapshots_api
            .create_full(CreateFullSnapshotRequest {})
            .await?;

        Ok(result.into_inner())
    }

    pub async fn list_full_snapshots(&self) -> Result<ListSnapshotsResponse> {
        let mut snapshots_api = SnapshotsClient::new(self.channel.clone());
        let result = snapshots_api.list_full(ListFullSnapshotsRequest {}).await?;
        Ok(result.into_inner())
    }

    #[cfg(feature = "download_snapshots")]
    pub async fn download_snapshot<T>(
        &self,
        out_path: impl Into<PathBuf>,
        collection_name: T,
        snapshot_name: Option<T>,
        rest_api_uri: Option<T>,
    ) -> Result<()>
    where
        T: ToString + Clone,
    {
        use futures_util::StreamExt;
        use std::io::Write;

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

        let mut stream = reqwest::get(format!(
            "{}/collections/{}/snapshots/{}",
            rest_api_uri
                .map(|uri| uri.to_string())
                .unwrap_or_else(|| String::from("http://localhost:6333")),
            collection_name.to_string(),
            snapshot_name
        ))
        .await?
        .bytes_stream();

        let out_path = out_path.into();
        let _ = std::fs::remove_file(&out_path);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_path)?;

        while let Some(chunk) = stream.next().await {
            file.write(&chunk?)?;
        }

        Ok(())
    }
}

impl PointStruct {
    pub fn new(id: impl Into<PointId>, vectors: impl Into<Vectors>, payload: Payload) -> Self {
        Self {
            id: Some(id.into()),
            vector: Vec::new(),
            payload: payload.into(),
            vectors: Some(vectors.into()),
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

impl From<HashMap<&str, Value>> for Payload {
    #[inline]
    fn from(payload: HashMap<&str, Value>) -> Self {
        Self(
            payload
                .into_iter()
                .map(|(k, v)| (k.to_string(), v))
                .collect(),
        )
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
