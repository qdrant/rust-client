use crate::qdrant::*;

#[derive(Clone)]
pub struct FloatIndexParamsBuilder {
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    pub(crate) is_principal: Option<Option<bool>>,
}

impl Default for FloatIndexParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl FloatIndexParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// If true - store index on disk.
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    pub fn is_principal(self, value: bool) -> Self {
        let mut new = self;
        new.is_principal = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<FloatIndexParams, std::convert::Infallible> {
        Ok(FloatIndexParams {
            on_disk: self.on_disk.unwrap_or_default(),
            is_principal: self.is_principal.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            on_disk: core::default::Default::default(),
            is_principal: core::default::Default::default(),
        }
    }
}

impl From<FloatIndexParamsBuilder> for FloatIndexParams {
    fn from(value: FloatIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "FloatIndexParamsBuilder", "FloatIndexParams"
            )
        })
    }
}

impl FloatIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> FloatIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "FloatIndexParamsBuilder", "FloatIndexParams"
            )
        })
    }
}
