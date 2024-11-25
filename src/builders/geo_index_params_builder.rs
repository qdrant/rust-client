use crate::qdrant::*;

pub struct GeoIndexParamsBuilder {
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl Default for GeoIndexParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl GeoIndexParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<GeoIndexParams, std::convert::Infallible> {
        Ok(GeoIndexParams {
            on_disk: self.on_disk.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            on_disk: core::default::Default::default(),
        }
    }
}

impl From<GeoIndexParamsBuilder> for GeoIndexParams {
    fn from(value: GeoIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "GeoIndexParamsBuilder", "GeoIndexParams"
            )
        })
    }
}

impl GeoIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> GeoIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "GeoIndexParamsBuilder", "GeoIndexParams"
            )
        })
    }
}
