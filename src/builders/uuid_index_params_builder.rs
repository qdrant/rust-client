use crate::qdrant::*;

#[derive(Clone)]
pub struct UuidIndexParamsBuilder {
    /// If true - used for tenant optimization.
    pub(crate) is_tenant: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
    /// If true - enable HNSW index for this field.
    pub(crate) enable_hnsw: Option<Option<bool>>,
}

impl UuidIndexParamsBuilder {
    /// If true - used for tenant optimization.
    pub fn is_tenant(self, value: bool) -> Self {
        let mut new = self;
        new.is_tenant = Option::Some(Option::Some(value));
        new
    }
    /// If true - store index on disk.
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    /// If true - enable HNSW index for this field.
    pub fn enable_hnsw(self, value: bool) -> Self {
        let mut new = self;
        new.enable_hnsw = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<UuidIndexParams, std::convert::Infallible> {
        Ok(UuidIndexParams {
            is_tenant: self.is_tenant.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
            enable_hnsw: self.enable_hnsw.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            is_tenant: core::default::Default::default(),
            on_disk: core::default::Default::default(),
            enable_hnsw: core::default::Default::default(),
        }
    }
}

impl Default for UuidIndexParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl From<UuidIndexParamsBuilder> for UuidIndexParams {
    fn from(value: UuidIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "UuidIndexParamsBuilder", "UuidIndexParams"
            )
        })
    }
}

impl UuidIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UuidIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "UuidIndexParamsBuilder", "UuidIndexParams"
            )
        })
    }
}
