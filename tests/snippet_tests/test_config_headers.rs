use qdrant_client::config::{CompressionEncoding, QdrantConfig};

#[test]
fn header_adds_single_header() {
    let config = QdrantConfig::from_url("http://localhost:6334").header("x-custom-id", "my-client");

    assert_eq!(
        config.custom_headers,
        vec![("x-custom-id".to_string(), "my-client".to_string())]
    );
}

#[test]
fn header_chain_preserves_order() {
    let config = QdrantConfig::from_url("http://localhost:6334")
        .header("x-a", "1")
        .header("x-b", "2")
        .header("x-a", "3");

    assert_eq!(
        config.custom_headers,
        vec![
            ("x-a".to_string(), "1".to_string()),
            ("x-b".to_string(), "2".to_string()),
            ("x-a".to_string(), "3".to_string()),
        ]
    );
}

#[test]
fn header_allows_duplicate_keys() {
    let config = QdrantConfig::from_url("http://localhost:6334")
        .header("openai-api-key", "k1")
        .header("openai-api-key", "k2");

    assert_eq!(config.custom_headers.len(), 2);
    assert_eq!(
        config.custom_headers,
        vec![
            ("openai-api-key".to_string(), "k1".to_string()),
            ("openai-api-key".to_string(), "k2".to_string()),
        ]
    );
}

#[test]
fn header_does_not_mutate_other_config() {
    let base = QdrantConfig::from_url("http://localhost:6334")
        .api_key("secret")
        .timeout(10u64)
        .connect_timeout(20u64)
        .compression(Some(CompressionEncoding::Gzip))
        .skip_compatibility_check();

    let with_header = base.clone().header("x-feature", "on");

    assert_eq!(with_header.uri, base.uri);
    assert_eq!(with_header.timeout, base.timeout);
    assert_eq!(with_header.connect_timeout, base.connect_timeout);
    assert_eq!(
        with_header.keep_alive_while_idle,
        base.keep_alive_while_idle
    );
    assert_eq!(with_header.api_key, base.api_key);
    assert_eq!(with_header.compression, base.compression);
    assert_eq!(with_header.check_compatibility, base.check_compatibility);
    assert_eq!(with_header.pool_size, base.pool_size);

    assert_eq!(
        with_header.custom_headers,
        vec![("x-feature".to_string(), "on".to_string())]
    );
    assert!(base.custom_headers.is_empty());
}
