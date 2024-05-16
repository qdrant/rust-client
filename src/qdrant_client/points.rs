use std::future::Future;

use tonic::codegen::InterceptedService;
use tonic::transport::Channel;
use tonic::Status;

use crate::auth::TokenInterceptor;
use crate::prelude::SearchPoints;
use crate::qdrant::points_client::PointsClient;
use crate::qdrant::SearchResponse;
use crate::qdrant_client::{Qdrant, Result};

impl Qdrant {
    async fn with_points_client<T, O: Future<Output = std::result::Result<T, Status>>>(
        &self,
        f: impl Fn(PointsClient<InterceptedService<Channel, TokenInterceptor>>) -> O,
    ) -> Result<T> {
        let result = self
            .channel
            .with_channel(
                |channel| {
                    let service = self.with_api_key(channel);
                    let mut client =
                        PointsClient::new(service).max_decoding_message_size(usize::MAX);
                    if let Some(compression) = self.cfg.compression {
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

    pub async fn search_points(&self, request: impl Into<SearchPoints>) -> Result<SearchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
