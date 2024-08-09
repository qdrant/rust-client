use crate::qdrant::{
    ContextInput, ContextInputBuilder, ContextInputPairBuilder, DiscoverInput,
    DiscoverInputBuilder, OrderBy, OrderByBuilder, PrefetchQuery, PrefetchQueryBuilder, Query,
    QueryPointsBuilder, RecommendInput, RecommendInputBuilder, VectorInput,
};

impl QueryPointsBuilder {
    pub fn add_prefetch(mut self, prefetch_query: impl Into<PrefetchQuery>) -> Self {
        match self.prefetch {
            Some(ref mut prefetch) => prefetch.push(prefetch_query.into()),
            None => self.prefetch = Some(vec![prefetch_query.into()]),
        }
        self
    }
}

impl PrefetchQueryBuilder {
    pub fn add_prefetch(mut self, prefetch_query: impl Into<PrefetchQuery>) -> Self {
        match self.prefetch {
            Some(ref mut prefetch) => prefetch.push(prefetch_query.into()),
            None => self.prefetch = Some(vec![prefetch_query.into()]),
        }
        self
    }
}

impl Query {
    pub fn new_nearest(value: impl Into<VectorInput>) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Nearest(value.into())),
        }
    }

    pub fn new_recommend(value: impl Into<RecommendInput>) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Recommend(value.into())),
        }
    }

    pub fn new_discover(value: impl Into<DiscoverInput>) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Discover(value.into())),
        }
    }

    pub fn new_context(value: impl Into<crate::qdrant::ContextInput>) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Context(value.into())),
        }
    }

    pub fn new_order_by(value: impl Into<OrderBy>) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::OrderBy(value.into())),
        }
    }

    pub fn new_fusion(value: crate::qdrant::Fusion) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Fusion(value.into())),
        }
    }

    pub fn new_sample(value: crate::qdrant::Sample) -> Self {
        Self {
            variant: Some(crate::qdrant::query::Variant::Sample(value.into())),
        }
    }
}

impl RecommendInputBuilder {
    pub fn add_positive(mut self, value: impl Into<VectorInput>) -> Self {
        match self.positive {
            Some(ref mut positive) => positive.push(value.into()),
            None => self.positive = Some(vec![value.into()]),
        }
        self
    }

    pub fn add_negative(mut self, value: impl Into<VectorInput>) -> Self {
        match self.negative {
            Some(ref mut negative) => negative.push(value.into()),
            None => self.negative = Some(vec![value.into()]),
        }
        self
    }
}

impl DiscoverInputBuilder {
    pub fn new(target: impl Into<VectorInput>, context: impl Into<ContextInput>) -> Self {
        let builder = Self::empty();
        builder.target(target).context(context)
    }
}

impl ContextInputPairBuilder {
    pub fn new(positive: impl Into<VectorInput>, negative: impl Into<VectorInput>) -> Self {
        ContextInputPairBuilder::empty()
            .positive(positive)
            .negative(negative)
    }
}

impl ContextInputBuilder {
    pub fn add_pair(
        mut self,
        positive: impl Into<VectorInput>,
        negative: impl Into<VectorInput>,
    ) -> Self {
        match self.pairs {
            Some(ref mut pairs) => {
                pairs.push(ContextInputPairBuilder::new(positive, negative).build())
            }
            None => {
                self.pairs = Some(vec![
                    ContextInputPairBuilder::new(positive, negative).build()
                ])
            }
        }
        self
    }
}

impl OrderByBuilder {
    pub fn new(key: impl Into<String>) -> Self {
        let builder = Self::empty();
        builder.key(key.into())
    }
}
