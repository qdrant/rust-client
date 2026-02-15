mod extensions;
pub mod metadata;
mod primitives;
pub mod vectors;

use crate::payload::Payload;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::value::Kind;
use crate::qdrant::{
    alias_operations, condition, group_id, points_update_operation, quantization_config,
    quantization_config_diff, r#match, read_consistency, shard_key, start_from, target_vector,
    update_collection_cluster_setup_request, vector_example, vectors_config, vectors_config_diff,
    with_payload_selector, with_vectors_selector, AbortShardTransfer, AbortShardTransferBuilder,
    AliasOperations, BinaryQuantization, BinaryQuantizationBuilder, Condition, CreateAlias,
    CreateShardKey, DeleteAlias, DeleteShardKey, Disabled, FieldCondition, Filter, GeoLineString,
    GeoPoint, GroupId, HasIdCondition, IsEmptyCondition, IsNullCondition, ListValue, Match,
    MoveShard, MoveShardBuilder, NestedCondition, PayloadExcludeSelector, PayloadIncludeSelector,
    PointId, PointsIdsList, PointsSelector, PointsUpdateOperation, ProductQuantization,
    ProductQuantizationBuilder, QuantizationConfig, QuantizationConfigDiff, ReadConsistency,
    RenameAlias, Replica, ReplicateShard, ReplicateShardBuilder, RestartTransfer,
    ScalarQuantization, ScalarQuantizationBuilder, ShardKey, ShardKeySelector, SparseIndexConfig,
    SparseVectorParams, StartFrom, Struct, TargetVector, Value, Vector, VectorExample,
    VectorParams, VectorParamsBuilder, VectorParamsDiff, VectorParamsDiffBuilder,
    VectorParamsDiffMap, VectorParamsMap, VectorsConfig, VectorsConfigDiff, VectorsSelector,
    WithPayloadSelector, WithVectorsSelector,
};

impl From<Vec<PointId>> for PointsSelector {
    fn from(point_ids: Vec<PointId>) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: point_ids,
            })),
        }
    }
}

impl<T> From<Vec<T>> for Value
where
    T: Into<Value>,
{
    fn from(val: Vec<T>) -> Self {
        Self {
            kind: Some(Kind::ListValue(ListValue {
                values: val.into_iter().map(|v| v.into()).collect(),
            })),
        }
    }
}

impl From<Payload> for Value {
    fn from(val: Payload) -> Self {
        Self {
            kind: Some(Kind::StructValue(Struct { fields: val.0 })),
        }
    }
}

impl From<shard_key::Key> for ShardKey {
    fn from(key: shard_key::Key) -> Self {
        ShardKey { key: Some(key) }
    }
}

impl From<Vec<shard_key::Key>> for ShardKeySelector {
    fn from(shard_keys: Vec<shard_key::Key>) -> Self {
        ShardKeySelector {
            shard_keys: shard_keys.into_iter().map(ShardKey::from).collect(),
            fallback: None,
        }
    }
}

impl From<Vec<ShardKey>> for ShardKeySelector {
    fn from(value: Vec<ShardKey>) -> Self {
        Self {
            shard_keys: value,
            fallback: None,
        }
    }
}

impl From<vectors_config::Config> for VectorsConfig {
    fn from(value: vectors_config::Config) -> Self {
        VectorsConfig {
            config: Some(value),
        }
    }
}

impl From<VectorParams> for vectors_config::Config {
    fn from(value: VectorParams) -> Self {
        Self::Params(value)
    }
}

impl From<VectorParamsMap> for vectors_config::Config {
    fn from(value: VectorParamsMap) -> Self {
        Self::ParamsMap(value)
    }
}

impl From<vectors_config_diff::Config> for VectorsConfigDiff {
    fn from(value: vectors_config_diff::Config) -> Self {
        Self {
            config: Some(value),
        }
    }
}

impl From<VectorParamsDiff> for vectors_config_diff::Config {
    fn from(value: VectorParamsDiff) -> Self {
        Self::Params(value)
    }
}

impl From<VectorParamsDiffMap> for vectors_config_diff::Config {
    fn from(value: VectorParamsDiffMap) -> Self {
        Self::ParamsMap(value)
    }
}

impl From<SparseIndexConfig> for SparseVectorParams {
    fn from(value: SparseIndexConfig) -> Self {
        Self {
            index: Some(value),
            modifier: None,
        }
    }
}

impl From<quantization_config::Quantization> for QuantizationConfig {
    fn from(value: quantization_config::Quantization) -> Self {
        Self {
            quantization: Some(value),
        }
    }
}

impl From<ScalarQuantization> for quantization_config::Quantization {
    fn from(value: ScalarQuantization) -> Self {
        Self::Scalar(value)
    }
}

impl From<ProductQuantization> for quantization_config::Quantization {
    fn from(value: ProductQuantization) -> Self {
        Self::Product(value)
    }
}

impl From<BinaryQuantization> for quantization_config::Quantization {
    fn from(value: BinaryQuantization) -> Self {
        Self::Binary(value)
    }
}

impl From<quantization_config_diff::Quantization> for QuantizationConfigDiff {
    fn from(value: quantization_config_diff::Quantization) -> Self {
        Self {
            quantization: Some(value),
        }
    }
}

impl From<ScalarQuantization> for quantization_config_diff::Quantization {
    fn from(value: ScalarQuantization) -> Self {
        Self::Scalar(value)
    }
}

impl From<ProductQuantization> for quantization_config_diff::Quantization {
    fn from(value: ProductQuantization) -> Self {
        Self::Product(value)
    }
}

impl From<Disabled> for quantization_config_diff::Quantization {
    fn from(value: Disabled) -> Self {
        Self::Disabled(value)
    }
}

impl From<BinaryQuantization> for quantization_config_diff::Quantization {
    fn from(value: BinaryQuantization) -> Self {
        Self::Binary(value)
    }
}

impl From<alias_operations::Action> for AliasOperations {
    fn from(value: alias_operations::Action) -> Self {
        AliasOperations {
            action: Some(value),
        }
    }
}

impl From<CreateAlias> for alias_operations::Action {
    fn from(value: CreateAlias) -> Self {
        Self::CreateAlias(value)
    }
}

impl From<RenameAlias> for alias_operations::Action {
    fn from(value: RenameAlias) -> Self {
        Self::RenameAlias(value)
    }
}

impl From<DeleteAlias> for alias_operations::Action {
    fn from(value: DeleteAlias) -> Self {
        Self::DeleteAlias(value)
    }
}

impl From<Kind> for Value {
    fn from(value: Kind) -> Self {
        Self { kind: Some(value) }
    }
}

impl From<Vec<Value>> for ListValue {
    fn from(value: Vec<Value>) -> Self {
        Self { values: value }
    }
}

impl From<read_consistency::Value> for ReadConsistency {
    fn from(value: read_consistency::Value) -> Self {
        Self { value: Some(value) }
    }
}

impl From<PointIdOptions> for PointId {
    fn from(value: PointIdOptions) -> Self {
        Self {
            point_id_options: Some(value),
        }
    }
}

impl From<with_payload_selector::SelectorOptions> for WithPayloadSelector {
    fn from(value: with_payload_selector::SelectorOptions) -> Self {
        Self {
            selector_options: Some(value),
        }
    }
}

impl From<PayloadIncludeSelector> for with_payload_selector::SelectorOptions {
    fn from(value: PayloadIncludeSelector) -> Self {
        Self::Include(value)
    }
}

impl From<PayloadExcludeSelector> for with_payload_selector::SelectorOptions {
    fn from(value: PayloadExcludeSelector) -> Self {
        Self::Exclude(value)
    }
}

impl From<with_vectors_selector::SelectorOptions> for WithVectorsSelector {
    fn from(value: with_vectors_selector::SelectorOptions) -> Self {
        Self {
            selector_options: Some(value),
        }
    }
}

impl From<VectorsSelector> for with_vectors_selector::SelectorOptions {
    fn from(value: VectorsSelector) -> Self {
        Self::Include(value)
    }
}

impl From<start_from::Value> for StartFrom {
    fn from(value: start_from::Value) -> Self {
        Self { value: Some(value) }
    }
}

impl From<target_vector::Target> for TargetVector {
    fn from(value: target_vector::Target) -> Self {
        Self {
            target: Some(value),
        }
    }
}

impl From<VectorExample> for target_vector::Target {
    fn from(value: VectorExample) -> Self {
        Self::Single(value)
    }
}

impl From<vector_example::Example> for VectorExample {
    fn from(value: vector_example::Example) -> Self {
        Self {
            example: Some(value),
        }
    }
}

impl From<PointId> for vector_example::Example {
    fn from(value: PointId) -> Self {
        Self::Id(value)
    }
}

impl From<Vector> for vector_example::Example {
    fn from(value: Vector) -> Self {
        Self::Vector(value)
    }
}

impl From<points_update_operation::Operation> for PointsUpdateOperation {
    fn from(value: points_update_operation::Operation) -> Self {
        Self {
            operation: Some(value),
        }
    }
}

impl From<group_id::Kind> for GroupId {
    fn from(value: group_id::Kind) -> Self {
        Self { kind: Some(value) }
    }
}

impl From<condition::ConditionOneOf> for Condition {
    fn from(value: condition::ConditionOneOf) -> Self {
        Self {
            condition_one_of: Some(value),
        }
    }
}

impl From<FieldCondition> for condition::ConditionOneOf {
    fn from(value: FieldCondition) -> Self {
        Self::Field(value)
    }
}

impl From<IsEmptyCondition> for condition::ConditionOneOf {
    fn from(value: IsEmptyCondition) -> Self {
        Self::IsEmpty(value)
    }
}

impl From<HasIdCondition> for condition::ConditionOneOf {
    fn from(value: HasIdCondition) -> Self {
        Self::HasId(value)
    }
}

impl From<Filter> for condition::ConditionOneOf {
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}

impl From<IsNullCondition> for condition::ConditionOneOf {
    fn from(value: IsNullCondition) -> Self {
        Self::IsNull(value)
    }
}

impl From<NestedCondition> for condition::ConditionOneOf {
    fn from(value: NestedCondition) -> Self {
        Self::Nested(value)
    }
}

impl From<Vec<PointId>> for HasIdCondition {
    fn from(value: Vec<PointId>) -> Self {
        Self { has_id: value }
    }
}

impl From<r#match::MatchValue> for Match {
    fn from(value: r#match::MatchValue) -> Self {
        Self {
            match_value: Some(value),
        }
    }
}

impl From<Vec<GeoPoint>> for GeoLineString {
    fn from(value: Vec<GeoPoint>) -> Self {
        Self { points: value }
    }
}

impl From<PointsSelectorOneOf> for PointsSelector {
    fn from(value: PointsSelectorOneOf) -> Self {
        Self {
            points_selector_one_of: Some(value),
        }
    }
}

impl From<Filter> for PointsSelectorOneOf {
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}

impl<I: Into<PointId>, const N: usize> From<[I; N]> for PointsIdsList {
    fn from(value: [I; N]) -> Self {
        let ids: Vec<_> = value.into_iter().map(|i| i.into()).collect();
        Self { ids }
    }
}

impl<I: Into<PointId>> From<Vec<I>> for PointsIdsList {
    fn from(value: Vec<I>) -> Self {
        let ids: Vec<_> = value.into_iter().map(|i| i.into()).collect();
        Self { ids }
    }
}

impl<I: Into<PointsIdsList>> From<I> for PointsSelectorOneOf {
    fn from(value: I) -> Self {
        Self::Points(value.into())
    }
}

impl From<VectorParamsDiffBuilder> for vectors_config_diff::Config {
    fn from(value: VectorParamsDiffBuilder) -> Self {
        value.build().into()
    }
}

impl From<VectorParamsBuilder> for vectors_config::Config {
    fn from(value: VectorParamsBuilder) -> Self {
        value.build().into()
    }
}

impl From<ScalarQuantizationBuilder> for quantization_config_diff::Quantization {
    fn from(value: ScalarQuantizationBuilder) -> Self {
        Self::Scalar(value.build())
    }
}

impl From<ProductQuantizationBuilder> for quantization_config_diff::Quantization {
    fn from(value: ProductQuantizationBuilder) -> Self {
        Self::Product(value.build())
    }
}

impl From<BinaryQuantizationBuilder> for quantization_config_diff::Quantization {
    fn from(value: BinaryQuantizationBuilder) -> Self {
        Self::Binary(value.build())
    }
}

impl From<ScalarQuantizationBuilder> for quantization_config::Quantization {
    fn from(value: ScalarQuantizationBuilder) -> Self {
        Self::Scalar(value.build())
    }
}

impl From<ProductQuantizationBuilder> for quantization_config::Quantization {
    fn from(value: ProductQuantizationBuilder) -> Self {
        Self::Product(value.build())
    }
}

impl From<BinaryQuantizationBuilder> for quantization_config::Quantization {
    fn from(value: BinaryQuantizationBuilder) -> Self {
        Self::Binary(value.build())
    }
}

impl From<points_update_operation::PointStructList> for points_update_operation::Operation {
    fn from(value: points_update_operation::PointStructList) -> Self {
        Self::Upsert(value)
    }
}

impl From<points_update_operation::SetPayload> for points_update_operation::Operation {
    fn from(value: points_update_operation::SetPayload) -> Self {
        Self::SetPayload(value)
    }
}

impl From<points_update_operation::OverwritePayload> for points_update_operation::Operation {
    fn from(value: points_update_operation::OverwritePayload) -> Self {
        Self::OverwritePayload(value)
    }
}

impl From<points_update_operation::DeletePayload> for points_update_operation::Operation {
    fn from(value: points_update_operation::DeletePayload) -> Self {
        Self::DeletePayload(value)
    }
}

impl From<points_update_operation::UpdateVectors> for points_update_operation::Operation {
    fn from(value: points_update_operation::UpdateVectors) -> Self {
        Self::UpdateVectors(value)
    }
}

impl From<points_update_operation::DeleteVectors> for points_update_operation::Operation {
    fn from(value: points_update_operation::DeleteVectors) -> Self {
        Self::DeleteVectors(value)
    }
}

impl From<points_update_operation::DeletePoints> for points_update_operation::Operation {
    fn from(value: points_update_operation::DeletePoints) -> Self {
        Self::DeletePoints(value)
    }
}

impl From<points_update_operation::ClearPayload> for points_update_operation::Operation {
    fn from(value: points_update_operation::ClearPayload) -> Self {
        Self::ClearPayload(value)
    }
}

impl From<MoveShard> for update_collection_cluster_setup_request::Operation {
    fn from(value: MoveShard) -> Self {
        Self::MoveShard(value)
    }
}

impl From<MoveShardBuilder> for update_collection_cluster_setup_request::Operation {
    fn from(value: MoveShardBuilder) -> Self {
        Self::MoveShard(value.build())
    }
}

impl From<ReplicateShard> for update_collection_cluster_setup_request::Operation {
    fn from(value: ReplicateShard) -> Self {
        Self::ReplicateShard(value)
    }
}

impl From<ReplicateShardBuilder> for update_collection_cluster_setup_request::Operation {
    fn from(value: ReplicateShardBuilder) -> Self {
        Self::ReplicateShard(value.build())
    }
}

impl From<AbortShardTransfer> for update_collection_cluster_setup_request::Operation {
    fn from(value: AbortShardTransfer) -> Self {
        Self::AbortTransfer(value)
    }
}

impl From<AbortShardTransferBuilder> for update_collection_cluster_setup_request::Operation {
    fn from(value: AbortShardTransferBuilder) -> Self {
        Self::AbortTransfer(value.build())
    }
}

impl From<Replica> for update_collection_cluster_setup_request::Operation {
    fn from(value: Replica) -> Self {
        Self::DropReplica(value)
    }
}

impl From<CreateShardKey> for update_collection_cluster_setup_request::Operation {
    fn from(value: CreateShardKey) -> Self {
        Self::CreateShardKey(value)
    }
}

impl From<DeleteShardKey> for update_collection_cluster_setup_request::Operation {
    fn from(value: DeleteShardKey) -> Self {
        Self::DeleteShardKey(value)
    }
}

impl From<RestartTransfer> for update_collection_cluster_setup_request::Operation {
    fn from(value: RestartTransfer) -> Self {
        Self::RestartTransfer(value)
    }
}
