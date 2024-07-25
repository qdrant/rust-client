use std::collections::HashSet;

use tonic_build::Builder;

fn timestamp(f: impl AsRef<std::path::Path>) -> std::time::SystemTime {
    std::fs::metadata(f).unwrap().modified().unwrap()
}

const GRPC_OUTPUT_FILE: &str = "src/qdrant.rs";

#[test]
fn protos() {
    let out_time = timestamp(GRPC_OUTPUT_FILE);
    let mut protos = std::fs::read_dir("proto").unwrap();
    if !protos.any(|d| timestamp(d.unwrap().path()) > out_time)
        && timestamp("tests/protos.rs") <= out_time
    {
        println!("protobuf files not changed. Exiting early!");
        return;
    }

    tonic_build::configure()
        .configure_deprecations()
        .configure_derive_builder()
        .out_dir("src/") // saves generated structures at this location
        .compile(
            &["proto/qdrant.proto"], // proto entry point
            &["proto"],              // specify the root location to search proto dependencies
        )
        .unwrap();

    append_to_file(GRPC_OUTPUT_FILE, "use crate::grpc_macros::*;");

    add_builder_macro_impls(
        GRPC_OUTPUT_FILE,
        builder_derive_options(),
        additional_builder_derive_options(),
    );

    // Re-export all custom builder here so they are all located in the same module in the end-user
    // API.
    let custom_reexports = [
        "pub use crate::manual_builder::*;",
        "pub use crate::builder_types::*;",
        "pub use crate::qdrant_client::builders::*;",
    ];
    append_to_file(GRPC_OUTPUT_FILE, &custom_reexports.join("\n"));

    // Vendor gRPC types used in our objects
    append_to_file(GRPC_OUTPUT_FILE, "pub use prost_types::Timestamp;");

    panic!("proto definitions changed. Stubs recompiled. Please commit the changes.")
}

/// Derive options for structs. (Path, build attributes, 'from' macro generation enabled)
type BuildDeriveOptions = (&'static str, &'static str, MacroConfig);

/// Configures how/which macros should be implemented for each builder.
enum MacroConfig {
    /// No macro call should be implemented
    NoMacro,

    /// Default implementation including `From<Builder>` and `build()`.
    DefaultImpl,

    /// All implementations from `DefaultImpl` and additionally a `pub(super) empty()` function
    /// that allows creating an empty builder from within this crate. This can be helpful for
    /// builder that don't implement `Default` due to some required parameters.
    WithDefaultFn,
}

/// Extension to [`Builder`] to configure builder attributes.
trait BuilderExt {
    fn configure_deprecations(self) -> Self;
    fn configure_derive_builder(self) -> Self;
    fn derive_builders(self, paths: &[(&str, &str)], derive_options: &[BuildDeriveOptions])
        -> Self;
    fn derive_builder(self, path: &str, derive_options: Option<&str>) -> Self;
    fn field_build_attributes(self, paths: &[(&str, &str)]) -> Self;
}

impl BuilderExt for Builder {
    fn configure_deprecations(self) -> Self {
        self.field_attribute(
            "PointsUpdateOperation.operation.delete_deprecated",
            "#[deprecated(since = \"1.7.0\", note = \"use `DeletePoints` instead\")]",
        )
        .field_attribute(
            "PointsUpdateOperation.operation.clear_payload_deprecated",
            "#[deprecated(since = \"1.7.0\", note = \"use `ClearPayload` instead\")]",
        )
    }

    fn configure_derive_builder(self) -> Self {
        configure_builder(self)
    }

    fn derive_builders(
        self,
        paths: &[(&str, &str)],
        derive_options: &[BuildDeriveOptions],
    ) -> Self {
        let structs = unique_structs_from_paths(paths.iter().map(|i| i.0), &[]);

        // Check we don't specify the same filed twice.
        let mut seen = HashSet::new();
        for (field, _) in paths.iter() {
            if seen.contains(field) {
                panic!("Field specified twice!");
            }
            seen.insert(field);
        }

        let derives = structs.into_iter().fold(self, |c, path| {
            let derive_options = derive_options.iter().find(|i| i.0 == path).map(|i| i.1);
            c.derive_builder(path, derive_options)
        });

        derives.field_build_attributes(paths)
    }

    fn derive_builder(self, path: &str, derive_options: Option<&str>) -> Self {
        let builder = self.type_attribute(path, "#[derive(derive_builder::Builder)]");

        if let Some(derive_options) = derive_options {
            builder.type_attribute(path, format!("#[builder({derive_options})]"))
        } else {
            builder
        }
    }

    fn field_build_attributes(self, paths: &[(&str, &str)]) -> Self {
        paths.iter().fold(self, |c, (path, attribute)| {
            c.field_attribute(path, format!("#[builder({attribute})]"))
        })
    }
}

/// Generates a str for field attributes required by some attributes for derive_builder.
/// This allows the builders fields being represented by different types than the actual built struct, if they can be converted.
/// This happens to be the case for quite some generated grpc types, so we use this to allow a simpler API for users.
///
/// The generated str looks like this:
/// setter(into, strip_option), field(ty="Option<$ty>", build="convert_option(&$id)")
///
/// For more infos of those atributes see the [dervie_builder's docs](https://docs.rs/derive_builder/latest/derive_builder/#completely-custom-fields-in-the-builder)
macro_rules! builder_custom_into {
    ($ty:ty, $id:expr) => {
        // Builds the following string as &'static str:
        // setter(into, strip_option), field(ty="Option<$ty>", build="convert_option(&$id)")
        concat!(
            "setter(into, strip_option)",
            ",",
            concat!("field(ty=\"", "Option<", stringify!($ty), ">\""),
            ",",
            concat!("build=\"", "convert_option(&", stringify!($id), ")\""),
            ")"
        )
    };
}

fn configure_builder(builder: Builder) -> Builder {
    const DEFAULT_OPTION: &str = "default, setter(strip_option), field(vis=\"pub(crate)\")";
    const DEFAULT_OPTION_INTO: &str =
        "default, setter(into, strip_option), field(vis=\"pub(crate)\")";
    const DEFAULT: &str = "default, field(vis=\"pub(crate)\")";
    const DEFAULT_INTO: &str = "default, setter(into), field(vis=\"pub(crate)\")";
    const CUSTOM_SETTER: &str = "default, setter(custom), field(vis=\"pub(crate)\")";
    const PUBLIC_ONLY: &str = r#"field(vis = "pub(crate)")"#;

    builder.derive_builders(
        &[
            // VectorParams
            ("VectorParams.size", DEFAULT),
            ("VectorParams.distance", DEFAULT_INTO),
            ("VectorParams.hnsw_config", DEFAULT_OPTION_INTO),
            // ("VectorParams.quantization_config", DEFAULT_OPTION_INTO),
            (
                "VectorParams.quantization_config",
                builder_custom_into!(quantization_config::Quantization, self.quantization_config),
            ),
            ("VectorParams.on_disk", DEFAULT_OPTION),
            ("VectorParams.datatype", DEFAULT_OPTION_INTO),
            ("VectorParams.multivector_config", DEFAULT_OPTION_INTO),
            // Create collection
            ("CreateCollection.collection_name", DEFAULT_INTO),
            ("CreateCollection.hnsw_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.wal_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.optimizers_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.shard_number", DEFAULT_OPTION),
            ("CreateCollection.on_disk_payload", DEFAULT_OPTION),
            ("CreateCollection.timeout", DEFAULT_OPTION),
            ("CreateCollection.vectors_config", DEFAULT_OPTION_INTO),
            ("CreateCollection.replication_factor", DEFAULT_OPTION),
            ("CreateCollection.write_consistency_factor", DEFAULT_OPTION),
            ("CreateCollection.init_from_collection", DEFAULT_OPTION_INTO),
            // ("CreateCollection.quantization_config", DEFAULT_OPTION_INTO),  (
            (
                "CreateCollection.quantization_config",
                builder_custom_into!(quantization_config::Quantization, self.quantization_config),
            ),
            ("CreateCollection.sharding_method", DEFAULT_OPTION),
            (
                "CreateCollection.sparse_vectors_config",
                DEFAULT_OPTION_INTO,
            ),
            // HnswConfig
            ("HnswConfigDiff.m", DEFAULT_OPTION),
            ("HnswConfigDiff.ef_construct", DEFAULT_OPTION),
            ("HnswConfigDiff.full_scan_threshold", DEFAULT_OPTION),
            ("HnswConfigDiff.max_indexing_threads", DEFAULT_OPTION),
            ("HnswConfigDiff.on_disk", DEFAULT_OPTION),
            ("HnswConfigDiff.payload_m", DEFAULT_OPTION),
            // ScalarQuantization
            ("ScalarQuantization.type", PUBLIC_ONLY),
            ("ScalarQuantization.quantile", DEFAULT_OPTION),
            ("ScalarQuantization.always_ram", DEFAULT_OPTION),
            // ProductQuantization
            ("ProductQuantization.compression", PUBLIC_ONLY),
            ("ProductQuantization.always_ram", DEFAULT_OPTION),
            // BinaryQuantization
            ("BinaryQuantization.always_ram", DEFAULT_OPTION),
            // OptimizersConfigDiff
            ("OptimizersConfigDiff.deleted_threshold", DEFAULT_OPTION),
            (
                "OptimizersConfigDiff.vacuum_min_vector_number",
                DEFAULT_OPTION,
            ),
            (
                "OptimizersConfigDiff.default_segment_number",
                DEFAULT_OPTION,
            ),
            ("OptimizersConfigDiff.max_segment_size", DEFAULT_OPTION),
            ("OptimizersConfigDiff.memmap_threshold", DEFAULT_OPTION),
            ("OptimizersConfigDiff.indexing_threshold", DEFAULT_OPTION),
            ("OptimizersConfigDiff.flush_interval_sec", DEFAULT_OPTION),
            (
                "OptimizersConfigDiff.max_optimization_threads",
                DEFAULT_OPTION,
            ),
            //WalConfigDiff
            ("WalConfigDiff.wal_capacity_mb", DEFAULT_OPTION),
            ("WalConfigDiff.wal_segments_ahead", DEFAULT_OPTION),
            // SearchPoints
            ("SearchPoints.collection_name", PUBLIC_ONLY),
            ("SearchPoints.vector", PUBLIC_ONLY),
            ("SearchPoints.limit", PUBLIC_ONLY),
            ("SearchPoints.filter", DEFAULT_OPTION_INTO),
            (
                "SearchPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("SearchPoints.params", DEFAULT_OPTION_INTO),
            ("SearchPoints.score_threshold", DEFAULT_OPTION),
            ("SearchPoints.offset", DEFAULT_OPTION),
            ("SearchPoints.vector_name", DEFAULT_OPTION_INTO),
            (
                "SearchPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "SearchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("SearchPoints.timeout", DEFAULT_OPTION),
            ("SearchPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            ("SearchPoints.sparse_indices", DEFAULT_OPTION_INTO),
            // SearchParams
            ("SearchParams.hnsw_ef", DEFAULT_OPTION),
            ("SearchParams.exact", DEFAULT_OPTION),
            ("SearchParams.quantization", DEFAULT_OPTION_INTO),
            ("SearchParams.indexed_only", DEFAULT_OPTION),
            // QuantizationSearchParams
            ("QuantizationSearchParams.ignore", DEFAULT_OPTION),
            ("QuantizationSearchParams.rescore", DEFAULT_OPTION),
            ("QuantizationSearchParams.oversampling", DEFAULT_OPTION),
            // UpdateCollection
            ("UpdateCollection.collection_name", PUBLIC_ONLY),
            ("UpdateCollection.optimizers_config", DEFAULT_OPTION_INTO),
            ("UpdateCollection.timeout", DEFAULT_OPTION),
            ("UpdateCollection.params", DEFAULT_OPTION_INTO),
            ("UpdateCollection.hnsw_config", DEFAULT_OPTION_INTO),
            ("UpdateCollection.vectors_config", DEFAULT_OPTION_INTO),
            (
                "UpdateCollection.quantization_config",
                builder_custom_into!(
                    quantization_config_diff::Quantization,
                    self.quantization_config
                ),
            ),
            (
                "UpdateCollection.sparse_vectors_config",
                DEFAULT_OPTION_INTO,
            ),
            // SetPayloadPoints
            ("SetPayloadPoints.collection_name", PUBLIC_ONLY),
            ("SetPayloadPoints.payload", PUBLIC_ONLY),
            ("SetPayloadPoints.wait", DEFAULT_OPTION),
            (
                "SetPayloadPoints.points_selector",
                builder_custom_into!(points_selector::PointsSelectorOneOf, self.points_selector),
            ),
            ("SetPayloadPoints.ordering", DEFAULT_OPTION_INTO),
            ("SetPayloadPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            ("SetPayloadPoints.key", DEFAULT_OPTION_INTO),
            // UpsertPoints
            ("UpsertPoints.collection_name", PUBLIC_ONLY),
            ("UpsertPoints.points", PUBLIC_ONLY),
            ("UpsertPoints.wait", DEFAULT_OPTION),
            ("UpsertPoints.ordering", DEFAULT_OPTION_INTO),
            ("UpsertPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // UpdateBatchPoints
            ("UpdateBatchPoints.collection_name", PUBLIC_ONLY),
            ("UpdateBatchPoints.operations", PUBLIC_ONLY),
            ("UpdateBatchPoints.wait", DEFAULT_OPTION),
            ("UpdateBatchPoints.ordering", DEFAULT_OPTION_INTO),
            // DeletePayloadPoints
            ("DeletePayloadPoints.collection_name", PUBLIC_ONLY),
            ("DeletePayloadPoints.keys", PUBLIC_ONLY),
            ("DeletePayloadPoints.wait", DEFAULT_OPTION),
            (
                "DeletePayloadPoints.points_selector",
                builder_custom_into!(points_selector::PointsSelectorOneOf, self.points_selector),
            ),
            ("DeletePayloadPoints.ordering", DEFAULT_OPTION_INTO),
            (
                "DeletePayloadPoints.shard_key_selector",
                DEFAULT_OPTION_INTO,
            ),
            // ClearPayloadPoints
            ("ClearPayloadPoints.collection_name", PUBLIC_ONLY),
            ("ClearPayloadPoints.wait", DEFAULT_OPTION),
            (
                "ClearPayloadPoints.points",
                builder_custom_into!(points_selector::PointsSelectorOneOf, self.points),
            ),
            ("ClearPayloadPoints.ordering", DEFAULT_OPTION_INTO),
            ("ClearPayloadPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // GetPoints
            ("GetPoints.collection_name", PUBLIC_ONLY),
            ("GetPoints.ids", PUBLIC_ONLY),
            (
                "GetPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            (
                "GetPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "GetPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("GetPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // SearchBatchPoints
            ("SearchBatchPoints.collection_name", PUBLIC_ONLY),
            ("SearchBatchPoints.search_points", PUBLIC_ONLY),
            (
                "SearchBatchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("SearchBatchPoints.timeout", DEFAULT_OPTION),
            // SearchPointGroups
            ("SearchPointGroups.collection_name", PUBLIC_ONLY),
            ("SearchPointGroups.vector", PUBLIC_ONLY),
            ("SearchPointGroups.limit", PUBLIC_ONLY),
            ("SearchPointGroups.group_by", PUBLIC_ONLY),
            ("SearchPointGroups.group_size", PUBLIC_ONLY),
            ("SearchPointGroups.filter", DEFAULT_OPTION_INTO),
            (
                "SearchPointGroups.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("SearchPointGroups.params", DEFAULT_OPTION_INTO),
            ("SearchPointGroups.score_threshold", DEFAULT_OPTION_INTO),
            ("SearchPointGroups.vector_name", DEFAULT_OPTION_INTO),
            (
                "SearchPointGroups.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "SearchPointGroups.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("SearchPointGroups.with_lookup", DEFAULT_OPTION_INTO),
            ("SearchPointGroups.timeout", DEFAULT_OPTION_INTO),
            ("SearchPointGroups.shard_key_selector", DEFAULT_OPTION_INTO),
            ("SearchPointGroups.sparse_indices", DEFAULT_OPTION_INTO),
            // WithLookup
            ("WithLookup.collection", PUBLIC_ONLY),
            (
                "WithLookup.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            (
                "WithLookup.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            // DeletePoints
            ("qdrant.DeletePoints.collection_name", PUBLIC_ONLY),
            ("qdrant.DeletePoints.wait", DEFAULT_OPTION),
            (
                "qdrant.DeletePoints.points",
                builder_custom_into!(points_selector::PointsSelectorOneOf, self.points),
            ),
            ("qdrant.DeletePoints.ordering", DEFAULT_OPTION_INTO),
            (
                "qdrant.DeletePoints.shard_key_selector",
                DEFAULT_OPTION_INTO,
            ),
            // DeletePointVectors
            ("DeletePointVectors.collection_name", PUBLIC_ONLY),
            ("DeletePointVectors.wait", DEFAULT_OPTION),
            (
                "DeletePointVectors.points_selector",
                builder_custom_into!(points_selector::PointsSelectorOneOf, self.points_selector),
            ),
            ("DeletePointVectors.vectors", DEFAULT_OPTION_INTO),
            ("DeletePointVectors.ordering", DEFAULT_OPTION_INTO),
            ("DeletePointVectors.shard_key_selector", DEFAULT_OPTION_INTO),
            // UpdatePointVectors
            ("UpdatePointVectors.collection_name", PUBLIC_ONLY),
            ("UpdatePointVectors.points", PUBLIC_ONLY),
            ("UpdatePointVectors.wait", DEFAULT_OPTION),
            ("UpdatePointVectors.ordering", DEFAULT_OPTION_INTO),
            ("UpdatePointVectors.shard_key_selector", DEFAULT_OPTION_INTO),
            // ScrollPoints
            ("ScrollPoints.collection_name", PUBLIC_ONLY),
            ("ScrollPoints.filter", DEFAULT_OPTION_INTO),
            ("ScrollPoints.offset", DEFAULT_OPTION_INTO),
            ("ScrollPoints.limit", DEFAULT_OPTION),
            (
                "ScrollPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            (
                "ScrollPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "ScrollPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("ScrollPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            ("ScrollPoints.order_by", DEFAULT_OPTION_INTO),
            // OrderBy
            ("OrderBy.key", PUBLIC_ONLY),
            ("OrderBy.direction", DEFAULT_OPTION),
            (
                "OrderBy.start_from",
                builder_custom_into!(start_from::Value, self.start_from),
            ),
            // RecommendPoints
            ("RecommendPoints.collection_name", PUBLIC_ONLY),
            ("RecommendPoints.limit", PUBLIC_ONLY),
            ("RecommendPoints.filter", DEFAULT_OPTION_INTO),
            (
                "RecommendPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("RecommendPoints.params", DEFAULT_OPTION_INTO),
            ("RecommendPoints.score_threshold", DEFAULT_OPTION),
            ("RecommendPoints.offset", DEFAULT_OPTION),
            ("RecommendPoints.using", DEFAULT_OPTION_INTO),
            (
                "RecommendPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            ("RecommendPoints.lookup_from", DEFAULT_OPTION_INTO),
            ("RecommendPoints.positive", CUSTOM_SETTER),
            ("RecommendPoints.negative", CUSTOM_SETTER),
            ("RecommendPoints.positive_vectors", CUSTOM_SETTER),
            ("RecommendPoints.negative_vectors", CUSTOM_SETTER),
            (
                "RecommendPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("RecommendPoints.strategy", DEFAULT_OPTION_INTO),
            ("RecommendPoints.timeout", DEFAULT_OPTION),
            ("RecommendPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // LookupLocation
            ("LookupLocation.collection_name", PUBLIC_ONLY),
            ("LookupLocation.vector_name", DEFAULT_OPTION_INTO),
            ("LookupLocation.shard_key_selector", DEFAULT_OPTION_INTO),
            // RecommendBatchPoints
            ("RecommendBatchPoints.collection_name", PUBLIC_ONLY),
            ("RecommendBatchPoints.recommend_points", PUBLIC_ONLY),
            (
                "RecommendBatchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("RecommendBatchPoints.timeout", DEFAULT_OPTION),
            // RecommendPointGroups
            ("RecommendPointGroups.collection_name", PUBLIC_ONLY),
            ("RecommendPointGroups.group_by", PUBLIC_ONLY),
            ("RecommendPointGroups.group_size", PUBLIC_ONLY),
            ("RecommendPointGroups.limit", PUBLIC_ONLY),
            ("RecommendPointGroups.filter", DEFAULT_OPTION_INTO),
            (
                "RecommendPointGroups.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("RecommendPointGroups.params", DEFAULT_OPTION_INTO),
            ("RecommendPointGroups.score_threshold", DEFAULT_OPTION),
            ("RecommendPointGroups.using", DEFAULT_OPTION_INTO),
            (
                "RecommendPointGroups.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            ("RecommendPointGroups.positive", CUSTOM_SETTER),
            ("RecommendPointGroups.negative", CUSTOM_SETTER),
            ("RecommendPointGroups.positive_vectors", CUSTOM_SETTER),
            ("RecommendPointGroups.negative_vectors", CUSTOM_SETTER),
            ("RecommendPointGroups.lookup_from", DEFAULT_OPTION_INTO),
            (
                "RecommendPointGroups.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("RecommendPointGroups.with_lookup", DEFAULT_OPTION_INTO),
            ("RecommendPointGroups.strategy", DEFAULT_OPTION_INTO),
            ("RecommendPointGroups.timeout", DEFAULT_OPTION),
            (
                "RecommendPointGroups.shard_key_selector",
                DEFAULT_OPTION_INTO,
            ),
            // DiscoverPoints
            ("DiscoverPoints.collection_name", PUBLIC_ONLY),
            ("DiscoverPoints.context", PUBLIC_ONLY),
            ("DiscoverPoints.limit", PUBLIC_ONLY),
            ("DiscoverPoints.target", DEFAULT_OPTION_INTO),
            ("DiscoverPoints.filter", DEFAULT_OPTION_INTO),
            (
                "DiscoverPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            ("DiscoverPoints.params", DEFAULT_OPTION_INTO),
            ("DiscoverPoints.offset", DEFAULT_OPTION),
            ("DiscoverPoints.using", DEFAULT_OPTION_INTO),
            (
                "DiscoverPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            ("DiscoverPoints.lookup_from", DEFAULT_OPTION_INTO),
            (
                "DiscoverPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("DiscoverPoints.timeout", DEFAULT_OPTION),
            ("DiscoverPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // DiscoverBatchPoints
            ("DiscoverBatchPoints.collection_name", PUBLIC_ONLY),
            ("DiscoverBatchPoints.discover_points", PUBLIC_ONLY),
            (
                "DiscoverBatchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("DiscoverBatchPoints.timeout", DEFAULT_OPTION),
            // PrefetchQuery
            ("PrefetchQuery.prefetch", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.query", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.using", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.filter", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.params", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.score_threshold", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.limit", DEFAULT_OPTION_INTO),
            ("PrefetchQuery.lookup_from", DEFAULT_OPTION_INTO),
            // Query
            ("QueryPoints.collection_name", PUBLIC_ONLY),
            ("QueryPoints.prefetch", DEFAULT_OPTION_INTO),
            ("QueryPoints.query", DEFAULT_OPTION_INTO),
            ("QueryPoints.using", DEFAULT_OPTION_INTO),
            ("QueryPoints.filter", DEFAULT_OPTION_INTO),
            ("QueryPoints.params", DEFAULT_OPTION_INTO),
            ("QueryPoints.score_threshold", DEFAULT_OPTION_INTO),
            ("QueryPoints.limit", DEFAULT_OPTION),
            ("QueryPoints.offset", DEFAULT_OPTION),
            (
                "QueryPoints.with_payload",
                builder_custom_into!(with_payload_selector::SelectorOptions, self.with_payload),
            ),
            (
                "QueryPoints.with_vectors",
                builder_custom_into!(with_vectors_selector::SelectorOptions, self.with_vectors),
            ),
            (
                "QueryPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("QueryPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            ("QueryPoints.lookup_from", DEFAULT_OPTION_INTO),
            ("QueryPoints.timeout", DEFAULT_OPTION),
            // QueryBatchPoints
            ("QueryBatchPoints.collection_name", PUBLIC_ONLY),
            ("QueryBatchPoints.query_points", PUBLIC_ONLY),
            (
                "QueryBatchPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("QueryBatchPoints.timeout", DEFAULT_OPTION),
            // CountPoints
            ("CountPoints.collection_name", PUBLIC_ONLY),
            ("CountPoints.filter", DEFAULT_OPTION_INTO),
            ("CountPoints.exact", DEFAULT_OPTION),
            (
                "CountPoints.read_consistency",
                builder_custom_into!(read_consistency::Value, self.read_consistency),
            ),
            ("CountPoints.shard_key_selector", DEFAULT_OPTION_INTO),
            // CreateFieldIndexCollection
            ("CreateFieldIndexCollection.collection_name", PUBLIC_ONLY),
            ("CreateFieldIndexCollection.field_name", PUBLIC_ONLY),
            ("CreateFieldIndexCollection.wait", DEFAULT_OPTION),
            ("CreateFieldIndexCollection.field_type", DEFAULT_OPTION_INTO),
            (
                "CreateFieldIndexCollection.field_index_params",
                DEFAULT_OPTION_INTO,
            ),
            ("CreateFieldIndexCollection.ordering", DEFAULT_OPTION_INTO),
            // DeleteFieldIndexCollection
            ("DeleteFieldIndexCollection.collection_name", PUBLIC_ONLY),
            ("DeleteFieldIndexCollection.field_name", PUBLIC_ONLY),
            ("DeleteFieldIndexCollection.wait", DEFAULT_OPTION),
            ("DeleteFieldIndexCollection.ordering", DEFAULT_OPTION),
            // UpdateCollectionClusterSetupRequest
            (
                "UpdateCollectionClusterSetupRequest.collection_name",
                PUBLIC_ONLY,
            ),
            (
                "UpdateCollectionClusterSetupRequest.timeout",
                DEFAULT_OPTION,
            ),
            ("UpdateCollectionClusterSetupRequest.operation", PUBLIC_ONLY),
            // MoveShard
            ("MoveShard.shard_id", PUBLIC_ONLY),
            ("MoveShard.to_shard_id", DEFAULT_OPTION),
            ("MoveShard.from_peer_id", PUBLIC_ONLY),
            ("MoveShard.to_peer_id", PUBLIC_ONLY),
            ("MoveShard.method", DEFAULT_OPTION_INTO),
            // ReplicateShard
            ("ReplicateShard.shard_id", PUBLIC_ONLY),
            ("ReplicateShard.to_shard_id", DEFAULT_OPTION),
            ("ReplicateShard.from_peer_id", PUBLIC_ONLY),
            ("ReplicateShard.to_peer_id", PUBLIC_ONLY),
            ("ReplicateShard.method", DEFAULT_OPTION_INTO),
            // AbortShardTransfer
            ("AbortShardTransfer.shard_id", PUBLIC_ONLY),
            ("AbortShardTransfer.to_shard_id", DEFAULT_OPTION),
            ("AbortShardTransfer.from_peer_id", PUBLIC_ONLY),
            ("AbortShardTransfer.to_peer_id", PUBLIC_ONLY),
            // Replica
            ("Replica.shard_id", PUBLIC_ONLY),
            ("Replica.peer_id", PUBLIC_ONLY),
            // CreateShardKeyRequest
            ("CreateShardKeyRequest.collection_name", PUBLIC_ONLY),
            ("CreateShardKeyRequest.request", DEFAULT_OPTION_INTO),
            ("CreateShardKeyRequest.timeout", DEFAULT_OPTION),
            // DeleteShardKeyRequest
            ("DeleteShardKeyRequest.collection_name", PUBLIC_ONLY),
            ("DeleteShardKeyRequest.request", CUSTOM_SETTER),
            ("DeleteShardKeyRequest.timeout", DEFAULT_OPTION),
            // DeleteCollection
            ("DeleteCollection.collection_name", PUBLIC_ONLY),
            ("DeleteCollection.timeout", DEFAULT_OPTION),
            // CollectionParamsDiff
            ("CollectionParamsDiff.replication_factor", DEFAULT_OPTION),
            (
                "CollectionParamsDiff.write_consistency_factor",
                DEFAULT_OPTION,
            ),
            ("CollectionParamsDiff.on_disk_payload", DEFAULT_OPTION),
            ("CollectionParamsDiff.read_fan_out_factor", DEFAULT_OPTION),
            // VectorParamsDiff
            ("VectorParamsDiff.hnsw_config", DEFAULT_OPTION_INTO),
            (
                "VectorParamsDiff.quantization_config",
                builder_custom_into!(
                    quantization_config_diff::Quantization,
                    self.quantization_config
                ),
            ),
            ("VectorParamsDiff.on_disk", DEFAULT_OPTION),
            // SparseVectorParams
            ("SparseVectorParams.index", DEFAULT_OPTION_INTO),
            ("SparseVectorParams.modifier", DEFAULT_OPTION_INTO),
            // SparseIndexConfig
            ("SparseIndexConfig.full_scan_threshold", DEFAULT_OPTION_INTO),
            ("SparseIndexConfig.on_disk", DEFAULT_OPTION),
            ("SparseIndexConfig.datatype", DEFAULT_OPTION_INTO),
            // CreateShardKey
            ("CreateShardKey.shard_key", DEFAULT_OPTION_INTO),
            ("CreateShardKey.shards_number", DEFAULT_OPTION),
            ("CreateShardKey.replication_factor", DEFAULT_OPTION),
            ("CreateShardKey.placement", DEFAULT_OPTION),
            // ContextExamplePair
            ("ContextExamplePair.positive", DEFAULT_OPTION_INTO),
            ("ContextExamplePair.negative", DEFAULT_OPTION_INTO),
            // TextIndexParams
            ("TextIndexParams.tokenizer", PUBLIC_ONLY),
            ("TextIndexParams.lowercase", DEFAULT_OPTION),
            ("TextIndexParams.min_token_len", DEFAULT_OPTION),
            ("TextIndexParams.max_token_len", DEFAULT_OPTION),
            // CreateAlias
            ("CreateAlias.collection_name", PUBLIC_ONLY),
            ("CreateAlias.alias_name", PUBLIC_ONLY),
            // RenameAlias
            ("RenameAlias.old_alias_name", PUBLIC_ONLY),
            ("RenameAlias.new_alias_name", PUBLIC_ONLY),
            // DeleteSnapshotRequest
            ("DeleteSnapshotRequest.collection_name", PUBLIC_ONLY),
            ("DeleteSnapshotRequest.snapshot_name", PUBLIC_ONLY),
            // IntegerIndexParams
            ("IntegerIndexParams.lookup", DEFAULT_OPTION_INTO),
            ("IntegerIndexParams.range", DEFAULT_OPTION_INTO),
            ("IntegerIndexParams.is_tenant", DEFAULT_OPTION),
            // KeywordIndexParams
            ("KeywordIndexParams.is_tenant", DEFAULT_OPTION),
            // RecommendInput
            ("RecommendInput.positive", DEFAULT_OPTION_INTO),
            ("RecommendInput.negative", DEFAULT_OPTION_INTO),
            ("RecommendInput.strategy", DEFAULT_OPTION_INTO),
            // DiscoverInput
            ("DiscoverInput.target", DEFAULT_OPTION_INTO),
            ("DiscoverInput.context", DEFAULT_OPTION_INTO),
            // ContextInput
            ("ContextInput.pairs", DEFAULT_OPTION_INTO),
            // ContextInputPair
            ("ContextInputPair.positive", DEFAULT_OPTION_INTO),
            ("ContextInputPair.negative", DEFAULT_OPTION_INTO),
            // MultiVectorConfig
            ("MultiVectorConfig.comparator", DEFAULT_OPTION_INTO),
            // QueryPointGroups
            ("QueryPointGroups.collection_name", PUBLIC_ONLY),
            ("QueryPointGroups.prefetch", PUBLIC_ONLY),
            ("QueryPointGroups.group_by", PUBLIC_ONLY),
            ("QueryPointGroups.query", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.using", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.filter", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.params", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.score_threshold", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.with_payload", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.with_vectors", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.lookup_from", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.limit", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.group_size", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.read_consistency", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.with_lookup", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.timeout", DEFAULT_OPTION_INTO),
            ("QueryPointGroups.shard_key_selector", DEFAULT_OPTION_INTO),
        ],
        builder_derive_options(),
    )
}

/// Builder configurations for grpc structs.
fn builder_derive_options() -> &'static [BuildDeriveOptions] {
    // We populate our own .build() function that doesn't return a Result but the built type directly.
    // For this we rename and make the automatically generated build function private.
    // Infallible allows secure unwrapping and compiler errors on missing fields.
    const DEFAULT_BUILDER_DERIVE_OPTIONS: &str =
        "build_fn(private, error = \"std::convert::Infallible\", name = \"build_inner\"), pattern = \"owned\"";
    const NO_DEFAULT_BUILDER_DERIVE_OPTIONS: &str =
        "build_fn(private, name = \"build_inner\"), pattern = \"owned\", custom_constructor";

    // Tuple structure: (Path, build attributes, 'from' macro generation enabled)
    &[
        (
            "CreateCollection",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "VectorParams",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "HnswConfigDiff",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "ScalarQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "ProductQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "BinaryQuantization",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "OptimizersConfigDiff",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "WalConfigDiff",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "SearchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "SearchParams",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "QuantizationSearchParams",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "UpdateCollection",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "SetPayloadPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "UpsertPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "UpdateBatchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeletePayloadPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "ClearPayloadPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "GetPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "SearchBatchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "SearchPointGroups",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "WithLookup",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeletePointVectors",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "UpdatePointVectors",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "ScrollPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "OrderBy",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "RecommendPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "LookupLocation",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "RecommendBatchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "RecommendPointGroups",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DiscoverPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DiscoverBatchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "QueryPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "QueryBatchPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "CountPoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "CreateFieldIndexCollection",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeleteFieldIndexCollection",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "qdrant.DeletePoints",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::NoMacro,
        ),
        (
            "UpdateCollectionClusterSetupRequest",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "MoveShard",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "ReplicateShard",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "AbortShardTransfer",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "Replica",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "CreateShardKeyRequest",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeleteShardKeyRequest",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeleteCollection",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "CollectionParamsDiff",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "VectorParamsDiff",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "SparseVectorParams",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "SparseIndexConfig",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "CreateShardKey",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "ContextExamplePair",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "TextIndexParams",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "IntegerIndexParams",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "KeywordIndexParams",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "CreateAlias",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "RenameAlias",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "DeleteSnapshotRequest",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "PrefetchQuery",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "RecommendInput",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "DiscoverInput",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "ContextInput",
            DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::DefaultImpl,
        ),
        (
            "ContextInputPair",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "MultiVectorConfig",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
        (
            "QueryPointGroups",
            NO_DEFAULT_BUILDER_DERIVE_OPTIONS,
            MacroConfig::WithDefaultFn,
        ),
    ]
}

fn additional_builder_derive_options() -> &'static [BuildDeriveOptions] {
    &[("DeletePoints", "", MacroConfig::WithDefaultFn)]
}

/// Returns a list of all unique structs that appear in a list of paths.
fn unique_structs_from_paths<'a, I>(paths: I, extra: &[&'a str]) -> Vec<&'a str>
where
    I: IntoIterator<Item = &'a str>,
{
    let mut derives = paths
        .into_iter()
        .map(|field| {
            // Types themselves can also be specified directly with a path, separated by '.'.
            // The last element is the fields name. We want to only strip the fields name and preserve
            // the whole path to also identify types specified using one.
            field.rsplit_once('.').unwrap().0
        })
        .collect::<Vec<&str>>();
    derives.extend(extra);
    derives.sort_unstable();
    derives.dedup();
    derives
}

fn append_to_file(path: &str, line: &str) {
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    writeln!(
        OpenOptions::new().append(true).open(path).unwrap(),
        "{line}",
    )
    .unwrap()
}

/// Generates all necessary macro calls for builders who should have them.
fn add_builder_macro_impls(
    file: &str,
    derive_options: &[BuildDeriveOptions],
    additional: &[BuildDeriveOptions],
) {
    let to_append = derive_options
        .iter()
        .chain(additional)
        .filter_map(|(type_name, _, macro_config)| {
            let macro_call = match macro_config {
                MacroConfig::NoMacro => {
                    return None;
                }
                MacroConfig::DefaultImpl => {
                    format!("builder_type_conversions!({type_name}, {type_name}Builder);\n")
                }
                MacroConfig::WithDefaultFn => {
                    format!("builder_type_conversions!({type_name}, {type_name}Builder, true);\n")
                }
            };

            Some(macro_call)
        })
        .fold(String::new(), |mut s, line| {
            s.push_str(&line);
            s
        });
    append_to_file(file, &to_append);
}
