use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::{MetadataInterceptor, API_KEY_HEADER};
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
        f: impl Fn(SnapshotsClient<InterceptedService<Channel, MetadataInterceptor>>) -> O,
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

        let url = format!(
            "{}/collections/{}/snapshots/{snapshot_name}",
            options
                .rest_api_uri
                .as_ref()
                .map(|uri| uri.to_string())
                .unwrap_or_else(|| String::from("http://localhost:6333")),
            options.collection_name,
        );

        let client = reqwest::Client::new();
        let mut request = client.get(&url);
        if let Some(api_key) = &self.config.api_key {
            request = request.header(API_KEY_HEADER, api_key.as_str());
        }
        for (key, value) in &self.config.custom_headers {
            request = request.header(key.as_str(), value.as_str());
        }
        let mut stream = request.send().await?.error_for_status()?.bytes_stream();

        // Download into a temporary file next to the target, and only move it into place once
        // complete. A failed or cancelled download never touches an existing file at `out_path`.
        let mut tmp_path = options.out_path.clone().into_os_string();
        tmp_path.push(".part");
        let tmp_path = std::path::PathBuf::from(tmp_path);

        /// Removes the temporary file unless disarmed.
        struct RemoveOnDrop<'a>(Option<&'a std::path::Path>);
        impl Drop for RemoveOnDrop<'_> {
            fn drop(&mut self) {
                if let Some(path) = self.0 {
                    let _ = std::fs::remove_file(path);
                }
            }
        }

        let mut tmp_guard = RemoveOnDrop(Some(&tmp_path));
        let mut file = std::fs::File::create(&tmp_path)?;
        while let Some(chunk) = stream.next().await {
            file.write_all(&chunk?)?;
        }
        file.sync_all()?;
        drop(file);

        std::fs::rename(&tmp_path, &options.out_path)?;
        tmp_guard.0 = None;

        Ok(())
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
}

#[cfg(all(test, feature = "download_snapshots"))]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    use crate::qdrant::SnapshotDownloadBuilder;
    use crate::Qdrant;

    /// Serve a single HTTP response with the given status line and body.
    async fn serve_once(status: &'static str, body: &'static str) -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 4096];
            let _ = socket.read(&mut buf).await;
            let response = format!(
                "HTTP/1.1 {status}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
                body.len()
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });
        format!("http://{addr}")
    }

    #[tokio::test]
    async fn download_snapshot_http_error_keeps_existing_file() {
        let dir = std::env::temp_dir().join(format!("qdrant-dl-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let out_path = dir.join("existing.snapshot");
        std::fs::write(&out_path, b"previous snapshot").unwrap();

        let uri = serve_once("404 Not Found", "{\"status\":{\"error\":\"Not found\"}}").await;
        let client = Qdrant::from_url("http://localhost:6334")
            .skip_compatibility_check()
            .build()
            .unwrap();

        let result = client
            .download_snapshot(
                SnapshotDownloadBuilder::new(&out_path, "collection")
                    .snapshot_name("missing.snapshot")
                    .rest_api_uri(uri),
            )
            .await;

        assert!(result.is_err(), "HTTP 404 must be reported as an error");
        assert_eq!(std::fs::read(&out_path).unwrap(), b"previous snapshot");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[tokio::test]
    async fn download_snapshot_truncated_body_keeps_existing_file() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut buf = [0u8; 4096];
            let _ = socket.read(&mut buf).await;
            // Announce 1000 bytes but send only 100 before closing the connection
            let head = "HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\n";
            socket.write_all(head.as_bytes()).await.unwrap();
            socket.write_all(&[b'x'; 100]).await.unwrap();
            socket.flush().await.unwrap();
        });

        let dir = std::env::temp_dir().join(format!("qdrant-dl-trunc-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let out_path = dir.join("existing.snapshot");
        std::fs::write(&out_path, b"previous snapshot").unwrap();

        let client = Qdrant::from_url("http://localhost:6334")
            .skip_compatibility_check()
            .build()
            .unwrap();

        let result = client
            .download_snapshot(
                SnapshotDownloadBuilder::new(&out_path, "collection")
                    .snapshot_name("truncated.snapshot")
                    .rest_api_uri(format!("http://{addr}")),
            )
            .await;

        assert!(
            result.is_err(),
            "truncated body must be reported as an error"
        );
        assert_eq!(std::fs::read(&out_path).unwrap(), b"previous snapshot");
        assert!(
            !dir.join("existing.snapshot.part").exists(),
            "temporary file must be cleaned up"
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
