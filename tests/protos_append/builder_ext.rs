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
