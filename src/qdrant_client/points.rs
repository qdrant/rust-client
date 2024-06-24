use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::prelude::SearchPoints;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::{
    ClearPayloadPoints, CountPoints, CountResponse, CreateFieldIndexCollection,
    DeleteFieldIndexCollection, DeletePayloadPoints, DeletePointVectors, DeletePoints,
    DiscoverBatchPoints, DiscoverBatchResponse, DiscoverPoints, DiscoverResponse, GetPoints,
    GetResponse, PointsOperationResponse, RecommendBatchPoints, RecommendBatchResponse,
    RecommendGroupsResponse, RecommendPointGroups, RecommendPoints, RecommendResponse,
    ScrollPoints, ScrollResponse, SearchBatchPoints, SearchBatchResponse, SearchGroupsResponse,
    SearchPointGroups, SearchResponse, SetPayloadPoints, UpdateBatchPoints, UpdateBatchResponse,
    UpdatePointVectors, UpsertPoints,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

impl Qdrant {
    pub(crate) async fn with_points_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> QdrantResult<T> {
        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        PointsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.config.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                true,
            )
            .await?;
        Ok(result)
    }

    pub async fn search_points(
        &self,
        request: impl Into<SearchPoints>,
    ) -> QdrantResult<SearchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn search_batch_points(
        &self,
        request: impl Into<SearchBatchPoints>,
    ) -> QdrantResult<SearchBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn search_groups(
        &self,
        request: impl Into<SearchPointGroups>,
    ) -> QdrantResult<SearchGroupsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_groups(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn batch_updates(
        &self,
        request: impl Into<UpdateBatchPoints>,
    ) -> QdrantResult<UpdateBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            Ok(points_api.update_batch(request.clone()).await?.into_inner())
        })
        .await
    }

    /// Update or insert points into the collection.
    /// If points with given ID already exist, they will be overwritten.
    /// Also this method does not split the points to insert to avoid timeouts.
    /// Look at [`upsert_points_batch`](Self::upsert_points_batch) for that.
    pub async fn upsert_points(
        &self,
        request: impl Into<UpsertPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();
        self.with_points_client(|mut points_api| async move {
            Ok(points_api.upsert(request.clone()).await?.into_inner())
        })
        .await
    }

    /// Update or insert points into the collection, splitting in chunks.
    /// If points with given ID already exist, they will be overwritten.
    pub async fn upsert_points_batch(
        &self,
        request: impl Into<UpsertPoints>,
        chunk_size: usize,
    ) -> QdrantResult<PointsOperationResponse> {
        let mut request = request.into();
        let points = std::mem::take(&mut request.points);

        if points.len() < chunk_size {
            return self.upsert_points(request).await;
        }

        let request = &request;
        let points = &points;

        self.with_points_client(|mut points_api| async move {
            let mut resp = PointsOperationResponse {
                result: None,
                time: 0.0,
            };

            for chunk in points.clone().chunks(chunk_size) {
                let mut chunked_request = request.clone();
                chunked_request.points = chunk.to_vec();

                let PointsOperationResponse { result, time } =
                    points_api.upsert(chunked_request).await?.into_inner();

                resp.result = result;
                resp.time += time;
            }

            Ok(resp)
        })
        .await
    }

    pub async fn set_payload(
        &self,
        request: impl Into<SetPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.set_payload(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn overwrite_payload(
        &self,
        request: impl Into<SetPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.overwrite_payload(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_payload(
        &self,
        request: impl Into<DeletePayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.delete_payload(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn clear_payload(
        &self,
        request: impl Into<ClearPayloadPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.clear_payload(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn get_points(&self, request: impl Into<GetPoints>) -> QdrantResult<GetResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.get(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_points(
        &self,
        request: impl Into<DeletePoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.delete(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_vectors(
        &self,
        request: impl Into<DeletePointVectors>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.delete_vectors(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn update_vectors(
        &self,
        request: impl Into<UpdatePointVectors>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.update_vectors(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn scroll(&self, request: impl Into<ScrollPoints>) -> QdrantResult<ScrollResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.scroll(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn recommend(
        &self,
        request: impl Into<RecommendPoints>,
    ) -> QdrantResult<RecommendResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn recommend_batch(
        &self,
        request: impl Into<RecommendBatchPoints>,
    ) -> QdrantResult<RecommendBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn recommend_groups(
        &self,
        request: impl Into<RecommendPointGroups>,
    ) -> QdrantResult<RecommendGroupsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend_groups(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn discover(
        &self,
        request: impl Into<DiscoverPoints>,
    ) -> QdrantResult<DiscoverResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.discover(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn discover_batch(
        &self,
        request: &DiscoverBatchPoints,
    ) -> QdrantResult<DiscoverBatchResponse> {
        self.with_points_client(|mut points_api| async move {
            let result = points_api.discover_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn count(&self, request: impl Into<CountPoints>) -> QdrantResult<CountResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.count(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn update_batch_points(
        &self,
        request: impl Into<UpdateBatchPoints>,
    ) -> QdrantResult<UpdateBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.update_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Create index for a payload field
    pub async fn create_field_index(
        &self,
        request: impl Into<CreateFieldIndexCollection>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut client| async move {
            let result = client.create_field_index(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_field_index(
        &self,
        request: impl Into<DeleteFieldIndexCollection>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut client| async move {
            let result = client.delete_field_index(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
