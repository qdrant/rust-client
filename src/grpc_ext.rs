use crate::client::Payload;
use crate::qdrant::payload_index_params::IndexParams;
use crate::qdrant::point_id::PointIdOptions;
use crate::qdrant::points_selector::PointsSelectorOneOf;
use crate::qdrant::value::Kind;
use crate::qdrant::vectors::VectorsOptions;
use crate::qdrant::{alias_operations, condition, group_id, points_update_operation, quantization_config, quantization_config_diff, r#match, read_consistency, shard_key, start_from, target_vector, update_collection_cluster_setup_request, vector_example, vectors_config, vectors_config_diff, with_payload_selector, with_vectors_selector, AbortShardTransfer, AliasOperations, BinaryQuantization, BinaryQuantizationBuilder, Condition, CreateAlias, CreateShardKey, DeleteAlias, DeleteShardKey, Disabled, FieldCondition, Filter, GeoLineString, GeoPoint, GroupId, HasIdCondition, IntegerIndexParams, IsEmptyCondition, IsNullCondition, ListValue, Match, MoveShard, NamedVectors, NestedCondition, PayloadExcludeSelector, PayloadIncludeSelector, PayloadIndexParams, PointId, PointStruct, PointsIdsList, PointsSelector, PointsUpdateOperation, ProductQuantization, ProductQuantizationBuilder, QuantizationConfig, QuantizationConfigDiff, ReadConsistency, RenameAlias, RepeatedIntegers, RepeatedStrings, Replica, ReplicateShard, RestartTransfer, ScalarQuantization, ScalarQuantizationBuilder, ShardKey, ShardKeySelector, SparseIndexConfig, SparseIndices, SparseVectorConfig, SparseVectorParams, StartFrom, Struct, TargetVector, TextIndexParams, Value, Vector, VectorExample, VectorParams, VectorParamsBuilder, VectorParamsDiff, VectorParamsDiffBuilder, VectorParamsDiffMap, VectorParamsMap, Vectors, VectorsConfig, VectorsConfigDiff, VectorsSelector, WithPayloadSelector, WithVectorsSelector, DeleteCollectionBuilder};
use std::collections::HashMap;
use crate::prelude::DeleteCollection;

impl From<bool> for WithPayloadSelector {
    fn from(flag: bool) -> Self {
        WithPayloadSelector {
            selector_options: Some(with_payload_selector::SelectorOptions::Enable(flag)),
        }
    }
}

impl From<Vec<&str>> for WithPayloadSelector {
    fn from(fields: Vec<&str>) -> Self {
        WithPayloadSelector {
            selector_options: Some(with_payload_selector::SelectorOptions::Include(
                PayloadIncludeSelector {
                    fields: fields.into_iter().map(|f| f.to_string()).collect(),
                },
            )),
        }
    }
}

impl From<Vec<PointId>> for PointsSelector {
    fn from(point_ids: Vec<PointId>) -> Self {
        PointsSelector {
            points_selector_one_of: Some(PointsSelectorOneOf::Points(PointsIdsList {
                ids: point_ids,
            })),
        }
    }
}

impl From<Vec<f32>> for Vector {
    fn from(vector: Vec<f32>) -> Self {
        Vector {
            data: vector,
            indices: None,
        }
    }
}

impl From<Vec<(u32, f32)>> for Vector {
    fn from(tuples: Vec<(u32, f32)>) -> Self {
        Self::from(tuples.as_slice())
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<&[(u32, f32)]> for Vector {
    fn from(tuples: &[(u32, f32)]) -> Self {
        let mut indices = Vec::with_capacity(tuples.len());
        let mut values = Vec::with_capacity(tuples.len());
        for (i, w) in tuples {
            indices.push(*i);
            values.push(*w);
        }
        Vector {
            data: values,
            indices: Some(SparseIndices { data: indices }),
        }
    }
}

impl From<HashMap<String, Vec<f32>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<f32>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vector>> for Vectors {
    fn from(named_vectors: HashMap<String, Vector>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors.into_iter().collect(),
            })),
        }
    }
}

impl From<HashMap<String, Vec<(u32, f32)>>> for Vectors {
    fn from(named_vectors: HashMap<String, Vec<(u32, f32)>>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

// Since we construct two new Vec's anyway it's fine to source from a reference
impl From<HashMap<String, &[(u32, f32)]>> for Vectors {
    fn from(named_vectors: HashMap<String, &[(u32, f32)]>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vectors(NamedVectors {
                vectors: named_vectors
                    .into_iter()
                    .map(|(k, v)| (k, v.into()))
                    .collect(),
            })),
        }
    }
}

impl From<Vec<f32>> for Vectors {
    fn from(vector: Vec<f32>) -> Self {
        Vectors {
            vectors_options: Some(VectorsOptions::Vector(vector.into())),
        }
    }
}

impl From<Vec<&str>> for WithVectorsSelector {
    fn from(names: Vec<&str>) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Include(
                VectorsSelector {
                    names: names.into_iter().map(|name| name.to_string()).collect(),
                },
            )),
        }
    }
}

impl From<bool> for WithVectorsSelector {
    fn from(flag: bool) -> Self {
        WithVectorsSelector {
            selector_options: Some(with_vectors_selector::SelectorOptions::Enable(flag)),
        }
    }
}

impl From<f64> for Value {
    fn from(val: f64) -> Self {
        Self {
            kind: Some(Kind::DoubleValue(val)),
        }
    }
}

impl From<i64> for Value {
    fn from(val: i64) -> Self {
        Self {
            kind: Some(Kind::IntegerValue(val)),
        }
    }
}

impl From<bool> for Value {
    fn from(val: bool) -> Self {
        Self {
            kind: Some(Kind::BoolValue(val)),
        }
    }
}

impl From<String> for Value {
    fn from(val: String) -> Self {
        Self {
            kind: Some(Kind::StringValue(val)),
        }
    }
}

impl From<&str> for Value {
    fn from(val: &str) -> Self {
        Self {
            kind: Some(Kind::StringValue(val.into())),
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

impl<T> From<Vec<(&str, T)>> for Value
    where
        T: Into<Value>,
{
    fn from(val: Vec<(&str, T)>) -> Self {
        Self {
            kind: Some(Kind::StructValue(Struct {
                fields: val
                    .into_iter()
                    .map(|(k, v)| (k.to_string(), v.into()))
                    .collect(),
            })),
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
        }
    }
}

impl From<Vec<ShardKey>> for ShardKeySelector {
    fn from(value: Vec<ShardKey>) -> Self {
        Self { shard_keys: value }
    }
}

impl From<String> for shard_key::Key {
    fn from(keyword: String) -> Self {
        shard_key::Key::Keyword(keyword)
    }
}

impl From<u64> for shard_key::Key {
    fn from(number: u64) -> Self {
        shard_key::Key::Number(number)
    }
}

impl From<String> for ShardKeySelector {
    fn from(keyword: String) -> Self {
        keyword.into()
    }
}

impl From<u64> for ShardKeySelector {
    fn from(number: u64) -> Self {
        number.into()
    }
}

impl From<Vec<String>> for ShardKeySelector {
    fn from(keywords: Vec<String>) -> Self {
        keywords.into()
    }
}

impl From<Vec<u64>> for ShardKeySelector {
    fn from(numbers: Vec<u64>) -> Self {
        numbers.into()
    }
}

impl From<String> for PointId {
    fn from(val: String) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Uuid(val)),
        }
    }
}

impl From<u64> for PointId {
    fn from(val: u64) -> Self {
        Self {
            point_id_options: Some(PointIdOptions::Num(val)),
        }
    }
}

impl PointStruct {
    pub fn new(id: impl Into<PointId>, vectors: impl Into<Vectors>, payload: Payload) -> Self {
        Self {
            id: Some(id.into()),
            payload: payload.into(),
            vectors: Some(vectors.into()),
        }
    }
}

impl From<HashMap<String, VectorParams>> for VectorParamsMap {
    fn from(value: HashMap<String, VectorParams>) -> Self {
        VectorParamsMap { map: value }
    }
}

impl From<HashMap<String, VectorParamsDiff>> for VectorParamsDiffMap {
    fn from(value: HashMap<String, VectorParamsDiff>) -> Self {
        VectorParamsDiffMap { map: value }
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

impl From<HashMap<String, SparseVectorParams>> for SparseVectorConfig {
    fn from(value: HashMap<String, SparseVectorParams>) -> Self {
        Self { map: value }
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

impl From<IndexParams> for PayloadIndexParams {
    fn from(value: IndexParams) -> Self {
        Self {
            index_params: Some(value),
        }
    }
}

impl From<TextIndexParams> for IndexParams {
    fn from(value: TextIndexParams) -> Self {
        Self::TextIndexParams(value)
    }
}

impl From<IntegerIndexParams> for IndexParams {
    fn from(value: IntegerIndexParams) -> Self {
        Self::IntegerIndexParams(value)
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

impl From<HashMap<String, Value>> for Struct {
    fn from(value: HashMap<String, Value>) -> Self {
        Self { fields: value }
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

impl From<Vec<u32>> for SparseIndices {
    fn from(value: Vec<u32>) -> Self {
        Self { data: value }
    }
}

impl From<Vec<String>> for PayloadIncludeSelector {
    fn from(value: Vec<String>) -> Self {
        Self { fields: value }
    }
}

impl From<Vec<String>> for PayloadExcludeSelector {
    fn from(value: Vec<String>) -> Self {
        Self { fields: value }
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

impl From<HashMap<String, Vector>> for NamedVectors {
    fn from(value: HashMap<String, Vector>) -> Self {
        Self { vectors: value }
    }
}

impl From<VectorsOptions> for Vectors {
    fn from(value: VectorsOptions) -> Self {
        Self {
            vectors_options: Some(value),
        }
    }
}

impl From<Vector> for VectorsOptions {
    fn from(value: Vector) -> Self {
        Self::Vector(value)
    }
}

impl From<NamedVectors> for VectorsOptions {
    fn from(value: NamedVectors) -> Self {
        Self::Vectors(value)
    }
}

impl From<Vec<String>> for VectorsSelector {
    fn from(value: Vec<String>) -> Self {
        VectorsSelector { names: value }
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

impl From<String> for IsEmptyCondition {
    fn from(value: String) -> Self {
        Self { key: value }
    }
}

impl From<String> for IsNullCondition {
    fn from(value: String) -> Self {
        Self { key: value }
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

impl From<Vec<String>> for RepeatedStrings {
    fn from(value: Vec<String>) -> Self {
        Self { strings: value }
    }
}

impl From<Vec<i64>> for RepeatedIntegers {
    fn from(value: Vec<i64>) -> Self {
        Self { integers: value }
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

impl From<PointsIdsList> for PointsSelectorOneOf {
    fn from(value: PointsIdsList) -> Self {
        Self::Points(value)
    }
}

impl From<Filter> for PointsSelectorOneOf {
    fn from(value: Filter) -> Self {
        Self::Filter(value)
    }
}

impl From<Vec<PointId>> for PointsIdsList {
    fn from(value: Vec<PointId>) -> Self {
        Self { ids: value }
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

impl From<ReplicateShard> for update_collection_cluster_setup_request::Operation {
    fn from(value: ReplicateShard) -> Self {
        Self::ReplicateShard(value)
    }
}

impl From<AbortShardTransfer> for update_collection_cluster_setup_request::Operation {
    fn from(value: AbortShardTransfer) -> Self {
        Self::AbortTransfer(value)
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

impl<S: Into<String>> From<S> for DeleteCollection {
    fn from(value: S) -> Self {
        DeleteCollectionBuilder::new(value).build()
    }
}