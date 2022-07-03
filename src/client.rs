use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::value::Kind;
use crate::qdrant::{
    CollectionOperationResponse, CreateCollection, GetCollectionInfoRequest,
    GetCollectionInfoResponse, ListValue, PointId, PointStruct, PointsOperationResponse, Struct,
    UpsertPoints, Value,
};
use anyhow::Result;
use core::any::Any;
use std::collections::HashMap;
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
            uri: String::from("http://[::1]:6333"),
            timeout: Duration::from_secs(5),
            connect_timeout: Duration::from_secs(5),
            keep_alive_while_idle: true,
        }
    }
}

pub struct QdrantClient {
    pub collection_api: CollectionsClient<Channel>,
    pub points_api: PointsClient<Channel>,
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
        let points_api = PointsClient::new(channel);

        let client = Self {
            collection_api,
            points_api,
        };

        Ok(client)
    }

    pub async fn create_collection(
        &mut self,
        details: CreateCollection,
    ) -> Result<CollectionOperationResponse> {
        let result = self.collection_api.create(details).await?;
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
        points: Vec<Point>,
    ) -> Result<PointsOperationResponse> {
        let mut hm = HashMap::<String, Box<dyn Any>>::new();
        hm.insert("nice".into(), Box::new(12));

        let result = self
            .points_api
            .upsert(UpsertPoints {
                collection_name: collection_name.to_string(),
                wait: Some(false),
                points: points.into_iter().map(|p| p.into()).collect(),
            })
            .await?;

        Ok(result.into_inner())
    }

    pub async fn upsert_blocking(
        &mut self,
        collection_name: impl ToString,
        points: Vec<Point>,
    ) -> Result<PointsOperationResponse> {
        let result = self
            .points_api
            .upsert(UpsertPoints {
                collection_name: collection_name.to_string(),
                wait: Some(true),
                points: points.into_iter().map(|p| p.into()).collect(),
            })
            .await?;

        Ok(result.into_inner())
    }
}

pub struct Point {
    pub id: Option<Id>,
    pub vec: Vec<f32>,
    pub payload: Payload,
}

impl From<Point> for PointStruct {
    fn from(point: Point) -> Self {
        Self {
            id: point.id.map(|id| PointId {
                point_id_options: Some(match id {
                    Id::Num(v) => PointIdOptions::Num(v),
                    Id::Uuid(v) => PointIdOptions::Uuid(v),
                }),
            }),
            vector: point.vec,
            payload: point.payload.into(),
        }
    }
}

pub enum Id {
    Num(u64),
    Uuid(String),
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