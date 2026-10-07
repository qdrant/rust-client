use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct IdTrackerParamsBuilder {
    /// Memory placement of the point id mapping in indexed segments.
    pub(crate) memory: Option<Option<i32>>,
}

impl Default for IdTrackerParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IdTrackerParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Memory placement of the point id mapping in indexed segments:
    /// [`Memory::Cold`] keeps it on disk and reads it on demand, [`Memory::Cached`] keeps it on disk
    /// but primes the page cache with it on load, [`Memory::Pinned`] keeps it in RAM.
    pub fn memory<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.memory = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<IdTrackerParams, std::convert::Infallible> {
        Ok(IdTrackerParams {
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

impl From<IdTrackerParamsBuilder> for IdTrackerParams {
    fn from(value: IdTrackerParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "IdTrackerParamsBuilder", "IdTrackerParams"
            )
        })
    }
}

impl IdTrackerParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> IdTrackerParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "IdTrackerParamsBuilder", "IdTrackerParams"
            )
        })
    }
}
