use crate::qdrant::*;

#[derive(Clone)]
pub struct KeywordIndexParamsBuilder {
    /// If true - used for tenant optimization.
    pub(crate) is_tenant: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl Default for KeywordIndexParamsBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl KeywordIndexParamsBuilder {
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

    fn build_inner(self) -> Result<KeywordIndexParams, std::convert::Infallible> {
        Ok(KeywordIndexParams {
            is_tenant: self.is_tenant.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
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

impl From<KeywordIndexParamsBuilder> for KeywordIndexParams {
    fn from(value: KeywordIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "KeywordIndexParamsBuilder", "KeywordIndexParams"
            )
        })
    }
}

impl KeywordIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> KeywordIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "KeywordIndexParamsBuilder", "KeywordIndexParams"
            )
        })
    }
}
