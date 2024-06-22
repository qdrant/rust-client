use crate::auth::TokenInterceptor;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::{
    CreateFullSnapshotRequest, CreateSnapshotRequest, CreateSnapshotResponse,
    DeleteFullSnapshotRequest, DeleteSnapshotRequest, DeleteSnapshotResponse,
    ListFullSnapshotsRequest, ListSnapshotsRequest, ListSnapshotsResponse,
};
use crate::qdrant_client::{Qdrant, Result};
use std::future::Future;
use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

impl Qdrant {
    pub async fn with_snapshot_client<T, O: Future<Output = std::result::Result<T, Status>>>(
        &self,
        f: impl Fn(SnapshotsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T> {
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

    pub async fn create_snapshot(
        &self,
        request: impl Into<CreateSnapshotRequest>,
    ) -> Result<CreateSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.create(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn list_snapshots(
        &self,
        request: impl Into<ListSnapshotsRequest>,
    ) -> Result<ListSnapshotsResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.list(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_snapshot(
        &self,
        request: impl Into<DeleteSnapshotRequest>,
    ) -> Result<DeleteSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.delete(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn create_full_snapshot(&self) -> Result<CreateSnapshotResponse> {
        self.with_snapshot_client(|mut client| async move {
            let result = client.create_full(CreateFullSnapshotRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn list_full_snapshots(&self) -> Result<ListSnapshotsResponse> {
        self.with_snapshot_client(|mut client| async move {
            let result = client.list_full(ListFullSnapshotsRequest {}).await?;
            Ok(result.into_inner())
        })
        .await
    }

    pub async fn delete_full_snapshot(
        &self,
        request: impl Into<DeleteFullSnapshotRequest>,
    ) -> Result<DeleteSnapshotResponse> {
        let request = &request.into();
        self.with_snapshot_client(|mut client| async move {
            let result = client.delete_full(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    #[cfg(feature = "download_snapshots")]
    pub async fn download_snapshot(
        &self,
        options: impl Into<crate::qdrant::SnapshotDownload>,
    ) -> Result<()> {
        use crate::qdrant_client::error::QdrantError;
        use futures_util::StreamExt;
        use std::io::Write;

        let options = options.into();

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
            "{}/collections/{}/snapshots/{}",
            options
                .rest_api_uri
                .as_ref()
                .map(|uri| uri.to_string())
                .unwrap_or_else(|| String::from("http://localhost:6333")),
            options.collection_name,
            snapshot_name
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
