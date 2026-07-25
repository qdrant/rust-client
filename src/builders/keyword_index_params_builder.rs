use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct KeywordIndexParamsBuilder {
    /// If true - used for tenant optimization.
    pub(crate) is_tenant: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
    /// If true - enable HNSW index for this field.
    pub(crate) enable_hnsw: Option<Option<bool>>,
    /// If set - enable prefix matching on this field.
    pub(crate) prefix: Option<Option<KeywordPrefixParams>>,
    /// Memory placement of the index.
    pub(crate) memory: Option<Option<i32>>,
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
    #[deprecated(since = "1.19.0", note = "use `memory` instead")]
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
    /// If true - enable prefix matching (`Condition::matches_prefix`) on this field.
    pub fn prefix(self, value: bool) -> Self {
        let mut new = self;
        new.prefix = Option::Some(value.then(KeywordPrefixParams::default));
        new
    }
    /// Memory placement of the index.
    /// Overrides the deprecated `on_disk` flag if both are set.
    pub fn memory<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.memory = Option::Some(Option::Some(value.into()));
        new
    }

    #[allow(deprecated)]
    fn build_inner(self) -> Result<KeywordIndexParams, std::convert::Infallible> {
        Ok(KeywordIndexParams {
            is_tenant: self.is_tenant.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
            enable_hnsw: self.enable_hnsw.unwrap_or_default(),
            prefix: self.prefix.unwrap_or_default(),
            memory: self.memory.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            is_tenant: core::default::Default::default(),
            on_disk: core::default::Default::default(),
            enable_hnsw: core::default::Default::default(),
            prefix: core::default::Default::default(),
            memory: core::default::Default::default(),
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
