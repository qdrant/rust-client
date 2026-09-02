//! Public models for the Qdrant Serverless collection management API.
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

/// Exact match on string values, e.g. `color: "red"`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct KeywordIndex;

/// Exact match and/or range filters on integers. Both default to enabled.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct IntegerIndex {
    pub lookup: Option<bool>,
    pub range: Option<bool>,
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
}

/// Range filters on floating point (and integer) numbers.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FloatIndex;

/// Exact match on UUID strings; like keyword but stored compactly.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UuidIndex;

/// Range filters on RFC 3339 datetimes.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct DatetimeIndex;

/// Full-text filtering on string values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TextIndex {
    pub tokenizer: Option<Tokenizer>,
    pub lowercase: Option<bool>,
    pub phrase_matching: Option<bool>,
    pub min_token_len: Option<u64>,
    pub max_token_len: Option<u64>,
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
}

/// Geo radius / bounding box / polygon filters on `{lon, lat}` values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GeoIndex;

/// Exact match on booleans.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct BoolIndex;

/// One payload index. Only the kind of filter the field supports is chosen;
/// storage placement of the index is the manager's decision.
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

/// A collection's configuration and stats, as returned by [`super::QdrantServerless::get_collection`].
///
/// `point_count` is eventually consistent and absent until stats have been
/// written for the collection.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionInfo {
    pub exists: bool,
    pub config: Option<CollectionConfig>,
    pub point_count: Option<u64>,
}

/// One collection in a [`super::QdrantServerless::list_collections`] listing.
#[derive(Debug, Clone, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CollectionSummary {
    pub collection_name: String,
    pub point_count: Option<u64>,
}
