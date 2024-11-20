use crate::qdrant::*;

pub struct UpdateBatchPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    pub(crate) operations: Option<Vec<PointsUpdateOperation>>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
}

impl UpdateBatchPointsBuilder {
    /// name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Wait until the changes have been applied?
    #[allow(unused_mut)]
    pub fn wait(self, value: bool) -> Self {
        let mut new = self;
        new.wait = Option::Some(Option::Some(value));
        new
    }
    #[allow(unused_mut)]
    pub fn operations(self, value: Vec<PointsUpdateOperation>) -> Self {
        let mut new = self;
        new.operations = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `UpdateBatchPoints`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<UpdateBatchPoints, UpdateBatchPointsBuilderError> {
        Ok(UpdateBatchPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            wait: match self.wait {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            operations: match self.operations {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("operations"),
                    ));
                }
            },
            ordering: match self.ordering {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            wait: core::default::Default::default(),
            operations: core::default::Default::default(),
            ordering: core::default::Default::default(),
        }
    }
}

impl From<UpdateBatchPointsBuilder> for UpdateBatchPoints {
    fn from(value: UpdateBatchPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "UpdateBatchPointsBuilder", "UpdateBatchPoints",
        ))
    }
}

impl UpdateBatchPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> UpdateBatchPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "UpdateBatchPointsBuilder", "UpdateBatchPoints",
        ))
    }
}

impl UpdateBatchPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
