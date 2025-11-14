use std::collections::HashMap;

use qdrant_client::qdrant::{
    AbortShardTransferBuilder, AcornSearchParamsBuilder, BinaryQuantizationBuilder,
    ClearPayloadPointsBuilder, ContextInputBuilder, ContextInputPairBuilder, CountPointsBuilder,
    CreateAliasBuilder, CreateFieldIndexCollectionBuilder, CreateShardKeyRequestBuilder,
    DeleteCollectionBuilder, DeleteFieldIndexCollectionBuilder, DeletePayloadPointsBuilder,
    DeletePointVectorsBuilder, DeletePointsBuilder, DeleteShardKeyRequestBuilder,
    DeleteSnapshotRequestBuilder, DiscoverBatchPointsBuilder, DiscoverInputBuilder,
    DiscoverPointsBuilder, Distance, FacetCountsBuilder, FieldType, GetPointsBuilder,
    LookupLocationBuilder, MoveShardBuilder, MultiVectorComparator, MultiVectorConfigBuilder,
    OrderByBuilder, ProductQuantizationBuilder, QueryBatchPointsBuilder, QueryPointGroupsBuilder,
    QueryPointsBuilder, RecommendBatchPointsBuilder, RecommendPointGroupsBuilder,
    RecommendPointsBuilder, RenameAliasBuilder, ReplicaBuilder, ReplicatePointsBuilder,
    ReplicateShardBuilder, RrfBuilder, ScrollPointsBuilder, SearchBatchPointsBuilder,
    SearchMatrixPointsBuilder, SearchPointGroupsBuilder, SearchPointsBuilder,
    SetPayloadPointsBuilder, ShardKey, ShardKeySelectorBuilder, TextIndexParamsBuilder,
    TokenizerType, UpdateBatchPointsBuilder, UpdateCollectionBuilder,
    UpdateCollectionClusterSetupRequestBuilder, UpdatePointVectorsBuilder, UpsertPointsBuilder,
    VectorParamsBuilder, WithLookupBuilder,
};

/// TLDR; Ensures new fields introduced in protobuf updates won't cause a panic at runtime due to missing derive_builder attributes.
///
/// Builder with required fields may lack coverage for all required fields in their custom constructor.
/// This can happen if a new (even optional) field gets introduced in an API update. Those fields need to be
/// explicitly set to 'default' using the #[builder(default)] attribute. However this can easily be forgotten
/// when updating the clients protobuf types. This test covers all builder with required parameters in order
/// to catch new fields lacking the above described field attribute.
/// Simply calling build() is sufficient as derive_builder will fail on unset fields without #[builder(default)].
///
/// Builder without any required fields will throw a compiler error due to the 'error = "std::convert::Infallible"'
/// builder config. For this reason testing them is not needed.
#[test]
fn builder_coverage() {
    VectorParamsBuilder::new(1, Distance::Cosine).build();
    ProductQuantizationBuilder::new(1).build();
    BinaryQuantizationBuilder::new(true).build();
    SearchPointsBuilder::new("my_collection", [11.; 5], 3).build();
    UpdateCollectionBuilder::new("my_collection").build();
    SetPayloadPointsBuilder::new("my_collection", HashMap::default()).build();
    UpdateBatchPointsBuilder::new("my_collection", []).build();
    DeletePayloadPointsBuilder::new("my_collection", []).build();
    ClearPayloadPointsBuilder::new("my_collection").build();
    GetPointsBuilder::new("my_collection", []).build();
    SearchBatchPointsBuilder::new("my_collection", []).build();
    SearchPointGroupsBuilder::new("my_collection", [11.; 5], 10, "mygroup", 5).build();
    WithLookupBuilder::new("my_collection").build();
    DeletePointsBuilder::new("my_collection").build();
    DeletePointVectorsBuilder::new("my_collection").build();
    UpdatePointVectorsBuilder::new("my_collection", []).build();
    ScrollPointsBuilder::new("my_collection").build();
    OrderByBuilder::new("key").build();
    RecommendPointsBuilder::new("my_collection", 10).build();
    LookupLocationBuilder::new("my_collection").build();
    RecommendBatchPointsBuilder::new("my_collection", []).build();
    RecommendPointGroupsBuilder::new("my_collection", "group", 10, 10).build();
    DiscoverPointsBuilder::new("my_collection", [], 10).build();
    DiscoverBatchPointsBuilder::new("my_collection", []).build();
    CountPointsBuilder::new("my_collection").build();
    UpsertPointsBuilder::new("my_collection", []).build();
    CreateFieldIndexCollectionBuilder::new("my_collection", " myfield", FieldType::Integer).build();
    DeleteFieldIndexCollectionBuilder::new("my_collection", " myfield").build();
    UpdateCollectionClusterSetupRequestBuilder::new(
        "my_collection",
        MoveShardBuilder::new(0, 0, 0),
    )
    .build();
    MoveShardBuilder::new(0, 0, 0).build();
    ReplicateShardBuilder::new(0, 0, 0).build();
    ReplicatePointsBuilder::new(
        ShardKey::from("shard1".to_string()),
        ShardKey::from("shard2".to_string()),
    )
    .build();
    ReplicaBuilder::new(0, 0).build();
    AbortShardTransferBuilder::new(0, 0, 0).build();
    CreateShardKeyRequestBuilder::new("my_collection").build();
    DeleteShardKeyRequestBuilder::new("my_collection").build();
    DeleteCollectionBuilder::new("my_collection").build();
    TextIndexParamsBuilder::new(TokenizerType::Word).build();
    CreateAliasBuilder::new("", "").build();
    RenameAliasBuilder::new("", "").build();
    QueryPointsBuilder::new("my_collection").build();
    QueryBatchPointsBuilder::new("my_collection", []).build();
    DeleteSnapshotRequestBuilder::new("my_collection", "snapshot").build();
    ContextInputPairBuilder::new(vec![1.0], vec![2.0]).build();
    DiscoverInputBuilder::new(vec![1.0], ContextInputBuilder::default()).build();
    MultiVectorConfigBuilder::new(MultiVectorComparator::MaxSim).build();
    QueryPointGroupsBuilder::new("", "").build();
    FacetCountsBuilder::new("", "").build();
    SearchMatrixPointsBuilder::new("").build();

    // New parameter constructors
    ShardKeySelectorBuilder::new()
        .add_shard_key(ShardKey::from("key1".to_string()))
        .build();
    ShardKeySelectorBuilder::with_shard_keys(vec![ShardKey::from("key1".to_string())]).build();
    ShardKeySelectorBuilder::with_shard_keys(vec![ShardKey::from("key1".to_string())])
        .fallback(ShardKey::from("fallback".to_string()))
        .build();
    AcornSearchParamsBuilder::new(true).build();
    AcornSearchParamsBuilder::new(false).build();
    AcornSearchParamsBuilder::new(true)
        .max_selectivity(0.5)
        .build();
    RrfBuilder::new().build();
    RrfBuilder::with_k(100).build();
}
