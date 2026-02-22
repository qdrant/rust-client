use tonic::service::Interceptor;

use crate::qdrant::{
    CreateFieldIndexCollection, DeleteFieldIndexCollection, PointsOperationResponse,
};
use crate::qdrant_client::{GenericQdrant, QdrantResult};

/// # Index operations
///
/// Manage field and payload indices in collections.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/indexing/>
impl<I: Send + Sync + 'static + Clone + Interceptor> GenericQdrant<I> {
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
