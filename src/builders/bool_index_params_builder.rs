use crate::qdrant::*;

#[derive(Clone)]
pub struct BoolIndexParamsBuilder {
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl Default for BoolIndexParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl BoolIndexParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// If true - store index on disk.
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<BoolIndexParams, std::convert::Infallible> {
        Ok(BoolIndexParams {
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

impl From<BoolIndexParamsBuilder> for BoolIndexParams {
    fn from(value: BoolIndexParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "BoolIndexParamsBuilder", "BoolIndexParams"
            )
        })
    }
}

impl BoolIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> BoolIndexParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "BoolIndexParamsBuilder", "BoolIndexParams"
            )
        })
    }
}
