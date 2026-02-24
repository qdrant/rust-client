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
                // Treat empty values as "missing key" (e.g. absent env var), so requests can proceed.
                if v.trim().is_empty() {
                    continue;
                }
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use tonic::service::Interceptor;
    use tonic::Request;

    use super::ExternalApiKeysInterceptor;

    #[test]
    fn inserts_external_api_keys_into_metadata_headers() {
        let api_keys = HashMap::from([
            ("openai-api-key".to_string(), "openai-secret".to_string()),
            ("cohere-api-key".to_string(), "cohere-secret".to_string()),
        ]);

        let mut interceptor = ExternalApiKeysInterceptor::new(Some(api_keys));
        let request = interceptor
            .call(Request::new(()))
            .expect("interceptor must accept valid external API keys");

        let openai = request
            .metadata()
            .get("openai-api-key")
            .expect("missing openai-api-key header")
            .to_str()
            .expect("openai-api-key header must be valid ASCII");
        let cohere = request
            .metadata()
            .get("cohere-api-key")
            .expect("missing cohere-api-key header")
            .to_str()
            .expect("cohere-api-key header must be valid ASCII");

        assert_eq!(openai, "openai-secret");
        assert_eq!(cohere, "cohere-secret");
    }

    #[test]
    fn keeps_request_unchanged_when_external_keys_are_missing() {
        let mut interceptor = ExternalApiKeysInterceptor::new(None);
        let request = interceptor
            .call(Request::new(()))
            .expect("interceptor must accept empty external API key config");

        assert!(request.metadata().is_empty());
    }

    #[test]
    fn returns_invalid_argument_for_invalid_metadata_key() {
        let api_keys = HashMap::from([("openai api key".to_string(), "secret".to_string())]);

        let mut interceptor = ExternalApiKeysInterceptor::new(Some(api_keys));
        let error = interceptor
            .call(Request::new(()))
            .expect_err("interceptor must reject invalid metadata keys");

        assert_eq!(error.code(), tonic::Code::InvalidArgument);
        assert!(error.message().contains("Invalid metadata key"));
    }

    #[test]
    fn returns_invalid_argument_for_invalid_metadata_value() {
        let api_keys = HashMap::from([("openai-api-key".to_string(), "bad\nkey".to_string())]);

        let mut interceptor = ExternalApiKeysInterceptor::new(Some(api_keys));
        let error = interceptor
            .call(Request::new(()))
            .expect_err("interceptor must reject invalid metadata values");

        assert_eq!(error.code(), tonic::Code::InvalidArgument);
        assert!(error.message().contains("Invalid metadata value"));
    }

    #[test]
    fn skips_empty_external_api_key_values() {
        let api_keys = HashMap::from([
            ("openai-api-key".to_string(), "".to_string()),
            ("cohere-api-key".to_string(), "cohere-secret".to_string()),
        ]);

        let mut interceptor = ExternalApiKeysInterceptor::new(Some(api_keys));
        let request = interceptor
            .call(Request::new(()))
            .expect("interceptor must ignore empty external API key values");

        assert!(request.metadata().get("openai-api-key").is_none());
        assert_eq!(
            request
                .metadata()
                .get("cohere-api-key")
                .expect("cohere-api-key header must exist")
                .to_str()
                .expect("cohere-api-key header must be valid ASCII"),
            "cohere-secret"
        );
    }
}
