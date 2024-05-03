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
