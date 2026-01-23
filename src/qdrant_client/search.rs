use crate::qdrant::{
    DiscoverBatchPoints, DiscoverBatchResponse, DiscoverPoints, DiscoverResponse,
    RecommendBatchPoints, RecommendBatchResponse, RecommendGroupsResponse, RecommendPointGroups,
    RecommendPoints, RecommendResponse, SearchBatchPoints, SearchBatchResponse,
    SearchGroupsResponse, SearchPointGroups, SearchPoints, SearchResponse,
};
use crate::qdrant_client::{Qdrant, QdrantResult};

/// # Search operations
///
/// <div class="warning">
/// For searching, please switch to the more fully featured <a href="#query-operations">Query API</a> instead. The search API will be removed in the future.
/// </div>
///
/// Search and explore points.
///
/// Documentation: <https://qdrant.tech/documentation/concepts/search/>
impl Qdrant {
    /// Search points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, SearchParamsBuilder, SearchPointsBuilder};
    ///
    ///# async fn search_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .search_points(
    ///         SearchPointsBuilder::new("my_collection", vec![0.2, 0.1, 0.9, 0.7], 3)
    ///             .filter(Filter::must([Condition::matches(
    ///                 "city",
    ///                 "London".to_string(),
    ///             )]))
    ///             .params(SearchParamsBuilder::default().hnsw_ef(128).exact(false)),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#search-api>
    pub async fn search_points(
        &self,
        request: impl Into<SearchPoints>,
    ) -> QdrantResult<SearchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Batch multiple points searches in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, SearchBatchPointsBuilder, SearchPointsBuilder,};
    ///
    ///# async fn search_batch_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let filter = Filter::must([Condition::matches("city", "London".to_string())]);
    ///
    /// let searches = vec![
    ///     SearchPointsBuilder::new("my_collection", vec![0.2, 0.1, 0.9, 0.7], 3)
    ///         .filter(filter.clone())
    ///         .build(),
    ///     SearchPointsBuilder::new("my_collection", vec![0.5, 0.3, 0.2, 0.3], 3)
    ///         .filter(filter)
    ///         .build(),
    /// ];
    ///
    /// client
    ///     .search_batch_points(SearchBatchPointsBuilder::new("my_collection", searches))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#batch-search-api>
    pub async fn search_batch_points(
        &self,
        request: impl Into<SearchBatchPoints>,
    ) -> QdrantResult<SearchBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Search points in a collection and group results by a payload field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::SearchPointGroupsBuilder;
    ///
    ///# async fn search_points(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .search_groups(SearchPointGroupsBuilder::new(
    ///         "my_collection", // Collection name
    ///         vec![1.1],       // Search vector
    ///         4,               // Search limit
    ///         "document_id",   // Group by field
    ///         2,               // Group size
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/search/#search-groups>
    pub async fn search_groups(
        &self,
        request: impl Into<SearchPointGroups>,
    ) -> QdrantResult<SearchGroupsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.search_groups(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Recommend points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, RecommendPointsBuilder, RecommendStrategy};
    ///
    ///# async fn recommend(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .recommend(
    ///         RecommendPointsBuilder::new("my_collection", 3)
    ///             .add_positive(100)
    ///             .add_positive(200)
    ///             .add_positive(vec![100.0, 231.0])
    ///             .add_negative(718)
    ///             .add_negative(vec![0.2, 0.3, 0.4, 0.5])
    ///             .strategy(RecommendStrategy::AverageVector)
    ///             .filter(Filter::must([Condition::matches(
    ///                 "city",
    ///                 "London".to_string(),
    ///             )])),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#recommendation-api>
    pub async fn recommend(
        &self,
        request: impl Into<RecommendPoints>,
    ) -> QdrantResult<RecommendResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Batch multiple points recommendations in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{Condition, Filter, RecommendBatchPointsBuilder, RecommendPointsBuilder};
    ///
    ///# async fn recommend_batch(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let filter = Filter::must([Condition::matches("city", "London".to_string())]);
    ///
    /// let recommend_queries = vec![
    ///     RecommendPointsBuilder::new("my_collection", 3)
    ///         .add_positive(100)
    ///         .add_positive(231)
    ///         .add_negative(718)
    ///         .filter(filter.clone())
    ///         .build(),
    ///     RecommendPointsBuilder::new("my_collection", 3)
    ///         .add_positive(200)
    ///         .add_positive(67)
    ///         .add_negative(300)
    ///         .filter(filter.clone())
    ///         .build(),
    /// ];
    ///
    /// client
    ///     .recommend_batch(RecommendBatchPointsBuilder::new(
    ///         "my_collection",
    ///         recommend_queries,
    ///     ))
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#batch-recommendation-api>
    pub async fn recommend_batch(
        &self,
        request: impl Into<RecommendBatchPoints>,
    ) -> QdrantResult<RecommendBatchResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Recommend points in a collection and group results by a payload field.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{RecommendPointGroupsBuilder, RecommendStrategy};
    ///
    ///# async fn recommend_groups(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .recommend_groups(
    ///         RecommendPointGroupsBuilder::new(
    ///             "my_collection", // Collection name
    ///             "document_id",   // Group by field
    ///             2,               // Group size
    ///             3,               // Search limit
    ///         )
    ///         .add_positive(100)
    ///         .add_positive(200)
    ///         .add_negative(718)
    ///         .strategy(RecommendStrategy::AverageVector),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#recommendation-api>
    pub async fn recommend_groups(
        &self,
        request: impl Into<RecommendPointGroups>,
    ) -> QdrantResult<RecommendGroupsResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.recommend_groups(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Discover points in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{
    ///     target_vector::Target, vector_example::Example, ContextExamplePairBuilder,
    ///     DiscoverPointsBuilder, VectorExample,
    /// };
    ///
    ///# async fn discover(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// client
    ///     .discover(
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection", // Collection name
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(718.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(200.into()))
    ///                     .negative(Example::Id(300.into()))
    ///                     .build(),
    ///             ],
    ///             10,              // Search limit
    ///         )
    ///         .target(Target::Single(VectorExample {
    ///             example: Some(Example::Vector(vec![0.2, 0.1, 0.9, 0.7].into())),
    ///         })),
    ///     )
    ///     .await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#discovery-api>
    pub async fn discover(
        &self,
        request: impl Into<DiscoverPoints>,
    ) -> QdrantResult<DiscoverResponse> {
        let request = &request.into();

        self.with_points_client(|mut points_api| async move {
            let result = points_api.discover(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }

    /// Batch multiple point discoveries in a collection.
    ///
    /// ```no_run
    ///# use qdrant_client::{Qdrant, QdrantError};
    /// use qdrant_client::qdrant::{
    ///     vector_example::Example, ContextExamplePairBuilder, DiscoverBatchPointsBuilder,
    ///     DiscoverPointsBuilder,
    /// };
    ///
    ///# async fn discover_batch(client: &Qdrant)
    ///# -> Result<(), QdrantError> {
    /// let discover_points = DiscoverBatchPointsBuilder::new(
    ///     "my_collection",
    ///     vec![
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(718.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(200.into()))
    ///                     .negative(Example::Id(300.into()))
    ///                     .build(),
    ///             ],
    ///             10,
    ///         )
    ///         .build(),
    ///         DiscoverPointsBuilder::new(
    ///             "my_collection",
    ///             vec![
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(342.into()))
    ///                     .negative(Example::Id(213.into()))
    ///                     .build(),
    ///                 ContextExamplePairBuilder::default()
    ///                     .positive(Example::Id(100.into()))
    ///                     .negative(Example::Id(200.into()))
    ///                     .build(),
    ///             ],
    ///             10,
    ///         )
    ///         .build(),
    ///     ],
    /// );
    ///
    /// client.discover_batch(&discover_points.build()).await?;
    ///# Ok(())
    ///# }
    /// ```
    ///
    /// Documentation: <https://qdrant.tech/documentation/concepts/explore/#discovery-api>
    pub async fn discover_batch(
        &self,
        request: &DiscoverBatchPoints,
    ) -> QdrantResult<DiscoverBatchResponse> {
        self.with_points_client(|mut points_api| async move {
            let result = points_api.discover_batch(request.clone()).await?;
            Ok(result.into_inner())
        })
        .await
    }
}
