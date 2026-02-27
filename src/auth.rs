use tonic::metadata::MetadataKey;
use tonic::service::Interceptor;
use tonic::{Request, Status};

/// Header name used for API key / token authentication.
pub const API_KEY_HEADER: &str = "api-key";

pub struct MetadataInterceptor {
    api_key: Option<String>,
    custom_headers: Vec<(String, String)>,
}

impl MetadataInterceptor {
    pub fn new(api_key: Option<String>, custom_headers: Vec<(String, String)>) -> Self {
        Self {
            api_key,
            custom_headers,
        }
    }
}

impl Interceptor for MetadataInterceptor {
    fn call(&mut self, mut req: Request<()>) -> anyhow::Result<Request<()>, Status> {
        if let Some(api_key) = &self.api_key {
            req.metadata_mut().insert(
                API_KEY_HEADER,
                api_key.parse().map_err(|_| {
                    Status::invalid_argument(format!("Malformed API key or token: {api_key}"))
                })?,
            );
        }
        for (key, value) in &self.custom_headers {
            let key = MetadataKey::from_bytes(key.as_bytes())
                .map_err(|_| Status::invalid_argument(format!("Malformed header name: {key}")))?;
            let value = value.parse().map_err(|_| {
                Status::invalid_argument(format!("Malformed header value for {key}: {value}"))
            })?;
            req.metadata_mut().insert(key, value);
        }
        Ok(req)
    }
}
