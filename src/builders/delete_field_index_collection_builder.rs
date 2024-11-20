use crate::qdrant::*;

pub struct DeleteFieldIndexCollectionBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Field name to delete
    pub(crate) field_name: Option<String>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
}

impl DeleteFieldIndexCollectionBuilder {
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
    /// Field name to delete
    #[allow(unused_mut)]
    pub fn field_name(self, value: String) -> Self {
        let mut new = self;
        new.field_name = Option::Some(value);
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering(self, value: WriteOrdering) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `DeleteFieldIndexCollection`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(
        self,
    ) -> Result<DeleteFieldIndexCollection, DeleteFieldIndexCollectionBuilderError> {
        Ok(DeleteFieldIndexCollection {
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
            field_name: match self.field_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("field_name"),
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
            field_name: core::default::Default::default(),
            ordering: core::default::Default::default(),
        }
    }
}

impl From<DeleteFieldIndexCollectionBuilder> for DeleteFieldIndexCollection {
    fn from(value: DeleteFieldIndexCollectionBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DeleteFieldIndexCollectionBuilder", "DeleteFieldIndexCollection",
        ))
    }
}

impl DeleteFieldIndexCollectionBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteFieldIndexCollection {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DeleteFieldIndexCollectionBuilder", "DeleteFieldIndexCollection",
        ))
    }
}

impl DeleteFieldIndexCollectionBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
