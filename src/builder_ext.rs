use crate::builder_types::RecommendExample;
use crate::qdrant::{
    shard_key, BinaryQuantizationBuilder, ClearPayloadPointsBuilder, ContextExamplePair,
    CountPointsBuilder, CreateAliasBuilder, CreateCollectionBuilder,
    CreateFieldIndexCollectionBuilder, CreateShardKeyRequestBuilder, DeleteCollectionBuilder,
    DeleteFieldIndexCollectionBuilder, DeletePayloadPointsBuilder, DeletePointVectorsBuilder,
    DeletePointsBuilder, DeleteShardKey, DeleteShardKeyRequestBuilder, DiscoverBatchPointsBuilder,
    DiscoverPoints, DiscoverPointsBuilder, Distance, GetPointsBuilder, LookupLocationBuilder,
    OrderByBuilder, PayloadExcludeSelector, PayloadIncludeSelector, PointId, PointStruct,
    PointVectors, PointsUpdateOperation, ProductQuantizationBuilder, QuantizationType,
    RecommendBatchPointsBuilder, RecommendPointGroups, RecommendPointGroupsBuilder,
    RecommendPoints, RecommendPointsBuilder, RenameAliasBuilder, ScalarQuantizationBuilder,
    ScrollPointsBuilder, SearchBatchPointsBuilder, SearchPointGroupsBuilder, SearchPoints,
    SearchPointsBuilder, SetPayloadPointsBuilder, ShardKey, TextIndexParamsBuilder, TokenizerType,
    UpdateBatchPointsBuilder, UpdateCollectionBuilder, UpdateCollectionClusterSetupRequestBuilder,
    UpdatePointVectorsBuilder, UpsertPointsBuilder, Value, VectorParamsBuilder, VectorsSelector,
    WithLookupBuilder,
};
use std::collections::HashMap;

impl VectorParamsBuilder {
    pub fn new(size: u64, distance: Distance) -> Self {
        let mut builder = Self::empty();
        builder.size = Some(size);
        builder.distance = Some(distance.into());
        builder
    }
}

impl ScalarQuantizationBuilder {
    pub fn new(r#type: QuantizationType) -> Self {
        let mut builder = Self::empty();
        builder.r#type = Some(r#type.into());
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

impl OrderByBuilder {
    pub fn new(key: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.key = Some(key.into());
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
    pub fn new(collection_name: impl Into<String>, field_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
        builder.field_name = Some(field_name.into());
        builder
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
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::empty();
        builder.collection_name = Some(collection_name.into());
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

impl TextIndexParamsBuilder {
    pub fn new(tokenizer: TokenizerType) -> Self {
        let mut builder = Self::empty();
        builder.tokenizer = Some(tokenizer.into());
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
    pub fn positive(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
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
    pub fn negative(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
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

impl RecommendPointGroups {
    /// Look for vectors closest to the vectors from these points or vectors
    pub fn positive(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.positive.push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.positive_vectors.push(vector);
            }
        }
        self
    }

    /// Try to avoid vectors like the vector from these points or vectors
    pub fn negative(mut self, recommend_example: impl Into<RecommendExample>) -> Self {
        let recommend_example = recommend_example.into();
        match recommend_example {
            RecommendExample::PointId(point_id) => {
                self.negative.push(point_id);
            }
            RecommendExample::Vector(vector) => {
                self.negative_vectors.push(vector);
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
