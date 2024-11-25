use std::collections::HashMap;

use crate::qdrant::update_collection_cluster_setup_request::Operation;
use crate::qdrant::{
    shard_key, AbortShardTransferBuilder, BinaryQuantizationBuilder, ClearPayloadPointsBuilder,
    ContextExamplePair, CountPointsBuilder, CreateAliasBuilder, CreateCollectionBuilder,
    CreateFieldIndexCollectionBuilder, CreateShardKeyRequestBuilder, DeleteCollectionBuilder,
    DeleteFieldIndexCollectionBuilder, DeletePayloadPointsBuilder, DeletePointVectorsBuilder,
    DeletePointsBuilder, DeleteShardKey, DeleteShardKeyRequestBuilder,
    DeleteSnapshotRequestBuilder, DiscoverBatchPointsBuilder, DiscoverPoints,
    DiscoverPointsBuilder, Distance, FacetCountsBuilder, FieldType, GetPointsBuilder,
    LookupLocationBuilder, MoveShardBuilder, PayloadExcludeSelector, PayloadIncludeSelector,
    PointId, PointStruct, PointVectors, PointsUpdateOperation, ProductQuantizationBuilder,
    QuantizationType, QueryBatchPointsBuilder, QueryPointGroupsBuilder, QueryPoints,
    QueryPointsBuilder, RecommendBatchPointsBuilder, RecommendExample, RecommendPointGroupsBuilder,
    RecommendPoints, RecommendPointsBuilder, RenameAliasBuilder, ReplicaBuilder,
    ReplicateShardBuilder, ScalarQuantizationBuilder, ScrollPointsBuilder,
    SearchBatchPointsBuilder, SearchMatrixPointsBuilder, SearchPointGroupsBuilder, SearchPoints,
    SearchPointsBuilder, SetPayloadPointsBuilder, ShardKey, UpdateBatchPointsBuilder,
    UpdateCollectionBuilder, UpdateCollectionClusterSetupRequestBuilder, UpdatePointVectorsBuilder,
    UpsertPointsBuilder, Value, VectorParamsBuilder, VectorsSelector, WithLookupBuilder,
};

impl VectorParamsBuilder {
    pub fn new(size: u64, distance: Distance) -> Self {
        let mut builder = Self::empty();
        builder.size = Some(size);
        builder.distance = Some(distance.into());
        builder
    }
}

impl Default for ScalarQuantizationBuilder {
    fn default() -> Self {
        let mut builder = Self::empty();
        builder.r#type = Some(QuantizationType::Int8.into());
        builder
    }
}

impl ProductQuantizationBuilder {
    pub fn new(compression: i32) -> Self {
        let mut builder = Self::empty();
        builder.compression = Some(compression);
        builder
    }
}

impl BinaryQuantizationBuilder {
    pub fn new(always_ram: bool) -> Self {
        let mut builder = Self::empty();
        builder.always_ram = Some(Some(always_ram));
        builder
    }
}

impl SearchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        vector: impl Into<Vec<f32>>,
        limit: u64,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.vector = Some(vector.into());
        builder.limit = Some(limit);
        builder
    }
}

impl UpdateCollectionBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl SetPayloadPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        payload: impl Into<HashMap<String, Value>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.payload = Some(payload.into());
        builder
    }
}

impl UpdateBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        operations: impl Into<Vec<PointsUpdateOperation>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.operations = Some(operations.into());
        builder
    }
}

impl DeletePayloadPointsBuilder {
    pub fn new(collection_name: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.keys = Some(keys.into());
        builder
    }
}

impl ClearPayloadPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl GetPointsBuilder {
    pub fn new(collection_name: impl Into<String>, ids: impl Into<Vec<PointId>>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.ids = Some(ids.into());
        builder
    }
}

impl SearchBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        search_points: impl Into<Vec<SearchPoints>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.search_points = Some(search_points.into());
        builder
    }
}

impl SearchPointGroupsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        vector: impl Into<Vec<f32>>,
        limit: u32,
        group_by: impl Into<String>,
        group_size: u32,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.vector = Some(vector.into());
        builder.limit = Some(limit);
        builder.group_by = Some(group_by.into());
        builder.group_size = Some(group_size);
        builder
    }
}

impl WithLookupBuilder {
    pub fn new(collection: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection = Some(collection.into());
        builder
    }
}

impl DeletePointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl DeletePointVectorsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl UpdatePointVectorsBuilder {
    pub fn new(collection_name: impl Into<String>, points: impl Into<Vec<PointVectors>>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.points = Some(points.into());
        builder
    }
}

impl ScrollPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl RecommendPointsBuilder {
    pub fn new(collection_name: impl Into<String>, limit: u64) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.limit = Some(limit);
        builder
    }
}

impl LookupLocationBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl RecommendBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        recommend_points: impl Into<Vec<RecommendPoints>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.recommend_points = Some(recommend_points.into());
        builder
    }
}

impl RecommendPointGroupsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        group_by: impl Into<String>,
        group_size: u32,
        limit: u32,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.group_by = Some(group_by.into());
        builder.group_size = Some(group_size);
        builder.limit = Some(limit);
        builder
    }
}

impl DiscoverPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        context: impl Into<Vec<ContextExamplePair>>,
        limit: u64,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.context = Some(context.into());
        builder.limit = Some(limit);
        builder
    }
}

impl DiscoverBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        discover_points: impl Into<Vec<DiscoverPoints>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.discover_points = Some(discover_points.into());
        builder
    }
}

impl CountPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl UpsertPointsBuilder {
    pub fn new(collection_name: impl Into<String>, points: impl Into<Vec<PointStruct>>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.points = Some(points.into());
        builder
    }
}

impl CreateFieldIndexCollectionBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        field_name: impl Into<String>,
        field_type: FieldType,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.field_name = Some(field_name.into());
        builder.field_type(field_type)
    }
}

impl DeleteFieldIndexCollectionBuilder {
    pub fn new(collection_name: impl Into<String>, field_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.field_name = Some(field_name.into());
        builder
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    pub fn new(collection_name: impl Into<String>, operation: impl Into<Operation>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.operation = Some(Some(operation.into()));
        builder
    }
}

impl MoveShardBuilder {
    pub fn new(shard_id: u32, from_peer_id: u64, to_peer_id: u64) -> Self {
        let mut builder = Self::empty();
        builder.shard_id = Some(shard_id);
        builder.from_peer_id = Some(from_peer_id);
        builder.to_peer_id = Some(to_peer_id);
        builder
    }
}

impl ReplicateShardBuilder {
    pub fn new(shard_id: u32, from_peer_id: u64, to_peer_id: u64) -> Self {
        let mut builder = Self::empty();
        builder.shard_id = Some(shard_id);
        builder.from_peer_id = Some(from_peer_id);
        builder.to_peer_id = Some(to_peer_id);
        builder
    }
}

impl AbortShardTransferBuilder {
    pub fn new(shard_id: u32, from_peer_id: u64, to_peer_id: u64) -> Self {
        let mut builder = Self::empty();
        builder.shard_id = Some(shard_id);
        builder.from_peer_id = Some(from_peer_id);
        builder.to_peer_id = Some(to_peer_id);
        builder
    }
}

impl ReplicaBuilder {
    pub fn new(shard_id: u32, peer_id: u64) -> Self {
        let mut builder = Self::empty();
        builder.shard_id = Some(shard_id);
        builder.peer_id = Some(peer_id);
        builder
    }
}

impl CreateShardKeyRequestBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl DeleteShardKeyRequestBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }

    /// Shard key to delete
    pub fn key(mut self, key: impl Into<shard_key::Key>) -> Self {
        self.request = Some(Some(DeleteShardKey {
            shard_key: Some(ShardKey {
                key: Some(key.into()),
            }),
        }));
        self
    }
}

impl DeleteCollectionBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl PayloadIncludeSelector {
    pub fn new(fileds: impl Into<Vec<String>>) -> Self {
        Self {
            fields: fileds.into(),
        }
    }
}

impl PayloadExcludeSelector {
    pub fn new(fileds: impl Into<Vec<String>>) -> Self {
        Self {
            fields: fileds.into(),
        }
    }
}

impl VectorsSelector {
    pub fn new(names: impl Into<Vec<String>>) -> Self {
        Self {
            names: names.into(),
        }
    }
}

impl RecommendPointsBuilder {
    /// Look for vectors closest to the vectors from these points or vectors
    pub fn add_positive(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.positive.get_or_insert_with(Vec::new).push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.positive_vectors
                    .get_or_insert_with(Vec::new)
                    .push(vector);
            }
        }
        self
    }

    /// Try to avoid vectors like the vector from these points or vectors
    pub fn add_negative(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.negative.get_or_insert_with(Vec::new).push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.negative_vectors
                    .get_or_insert_with(Vec::new)
                    .push(vector);
            }
        }
        self
    }
}

impl RecommendPointGroupsBuilder {
    /// Look for vectors closest to the vectors from these points or vectors
    pub fn add_positive(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.positive.get_or_insert_with(Vec::new).push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.positive_vectors
                    .get_or_insert_with(Vec::new)
                    .push(vector);
            }
        }
        self
    }

    /// Try to avoid vectors like the vector from these points or vectors
    pub fn add_negative(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.negative.get_or_insert_with(Vec::new).push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.negative_vectors
                    .get_or_insert_with(Vec::new)
                    .push(vector);
            }
        }
        self
    }
}

impl CreateCollectionBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        Self::default().collection_name(collection_name)
    }
}

impl CreateAliasBuilder {
    pub fn new(collection_name: impl Into<String>, alias_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.alias_name = Some(alias_name.into());
        builder
    }
}

impl RenameAliasBuilder {
    pub fn new(old_alias_name: impl Into<String>, new_alias_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.old_alias_name = Some(old_alias_name.into());
        builder.new_alias_name = Some(new_alias_name.into());
        builder
    }
}

impl QueryPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl QueryBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        query_points: impl Into<Vec<QueryPoints>>,
    ) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.query_points = Some(query_points.into());
        builder
    }
}

impl DeleteSnapshotRequestBuilder {
    pub fn new(collection_name: impl Into<String>, snapshot_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.snapshot_name = Some(snapshot_name.into());
        builder
    }
}

impl QueryPointGroupsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        group_by: impl Into<String>,
    ) -> QueryPointGroupsBuilder {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.group_by = Some(group_by.into());
        builder
    }
}

impl FacetCountsBuilder {
    pub fn new(collection_name: impl Into<String>, key: impl Into<String>) -> FacetCountsBuilder {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.key = Some(key.into());
        builder
    }
}

impl SearchMatrixPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> SearchMatrixPointsBuilder {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}
