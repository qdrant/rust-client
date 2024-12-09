use tonic::service::Interceptor;
use tonic::{Request, Status};

pub struct MetadataInterceptor {
    api_key: Option<String>,
}

impl MetadataInterceptor {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl Interceptor for MetadataInterceptor {
    fn call(&mut self, mut req: Request<()>) -> anyhow::Result<Request<()>, Status> {
        req.metadata_mut().insert( "x-user-agent", "TEST".parse().unwrap());

        if let Some(api_key) = &self.api_key {
            req.metadata_mut().insert(
                "api-key",
                api_key.parse().map_err(|_| {
                    Status::invalid_argument(format!("Malformed API key or token: {}", api_key))
                })?,
            );
        }
        Ok(req)
    }
}
