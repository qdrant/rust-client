use crate::qdrant::*;

#[derive(Clone)]
pub struct PrefetchQueryBuilder {
    /// Sub-requests to perform first. If present, the query will be performed on the results of the prefetches.
    pub(crate) prefetch: Option<Vec<PrefetchQuery>>,
    /// Query to perform. If missing, returns points ordered by their IDs.
    pub(crate) query: Option<Option<Query>>,
    /// Define which vector to use for querying. If missing, the default vector is is used.
    pub(crate) using: Option<Option<String>>,
    /// Filter conditions - return only those points that satisfy the specified conditions.
    pub(crate) filter: Option<Option<Filter>>,
    /// Search params for when there is no prefetch.
    pub(crate) params: Option<Option<SearchParams>>,
    /// Return points with scores better than this threshold.
    pub(crate) score_threshold: Option<Option<f32>>,
    /// Max number of points. Default is 10
    pub(crate) limit: Option<Option<u64>>,
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    pub(crate) lookup_from: Option<Option<LookupLocation>>,
}

impl PrefetchQueryBuilder {
    /// Sub-requests to perform first. If present, the query will be performed on the results of the prefetches.
    #[allow(unused_mut)]
    pub fn prefetch<VALUE: core::convert::Into<Vec<PrefetchQuery>>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.prefetch = Option::Some(value.into());
        new
    }
    /// Query to perform. If missing, returns points ordered by their IDs.
    #[allow(unused_mut)]
    pub fn query<VALUE: core::convert::Into<Query>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.query = Option::Some(Option::Some(value.into()));
        new
    }
    /// Define which vector to use for querying. If missing, the default vector is is used.
    #[allow(unused_mut)]
    pub fn using<VALUE: core::convert::Into<String>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.using = Option::Some(Option::Some(value.into()));
        new
    }
    /// Filter conditions - return only those points that satisfy the specified conditions.
    #[allow(unused_mut)]
    pub fn filter<VALUE: core::convert::Into<Filter>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.filter = Option::Some(Option::Some(value.into()));
        new
    }
    /// Search params for when there is no prefetch.
    #[allow(unused_mut)]
    pub fn params<VALUE: core::convert::Into<SearchParams>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.params = Option::Some(Option::Some(value.into()));
        new
    }
    /// Return points with scores better than this threshold.
    #[allow(unused_mut)]
    pub fn score_threshold<VALUE: core::convert::Into<f32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.score_threshold = Option::Some(Option::Some(value.into()));
        new
    }
    /// Max number of points. Default is 10
    #[allow(unused_mut)]
    pub fn limit<VALUE: core::convert::Into<u64>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.limit = Option::Some(Option::Some(value.into()));
        new
    }
    /// The location to use for IDs lookup, if not specified - use the current collection and the 'using' vector
    #[allow(unused_mut)]
    pub fn lookup_from<VALUE: core::convert::Into<LookupLocation>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup_from = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<PrefetchQuery, std::convert::Infallible> {
        Ok(PrefetchQuery {
            prefetch: self.prefetch.unwrap_or_default(),
            query: self.query.unwrap_or_default(),
            using: self.using.unwrap_or_default(),
            filter: self.filter.unwrap_or_default(),
            params: self.params.unwrap_or_default(),
            score_threshold: self.score_threshold.unwrap_or_default(),
            limit: self.limit.unwrap_or_default(),
            lookup_from: self.lookup_from.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            prefetch: core::default::Default::default(),
            query: core::default::Default::default(),
            using: core::default::Default::default(),
            filter: core::default::Default::default(),
            params: core::default::Default::default(),
            score_threshold: core::default::Default::default(),
            limit: core::default::Default::default(),
            lookup_from: core::default::Default::default(),
        }
    }
}

impl From<PrefetchQueryBuilder> for PrefetchQuery {
    fn from(value: PrefetchQueryBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "PrefetchQueryBuilder", "PrefetchQuery"
            )
        })
    }
}

impl PrefetchQueryBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> PrefetchQuery {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "PrefetchQueryBuilder", "PrefetchQuery"
            )
        })
    }
}

impl Default for PrefetchQueryBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
