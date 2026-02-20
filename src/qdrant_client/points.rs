use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::service::Interceptor;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::WrappedInterceptor;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::{
    CountPoints, CountResponse, DeletePointVectors, DeletePoints, FacetCounts, FacetResponse,
    GetPoints, GetResponse, PointsOperationResponse, ScrollPoints, ScrollResponse,
    SearchMatrixOffsetsResponse, SearchMatrixPairsResponse, SearchMatrixPoints, UpdateBatchPoints,
    UpdateBatchResponse, UpdatePointVectors, UpsertPoints, Usage,
};
use crate::qdrant_client::{GenericQdrant, QdrantResult};

/// # Point operations
///
/// Manage points and vectors.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/points/>
impl<I: Send + Sync + 'static + Clone + Interceptor> GenericQdrant<I> {
    pub(crate) async fn with_points_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, WrappedInterceptor<I>>>) -> O,
    ) -> QdrantResult<T> {
        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_interceptor(channel);
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

        if request.points.len() < chunk_size {
            return self.upsert_points(request).await;
        }

        let points = &std::mem::take(&mut request.points);
        let request = &request;

        self.with_points_client(|mut points_api| async move {
            let mut resp = PointsOperationResponse {
                result: None,
                time: 0.0,
                usage: None,
            };

            for chunk in points.clone().chunks(chunk_size) {
                let mut chunked_request = request.clone();
                chunked_request.points = chunk.to_vec();

                let PointsOperationResponse {
                    result,
                    time,
                    usage,
                } = points_api.upsert(chunked_request).await?.into_inner();

                resp.result = result;
                resp.time += time;
                resp.usage = Usage::aggregate_opts(resp.usage, usage);
            }

            Ok(resp)
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

    /// Batch point operations in a collection.
    ///
    /// Perform a batch of point [`Operation`](crate::qdrant::points_update_operation::Operation)s in a single request.
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
    ///# async fn update_points_batch(client: &Qdrant)
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

    /// Get the amount of records for each unique value of a field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, FacetCountsBuilder, Filter};
    ///
    ///# async fn facets(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let ten_countries_with_most_points_in_europe = client
    ///    .facet(
    ///         FacetCountsBuilder::new("world_data", "country")
    ///             .limit(10)
    ///             .filter(Filter::must(vec![Condition::matches(
    ///                 "continent",
    ///                 "Europe".to_string(),
    ///             )])),
    ///     )
    ///     .await
    ///     .unwrap();
    ///# Ok(())
    ///# }
    /// ```
    pub async fn facet(&self, request: impl Into<FacetCounts>) -> QdrantResult<FacetResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.facet(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Get a (sparse) matrix of points with closest distance, returned as pairs.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, SearchMatrixPointsBuilder, Filter};
    ///
    ///# async fn search_matrix_pairs(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let matrix = client
    ///     .search_matrix_pairs(
    ///         SearchMatrixPointsBuilder::new("collection_name")
    ///             .filter(Filter::must(vec![Condition::matches(
    ///                 "color",
    ///                 "red".to_string(),
    ///             )]))
    ///             .sample(1000)
    ///             .limit(10),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn search_matrix_pairs(
        &self,
        request: impl Into<SearchMatrixPoints>,
    ) -> QdrantResult<SearchMatrixPairsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_matrix_pairs(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Get a (sparse) matrix of points with closest distance.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, SearchMatrixPointsBuilder, Filter};
    ///
    ///# async fn search_matrix_offsets(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let matrix = client
    ///     .search_matrix_offsets(
    ///         SearchMatrixPointsBuilder::new("collection_name")
    ///             .filter(Filter::must(vec![Condition::matches(
    ///                 "color",
    ///                 "red".to_string(),
    ///             )]))
    ///             .sample(1000)
    ///             .limit(10),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn search_matrix_offsets(
        &self,
        request: impl Into<SearchMatrixPoints>,
    ) -> QdrantResult<SearchMatrixOffsetsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_matrix_offsets(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
