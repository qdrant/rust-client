use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct CountPointsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    pub(crate) filter: Option<Option<Filter>>,
    /// If `true` - return exact count, if `false` - return approximate count
    pub(crate) exact: Option<Option<bool>>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// Specify in which shards to look for the points, if not specified - look in all shards
    pub(crate) shard_key_selector: Option<Option<ShardKeySelector>>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl CountPointsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// If `true` - return exact count, if `false` - return approximate count
    #[allow(unused_mut)]
    pub fn exact(self, value: bool) -> Self {
        let mut new = self;
        new.exact = Option::Some(Option::Some(value));
        new
    }
    /// Options for specifying read consistency guarantees
    #[allow(unused_mut)]
    pub fn read_consistency<VALUE: core::convert::Into<read_consistency::Value>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.read_consistency = Option::Some(value.into());
        new
    }
    /// Specify in which shards to look for the points, if not specified - look in all shards
    #[allow(unused_mut)]
    pub fn shard_key_selector<VALUE: core::convert::Into<ShardKeySelector>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.shard_key_selector = Option::Some(Option::Some(value.into()));
        new
    }
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `CountPoints`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<CountPoints, CountPointsBuilderError> {
        Ok(CountPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            filter: match self.filter {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            exact: match self.exact {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            read_consistency: { convert_option(&self.read_consistency) },
            shard_key_selector: match self.shard_key_selector {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            timeout: match self.timeout {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            filter: core::default::Default::default(),
            exact: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            shard_key_selector: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<CountPointsBuilder> for CountPoints {
    fn from(value: CountPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CountPointsBuilder", "CountPoints",
        ))
    }
}

impl CountPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CountPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CountPointsBuilder", "CountPoints",
        ))
    }
}

impl CountPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
