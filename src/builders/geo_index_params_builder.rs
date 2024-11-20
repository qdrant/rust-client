use crate::qdrant::*;

pub struct GeoIndexParamsBuilder {
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl GeoIndexParamsBuilder {
    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `GeoIndexParams`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<GeoIndexParams, std::convert::Infallible> {
        Ok(GeoIndexParams {
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
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
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "GeoIndexParamsBuilder", "GeoIndexParams",
        ))
    }
}

impl GeoIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> GeoIndexParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "GeoIndexParamsBuilder", "GeoIndexParams",
        ))
    }
}
