use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct SearchBatchPointsBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    pub(crate) search_points: Option<Vec<SearchPoints>>,
    /// Options for specifying read consistency guarantees
    read_consistency: Option<read_consistency::Value>,
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    pub(crate) timeout: Option<Option<u64>>,
}

impl SearchBatchPointsBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    #[allow(unused_mut)]
    pub fn search_points(self, value: Vec<SearchPoints>) -> Self {
        let mut new = self;
        new.search_points = Option::Some(value);
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
    /// If set, overrides global timeout setting for this request. Unit is seconds.
    #[allow(unused_mut)]
    pub fn timeout(self, value: u64) -> Self {
        let mut new = self;
        new.timeout = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `SearchBatchPoints`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<SearchBatchPoints, SearchBatchPointsBuilderError> {
        Ok(SearchBatchPoints {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            search_points: match self.search_points {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("search_points"),
                    ));
                }
            },
            read_consistency: { convert_option(&self.read_consistency) },
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
            search_points: core::default::Default::default(),
            read_consistency: core::default::Default::default(),
            timeout: core::default::Default::default(),
        }
    }
}

impl From<SearchBatchPointsBuilder> for SearchBatchPoints {
    fn from(value: SearchBatchPointsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "SearchBatchPointsBuilder", "SearchBatchPoints",
        ))
    }
}

impl SearchBatchPointsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SearchBatchPoints {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "SearchBatchPointsBuilder", "SearchBatchPoints",
        ))
    }
}

impl SearchBatchPointsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
