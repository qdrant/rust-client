use std::future::Future;
#[cfg(feature = "download_snapshots")]
use std::path::PathBuf;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::client::QdrantClient;
use crate::qdrant::snapshots_client::SnapshotsClient;
use crate::qdrant::{
    CreateFullSnapshotRequest, CreateSnapshotRequest, CreateSnapshotResponse,
    DeleteFullSnapshotRequest, DeleteSnapshotRequest, DeleteSnapshotResponse,
    ListFullSnapshotsRequest, ListSnapshotsRequest, ListSnapshotsResponse,
};

impl QdrantClient {
    pub async fn with_snapshot_client<T, O: Future<Output = anyhow::Result<T, Status>>>(
        &self,
        f: impl Fn(SnapshotsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> anyhow::Result<T, Status> {
        self.channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        SnapshotsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
                        client = client
                            .send_compressed(compression.into())
                            .accept_compressed(compression.into());
                    }
                    f(client)
                },
                false,
            )
            .await
    }

    pub async fn create_snapshot(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<CreateSnapshotResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .create(CreateSnapshotRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_snapshots(
        &self,
        collection_name: impl ToString,
    ) -> anyhow::Result<ListSnapshotsResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .list(ListSnapshotsRequest {
                        collection_name: collection_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_snapshot(
        &self,
        collection_name: impl ToString,
        snapshot_name: impl ToString,
    ) -> anyhow::Result<DeleteSnapshotResponse> {
        let collection_name = collection_name.to_string();
        let collection_name_ref = collection_name.as_str();
        let snapshot_name = snapshot_name.to_string();
        let snapshot_name_ref = snapshot_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .delete(DeleteSnapshotRequest {
                        collection_name: collection_name_ref.to_string(),
                        snapshot_name: snapshot_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn create_full_snapshot(&self) -> anyhow::Result<CreateSnapshotResponse> {
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client.create_full(CreateFullSnapshotRequest {}).await?;

                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn list_full_snapshots(&self) -> anyhow::Result<ListSnapshotsResponse> {
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client.list_full(ListFullSnapshotsRequest {}).await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    pub async fn delete_full_snapshot(
        &self,
        snapshot_name: impl ToString,
    ) -> anyhow::Result<DeleteSnapshotResponse> {
        let snapshot_name = snapshot_name.to_string();
        let snapshot_name_ref = snapshot_name.as_str();
        Ok(self
            .with_snapshot_client(|mut client| async move {
                let result = client
                    .delete_full(DeleteFullSnapshotRequest {
                        snapshot_name: snapshot_name_ref.to_string(),
                    })
                    .await?;
                Ok(result.into_inner())
            })
            .await?)
    }

    #[cfg(feature = "download_snapshots")]
    pub async fn download_snapshot<T>(
        &self,
        out_path: impl Into<PathBuf>,
        collection_name: T,
        snapshot_name: Option<T>,
        rest_api_uri: Option<T>,
    ) -> anyhow::Result<()>
    where
        T: ToString + Clone,
    {
        use std::io::Write;

        use futures_util::StreamExt;

        let snapshot_name = match snapshot_name {
            Some(sn) => sn.to_string(),
            _ => match self
                .list_snapshots(collection_name.clone())
                .await?
                .snapshot_descriptions
                .first()
            {
                Some(sn) => sn.name.clone(),
                _ => anyhow::bail!(
                    "No snapshots found for collection {}",
                    collection_name.to_string()
                ),
            },
        };

        let mut stream = reqwest::get(format!(
            "{}/collections/{}/snapshots/{}",
            rest_api_uri
                .map(|uri| uri.to_string())
                .unwrap_or_else(|| String::from("http://localhost:6333")),
            collection_name.to_string(),
            snapshot_name
        ))
        .await?
        .bytes_stream();

        let out_path = out_path.into();
        let _ = std::fs::remove_file(&out_path);
        let mut file = std::fs::OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(out_path)?;

        while let Some(chunk) = stream.next().await {
            let _written = file.write(&chunk?)?;
        }

        Ok(())
    }
}
