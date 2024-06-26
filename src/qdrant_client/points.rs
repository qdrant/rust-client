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

/// Point operations.
///
/// Manage points, vectors and payloads. Search and explore them.
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

    /// Search points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, SearchParamsBuilder, SearchPointsBuilder};
    ///
    ///# async fn search_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .search_points(
    ///         SearchPointsBuilder::new("my_collection", vec![0.2, 0.1, 0.9, 0.7], 3)
    ///             .filter(Filter::must([Condition::matches(
    ///                 "city",
    ///                 "London".to_string(),
    ///             )]))
    ///             .params(SearchParamsBuilder::default().hnsw_ef(128).exact(false)),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#search-api>
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

    /// Batch multiple points searches in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, SearchBatchPointsBuilder, SearchPointsBuilder,};
    ///
    ///# async fn search_batch_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let filter = Filter::must([Condition::matches("city", "London".to_string())]);
    ///
    /// let searches = vec![
    ///     SearchPointsBuilder::new("my_collection", vec![0.2, 0.1, 0.9, 0.7], 3)
    ///         .filter(filter.clone())
    ///         .build(),
    ///     SearchPointsBuilder::new("my_collection", vec![0.5, 0.3, 0.2, 0.3], 3)
    ///         .filter(filter)
    ///         .build(),
    /// ];
    ///
    /// client
    ///     .search_batch_points(SearchBatchPointsBuilder::new("my_collection", searches))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#batch-search-api>
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

    /// Search points in a collection and group results by a payload field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::SearchPointGroupsBuilder;
    ///
    ///# async fn search_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .search_groups(SearchPointGroupsBuilder::new(
    ///         "my_collection", // Collection name
    ///         vec![1.1],       // Search vector
    ///         4,               // Search limit
    ///         "document_id",   // Group by field
    ///         2,               // Group size
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#search-groups>
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

    /// Insert or update points in a collection.
    ///
    /// If points with the specified IDs already exist, they will be overwritten.
    ///
    /// All points are upserted in a single operation. For a large number of points you likley want
    /// split all upsertions into separate operations to avoid timing out. You can use
    /// [`upsert_points_chunked`](Self::upsert_points_chunked) to automatically do that for you.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::Payload;
    /// use qdrant_client::qdrant::{PointStruct, UpsertPointsBuilder};
    /// use serde_json::json;
    ///
    ///# async fn upsert_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .upsert_points(
    ///         UpsertPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 PointStruct::new(
    ///                     1,
    ///                     vec![0.9, 0.1, 0.1],
    ///                     Payload::try_from(json!(
    ///                         {"color": "red"}
    ///                     ))
    ///                     .unwrap(),
    ///                 ),
    ///                 PointStruct::new(
    ///                     2,
    ///                     vec![0.1, 0.9, 0.1],
    ///                     Payload::try_from(json!(
    ///                         {"color": "green"}
    ///                     ))
    ///                     .unwrap(),
    ///                 ),
    ///                 PointStruct::new(
    ///                     3,
    ///                     vec![0.1, 0.1, 0.9],
    ///                     Payload::try_from(json!(
    ///                         {"color": "blue"}
    ///                     ))
    ///                     .unwrap(),
    ///                 ),
    ///             ],
    ///         )
    ///         .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#upload-points>
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

    /// Insert or update points in a collection.
    ///
    /// The same as [`upsert_points`](Self::upsert_points), but it automatically splits all points
    /// into chunks of `chunk_size` to prevent timing out.
    #[doc(alias = "upsert_points_batch")]
    pub async fn upsert_points_chunked(
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

    /// Set payload of points.
    ///
    /// Sets only the given payload values on a point, leaving other existing payloads in place.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::Payload;
    /// use qdrant_client::qdrant::{PointsIdsList, SetPayloadPointsBuilder};
    /// use serde_json::json;
    ///
    ///# async fn set_payload(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let payload: Payload = json!({
    ///     "property1": "string",
    ///     "property2": "string",
    /// })
    /// .try_into()
    /// .unwrap();
    ///
    /// client
    ///     .set_payload(
    ///         SetPayloadPointsBuilder::new("my_collection", payload)
    ///             .points_selector(PointsIdsList {
    ///                 ids: vec![0.into(), 3.into(), 10.into()],
    ///             })
    ///             .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/payload/#set-payload>
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

    /// Overwrite payload of points.
    ///
    /// Sets the given payload values on a point, completely replacing existing payload.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::Payload;
    /// use qdrant_client::qdrant::{
    ///     points_selector::PointsSelectorOneOf, PointsIdsList, SetPayloadPointsBuilder,
    /// };
    /// use serde_json::json;
    ///
    ///# async fn overwrite_payload(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let payload: Payload = json!({
    ///     "property1": "string",
    ///     "property2": "string",
    /// })
    /// .try_into()
    /// .unwrap();
    ///
    /// client
    ///     .overwrite_payload(
    ///         SetPayloadPointsBuilder::new("my_collection", payload)
    ///             .points_selector(PointsSelectorOneOf::Points(PointsIdsList {
    ///                 ids: vec![0.into(), 3.into(), 10.into()],
    ///             }))
    ///             .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/payload/#overwrite-payload>
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

    /// Delete specified payload keys of points.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{DeletePayloadPointsBuilder, PointsIdsList};
    ///
    ///# async fn delete_payload(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_payload(
    ///         DeletePayloadPointsBuilder::new(
    ///             "my_collection",
    ///             vec!["color".to_string(), "price".to_string()],
    ///         )
    ///         .points_selector(PointsIdsList {
    ///             ids: vec![0.into(), 3.into(), 100.into()],
    ///         })
    ///         .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/payload/#delete-payload-keys>
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

    /// Clear all payload of points.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{ClearPayloadPointsBuilder, PointsIdsList};
    ///
    ///# async fn clear_payload(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .clear_payload(ClearPayloadPointsBuilder::new("my_collection").points(
    ///         PointsIdsList {
    ///             ids: vec![0.into(), 3.into(), 100.into()],
    ///         },
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/payload/#clear-payload>
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

    /// Retrieve specific points from a collection.
    ///
    /// Use [`with_vectors`](crate::qdrant::GetPointsBuilder::with_vectors) and
    /// [`with_payload`](crate::qdrant::GetPointsBuilder::with_payload) to specify whether to
    /// include or exclude vector and payload data in the response. By default they are excluded to
    /// save bandwidth.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::GetPointsBuilder;
    ///
    ///# async fn get_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .get_points(
    ///         GetPointsBuilder::new(
    ///             "my_collection",
    ///             vec![0.into(), 30.into(), 100.into()],
    ///         )
    ///         .with_vectors(true)
    ///         .with_payload(true)
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#retrieve-points>
    pub async fn get_points(&self, request: impl Into<GetPoints>) -> QdrantResult<GetResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.get(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete points from a collection.
    ///
    /// Delete by point ID:
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{DeletePointsBuilder, PointsIdsList};
    ///
    ///# async fn delete_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_points(
    ///         DeletePointsBuilder::new("my_collection")
    ///             .points(PointsIdsList {
    ///                 ids: vec![0.into(), 3.into(), 100.into()],
    ///             })
    ///             .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Or delete by [`Filter`](crate::qdrant::Filter):
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, DeletePointsBuilder, Filter};
    ///
    ///# async fn delete_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_points(
    ///         DeletePointsBuilder::new("my_collection")
    ///             .points(Filter::must([Condition::matches(
    ///                 "color",
    ///                 "red".to_string(),
    ///             )]))
    ///             .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#delete-points>
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

    /// Delete vectors from points.
    ///
    /// Removes specified vectors from points in a collection, leaving existing vectors on these
    /// points with a different name in place.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{DeletePointVectorsBuilder, PointsIdsList, VectorsSelector};
    ///
    ///# async fn delete_vectors(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_vectors(
    ///         DeletePointVectorsBuilder::new("my_collection")
    ///             .points_selector(PointsIdsList {
    ///                 ids: vec![0.into(), 3.into(), 10.into()],
    ///             })
    ///             .vectors(VectorsSelector {
    ///                 names: vec!["text".into(), "image".into()],
    ///             })
    ///             .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#delete-vectors>
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

    /// Update vectors on points.
    ///
    /// Updates the given vectors on points in a collection, leaving existing vectors on these points
    /// with a different name in place.
    ///
    /// ```no_run
    ///# use std::collections::HashMap;
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{PointVectors, UpdatePointVectorsBuilder};
    ///
    ///# async fn update_vectors(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .update_vectors(
    ///         UpdatePointVectorsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 PointVectors {
    ///                     id: Some(1.into()),
    ///                     vectors: Some(
    ///                         HashMap::from([("image".to_string(), vec![0.1, 0.2, 0.3, 0.4])])
    ///                             .into(),
    ///                     ),
    ///                 },
    ///                 PointVectors {
    ///                     id: Some(2.into()),
    ///                     vectors: Some(
    ///                         HashMap::from([(
    ///                             "text".to_string(),
    ///                             vec![0.9, 0.8, 0.7, 0.6, 0.5, 0.4, 0.3, 0.2],
    ///                         )])
    ///                         .into(),
    ///                     ),
    ///                 },
    ///             ],
    ///         )
    ///         .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#update-vectors>
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

    /// Scroll points in a collection.
    ///
    /// Use [`with_vectors`](crate::qdrant::ScrollPointsBuilder::with_vectors) and
    /// [`with_payload`](crate::qdrant::ScrollPointsBuilder::with_payload) to specify whether to
    /// include or exclude vector and payload data in the response. By default they are excluded to
    /// save bandwidth.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, ScrollPointsBuilder};
    ///
    ///# async fn scroll(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .scroll(
    ///         ScrollPointsBuilder::new("my_collection")
    ///             .filter(Filter::must([Condition::matches(
    ///                 "color",
    ///                 "red".to_string(),
    ///             )]))
    ///             .limit(1)
    ///             .with_payload(true)
    ///             .with_vectors(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#scroll-points>
    pub async fn scroll(&self, request: impl Into<ScrollPoints>) -> QdrantResult<ScrollResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.scroll(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Recommend points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, RecommendPointsBuilder, RecommendStrategy};
    ///
    ///# async fn recommend(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .recommend(
    ///         RecommendPointsBuilder::new("my_collection", 3)
    ///             .add_positive(100)
    ///             .add_positive(200)
    ///             .add_positive(vec![100.0, 231.0])
    ///             .add_negative(718)
    ///             .add_negative(vec![0.2, 0.3, 0.4, 0.5])
    ///             .strategy(RecommendStrategy::AverageVector)
    ///             .filter(Filter::must([Condition::matches(
    ///                 "city",
    ///                 "London".to_string(),
    ///             )])),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#recommendation-api>
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

    /// Batch multiple points recommendations in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, RecommendBatchPointsBuilder, RecommendPointsBuilder};
    ///
    ///# async fn recommend_batch(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let filter = Filter::must([Condition::matches("city", "London".to_string())]);
    ///
    /// let recommend_queries = vec![
    ///     RecommendPointsBuilder::new("my_collection", 3)
    ///         .add_positive(100)
    ///         .add_positive(231)
    ///         .add_negative(718)
    ///         .filter(filter.clone())
    ///         .build(),
    ///     RecommendPointsBuilder::new("my_collection", 3)
    ///         .add_positive(200)
    ///         .add_positive(67)
    ///         .add_negative(300)
    ///         .filter(filter.clone())
    ///         .build(),
    /// ];
    ///
    /// client
    ///     .recommend_batch(RecommendBatchPointsBuilder::new(
    ///         "my_collection",
    ///         recommend_queries,
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#batch-recommendation-api>
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

    /// Recommend points in a collection and group results by a payload field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{RecommendPointGroupsBuilder, RecommendStrategy};
    ///
    ///# async fn recommend_groups(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .recommend_groups(
    ///         RecommendPointGroupsBuilder::new(
    ///             "my_collection", // Collection name
    ///             "document_id",   // Group by field
    ///             2,               // Group size
    ///             3,               // Search limit
    ///         )
    ///         .add_positive(100)
    ///         .add_positive(200)
    ///         .add_negative(718)
    ///         .strategy(RecommendStrategy::AverageVector),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#recommendation-api>
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

    /// Discover points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{
    ///     target_vector::Target, vector_example::Example, ContextExamplePairBuilder,
    ///     DiscoverPointsBuilder, VectorExample,
    /// };
    ///
    ///# async fn discover(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .discover(
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection", // Collection name
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(718.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(200.into()))
    ///                     .negative(Example::Id(300.into()))
    ///                     .build(),
    ///             ],
    ///             10,              // Search limit
    ///         )
    ///         .target(Target::Single(VectorExample {
    ///             example: Some(Example::Vector(vec![0.2, 0.1, 0.9, 0.7].into())),
    ///         })),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#discovery-api>
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

    /// Batch multiple point discoveries in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{
    ///     vector_example::Example, ContextExamplePairBuilder, DiscoverBatchPointsBuilder,
    ///     DiscoverPointsBuilder,
    /// };
    ///
    ///# async fn discover_batch(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let discover_points = DiscoverBatchPointsBuilder::new(
    ///     "my_collection",
    ///     vec![
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(718.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(200.into()))
    ///                     .negative(Example::Id(300.into()))
    ///                     .build(),
    ///             ],
    ///             10,
    ///         )
    ///         .build(),
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(342.into()))
    ///                     .negative(Example::Id(213.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(200.into()))
    ///                     .build(),
    ///             ],
    ///             10,
    ///         )
    ///         .build(),
    ///     ],
    /// );
    ///
    /// client.discover_batch(&discover_points.build()).await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#discovery-api>
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

    /// Count points in a collection.
    ///
    /// Use [`exact`](crate::qdrant::CountPointsBuilder::exact) to specify whether to use exact
    /// counting. Exact counting is more accurate but slower.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, CountPointsBuilder, Filter};
    ///
    ///# async fn count(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .count(
    ///         CountPointsBuilder::new("collection_name")
    ///             .filter(Filter::must([Condition::matches(
    ///                 "color",
    ///                 "red".to_string(),
    ///             )]))
    ///             .exact(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#counting-points>
    pub async fn count(&self, request: impl Into<CountPoints>) -> QdrantResult<CountResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.count(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Batch point updates in a collection.
    ///
    /// Execute a batch of point [updates](crate::qdrant::points_update_operation::Operation) in a single operation.
    ///
    /// ```no_run
    ///# use std::collections::HashMap;
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::Payload;
    /// use qdrant_client::qdrant::{
    ///     points_selector::PointsSelectorOneOf,
    ///     points_update_operation::{
    ///         Operation, OverwritePayload, PointStructList, UpdateVectors,
    ///     },
    ///     PointStruct, PointVectors, PointsIdsList, PointsSelector, PointsUpdateOperation,
    ///     UpdateBatchPointsBuilder,
    /// };
    /// use serde_json::json;
    ///
    ///# async fn update_batch_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .update_points_batch(
    ///         UpdateBatchPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 PointsUpdateOperation {
    ///                     operation: Some(Operation::Upsert(PointStructList {
    ///                         points: vec![PointStruct::new(
    ///                             1,
    ///                             vec![1.0, 2.0, 3.0, 4.0],
    ///                             Payload::try_from(json!({})).unwrap(),
    ///                         )],
    ///                         ..Default::default()
    ///                     })),
    ///                 },
    ///                 PointsUpdateOperation {
    ///                     operation: Some(Operation::UpdateVectors(UpdateVectors {
    ///                         points: vec![PointVectors {
    ///                             id: Some(1.into()),
    ///                             vectors: Some(vec![1.0, 2.0, 3.0, 4.0].into()),
    ///                         }],
    ///                         ..Default::default()
    ///                     })),
    ///                 },
    ///                 PointsUpdateOperation {
    ///                     operation: Some(Operation::OverwritePayload(OverwritePayload {
    ///                         points_selector: Some(PointsSelector {
    ///                             points_selector_one_of: Some(PointsSelectorOneOf::Points(
    ///                                 PointsIdsList {
    ///                                     ids: vec![1.into()],
    ///                                 },
    ///                             )),
    ///                         }),
    ///                         payload: HashMap::from([("test_payload".to_string(), 1.into())]),
    ///                         ..Default::default()
    ///                     })),
    ///                 },
    ///             ],
    ///         )
    ///         .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#batch-update>
    pub async fn update_points_batch(
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

    /// Create payload index in a collection.
    ///
    /// ```no_run
    ///# use std::collections::HashMap;
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{CreateFieldIndexCollectionBuilder, FieldType};
    ///
    ///# async fn create_field_index(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .create_field_index(
    ///         CreateFieldIndexCollectionBuilder::new(
    ///             "my_collection",
    ///             "city",
    ///             FieldType::Keyword,
    ///         ),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/indexing/#payload-index>
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

    /// Delete payload index from a collection.
    ///
    /// ```no_run
    ///# use std::collections::HashMap;
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::DeleteFieldIndexCollectionBuilder;
    ///
    ///# async fn create_field_index(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_field_index(DeleteFieldIndexCollectionBuilder::new(
    ///         "my_collection",
    ///         "city",
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/indexing/#payload-index>
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
