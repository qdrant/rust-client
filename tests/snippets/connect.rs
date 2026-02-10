use qdrant_client::Qdrant;
use std::time::Duration;

let client = Qdrant::from_url("http://localhost:1234")
    .connect_timeout(Duration::from_millis(500))
    .build()?;

let connection_result = client.connect().await;

assert!(connection_result.is_err());
