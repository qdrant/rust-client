use crate::qdrant::*;

#[must_use]
#[derive(Clone)]
pub struct IdfParamsBuilder {
    /// Filter defining the corpus IDF statistics are computed over.
    pub(crate) corpus: Option<Option<Filter>>,
}

impl Default for IdfParamsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl IdfParamsBuilder {
    pub fn new() -> Self {
        Self::create_empty()
    }

    /// Filter defining the corpus: IDF statistics are computed over the points matching
    /// this filter. If unset, statistics are collection-wide (global).
    pub fn corpus<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.corpus = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<IdfParams, std::convert::Infallible> {
        Ok(IdfParams {
            corpus: self.corpus.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            corpus: core::default::Default::default(),
        }
    }
}

impl From<IdfParamsBuilder> for IdfParams {
    fn from(value: IdfParamsBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "IdfParamsBuilder", "IdfParams"
            )
        })
    }
}

impl IdfParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> IdfParams {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "IdfParamsBuilder", "IdfParams"
            )
        })
    }
}
