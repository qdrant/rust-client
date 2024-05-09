use qdrant_client::prelude::Distance;
use qdrant_client::qdrant::{
    BinaryQuantizationBuilder, ClearPayloadPointsBuilder, CountPointsBuilder,
    CreateFieldIndexCollectionBuilder, DeleteFieldIndexCollectionBuilder,
    DeletePayloadPointsBuilder, DeletePointVectorsBuilder, DiscoverBatchPointsBuilder,
    DiscoverPointsBuilder, GetPointsBuilder, LookupLocationBuilder, OrderByBuilder,
    ProductQuantizationBuilder, QuantizationType, RecommendBatchPointsBuilder,
    RecommendPointGroupsBuilder, RecommendPointsBuilder, ScalarQuantizationBuilder,
    ScrollPointsBuilder, SearchBatchPointsBuilder, SearchPointGroupsBuilder, SearchPointsBuilder,
    SetPayloadPointsBuilder, UpdateBatchPointsBuilder, UpdateCollectionBuilder,
    UpdatePointVectorsBuilder, UpsertPointsBuilder, VectorParamsBuilder, WithLookupBuilder,
};
use std::collections::HashMap;

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
    ScalarQuantizationBuilder::new(QuantizationType::Int8).build();
    ProductQuantizationBuilder::new(1).build();
    BinaryQuantizationBuilder::new(true).build();
    SearchPointsBuilder::new("mycollection", [11.; 5], 3).build();
    UpdateCollectionBuilder::new("mycollection").build();
    SetPayloadPointsBuilder::new("mycollection", HashMap::default()).build();
    UpdateBatchPointsBuilder::new("mycollection", []).build();
    DeletePayloadPointsBuilder::new("mycollection", []).build();
    ClearPayloadPointsBuilder::new("mycollection").build();
    GetPointsBuilder::new("mycollection", []).build();
    SearchBatchPointsBuilder::new("mycollection", []).build();
    SearchPointGroupsBuilder::new("mycollection", [11.; 5], 10, "mygroup", 5).build();
    WithLookupBuilder::new("mycollection").build();
    DeletePointVectorsBuilder::new("mycollection").build();
    UpdatePointVectorsBuilder::new("mycollection", []).build();
    ScrollPointsBuilder::new("mycollection").build();
    OrderByBuilder::new("key").build();
    RecommendPointsBuilder::new("mycollection", 10).build();
    LookupLocationBuilder::new("mycollection").build();
    RecommendBatchPointsBuilder::new("mycollection", []).build();
    RecommendPointGroupsBuilder::new("mycollection", "group", 10, 10).build();
    DiscoverPointsBuilder::new("mycollection", [], 10).build();
    DiscoverBatchPointsBuilder::new("mycollection", []).build();
    CountPointsBuilder::new("mycollection").build();
    UpsertPointsBuilder::new("mycollection", []).build();
    CreateFieldIndexCollectionBuilder::new("mycollection", " myfield").build();
    DeleteFieldIndexCollectionBuilder::new("mycollection", " myfield").build();
}
