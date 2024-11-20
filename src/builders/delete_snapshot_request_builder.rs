use crate::qdrant::*;

pub struct DeleteSnapshotRequestBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Name of the collection snapshot
    pub(crate) snapshot_name: Option<String>,
}

impl DeleteSnapshotRequestBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Name of the collection snapshot
    #[allow(unused_mut)]
    pub fn snapshot_name(self, value: String) -> Self {
        let mut new = self;
        new.snapshot_name = Option::Some(value);
        new
    }
    /**Builds a new `DeleteSnapshotRequest`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<DeleteSnapshotRequest, DeleteSnapshotRequestBuilderError> {
        Ok(DeleteSnapshotRequest {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            snapshot_name: match self.snapshot_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("snapshot_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            snapshot_name: core::default::Default::default(),
        }
    }
}

impl From<DeleteSnapshotRequestBuilder> for DeleteSnapshotRequest {
    fn from(value: DeleteSnapshotRequestBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "DeleteSnapshotRequestBuilder", "DeleteSnapshotRequest",
        ))
    }
}

impl DeleteSnapshotRequestBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> DeleteSnapshotRequest {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "DeleteSnapshotRequestBuilder", "DeleteSnapshotRequest",
        ))
    }
}

impl DeleteSnapshotRequestBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
