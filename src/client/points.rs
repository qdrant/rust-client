use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::client::{Payload, QdrantClient};
use crate::prelude::{PointStruct, SearchPoints};
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::{
    shard_key, ClearPayloadPoints, CountPoints, CountResponse, CreateFieldIndexCollection,
    DeleteFieldIndexCollection, DeletePayloadPoints, DeletePointVectors, DeletePoints,
    DiscoverBatchPoints, DiscoverBatchResponse, DiscoverPoints, DiscoverResponse, FieldType,
    GetPoints, GetResponse, HardwareUsage, PayloadIndexParams, PointId, PointVectors,
    PointsOperationResponse, PointsSelector, PointsUpdateOperation, ReadConsistency,
    RecommendBatchPoints, RecommendBatchResponse, RecommendGroupsResponse, RecommendPointGroups,
    RecommendPoints, RecommendResponse, ScrollPoints, ScrollResponse, SearchBatchPoints,
    SearchBatchResponse, SearchGroupsResponse, SearchPointGroups, SearchResponse, SetPayloadPoints,
    ShardKeySelector, UpdateBatchPoints, UpdateBatchResponse, UpdatePointVectors, UpsertPoints,
    VectorsSelector, WithPayloadSelector, WithVectorsSelector, WriteOrdering,
};

impl QdrantClient {
    // Access to raw points API
    pub async fn with_points_client<T, O: Future<Output = anyhow::Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> anyhow::Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        PointsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                true,
            )
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    async fn _batch_updates(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
        wait: bool,
    ) -> anyhow::Result<UpdateBatchResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        Ok(self
            .with_points_client(|mut points_api| async move {
                Ok(points_api
                    .update_batch(UpdateBatchPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(wait),
                        operations: operations.to_vec(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?
                    .into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    pub async fn batch_updates(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<UpdateBatchResponse> {
        self._batch_updates(collection_name, operations, ordering, false)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    pub async fn batch_updates_blocking(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<UpdateBatchResponse> {
        self._batch_updates(collection_name, operations, ordering, true)
            .await
    }

    /// Insert or update points into the collection.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does *not* wait for completion of the operation, use
    /// [`upsert_points_blocking`](Self::upsert_points_blocking) for that.
    /// Also this method does not split the points to insert to avoid timeouts,
    /// look at [`upsert_points_batch`](Self::upsert_points_batch) for that.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points` instead"
    )]
    pub async fn upsert_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._upsert_points(
            collection_name,
            shard_key_selector,
            &points,
            false,
            ordering,
        )
        .await
    }

    /// Insert or update points into the collection, wait for completion.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does not split the points to insert to avoid timeouts,
    /// look at [`upsert_points_batch`](Self::upsert_points_batch) for that.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points` instead"
    )]
    pub async fn upsert_points_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._upsert_points(collection_name, shard_key_selector, &points, true, ordering)
            .await
    }

    #[inline]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points` instead"
    )]
    async fn _upsert_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointStruct],
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        Ok(self
            .with_points_client(|mut points_api| async move {
                Ok(points_api
                    .upsert(UpsertPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points.to_vec(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?
                    .into_inner())
            })
            .await?)
    }

    /// Insert or update points into the collection, splitting in chunks.
    /// If points with given ID already exist, they will be overwritten.
    /// This method does *not* wait for completion of the operation, use
    /// [`upsert_points_batch_blocking`](Self::upsert_points_batch_blocking) for that.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points_batch` instead"
    )]
    pub async fn upsert_points_batch(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._upsert_points_batch(
            collection_name,
            shard_key_selector,
            &points,
            false,
            ordering,
            chunk_size,
        )
        .await
    }

    /// Insert or update points into the collection, splitting in chunks and
    /// waiting for completion of each.
    /// If points with given ID already exist, they will be overwritten.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points_batch` instead"
    )]
    pub async fn upsert_points_batch_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: Vec<PointStruct>,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._upsert_points_batch(
            collection_name,
            shard_key_selector,
            &points,
            true,
            ordering,
            chunk_size,
        )
        .await
    }

    #[inline]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::upsert_points_batch` instead"
    )]
    async fn _upsert_points_batch(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointStruct],
        block: bool,
        ordering: Option<WriteOrdering>,
        chunk_size: usize,
    ) -> anyhow::Result<PointsOperationResponse> {
        if points.len() < chunk_size {
            return self
                ._upsert_points(collection_name, shard_key_selector, points, block, ordering)
                .await;
        }
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        Ok(self
            .with_points_client(|mut points_api| async move {
                let mut resp = PointsOperationResponse {
                    result: None,
                    time: 0.0,
                    usage: None,
                };
                for chunk in points.chunks(chunk_size) {
                    let PointsOperationResponse {
                        result,
                        time,
                        usage,
                    } = points_api
                        .upsert(UpsertPoints {
                            collection_name: collection_name_ref.to_string(),
                            wait: Some(block),
                            points: chunk.to_vec(),
                            ordering: ordering_ref.cloned(),
                            shard_key_selector: shard_keys_ref.clone(),
                        })
                        .await?
                        .into_inner();
                    resp.result = result;
                    resp.time += time;
                    if let Some(usage) = usage {
                        if let Some(resp_usage) = &mut resp.usage {
                            let HardwareUsage {
                                cpu,
                                payload_io_read,
                                payload_io_write,
                                payload_index_io_read,
                                vector_io_read,
                                vector_io_write,
                            } = usage;

                            resp_usage.cpu += cpu;
                            resp_usage.payload_io_read += payload_io_read;
                            resp_usage.payload_io_write += payload_io_write;
                            resp_usage.payload_index_io_read += payload_index_io_read;
                            resp_usage.vector_io_read += vector_io_read;
                            resp_usage.vector_io_write += vector_io_write;
                        } else {
                            resp.usage = Some(usage);
                        }
                    }
                }
                Ok(resp)
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::set_payload` instead"
    )]
    pub async fn set_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._set_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            false,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::set_payload` instead"
    )]
    pub async fn set_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._set_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::set_payload` instead"
    )]
    async fn _set_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: &Payload,
        payload_key: Option<String>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        let payload_key_ref = payload_key.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .set_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                        key: payload_key_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::overwrite_payload` instead"
    )]
    pub async fn overwrite_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._overwrite_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            false,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::overwrite_payload` instead"
    )]
    pub async fn overwrite_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: Payload,
        payload_key: Option<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._overwrite_payload(
            collection_name,
            shard_key_selector,
            points,
            &payload,
            payload_key,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[allow(clippy::too_many_arguments)]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::overwrite_payload` instead"
    )]
    async fn _overwrite_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        payload: &Payload,
        payload_key: Option<String>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;
        let payload_key_ref = payload_key.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .overwrite_payload(SetPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        payload: payload.0.clone(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                        key: payload_key_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_payload` instead"
    )]
    pub async fn delete_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_payload(
            collection_name,
            shard_key_selector,
            points,
            &keys,
            false,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_payload` instead"
    )]
    pub async fn delete_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: Vec<String>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_payload(
            collection_name,
            shard_key_selector,
            points,
            &keys,
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_payload` instead"
    )]
    async fn _delete_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        keys: &[String],
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete_payload(DeletePayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        keys: keys.to_owned(),
                        points_selector: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::clear_payload` instead"
    )]
    pub async fn clear_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._clear_payload(
            collection_name,
            shard_key_selector,
            points_selector.as_ref(),
            false,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::clear_payload` instead"
    )]
    pub async fn clear_payload_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<PointsSelector>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._clear_payload(
            collection_name,
            shard_key_selector,
            points_selector.as_ref(),
            true,
            ordering,
        )
        .await
    }

    #[inline]
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::clear_payload` instead"
    )]
    async fn _clear_payload(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: Option<&PointsSelector>,
        block: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .clear_payload(ClearPayloadPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(block),
                        points: points_selector.cloned(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::get_points` instead"
    )]
    pub async fn get_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointId],
        with_vectors: Option<impl Into<WithVectorsSelector>>,
        with_payload: Option<impl Into<WithPayloadSelector>>,
        read_consistency: Option<ReadConsistency>,
    ) -> anyhow::Result<GetResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();

        let with_vectors = with_vectors.map(|v| v.into());
        let with_payload = with_payload.map(|v| v.into());

        let with_vectors_ref = with_vectors.as_ref();
        let with_payload_ref = with_payload.as_ref();
        let read_consistency_ref = read_consistency.as_ref();

        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .get(GetPoints {
                        collection_name: collection_name_ref.to_string(),
                        ids: points.to_owned(),
                        with_payload: with_payload_ref.cloned(),
                        with_vectors: with_vectors_ref.cloned(),
                        read_consistency: read_consistency_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                        timeout: None,
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::search_points` instead"
    )]
    pub async fn search_points(&self, request: &SearchPoints) -> anyhow::Result<SearchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::search_batch_points` instead"
    )]
    pub async fn search_batch_points(
        &self,
        request: &SearchBatchPoints,
    ) -> anyhow::Result<SearchBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::search_groups` instead"
    )]
    pub async fn search_groups(
        &self,
        request: &SearchPointGroups,
    ) -> anyhow::Result<SearchGroupsResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.search_groups(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_points` instead"
    )]
    pub async fn delete_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_points(collection_name, shard_key_selector, false, points, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_points` instead"
    )]
    pub async fn delete_points_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_points(collection_name, shard_key_selector, true, points, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_points` instead"
    )]
    async fn _delete_points(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points: &PointsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete(DeletePoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points: Some(points.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_vectors` instead"
    )]
    pub async fn delete_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_vectors(
            collection_name,
            shard_key_selector,
            false,
            points_selector,
            vector_selector,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_vectors` instead"
    )]
    pub async fn delete_vectors_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_vectors(
            collection_name,
            shard_key_selector,
            true,
            points_selector,
            vector_selector,
            ordering,
        )
        .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_vectors` instead"
    )]
    async fn _delete_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points_selector: &PointsSelector,
        vector_selector: &VectorsSelector,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .delete_vectors(DeletePointVectors {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points_selector: Some(points_selector.clone()),
                        vectors: Some(vector_selector.clone()),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_vectors` instead"
    )]
    pub async fn update_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._update_vectors(collection_name, shard_key_selector, false, points, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_vectors` instead"
    )]
    pub async fn update_vectors_blocking(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._update_vectors(collection_name, shard_key_selector, true, points, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_vectors` instead"
    )]
    async fn _update_vectors(
        &self,
        collection_name: impl ToString,
        shard_key_selector: Option<Vec<shard_key::Key>>,
        blocking: bool,
        points: &[PointVectors],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();
        let shard_keys = shard_key_selector.map(ShardKeySelector::from);
        let shard_keys_ref = &shard_keys;

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .update_vectors(UpdatePointVectors {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        points: points.to_owned(),
                        ordering: ordering_ref.cloned(),
                        shard_key_selector: shard_keys_ref.clone(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::scroll` instead"
    )]
    pub async fn scroll(&self, request: &ScrollPoints) -> anyhow::Result<ScrollResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.scroll(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::recommend` instead"
    )]
    pub async fn recommend(&self, request: &RecommendPoints) -> anyhow::Result<RecommendResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::recommend_batch` instead"
    )]
    pub async fn recommend_batch(
        &self,
        request: &RecommendBatchPoints,
    ) -> anyhow::Result<RecommendBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::recommend_groups` instead"
    )]
    pub async fn recommend_groups(
        &self,
        request: &RecommendPointGroups,
    ) -> anyhow::Result<RecommendGroupsResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.recommend_groups(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::discover` instead"
    )]
    pub async fn discover(&self, request: &DiscoverPoints) -> anyhow::Result<DiscoverResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.discover(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::discover_batch` instead"
    )]
    pub async fn discover_batch(
        &self,
        request: &DiscoverBatchPoints,
    ) -> anyhow::Result<DiscoverBatchResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.discover_batch(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::count` instead"
    )]
    pub async fn count(&self, request: &CountPoints) -> anyhow::Result<CountResponse> {
        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api.count(request.clone()).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Perform multiple point, vector and payload insert, update and delete operations in one request.
    /// This method does *not* wait for completion of the operation, use
    /// [`update_batch_points_blocking`](Self::update_batch_points_blocking) for that.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    pub async fn update_batch_points(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<UpdateBatchResponse> {
        self._update_batch_points(collection_name, false, operations, ordering)
            .await
    }

    /// Perform multiple point, vector and payload insert, update and delete operations in one request.
    /// This method waits for completion on each operation.
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    pub async fn update_batch_points_blocking(
        &self,
        collection_name: impl ToString,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<UpdateBatchResponse> {
        self._update_batch_points(collection_name, true, operations, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::update_points_batch` instead"
    )]
    async fn _update_batch_points(
        &self,
        collection_name: impl ToString,
        blocking: bool,
        operations: &[PointsUpdateOperation],
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<UpdateBatchResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let ordering_ref = ordering.as_ref();

        Ok(self
            .with_points_client(|mut points_api| async move {
                let result = points_api
                    .update_batch(UpdateBatchPoints {
                        collection_name: collection_name_ref.to_string(),
                        wait: Some(blocking),
                        operations: operations.to_owned(),
                        ordering: ordering_ref.cloned(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    /// Create index for a payload field
    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_field_index` instead"
    )]
    pub async fn _create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_field_index` instead"
    )]
    pub async fn create_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::create_field_index` instead"
    )]
    pub async fn create_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        field_type: FieldType,
        field_index_params: Option<&PayloadIndexParams>,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_field_index` instead"
    )]
    pub async fn _delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        wait: bool,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
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

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_field_index` instead"
    )]
    pub async fn delete_field_index(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, false, ordering)
            .await
    }

    #[deprecated(
        since = "1.10.0",
        note = "use new `qdrant_client::Qdrant::delete_field_index` instead"
    )]
    pub async fn delete_field_index_blocking(
        &self,
        collection_name: impl ToString,
        field_name: impl ToString,
        ordering: Option<WriteOrdering>,
    ) -> anyhow::Result<PointsOperationResponse> {
        self._delete_field_index(collection_name, field_name, true, ordering)
            .await
    }
}
