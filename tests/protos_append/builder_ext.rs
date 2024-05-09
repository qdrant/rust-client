// import our manual builder here so all builder come from the same module in the end user API.
pub use crate::manual_builder::*;

// Needs special treatment as we can't generate this because DeletePoints is specified using a path and not
// only by it's identifyer.
builder_type_conversions!(DeletePoints, DeletePointsBuilder);

use std::collections::HashMap;

impl VectorParamsBuilder {
    pub fn new(size: u64, distance: Distance) -> Self {
        let mut builder = Self::create_empty();
        builder.size = Some(size);
        builder.distance = Some(distance.into());
        builder
    }
}

impl ScalarQuantizationBuilder {
    pub fn new(r#type: QuantizationType) -> Self {
        let mut builder = Self::create_empty();
        builder.r#type = Some(r#type.into());
        builder
    }
}

impl ProductQuantizationBuilder {
    pub fn new(compression: i32) -> Self {
        let mut builder = Self::create_empty();
        builder.compression = Some(compression);
        builder
    }
}

impl BinaryQuantizationBuilder {
    pub fn new(always_ram: bool) -> Self {
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.vector = Some(vector.into());
        builder.limit = Some(limit);
        builder
    }
}

impl UpdateCollectionBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl SetPayloadPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        payload: impl Into<HashMap<String, Value>>,
    ) -> Self {
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.operations = Some(operations.into());
        builder
    }
}

impl DeletePayloadPointsBuilder {
    pub fn new(collection_name: impl Into<String>, keys: impl Into<Vec<String>>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.keys = Some(keys.into());
        builder
    }
}

impl ClearPayloadPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl GetPointsBuilder {
    pub fn new(collection_name: impl Into<String>, ids: impl Into<Vec<PointId>>) -> Self {
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
        builder.collection = Some(collection.into());
        builder
    }
}

impl DeletePointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl DeletePointVectorsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl UpdatePointVectorsBuilder {
    pub fn new(collection_name: impl Into<String>, points: impl Into<Vec<PointVectors>>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.points = Some(points.into());
        builder
    }
}

impl ScrollPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl OrderByBuilder {
    pub fn new(key: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.key = Some(key.into());
        builder
    }
}

impl RecommendPointsBuilder {
    pub fn new(collection_name: impl Into<String>, limit: u64) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.limit = Some(limit);
        builder
    }
}

impl LookupLocationBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl RecommendBatchPointsBuilder {
    pub fn new(
        collection_name: impl Into<String>,
        recommend_points: impl Into<Vec<RecommendPoints>>,
    ) -> Self {
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
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
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.discover_points = Some(discover_points.into());
        builder
    }
}

impl CountPointsBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl UpsertPointsBuilder {
    pub fn new(collection_name: impl Into<String>, points: impl Into<Vec<PointStruct>>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.points = Some(points.into());
        builder
    }
}

impl CreateFieldIndexCollectionBuilder {
    pub fn new(collection_name: impl Into<String>, field_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.field_name = Some(field_name.into());
        builder
    }
}

impl DeleteFieldIndexCollectionBuilder {
    pub fn new(collection_name: impl Into<String>, field_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder.field_name = Some(field_name.into());
        builder
    }
}

impl UpdateCollectionClusterSetupRequestBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl CreateShardKeyRequestBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl DeleteShardKeyRequestBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}

impl DeleteCollectionBuilder {
    pub fn new(collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.collection_name = Some(collection_name.into());
        builder
    }
}
