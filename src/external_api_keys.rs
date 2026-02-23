use std::collections::HashMap;

use tonic::metadata::{MetadataKey, MetadataValue};
use tonic::service::Interceptor;
use tonic::{Request, Status};

pub struct ExternalApiKeysInterceptor {
    external_api_keys: Option<HashMap<String, String>>,
}

impl ExternalApiKeysInterceptor {
    pub fn new(external_api_keys: Option<HashMap<String, String>>) -> Self {
        Self { external_api_keys }
    }
}

impl Interceptor for ExternalApiKeysInterceptor {
    fn call(&mut self, mut request: Request<()>) -> anyhow::Result<Request<()>, Status> {
        if let Some(ext_api_keys) = &self.external_api_keys {
            for (k, v) in ext_api_keys {
                let key = MetadataKey::from_bytes(k.as_bytes())
                    .map_err(|_| Status::invalid_argument(format!("Invalid metadata key: {k}")))?;
                let value = MetadataValue::try_from(v.as_str()).map_err(|_| {
                    Status::invalid_argument(format!("Invalid metadata value for {k}: {v}"))
                })?;
                request.metadata_mut().insert(key, value);
            }
        }
        Ok(request)
    }
}
