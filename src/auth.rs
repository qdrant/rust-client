use tonic::service::Interceptor;
use tonic::{Request, Status};

pub struct TokenInterceptor {
    api_key: Option<String>,
}

impl TokenInterceptor {
    pub fn new(api_key: Option<String>) -> Self {
        Self { api_key }
    }
}

impl Interceptor for TokenInterceptor {
    fn call(&mut self, mut req: Request<()>) -> anyhow::Result<Request<()>, Status> {
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
