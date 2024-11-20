use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct ClearPayloadPointsBuilder {
    /// name of the collection
    pub(crate) collection_name: Option<String>,
    /// Wait until the changes have been applied?
    pub(crate) wait: Option<Option<bool>>,
    /// Affected points
    points: Option<points_selector::PointsSelectorOneOf>,
    /// Write ordering guarantees
    pub(crate) ordering: Option<Option<WriteOrdering>>,
    /// Option for custom sharding to specify used shard keys
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
}

impl ClearPayloadPointsBuilder {
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
    /// Affected points
    #[allow(unused_mut)]
    pub fn points<VALUE: core::convert::Into<points_selector::PointsSelectorOneOf>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.points = Option::Some(value.into());
        new
    }
    /// Write ordering guarantees
    #[allow(unused_mut)]
    pub fn ordering<VALUE: core::convert::Into<WriteOrdering>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.ordering = Option::Some(Option::Some(value.into()));
        new
    }
    /// Option for custom sharding to specify used shard keys
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /**Builds a new `ClearPayloadPoints`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<ClearPayloadPoints, ClearPayloadPointsBuilderError> {
        Ok(ClearPayloadPoints {
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
            points: { convert_option(&self.points) },
            ordering: match self.ordering {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            shard_key_selector: match self.shard_key_selector {
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
            points: core::default::Default::default(),
            ordering: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
        }
    }
}

impl From<ClearPayloadPointsBuilder> for ClearPayloadPoints {
    fn from(value: ClearPayloadPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ClearPayloadPointsBuilder", "ClearPayloadPoints",
        ))
    }
}

impl ClearPayloadPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ClearPayloadPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ClearPayloadPointsBuilder", "ClearPayloadPoints",
        ))
    }
}

impl ClearPayloadPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
