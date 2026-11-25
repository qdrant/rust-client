//! Public models for the Qdrant Serverless collection management API.
//!
//! **In development — do not use yet.** Part of the experimental serverless client;
//! the API may change without notice.
//!
//! These mirror the tenant-facing serverless config: unlike the regular client's
//! collection models, they deliberately expose no storage internals (quantization,
//! WAL, segments, on-disk placement, ...) — the serverless manager decides those.

use std::collections::HashMap;

/// Distance metric used to compare dense vectors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Distance {
    Cosine,
    Euclid,
    Dot,
    Manhattan,
}

/// How much of the original vector precision may be traded for cost.
///
/// The manager turns this into a concrete quantization / datatype choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum PrecisionTier {
    Low,
    Medium,
    High,
    Full,
}

/// Full-text tokenizer, mirrors Qdrant's `TokenizerType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum Tokenizer {
    Prefix,
    Whitespace,
    Word,
    Multilingual,
    /// Split source code into identifiers and their camelCase / snake_case subwords.
    Code,
}

/// Configuration of a single dense (embedding) vector.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DenseVectorConfig {
    /// Dimensionality of the embedding, e.g. 512, 1536, 3072.
    pub size: u64,
    /// Distance metric used to compare vectors.
    pub distance: Distance,
    /// Store several sub-vectors per point and compare with max-sim.
    pub multivector: bool,
    /// Precision/cost trade-off for this vector. Unset: HIGH.
    pub precision_tier: Option<PrecisionTier>,
}

impl DenseVectorConfig {
    /// Create a dense vector config with the given size and distance.
    pub fn new(size: u64, distance: Distance) -> Self {
        Self {
            size,
            distance,
            multivector: false,
            precision_tier: None,
        }
    }

    /// Enable multi-vector (late-interaction) storage.
    pub fn multivector(mut self, multivector: bool) -> Self {
        self.multivector = multivector;
        self
    }

    /// Set the precision/cost trade-off.
    pub fn precision_tier(mut self, tier: PrecisionTier) -> Self {
        self.precision_tier = Some(tier);
        self
    }
}

/// Configuration of a single sparse vector.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SparseVectorConfig {
    /// Apply the IDF modifier at query time.
    pub use_idf: bool,
    /// Precision/cost trade-off for this vector. Unset: HIGH.
    pub precision_tier: Option<PrecisionTier>,
}

impl SparseVectorConfig {
    /// Create a sparse vector config.
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable IDF weighting (BM25-style).
    pub fn use_idf(mut self, use_idf: bool) -> Self {
        self.use_idf = use_idf;
        self
    }

    /// Set the precision/cost trade-off.
    pub fn precision_tier(mut self, tier: PrecisionTier) -> Self {
        self.precision_tier = Some(tier);
        self
    }
}

/// Prefix matching options for a keyword index. Presence enables prefix matching.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeywordPrefixParams;

/// Exact match on string values, e.g. `color: "red"`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeywordIndex {
    /// If set, enable prefix matching (`match: { "prefix": ... }`) on this field.
    pub prefix: Option<KeywordPrefixParams>,
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].color`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl KeywordIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Enable prefix matching on this keyword field.
    pub fn with_prefix(mut self) -> Self {
        self.prefix = Some(KeywordPrefixParams);
        self
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Exact match and/or range filters on integers. Both default to enabled.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerIndex {
    pub lookup: Option<bool>,
    pub range: Option<bool>,
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].price`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl IntegerIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookup(mut self, lookup: bool) -> Self {
        self.lookup = Some(lookup);
        self
    }

    pub fn range(mut self, range: bool) -> Self {
        self.range = Some(range);
        self
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Range filters on floating point (and integer) numbers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FloatIndex {
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].price`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl FloatIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Exact match on UUID strings; like keyword but stored compactly.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UuidIndex {
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].id`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl UuidIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Range filters on RFC 3339 datetimes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DatetimeIndex {
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].updated_at`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl DatetimeIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Tokens ignored by a full-text index.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StopwordsSet {
    /// Languages whose predefined stopword lists to apply (e.g. `"english"`).
    pub languages: Vec<String>,
    /// Extra stopwords to ignore, merged with the language lists.
    pub custom: Vec<String>,
}

impl StopwordsSet {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn languages(mut self, languages: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.languages = languages.into_iter().map(Into::into).collect();
        self
    }

    pub fn custom(mut self, custom: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.custom = custom.into_iter().map(Into::into).collect();
        self
    }
}

/// Snowball stemming for a full-text index.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SnowballParams {
    /// Language for the snowball algorithm, e.g. `"english"`.
    pub language: String,
}

impl SnowballParams {
    pub fn new(language: impl Into<String>) -> Self {
        Self {
            language: language.into(),
        }
    }
}

/// Stemming algorithm for a full-text index. Unset: no stemming.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum StemmingAlgorithm {
    /// Snowball stemmer for the given language.
    Snowball(SnowballParams),
    /// Explicitly disable stemming.
    Disabled,
}

/// How a text index ranks documents.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum TextScoringType {
    /// Okapi BM25. The index records each document's length; `k` and `b` are set per query.
    Bm25,
}

/// Parameters of ranking over a text index. Presence enables scoring.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextScoringParams {
    /// How documents are ranked. Unset: BM25.
    pub r#type: Option<TextScoringType>,
}

impl TextScoringParams {
    /// Scoring with the default type (BM25).
    pub fn new() -> Self {
        Self::default()
    }

    /// Okapi BM25 scoring.
    pub fn bm25() -> Self {
        Self {
            r#type: Some(TextScoringType::Bm25),
        }
    }
}

/// Full-text filtering on string values.
///
/// With [`TextIndex::bm25_scoring`] the field can also rank points by BM25, through
/// [`Query::new_text`](crate::qdrant::Query::new_text) with `using` set to the field name.
///
/// ```no_run
/// use qdrant_client::serverless::{
///     CollectionConfig, Query, QdrantServerless, QueryPointsBuilder, TextIndex,
///     TextQueryBuilder, Tokenizer,
/// };
///
/// # async fn run(client: QdrantServerless) -> Result<(), qdrant_client::QdrantError> {
/// client
///     .create_collection(
///         "docs",
///         CollectionConfig::new()
///             // prose, ranked by BM25
///             .payload_index(
///                 "description",
///                 TextIndex::new()
///                     .tokenizer(Tokenizer::Word)
///                     .lowercase(true)
///                     .bm25_scoring(),
///             )
///             // source code: `parseHttpRequest` is also indexed as `parse`, `http`, `request`
///             .payload_index(
///                 "snippet",
///                 TextIndex::new().tokenizer(Tokenizer::Code).bm25_scoring(),
///             ),
///     )
///     .await?;
///
/// let hits = client
///     .query(
///         QueryPointsBuilder::new("docs")
///             .query(Query::new_text(TextQueryBuilder::new("http request").k(1.5)))
///             .using("snippet")
///             .limit(10),
///     )
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextIndex {
    pub tokenizer: Option<Tokenizer>,
    pub lowercase: Option<bool>,
    pub phrase_matching: Option<bool>,
    pub min_token_len: Option<u64>,
    pub max_token_len: Option<u64>,
    pub ascii_folding: Option<bool>,
    pub stopwords: Option<StopwordsSet>,
    pub stemmer: Option<StemmingAlgorithm>,
    /// Enable ranking points by BM25 over this field. Changing it rebuilds the index.
    /// Default: disabled.
    pub scoring: Option<TextScoringParams>,
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].title`. Every query on the field treats each element as its own document:
    /// BM25 scores elements separately. Default: disabled.
    pub nested: Option<String>,
}

impl TextIndex {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tokenizer(mut self, tokenizer: Tokenizer) -> Self {
        self.tokenizer = Some(tokenizer);
        self
    }

    pub fn lowercase(mut self, lowercase: bool) -> Self {
        self.lowercase = Some(lowercase);
        self
    }

    pub fn phrase_matching(mut self, phrase_matching: bool) -> Self {
        self.phrase_matching = Some(phrase_matching);
        self
    }

    pub fn min_token_len(mut self, min_token_len: u64) -> Self {
        self.min_token_len = Some(min_token_len);
        self
    }

    pub fn max_token_len(mut self, max_token_len: u64) -> Self {
        self.max_token_len = Some(max_token_len);
        self
    }

    pub fn ascii_folding(mut self, ascii_folding: bool) -> Self {
        self.ascii_folding = Some(ascii_folding);
        self
    }

    pub fn stopwords(mut self, stopwords: StopwordsSet) -> Self {
        self.stopwords = Some(stopwords);
        self
    }

    pub fn stemmer(mut self, stemmer: StemmingAlgorithm) -> Self {
        self.stemmer = Some(stemmer);
        self
    }

    /// Enable ranking points over this field with the given scoring parameters.
    pub fn scoring(mut self, scoring: TextScoringParams) -> Self {
        self.scoring = Some(scoring);
        self
    }

    /// Enable ranking points by BM25 over this field.
    pub fn bm25_scoring(self) -> Self {
        self.scoring(TextScoringParams::bm25())
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Geo radius / bounding box / polygon filters on `{lon, lat}` values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoIndex {
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].location`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl GeoIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// Exact match on booleans.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolIndex {
    /// Index every object of this array as a separate entry, e.g. `offers` for the field
    /// `offers[].in_stock`. The field must be inside the array's elements. Default: disabled.
    pub nested: Option<String>,
}

impl BoolIndex {
    pub fn new() -> Self {
        Self::default()
    }

    /// Index every object of the given array as a separate entry, e.g. `offers`.
    pub fn nested(mut self, nested: impl Into<String>) -> Self {
        self.nested = Some(nested.into());
        self
    }
}

/// One payload index. Only the kind of filter the field supports is chosen;
/// storage placement of the index is the manager's decision.
///
/// Every index kind can be `nested`: each object of the named array is indexed as a separate
/// entry, so a [`Condition::nested`](crate::qdrant::Condition::nested) filter matches
/// conditions on the same element.
///
/// ```no_run
/// use qdrant_client::serverless::{
///     CollectionConfig, Condition, Filter, FloatIndex, KeywordIndex, QdrantServerless,
///     QueryPointsBuilder, Range,
/// };
///
/// # async fn run(client: QdrantServerless) -> Result<(), qdrant_client::QdrantError> {
/// // payload: {"offers": [{"color": "red", "price": 10.0}, {"color": "blue", "price": 25.0}]}
/// client
///     .create_collection(
///         "products",
///         CollectionConfig::new()
///             .payload_index("offers[].color", KeywordIndex::new().nested("offers"))
///             .payload_index("offers[].price", FloatIndex::new().nested("offers")),
///     )
///     .await?;
///
/// // points with a red offer cheaper than 20
/// let hits = client
///     .query(QueryPointsBuilder::new("products").filter(Filter::must([
///         Condition::nested(
///             "offers",
///             Filter::must([
///                 Condition::matches("color", "red".to_string()),
///                 Condition::range("price", Range { lt: Some(20.0), ..Default::default() }),
///             ]),
///         ),
///     ])))
///     .await?;
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(tag = "type", rename_all = "snake_case"))]
pub enum PayloadIndex {
    Keyword(KeywordIndex),
    Integer(IntegerIndex),
    Float(FloatIndex),
    Uuid(UuidIndex),
    Datetime(DatetimeIndex),
    Text(TextIndex),
    Geo(GeoIndex),
    Bool(BoolIndex),
}

impl From<KeywordIndex> for PayloadIndex {
    fn from(value: KeywordIndex) -> Self {
        Self::Keyword(value)
    }
}

impl From<IntegerIndex> for PayloadIndex {
    fn from(value: IntegerIndex) -> Self {
        Self::Integer(value)
    }
}

impl From<FloatIndex> for PayloadIndex {
    fn from(value: FloatIndex) -> Self {
        Self::Float(value)
    }
}

impl From<UuidIndex> for PayloadIndex {
    fn from(value: UuidIndex) -> Self {
        Self::Uuid(value)
    }
}

impl From<DatetimeIndex> for PayloadIndex {
    fn from(value: DatetimeIndex) -> Self {
        Self::Datetime(value)
    }
}

impl From<TextIndex> for PayloadIndex {
    fn from(value: TextIndex) -> Self {
        Self::Text(value)
    }
}

impl From<GeoIndex> for PayloadIndex {
    fn from(value: GeoIndex) -> Self {
        Self::Geo(value)
    }
}

impl From<BoolIndex> for PayloadIndex {
    fn from(value: BoolIndex) -> Self {
        Self::Bool(value)
    }
}

/// The tenant-facing collection config.
///
/// Vector maps are keyed by vector name; the empty name `""` is the unnamed
/// default vector. Payload indexes are keyed by payload field name
/// (JSON path, e.g. `user_id` or `meta.tags`).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionConfig {
    pub dense_vectors: HashMap<String, DenseVectorConfig>,
    pub sparse_vectors: HashMap<String, SparseVectorConfig>,
    pub payload_indexes: HashMap<String, PayloadIndex>,
}

impl CollectionConfig {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a single unnamed dense vector (empty name `""`).
    pub fn dense_vector(mut self, config: DenseVectorConfig) -> Self {
        self.dense_vectors.insert(String::new(), config);
        self
    }

    /// Register a named dense vector.
    pub fn named_dense_vector(
        mut self,
        name: impl Into<String>,
        config: DenseVectorConfig,
    ) -> Self {
        self.dense_vectors.insert(name.into(), config);
        self
    }

    /// Register a single unnamed sparse vector (empty name `""`).
    pub fn sparse_vector(mut self, config: SparseVectorConfig) -> Self {
        self.sparse_vectors.insert(String::new(), config);
        self
    }

    /// Register a named sparse vector.
    pub fn named_sparse_vector(
        mut self,
        name: impl Into<String>,
        config: SparseVectorConfig,
    ) -> Self {
        self.sparse_vectors.insert(name.into(), config);
        self
    }

    /// Add a payload index for the given field path.
    pub fn payload_index(
        mut self,
        field: impl Into<String>,
        index: impl Into<PayloadIndex>,
    ) -> Self {
        self.payload_indexes.insert(field.into(), index.into());
        self
    }
}

/// Result of [`super::QdrantServerless::create_collection`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CreateCollectionResult {
    /// Tenant-facing name of the collection.
    pub collection_name: String,
    /// Outcome, e.g. `"created"`.
    pub result: String,
    /// Time spent to process the request, in seconds.
    pub time: f64,
}

/// Result of [`super::QdrantServerless::delete_collection`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DeleteCollectionResult {
    /// Whether the collection existed and was deleted.
    pub deleted: bool,
    /// Time spent to process the request, in seconds.
    pub time: f64,
}

/// Optimization status of a collection, as in the regular API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum CollectionStatus {
    /// All segments are optimized.
    Green,
    /// Optimization is pending or running; the collection stays searchable.
    Yellow,
}

/// Whether acknowledged writes have been applied.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "snake_case"))]
pub enum UpdatesStatus {
    /// Every acknowledged write is applied and visible to search.
    Applied,
    /// Some acknowledged writes are still queued and not yet visible to search.
    Pending,
}

/// Progress of acknowledged writes to a collection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdatesInfo {
    pub status: UpdatesStatus,
}

/// A collection's configuration and stats, as returned by [`super::QdrantServerless::get_collection`].
///
/// `point_count` is eventually consistent and absent until stats have been
/// written for the collection.
///
/// ```no_run
/// use qdrant_client::serverless::{CollectionStatus, QdrantServerless, UpdatesStatus};
///
/// # async fn run(client: QdrantServerless) -> Result<(), qdrant_client::QdrantError> {
/// let info = client.get_collection("my-collection").await?;
/// println!(
///     "points: {:?}, stored bytes: {:?}, indexed vectors: {:?}",
///     info.point_count, info.size_bytes, info.indexed_vectors_count,
/// );
/// if info.status == Some(CollectionStatus::Yellow) {
///     // optimization is pending or running; the collection stays searchable
/// }
/// if info.updates.as_ref().map(|u| u.status) == Some(UpdatesStatus::Pending) {
///     // some acknowledged writes are not visible to search yet
/// }
/// # Ok(())
/// # }
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionInfo {
    pub exists: bool,
    pub config: Option<CollectionConfig>,
    pub point_count: Option<u64>,
    /// Optimization status. `None` if it could not be determined.
    pub status: Option<CollectionStatus>,
    /// Estimated stored bytes, which the collection size limit applies to.
    pub size_bytes: Option<u64>,
    /// Approximate number of vectors in indexed segments; the rest are searched
    /// exhaustively until an optimization indexes them.
    pub indexed_vectors_count: Option<u64>,
    /// Progress of acknowledged writes. `None` if it could not be determined.
    pub updates: Option<UpdatesInfo>,
    /// Time spent to process the request, in seconds.
    pub time: f64,
}

/// Request for [`super::QdrantServerless::list_collections`].
///
/// Defaults to the server page size (20, max 100). Pass `offset_token` from a
/// previous response's `next_offset_token` to fetch the next page.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ListCollections {
    /// Maximum number of collections to return. Unset: server default (20).
    pub limit: Option<u32>,
    /// Opaque token from a previous page's `next_offset_token`.
    pub offset_token: Option<String>,
}

impl ListCollections {
    /// Create an empty request (server defaults).
    pub fn new() -> Self {
        Self::default()
    }
}

/// Builder for [`ListCollections`].
#[must_use]
#[derive(Clone, Default)]
pub struct ListCollectionsBuilder {
    limit: Option<u32>,
    offset_token: Option<String>,
}

impl ListCollectionsBuilder {
    /// Create an empty builder (server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Maximum number of collections to return (must be 1..=100).
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Opaque token from a previous page's `next_offset_token`.
    pub fn offset_token(mut self, offset_token: impl Into<String>) -> Self {
        self.offset_token = Some(offset_token.into());
        self
    }

    /// Build the [`ListCollections`] request.
    pub fn build(self) -> ListCollections {
        self.into()
    }
}

impl From<ListCollectionsBuilder> for ListCollections {
    fn from(builder: ListCollectionsBuilder) -> Self {
        ListCollections {
            limit: builder.limit,
            offset_token: builder.offset_token,
        }
    }
}

/// One collection in a [`super::QdrantServerless::list_collections`] listing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionSummary {
    pub collection_name: String,
    pub point_count: Option<u64>,
    /// Estimated stored bytes, as in [`CollectionInfo::size_bytes`].
    pub size_bytes: Option<u64>,
}

/// A page of collections returned by [`super::QdrantServerless::list_collections`].
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionsList {
    pub collections: Vec<CollectionSummary>,
    /// Opaque token to pass as `offset_token` for the next page. Absent when done.
    pub next_offset_token: Option<String>,
    /// Time spent to process the request, in seconds.
    pub time: f64,
}
