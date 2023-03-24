use crate::channel_pool::ChannelPool;
use crate::qdrant::alias_operations::Action;
use crate::qdrant::collections_client::CollectionsClient;
use crate::qdrant::condition::ConditionOneOf;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::value::Kind;
use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::with_payload_selector::SelectorOptions;
use crate::qdrant::{
    qdrant_client, with_vectors_selector, AliasOperations, ChangeAliases, ClearPayloadPoints,
    CollectionOperationResponse, Condition, CountPoints, CountResponse, CreateAlias,
    CreateCollection, CreateFieldIndexCollection, CreateFullSnapshotRequest, CreateSnapshotRequest,
    CreateSnapshotResponse, DeleteAlias, DeleteCollection, DeleteFieldIndexCollection,
    DeleteFullSnapshotRequest, DeletePayloadPoints, DeletePoints, DeleteSnapshotRequest,
    DeleteSnapshotResponse, FieldCondition, FieldType, Filter, GetCollectionInfoRequest,
    GetCollectionInfoResponse, GetPoints, GetResponse, HasIdCondition, HealthCheckReply,
    HealthCheckRequest, IsEmptyCondition, ListAliasesRequest, ListAliasesResponse,
    ListCollectionAliasesRequest, ListCollectionsRequest, ListCollectionsResponse,
    ListFullSnapshotsRequest, ListSnapshotsRequest, ListSnapshotsResponse, ListValue, NamedVectors,
    OptimizersConfigDiff, PayloadIncludeSelector, PayloadIndexParams, PointId, PointStruct,
    PointsIdsList, PointsOperationResponse, PointsSelector, ReadConsistency, RecommendBatchPoints,
    RecommendBatchResponse, RecommendPoints, RecommendResponse, RenameAlias, ScrollPoints,
    ScrollResponse, SearchBatchPoints, SearchBatchResponse, SearchPoints, SearchResponse,
    SetPayloadPoints, Struct, UpdateCollection, UpsertPoints, Value, Vector, Vectors,
    VectorsSelector, WithPayloadSelector, WithVectorsSelector, WriteOrdering,
};
use anyhow::{bail, Result};
use reqwest::multipart::{Form, Part};
use std::collections::HashMap;
use std::future::Future;
use std::path::PathBuf;
use std::time::Duration;
use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::{Channel, Uri};
use tonic::{Request, Status};

pub struct QdrantClientConfig {
    pub uri: String,
    pub timeout: Duration,
    pub connect_timeout: Duration,
    pub keep_alive_while_idle: bool,
    pub api_key: Option<String>,
}

impl QdrantClientConfig {
    pub fn from_url(url: &str) -> Self {
        QdrantClientConfig {
            uri: url.to_string(),
            ..Self::default()
        }
    }

    pub fn set_api_key(&mut self, api_key: &str) {
        self.api_key = Some(api_key.to_string());
    }

    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = timeout;
    }

    pub fn set_connect_timeout(&mut self, connect_timeout: Duration) {
        self.connect_timeout = connect_timeout;
    }

    pub fn set_keep_alive_while_idle(&mut self, keep_alive_while_idle: bool) {
        self.keep_alive_while_idle = keep_alive_while_idle;
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

impl From<FieldCondition> for Condition {
    fn from(field_condition: FieldCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::Field(field_condition)),
        }
    }
}

impl From<IsEmptyCondition> for Condition {
    fn from(is_empty_condition: IsEmptyCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::IsEmpty(is_empty_condition)),
        }
    }
}

impl From<HasIdCondition> for Condition {
    fn from(has_id_condition: HasIdCondition) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::HasId(has_id_condition)),
        }
    }
}

impl From<Filter> for Condition {
    fn from(filter: Filter) -> Self {
        Condition {
            condition_one_of: Some(ConditionOneOf::Filter(filter)),
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
            selector_options: Some(with_vectors_selector::SelectorOptions::Include(
                VectorsSelector {
                    names: names.into_iter().map(|name| name.to_string()).collect(),
                },
            )),
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
            api_key: None,
        }
    }
}

pub struct TokenInterceptor {
    api_key: Option<String>,
}

impl TokenInterceptor {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        if let Some(api_key) = &self.api_key {
            req.metadata_mut().insert(
                "api-key",
                api_key.parse().map_err(|_| {
                    Status::invalid_argument(format!("Malformed API key: {}", api_key))
                })?,
            );
        }
        Ok(req)
    }
}

pub struct QdrantClient {
    pub channel: ChannelPool,
    pub cfg: QdrantClientConfig,
}

impl QdrantClient {
    pub async fn with_snapshot_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(SnapshotsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    f(SnapshotsClient::with_interceptor(
                        channel,
                        TokenInterceptor::new(self.cfg.api_key.clone()),
                    ))
                },
                false,
            )
            .await
    }

    // Access to raw collection API
    pub async fn with_collections_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(CollectionsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    f(CollectionsClient::with_interceptor(
                        channel,
                        TokenInterceptor::new(self.cfg.api_key.clone()),
                    ))
                },
                false,
            )
            .await
    }

    // Access to raw points API
    pub async fn with_points_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    f(PointsClient::with_interceptor(
                        channel,
                        TokenInterceptor::new(self.cfg.api_key.clone()),
                    ))
                },
                true,
            )
            .await
    }

    // Access to raw root qdrant API
    pub async fn with_root_qdrant_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(qdrant_client::QdrantClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    f(qdrant_client::QdrantClient::with_interceptor(
                        channel,
                        TokenInterceptor::new(self.cfg.api_key.clone()),
                    ))
                },
                true,
            )
            .await
    }

    pub async fn new(cfg: Option<QdrantClientConfig>) -> Result<Self> {
        let cfg = cfg.unwrap_or_default();

        let channel = ChannelPool::new(
            cfg.uri.parse::<Uri>()?,
            cfg.timeout,
            cfg.connect_timeout,
            cfg.keep_alive_while_idle,
        );

        let client = Self { channel, cfg };

        Ok(client)
    }

    pub async fn health_check(&self) -> Result<HealthCheckReply> {
        Ok(self
            .with_root_qdrant_client(|mut qdrant_api| async move {
                let result = qdrant_api.health_check(HealthCheckRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_collections(&self) -> Result<ListCollectionsResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list(ListCollectionsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
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
        details: &CreateCollection,
    ) -> Result<CollectionOperationResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.create(details.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn update_collection(
        &self,
        collection_name: impl ToString,
        optimizers_config: &OptimizersConfigDiff,
    ) -> Result<CollectionOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .update(UpdateCollection {
                        collection_name: collection_name_ref.to_string(),
                        optimizers_config: Some(optimizers_config.clone()),
                        timeout: None,
                        params: None,
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_collection(
        &self,
        collection_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .delete(DeleteCollection {
                        collection_name: collection_name_ref.to_string(),
                        ..Default::default()
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn collection_info(
        &self,
        collection_name: impl ToString,
    ) -> Result<GetCollectionInfoResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .get(GetCollectionInfoRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_alias(
        &self,
        collection_name: impl ToString,
        alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let create_alias = CreateAlias {
            collection_name: collection_name.to_string(),
            alias_name: alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::CreateAlias(create_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    pub async fn delete_alias(
        &self,
        alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let delete_alias = DeleteAlias {
            alias_name: alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::DeleteAlias(delete_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    pub async fn rename_alias(
        &self,
        old_alias_name: impl ToString,
        new_alias_name: impl ToString,
    ) -> Result<CollectionOperationResponse> {
        let rename_alias = RenameAlias {
            old_alias_name: old_alias_name.to_string(),
            new_alias_name: new_alias_name.to_string(),
        };
        let change_aliases = ChangeAliases {
            actions: vec![AliasOperations {
                action: Some(Action::RenameAlias(rename_alias)),
            }],
            timeout: None,
        };
        self.update_aliases(change_aliases).await
    }

    // lower level API
    pub async fn update_aliases(
        &self,
        change_aliases: ChangeAliases,
    ) -> Result<CollectionOperationResponse> {
        let change_aliases = change_aliases.clone();
        let chang_aliases_ref = &change_aliases;
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .update_aliases(chang_aliases_ref.clone())
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_collection_aliases(
        &self,
        collection_name: impl ToString,
    ) -> Result<ListAliasesResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api
                    .list_collection_aliases(ListCollectionAliasesRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_aliases(&self) -> Result<ListAliasesResponse> {
        Ok(self
            .with_collections_client(|mut collection_api| async move {
                let result = collection_api.list_aliases(ListAliasesRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn upsert_points(
        &self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(collection_name, &points, false, ordering)
            .await
    }

    pub async fn upsert_points_blocking(
        &self,
        collection_name: impl ToString,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._upsert_points(collection_name, &points, true, ordering)
            .await
    }

    #[inline]
    async fn _upsert_points(
        &self,
        collection_name: impl ToString,
        points: &Vec<PointStruct>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .upsert(UpsertPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points.clone(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn set_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: Payload,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(collection_name, points, &payload, false, ordering)
            .await
    }

    pub async fn set_payload_blocking(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: Payload,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._set_payload(collection_name, points, &payload, true, ordering)
            .await
    }

    #[inline]
    async fn _set_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: &Payload,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .set_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn overwrite_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: Payload,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._overwrite_payload(collection_name, points, &payload, false, ordering)
            .await
    }

    pub async fn overwrite_payload_blocking(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: Payload,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._overwrite_payload(collection_name, points, &payload, true, ordering)
            .await
    }

    #[inline]
    async fn _overwrite_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        payload: &Payload,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .overwrite_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(collection_name, points, &keys, false, ordering)
            .await
    }

    pub async fn delete_payload_blocking(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_payload(collection_name, points, &keys, true, ordering)
            .await
    }

    #[inline]
    async fn _delete_payload(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        keys: &Vec<String>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete_payload(DeletePayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        keys: keys.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn clear_payload(
        &self,
        collection_name: impl ToString,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(collection_name, points_selector.as_ref(), false, ordering)
            .await
    }

    pub async fn clear_payload_blocking(
        &self,
        collection_name: impl ToString,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._clear_payload(collection_name, points_selector.as_ref(), true, ordering)
            .await
    }

    #[inline]
    async fn _clear_payload(
        &self,
        collection_name: impl ToString,
        points_selector: Option<&PointsSelector>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .clear_payload(ClearPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points_selector.cloned(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn get_points(
        &self,
        collection_name: impl ToString,
        points: &Vec<PointId>,
        with_vectors: Option<impl Into<WithVectorsSelector>>,
        with_payload: Option<impl Into<WithPayloadSelector>>,
        read_consistency: Option<ReadConsistency>,
    ) -> Result<GetResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        let with_vectors = with_vectors.map(|v| v.into());
        let with_payload = with_payload.map(|v| v.into());

        let with_vectors_ref = with_vectors.as_ref();
        let with_payload_ref = with_payload.as_ref();
        let read_consistency_ref = read_consistency.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .get(GetPoints {
                        collection_name: collection_name_ref.to_string(),
                        ids: points.clone(),
                        with_payload: with_payload_ref.cloned(),
                        with_vectors: with_vectors_ref.cloned(),
                        read_consistency: read_consistency_ref.cloned(),
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn search_points(&self, request: &SearchPoints) -> Result<SearchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn search_batch_points(
        &self,
        request: &SearchBatchPoints,
    ) -> Result<SearchBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_points(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, false, points, ordering)
            .await
    }

    pub async fn delete_points_blocking(
        &self,
        collection_name: impl ToString,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_points(collection_name, true, points, ordering)
            .await
    }

    async fn _delete_points(
        &self,
        collection_name: impl ToString,
        blocking: bool,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete(DeletePoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn scroll(&self, request: &ScrollPoints) -> Result<ScrollResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.scroll(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn recommend(&self, request: &RecommendPoints) -> Result<RecommendResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn recommend_batch(
        &self,
        request: &RecommendBatchPoints,
    ) -> Result<RecommendBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn count(&self, request: &CountPoints) -> Result<CountResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.count(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Create index for a payload field
    pub async fn _create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let field_name = field_name.to_string();
        let field_name_ref = field_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut client| async move {
                let result = client
                    .create_field_index(CreateFieldIndexCollection {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        field_name: field_name_ref.to_string(),
                        field_type: Some(field_type.into()),
                        field_index_params: field_index_params.cloned(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._create_field_index(
            collection_name,
            field_name,
            field_type,
            field_index_params,
            false,
            ordering,
        )
        .await
    }

    pub async fn create_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._create_field_index(
            collection_name,
            field_name,
            field_type,
            field_index_params,
            true,
            ordering,
        )
        .await
    }

    pub async fn _delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let field_name = field_name.to_string();
        let field_name_ref = field_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut client| async move {
                let result = client
                    .delete_field_index(DeleteFieldIndexCollection {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        field_name: field_name_ref.to_string(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, false, ordering)
            .await
    }

    pub async fn delete_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, true, ordering)
            .await
    }

    pub async fn create_snapshot(
        &self,
        collection_name: impl ToString,
    ) -> Result<CreateSnapshotResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .create(CreateSnapshotRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_snapshots(
        &self,
        collection_name: impl ToString,
    ) -> Result<ListSnapshotsResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .list(ListSnapshotsRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_snapshot(
        &self,
        collection_name: impl ToString,
        snapshot_name: impl ToString,
    ) -> Result<DeleteSnapshotResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let snapshot_name = snapshot_name.to_string();
        let snapshot_name_ref = snapshot_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .delete(DeleteSnapshotRequest {
                        collection_name: collection_name_ref.to_string(),
                        snapshot_name: snapshot_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_full_snapshot(&self) -> Result<CreateSnapshotResponse> {
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client.create_full(CreateFullSnapshotRequest {}).await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_full_snapshots(&self) -> Result<ListSnapshotsResponse> {
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client.list_full(ListFullSnapshotsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_full_snapshot(
        &self,
        snapshot_name: impl ToString,
    ) -> Result<DeleteSnapshotResponse> {
        let snapshot_name = snapshot_name.to_string();
        let snapshot_name_ref = snapshot_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .delete_full(DeleteFullSnapshotRequest {
                        snapshot_name: snapshot_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
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
        use tokio::io::AsyncWriteExt;

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
        let _ = tokio::fs::remove_file(&out_path).await;
        let mut file = tokio::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_path)
            .await?;

        while let Some(chunk) = stream.next().await {
            let _written = file.write(&chunk?).await?;
        }

        Ok(())
    }

    pub async fn upload_snapshot<T>(
        &self,
        in_path: impl Into<PathBuf>,
        collection_name: T,
        rest_api_uri: Option<T>,
    ) -> Result<()>
    where
        T: ToString + Clone,
    {
        let snapshot_path: PathBuf = in_path.into();
        let filename = snapshot_path
            .file_name()
            .and_then(|name| name.to_str().map(|name| name.to_string()));

        let snapshot_file = tokio::fs::read(snapshot_path).await?;

        let mut part = Part::bytes(snapshot_file);
        if let Some(filename) = filename {
            part = part.file_name(filename);
        }
        let form = Form::new().part("snapshot", part);
        let client = reqwest::Client::new();
        client
            .post(format!(
                "{}/collections/{}/snapshots/upload",
                rest_api_uri
                    .map(|uri| uri.to_string())
                    .unwrap_or_else(|| String::from("http://localhost:6333")),
                collection_name.to_string()
            ))
            .multipart(form)
            .send()
            .await?;

        Ok(())
    }
}

impl PointStruct {
    pub fn new(id: impl Into<PointId>, vectors: impl Into<Vectors>, payload: Payload) -> Self {
        Self {
            id: Some(id.into()),
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

#[derive(Clone, PartialEq)]
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
