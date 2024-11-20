use crate::qdrant::*;

pub struct UuidIndexParamsBuilder {
    /// If true - used for tenant optimization.
    pub(crate) is_tenant: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl UuidIndexParamsBuilder {
    /// If true - used for tenant optimization.
    #[allow(unused_mut)]
    pub fn is_tenant(self, value: bool) -> Self {
        let mut new = self;
        new.is_tenant = Option::Some(Option::Some(value));
        new
    }
    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<UuidIndexParams, std::convert::Infallible> {
        Ok(UuidIndexParams {
            is_tenant: match self.is_tenant {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            is_tenant: core::default::Default::default(),
            on_disk: core::default::Default::default(),
        }
    }
}

impl From<UuidIndexParamsBuilder> for UuidIndexParams {
    fn from(value: UuidIndexParamsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "UuidIndexParamsBuilder", "UuidIndexParams",
        ))
    }
}

impl UuidIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UuidIndexParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "UuidIndexParamsBuilder", "UuidIndexParams",
        ))
    }
}
