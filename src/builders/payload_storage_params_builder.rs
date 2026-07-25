use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct PayloadStorageParamsBuilder {
    /// Memory placement of the payload storage.
    pub(crate) memory: Option<Option<i32>>,
}

impl Default for PayloadStorageParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl PayloadStorageParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Memory placement of the payload storage.
    /// Overrides the deprecated `on_disk_payload` flag if both are set.
    /// [`Memory::Pinned`] is not supported for payload storage.
    pub fn memory<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.memory = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<PayloadStorageParams, std::convert::Infallible> {
        Ok(PayloadStorageParams {
            memory: self.memory.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            memory: core::default::Default::default(),
        }
    }
}

impl From<PayloadStorageParamsBuilder> for PayloadStorageParams {
    fn from(value: PayloadStorageParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "PayloadStorageParamsBuilder", "PayloadStorageParams"
            )
        })
    }
}

impl PayloadStorageParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> PayloadStorageParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "PayloadStorageParamsBuilder", "PayloadStorageParams"
            )
        })
    }
}
