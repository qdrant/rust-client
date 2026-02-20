use tonic::service::Interceptor;

use crate::qdrant::{
    ClearPayloadPoints, DeletePayloadPoints, PointsOperationResponse, SetPayloadPoints,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

/// # Payload operations
///
/// Manage point payloads.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/payload/>
impl<I: Send + Sync + 'static + Clone + Interceptor> Qdrant<I> {
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
}
