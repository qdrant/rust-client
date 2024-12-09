use rustc_version::version_meta;
use tonic::service::Interceptor;
use tonic::{Request, Status};

pub struct UserAgentInterceptor {
    rust_version: Option<String>,
    rust_client_version: Option<String>,
}

impl Default for UserAgentInterceptor {
    fn default() -> Self {
        Self::new()
    }
}

impl UserAgentInterceptor {
    pub fn new() -> Self {
        let rust_version = Some(version_meta().unwrap().semver.to_string());
        let rust_client_version = Some(env!("CARGO_PKG_VERSION").to_string());

        Self {
            rust_version,
            rust_client_version,
        }
    }
}

impl Interceptor for UserAgentInterceptor {
    fn call(&mut self, mut req: Request<()>) -> Result<Request<()>, Status> {
        let user_agent_value = format!(
            "rust-client/{} rust/{}",
            self.rust_version.clone().unwrap_or_default(),
            self.rust_client_version.clone().unwrap_or_default()
        );
        req.metadata_mut().insert(
            "x-user-agent",
            user_agent_value.parse().map_err(|_| {
                Status::invalid_argument(format!(
                    "Malformed user-agent value: {}",
                    user_agent_value
                ))
            })?,
        );
        Ok(req)
    }
}
