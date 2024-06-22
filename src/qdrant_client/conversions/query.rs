use crate::qdrant::{query, ContextInput, DiscoverInput, Fusion, OrderBy, Query, RecommendInput, VectorInput, OrderByBuilder};

impl From<VectorInput> for Query {
    fn from(value: VectorInput) -> Self {
        Self {
            variant: Some(query::Variant::Nearest(value)),
        }
    }
}

impl From<RecommendInput> for Query {
    fn from(value: RecommendInput) -> Self {
        Self {
            variant: Some(query::Variant::Recommend(value)),
        }
    }
}

impl From<DiscoverInput> for Query {
    fn from(value: DiscoverInput) -> Self {
        Self {
            variant: Some(query::Variant::Discover(value)),
        }
    }
}

impl From<ContextInput> for Query {
    fn from(value: ContextInput) -> Self {
        Self {
            variant: Some(query::Variant::Context(value)),
        }
    }
}

impl From<OrderBy> for Query {
    fn from(value: OrderBy) -> Self {
        Self {
            variant: Some(query::Variant::OrderBy(value)),
        }
    }
}

impl From<Fusion> for Query {
    fn from(value: Fusion) -> Self {
        Self {
            variant: Some(query::Variant::Fusion(value.into())),
        }
    }
}

impl<T: Into<String>> From<T> for OrderBy {
    fn from(value: T) -> Self {
        OrderByBuilder::new(value).build()
    }
}
