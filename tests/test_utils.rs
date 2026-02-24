//! Test utilities for integration testing with testcontainers
//!
//! Provides container setup for Qdrant vector database.
//! Uses tmpfs mounts for fast in-memory testing.

use std::env;
use std::sync::OnceLock;

use testcontainers::core::wait::HttpWaitStrategy;
use testcontainers::core::{IntoContainerPort, Mount, WaitFor};
use testcontainers::runners::AsyncRunner;
use testcontainers::{ContainerAsync, GenericImage, ImageExt, TestcontainersError};

// Environment variable keys
pub const QDRANT_VERSION_ENV: &str = "QDRANT_VERSION";

// Default version - matches the version used in integration-tests.sh
const DEFAULT_QDRANT_VERSION: &str = "v1.16.0";

// Qdrant ports
const QDRANT_GRPC_PORT: u16 = 6334;
const QDRANT_HTTP_PORT: u16 = 6333;

/// Global container instance for test reuse
pub static CONTAINER: OnceLock<QdrantContainer> = OnceLock::new();

/// Container for Qdrant
#[allow(dead_code)]
pub struct QdrantContainer {
    container: ContainerAsync<GenericImage>,
    pub grpc_port: u16,
    pub http_port: u16,
    pub grpc_url: String,
    pub http_url: String,
}

impl QdrantContainer {
    /// Create a new Qdrant container
    ///
    /// # Arguments
    ///
    /// * `use_tmpfs` - Enable tmpfs mount for storage directory (recommended for tests)
    ///
    /// # Errors
    ///
    /// Returns error if container fails to start
    pub async fn try_new(use_tmpfs: bool) -> Result<Self, TestcontainersError> {
        let version =
            env::var(QDRANT_VERSION_ENV).unwrap_or_else(|_| DEFAULT_QDRANT_VERSION.to_string());

        let grpc_port = QDRANT_GRPC_PORT.tcp();
        let http_port = QDRANT_HTTP_PORT.tcp();

        let http_strat = HttpWaitStrategy::new("/healthz")
            .with_port(testcontainers::core::ports::ContainerPort::Tcp(
                QDRANT_HTTP_PORT,
            ))
            .with_response_matcher(|response| response.status() == 200);

        // Create base image
        let image = GenericImage::new("qdrant/qdrant", &version)
            .with_exposed_port(grpc_port)
            .with_exposed_port(http_port)
            .with_wait_for(WaitFor::http(http_strat));

        // Start container with tmpfs if requested
        let container: ContainerAsync<GenericImage> = if use_tmpfs {
            image
                .with_mount(Mount::tmpfs_mount("/qdrant/storage").with_size("5g"))
                .start()
                .await?
        } else {
            image.start().await?
        };

        // Get mapped ports
        let grpc_port = container.get_host_port_ipv4(QDRANT_GRPC_PORT).await?;
        let http_port = container.get_host_port_ipv4(QDRANT_HTTP_PORT).await?;

        let grpc_url = format!("http://localhost:{grpc_port}");
        let http_url = format!("http://localhost:{http_port}");

        Ok(QdrantContainer {
            container,
            grpc_port,
            http_port,
            grpc_url,
            http_url,
        })
    }
}

/// Get or create a shared Qdrant container for tests
///
/// This function ensures only one container is created and reused across all tests.
/// Uses tmpfs for fast in-memory testing.
///
/// # Panics
///
/// Panics if container fails to start
pub async fn get_or_create_container() -> &'static QdrantContainer {
    if let Some(c) = CONTAINER.get() {
        return c;
    }

    let container = QdrantContainer::try_new(true)
        .await
        .expect("Failed to start Qdrant container");

    CONTAINER.get_or_init(|| container)
}

/// Create a new standalone Qdrant container
///
/// Unlike `get_or_create_container`, this creates a fresh container each time.
/// Useful when tests need isolation.
///
/// # Errors
///
/// Returns error if container fails to start
#[allow(dead_code)]
pub async fn create_container() -> Result<QdrantContainer, TestcontainersError> {
    QdrantContainer::try_new(true).await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_constants() {
        assert_eq!(DEFAULT_QDRANT_VERSION, "v1.16.0");
        assert_eq!(QDRANT_GRPC_PORT, 6334);
        assert_eq!(QDRANT_HTTP_PORT, 6333);
    }

    #[test]
    fn test_env_var_constants() {
        assert_eq!(QDRANT_VERSION_ENV, "QDRANT_VERSION");
    }
}
