use std::future::Future;
use std::marker::PhantomData;

use prost::bytes::{BufMut, Bytes};
use prost::Message;
use tonic::client::Grpc;
use tonic::codec::{Codec, EncodeBuf, Encoder};
use tonic::codegen::http::uri::PathAndQuery;
use tonic::codegen::{GrpcMethod, InterceptedService};
use tonic::transport::Channel;
use tonic::{Request, Status};
use tonic_prost::ProstCodec;

use crate::auth::MetadataInterceptor;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::{
    CountPoints, CountResponse, CreateVectorNameRequest, DeletePointVectors, DeletePoints,
    DeleteVectorNameRequest, FacetCounts, FacetResponse, GetPoints, GetResponse,
    PointsOperationResponse, ScrollPoints, ScrollResponse, SearchMatrixOffsetsResponse,
    SearchMatrixPairsResponse, SearchMatrixPoints, UpdateBatchPoints, UpdateBatchResponse,
    UpdatePointVectors, UpsertPoints, Usage,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

#[derive(Clone, Copy)]
struct PreEncodedCodec<Resp>(PhantomData<fn() -> Resp>);

impl<Resp> Default for PreEncodedCodec<Resp> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

#[derive(Clone, Copy, Default)]
struct PreEncodedEncoder;

impl Encoder for PreEncodedEncoder {
    type Item = Bytes;
    type Error = Status;

    fn encode(&mut self, item: Self::Item, dst: &mut EncodeBuf<'_>) -> Result<(), Self::Error> {
        dst.put_slice(&item);
        Ok(())
    }
}

impl<Resp> Codec for PreEncodedCodec<Resp>
where
    Resp: prost::Message + Default + Send + 'static,
{
    type Encode = Bytes;
    type Decode = Resp;
    type Encoder = PreEncodedEncoder;
    type Decoder = <ProstCodec<Resp, Resp> as Codec>::Decoder;

    fn encoder(&mut self) -> Self::Encoder {
        PreEncodedEncoder
    }

    fn decoder(&mut self) -> Self::Decoder {
        ProstCodec::<Resp, Resp>::default().decoder()
    }
}

/// # Point operations
///
/// Manage points and vectors.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/points/>
impl Qdrant {
    pub(crate) async fn with_points_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, MetadataInterceptor>>) -> O,
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
    fn points_grpc_client(
        &self,
        channel: Channel,
    ) -> Grpc<InterceptedService<Channel, MetadataInterceptor>> {
        let service = self.with_api_key(channel);
        let mut client = Grpc::new(service).max_decoding_message_size(usize::MAX);
        if let Some(compression) = self.config.compression {
            client = client
                .send_compressed(compression.into())
                .accept_compressed(compression.into());
        }
        client
    }

    async fn send_pre_encoded_upsert(
        grpc: &mut Grpc<InterceptedService<Channel, MetadataInterceptor>>,
        encoded_request: Bytes,
    ) -> Result<PointsOperationResponse, Status> {
        grpc.ready()
            .await
            .map_err(|e| Status::unknown(format!("Service was not ready: {e}")))?;
        let path = PathAndQuery::from_static("/qdrant.Points/Upsert");
        let mut req = Request::new(encoded_request);
        req.extensions_mut()
            .insert(GrpcMethod::new("qdrant.Points", "Upsert"));
        Ok(grpc
            .unary(
                req,
                path,
                PreEncodedCodec::<PointsOperationResponse>::default(),
            )
            .await?
            .into_inner())
    }

    /// Documentation: <https://qdrant.tech/documentation/concepts/points/#upload-points>
    pub async fn upsert_points(
        &self,
        request: impl Into<UpsertPoints>,
    ) -> QdrantResult<PointsOperationResponse> {
        // Encode once before entering the retry closure so retries reuse the Bytes buffer.
        let encoded_request = &Bytes::from(request.into().encode_to_vec());
        Ok(self
            .channel
            .with_channel(
                |channel| {
                    let mut grpc = self.points_grpc_client(channel);
                    let encoded = encoded_request.clone();
                    async move { Self::send_pre_encoded_upsert(&mut grpc, encoded).await }
                },
                true,
            )
            .await?)
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

        assert!(chunk_size != 0, "chunk size must be non-zero");

        let points = std::mem::take(&mut request.points);
        let mut encoded_chunks = Vec::with_capacity(points.len().div_ceil(chunk_size));
        let mut remaining = points.into_iter();
        loop {
            request.points.extend(remaining.by_ref().take(chunk_size));
            if request.points.is_empty() {
                break;
            }
            encoded_chunks.push(Bytes::from(request.encode_to_vec()));
            request.points.clear();
        }
        let encoded_chunks = &encoded_chunks;

        Ok(self
            .channel
            .with_channel(
                |channel| {
                    let mut grpc = self.points_grpc_client(channel);
                    async move {
                        let mut resp = PointsOperationResponse {
                            result: None,
                            time: 0.0,
                            usage: None,
                        };

                        for chunk in encoded_chunks {
                            let PointsOperationResponse {
                                result,
                                time,
                                usage,
                            } = Self::send_pre_encoded_upsert(&mut grpc, chunk.clone()).await?;

                            resp.result = result;
                            resp.time += time;
                            resp.usage = Usage::aggregate_opts(resp.usage, usage);
                        }

                        Ok(resp)
                    }
                },
                true,
            )
            .await?)
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

    /// Create a new named vector on an existing collection.
    ///
    /// Adds a new dense or sparse vector to the collection's vector configuration.
    /// Existing points will not have data for the new vector until it is set via
    /// [`update_vectors`](Self::update_vectors) or another upsert.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{
    ///     CreateVectorNameRequestBuilder, DenseVectorCreationConfigBuilder, Distance,
    /// };
    ///
    ///# async fn create_vector_name(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .create_vector_name(
    ///         CreateVectorNameRequestBuilder::new(
    ///             "my_collection",
    ///             "image",
    ///             DenseVectorCreationConfigBuilder::new(512, Distance::Cosine),
    ///         )
    ///         .wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn create_vector_name(
        &self,
        request: impl Into<CreateVectorNameRequest>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.create_vector_name(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete a named vector from an existing collection.
    ///
    /// Removes the named vector and all its data across all points. This operation
    /// cannot be undone.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::DeleteVectorNameRequestBuilder;
    ///
    ///# async fn delete_vector_name(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_vector_name(
    ///         DeleteVectorNameRequestBuilder::new("my_collection", "image").wait(true),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    pub async fn delete_vector_name(
        &self,
        request: impl Into<DeleteVectorNameRequest>,
    ) -> QdrantResult<PointsOperationResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.delete_vector_name(request.clone()).await?;
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

#[cfg(test)]
mod tests {
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};

    use tonic::transport::server::TcpIncoming;
    use tonic::transport::Server;
    use tonic::{Request, Response, Status};

    use super::*;
    use crate::qdrant::points_server::{Points, PointsServer};
    use crate::qdrant::{
        ClearPayloadPoints, CountPoints, CountResponse, CreateFieldIndexCollection,
        CreateVectorNameRequest, DeleteFieldIndexCollection, DeletePayloadPoints,
        DeletePointVectors, DeletePoints, DeleteVectorNameRequest, DiscoverBatchPoints,
        DiscoverBatchResponse, DiscoverPoints, DiscoverResponse, FacetCounts, FacetResponse,
        GetPoints, GetResponse, PointStruct, PointsOperationResponse, QueryBatchPoints,
        QueryBatchResponse, QueryGroupsResponse, QueryPointGroups, QueryPoints, QueryResponse,
        RecommendBatchPoints, RecommendBatchResponse, RecommendGroupsResponse,
        RecommendPointGroups, RecommendPoints, RecommendResponse, ScrollPoints, ScrollResponse,
        SearchBatchPoints, SearchBatchResponse, SearchGroupsResponse, SearchMatrixPoints,
        SearchPointGroups, SearchPoints, SearchResponse, SetPayloadPoints, UpdateBatchPoints,
        UpdateBatchResponse, UpdatePointVectors, UpsertPoints, UpsertPointsBuilder,
    };

    #[derive(Clone)]
    struct MockPoints {
        attempts: Arc<AtomicUsize>,
        fail_first: bool,
        received: Arc<Mutex<Vec<UpsertPoints>>>,
    }

    #[tonic::async_trait]
    impl Points for MockPoints {
        async fn upsert(
            &self,
            request: Request<UpsertPoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            let attempt = self.attempts.fetch_add(1, Ordering::SeqCst);
            if self.fail_first && attempt == 0 {
                return Err(Status::unavailable("transient error"));
            }
            self.received.lock().unwrap().push(request.into_inner());
            Ok(Response::new(PointsOperationResponse {
                result: None,
                time: 0.25,
                usage: None,
            }))
        }

        async fn delete(
            &self,
            _request: Request<DeletePoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn get(&self, _request: Request<GetPoints>) -> Result<Response<GetResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn update_vectors(
            &self,
            _request: Request<UpdatePointVectors>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn delete_vectors(
            &self,
            _request: Request<DeletePointVectors>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn set_payload(
            &self,
            _request: Request<SetPayloadPoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn overwrite_payload(
            &self,
            _request: Request<SetPayloadPoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn delete_payload(
            &self,
            _request: Request<DeletePayloadPoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn clear_payload(
            &self,
            _request: Request<ClearPayloadPoints>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn create_field_index(
            &self,
            _request: Request<CreateFieldIndexCollection>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn delete_field_index(
            &self,
            _request: Request<DeleteFieldIndexCollection>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn create_vector_name(
            &self,
            _request: Request<CreateVectorNameRequest>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn delete_vector_name(
            &self,
            _request: Request<DeleteVectorNameRequest>,
        ) -> Result<Response<PointsOperationResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn search(
            &self,
            _request: Request<SearchPoints>,
        ) -> Result<Response<SearchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn search_batch(
            &self,
            _request: Request<SearchBatchPoints>,
        ) -> Result<Response<SearchBatchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn search_groups(
            &self,
            _request: Request<SearchPointGroups>,
        ) -> Result<Response<SearchGroupsResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn scroll(
            &self,
            _request: Request<ScrollPoints>,
        ) -> Result<Response<ScrollResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn recommend(
            &self,
            _request: Request<RecommendPoints>,
        ) -> Result<Response<RecommendResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn recommend_batch(
            &self,
            _request: Request<RecommendBatchPoints>,
        ) -> Result<Response<RecommendBatchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn recommend_groups(
            &self,
            _request: Request<RecommendPointGroups>,
        ) -> Result<Response<RecommendGroupsResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn discover(
            &self,
            _request: Request<DiscoverPoints>,
        ) -> Result<Response<DiscoverResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn discover_batch(
            &self,
            _request: Request<DiscoverBatchPoints>,
        ) -> Result<Response<DiscoverBatchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn count(
            &self,
            _request: Request<CountPoints>,
        ) -> Result<Response<CountResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn update_batch(
            &self,
            _request: Request<UpdateBatchPoints>,
        ) -> Result<Response<UpdateBatchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn query(
            &self,
            _request: Request<QueryPoints>,
        ) -> Result<Response<QueryResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn query_batch(
            &self,
            _request: Request<QueryBatchPoints>,
        ) -> Result<Response<QueryBatchResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn query_groups(
            &self,
            _request: Request<QueryPointGroups>,
        ) -> Result<Response<QueryGroupsResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn facet(
            &self,
            _request: Request<FacetCounts>,
        ) -> Result<Response<FacetResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn search_matrix_pairs(
            &self,
            _request: Request<SearchMatrixPoints>,
        ) -> Result<Response<SearchMatrixPairsResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }

        async fn search_matrix_offsets(
            &self,
            _request: Request<SearchMatrixPoints>,
        ) -> Result<Response<SearchMatrixOffsetsResponse>, Status> {
            Err(Status::unimplemented("not used in test"))
        }
    }

    async fn spawn_mock_points(
        fail_first: bool,
    ) -> (String, Arc<AtomicUsize>, Arc<Mutex<Vec<UpsertPoints>>>) {
        let incoming = TcpIncoming::bind("127.0.0.1:0".parse().unwrap()).unwrap();
        let addr = incoming.local_addr().unwrap();
        let attempts = Arc::new(AtomicUsize::new(0));
        let received = Arc::new(Mutex::new(Vec::new()));

        let service = PointsServer::new(MockPoints {
            attempts: Arc::clone(&attempts),
            fail_first,
            received: Arc::clone(&received),
        });

        tokio::spawn(async move {
            Server::builder()
                .add_service(service)
                .serve_with_incoming(incoming)
                .await
                .unwrap();
        });

        (format!("http://{addr}"), attempts, received)
    }

    #[tokio::test]
    async fn upsert_points_pre_encoded_preserves_payload_and_retries() {
        let (uri, attempts, received) = spawn_mock_points(true).await;
        let client = Qdrant::from_url(&uri)
            .skip_compatibility_check()
            .build()
            .unwrap();

        let points = vec![
            PointStruct::new(1, vec![1.0, 2.0, 3.0], [("city", "Berlin".into())]),
            PointStruct::new(2, vec![4.0, 5.0, 6.0], [("city", "Tokyo".into())]),
        ];

        let response = client
            .upsert_points(UpsertPointsBuilder::new("test_col", points.clone()).wait(true))
            .await
            .unwrap();

        assert_eq!(attempts.load(Ordering::SeqCst), 2);
        assert_eq!(response.time, 0.25);

        let guard = received.lock().unwrap();
        assert_eq!(guard.len(), 1);
        assert_eq!(guard[0].collection_name, "test_col");
        assert_eq!(guard[0].wait, Some(true));
        assert_eq!(guard[0].points, points);
    }

    #[tokio::test]
    async fn upsert_points_chunked_splits_without_cloning_and_aggregates_time() {
        let (uri, _attempts, received) = spawn_mock_points(false).await;
        let client = Qdrant::from_url(&uri)
            .skip_compatibility_check()
            .build()
            .unwrap();

        let points: Vec<PointStruct> = (1..=5_u64)
            .map(|id| PointStruct::new(id, vec![id as f32, 0.5], [("city", "Paris".into())]))
            .collect();

        let response = client
            .upsert_points_chunked(
                UpsertPointsBuilder::new("chunk_col", points.clone()).wait(true),
                2,
            )
            .await
            .unwrap();

        assert_eq!(response.time, 0.75);

        let guard = received.lock().unwrap();
        let chunk_sizes: Vec<usize> = guard.iter().map(|req| req.points.len()).collect();
        assert_eq!(chunk_sizes, vec![2, 2, 1]);
        assert!(guard.iter().all(|req| req.collection_name == "chunk_col"));
        assert!(guard.iter().all(|req| req.wait == Some(true)));

        let reassembled: Vec<PointStruct> =
            guard.iter().flat_map(|req| req.points.clone()).collect();
        assert_eq!(reassembled, points);
    }
}
