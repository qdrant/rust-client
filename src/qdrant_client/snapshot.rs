use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::{
    CreateFullSnapshotRequest, CreateSnapshotRequest, CreateSnapshotResponse,
    DeleteFullSnapshotRequest, DeleteSnapshotRequest, DeleteSnapshotResponse,
    ListFullSnapshotsRequest, ListSnapshotsRequest, ListSnapshotsResponse,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

/// # Snapshot operations
///
/// Create, recover and manage snapshots for collections or a full Qdrant instance.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/>
impl Qdrant {
    async fn with_snapshot_client<T, O: Future<Output = Result<T, Status>>>(
        &self,
        f: impl Fn(SnapshotsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> QdrantResult<T> {
        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        SnapshotsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.config.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                false,
            )
            .await?;
        Ok(result)
    }

    /// Create snapshot of a collection on this node.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn create_snapshot(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.create_snapshot("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Note: Snapshots are node-local. They only contain data of a single node. In distributed
    /// mode you must create a snapshot on each node separately. Each node has their own list of
    /// snapshots.
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#create-snapshot>
    pub async fn create_snapshot(
        &self,
        request: impl Into<CreateSnapshotRequest>,
    ) -> QdrantResult<CreateSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.create(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// List collection snapshots on this node.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_snapshots(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.list_snapshots("my_collection").await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Note: Snapshots are node-local. They only contain data of a single node. In distributed
    /// mode you must create a snapshot on each node separately. Each node has their own list of
    /// snapshots.
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#list-snapshot>
    pub async fn list_snapshots(
        &self,
        request: impl Into<ListSnapshotsRequest>,
    ) -> QdrantResult<ListSnapshotsResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.list(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete a collection snapshot on this node.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::DeleteSnapshotRequestBuilder;
    ///
    ///# async fn delete_snapshot(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_snapshot(DeleteSnapshotRequestBuilder::new(
    ///         "my_collection",
    ///         "snapshot_name",
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Note: Snapshots are node-local. They only contain data of a single node. In distributed
    /// mode you must create a snapshot on each node separately. Each node has their own list of
    /// snapshots.
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#delete-snapshot>
    pub async fn delete_snapshot(
        &self,
        request: impl Into<DeleteSnapshotRequest>,
    ) -> QdrantResult<DeleteSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.delete(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Create full snapshot of this entire node.
    ///
    /// <div class="warning">Only supported in single-node deployment. Multi-node (<a href="https://qdrant.tech/documentation/guides/distributed_deployment/">distributed</a>) mode is not supported.</div>
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn create_snapshot(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.create_full_snapshot().await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#create-full-storage-snapshot>
    pub async fn create_full_snapshot(&self) -> QdrantResult<CreateSnapshotResponse> {
        self.with_snapshot_client(|mut client| async move {
            let result = client.create_full(CreateFullSnapshotRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// List full snapshots of this node.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn list_full_snapshots(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client.list_full_snapshots().await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#list-full-storage-snapshots>
    pub async fn list_full_snapshots(&self) -> QdrantResult<ListSnapshotsResponse> {
        self.with_snapshot_client(|mut client| async move {
            let result = client.list_full(ListFullSnapshotsRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Delete full snapshots of this node.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    ///# async fn delete_snapshot(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .delete_full_snapshot("snapshot_name")
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#delete-full-storage-snapshot>
    pub async fn delete_full_snapshot(
        &self,
        request: impl Into<DeleteFullSnapshotRequest>,
    ) -> QdrantResult<DeleteSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.delete_full(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Download a collection snapshot on this node.
    ///
    /// ```no_run
    ///# use std::fs::File;
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::SnapshotDownloadBuilder;
    ///
    ///# async fn download_snapshot(client: &Qdrant)
    ///# -> Result<File, QdrantError> {
    /// client.download_snapshot(
    ///     SnapshotDownloadBuilder::new("./target_path.snapshot", "my_collection")
    ///         .snapshot_name("snapshot_name")
    ///         .rest_api_uri("http://localhost:6333")
    /// ).await?;
    ///
    /// let snapshot_file = File::open("./target_path.snapshot")?;
    ///# Ok(snapshot_file)
    ///# }
    /// ```
    ///
    /// Note: Snapshots are node-local. They only contain data of a single node. In distributed
    /// mode you must create a snapshot on each node separately. Each node has their own list of
    /// snapshots.
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/snapshots/#retrieve-snapshot>
    #[cfg(feature = "download_snapshots")]
    pub async fn download_snapshot(
        &self,
        download: impl Into<crate::qdrant::SnapshotDownload>,
    ) -> QdrantResult<()> {
        use std::io::Write;

        use futures_util::StreamExt;

        use crate::qdrant_client::error::QdrantError;

        let options = download.into();

        let snapshot_name = match &options.snapshot_name {
            Some(sn) => sn.to_string(),
            _ => match self
                .list_snapshots(options.collection_name.clone())
                .await?
                .snapshot_descriptions
                .first()
            {
                Some(sn) => sn.name.clone(),
                _ => {
                    return Err(QdrantError::NoSnapshotFound(
                        options.collection_name.clone(),
                    ))
                }
            },
        };

        let mut stream = reqwest::get(format!(
            "{}/collections/{}/snapshots/{snapshot_name}",
            options
                .rest_api_uri
                .as_ref()
                .map(|uri| uri.to_string())
                .unwrap_or_else(|| String::from("http://localhost:6333")),
            options.collection_name,
        ))
        .await?
        .bytes_stream();

        let _ = std::fs::remove_file(&options.out_path);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&options.out_path)?;

        while let Some(chunk) = stream.next().await {
            let _written = file.write(&chunk?)?;
        }

        Ok(())
    }
}
