//! Conversions between serverless public models and the internal gRPC types.
//!
//! Generated gRPC types are an implementation detail and must not leak into
//! the public interface.
//!
//! All struct fields are bound by destructuring (never via `.field` or `_`) so
//! adding a proto/model field is a compile error until conversions are updated.

use crate::qdrant_client::error::QdrantError;
use crate::serverless::grpc::{
    self, payload_index_config, stemming_algorithm, BoolIndex as GrpcBoolIndex,
    CollectionStatus as GrpcCollectionStatus, DatetimeIndex as GrpcDatetimeIndex,
    DenseVectorConfig as GrpcDenseVectorConfig, DisabledStemmer as GrpcDisabledStemmer,
    Distance as GrpcDistance, FloatIndex as GrpcFloatIndex, GeoIndex as GrpcGeoIndex,
    IntegerIndex as GrpcIntegerIndex, KeywordIndex as GrpcKeywordIndex,
    KeywordPrefixParams as GrpcKeywordPrefixParams, PayloadIndexConfig,
    PrecisionTier as GrpcPrecisionTier, SnowballParams as GrpcSnowballParams,
    SparseVectorConfig as GrpcSparseVectorConfig, StemmingAlgorithm as GrpcStemmingAlgorithm,
    StopwordsSet as GrpcStopwordsSet, TextIndex as GrpcTextIndex,
    TextScoringParams as GrpcTextScoringParams, TextScoringType as GrpcTextScoringType,
    Tokenizer as GrpcTokenizer, UpdatesInfo as GrpcUpdatesInfo, UpdatesStatus as GrpcUpdatesStatus,
    UuidIndex as GrpcUuidIndex,
};
use crate::serverless::models::{
    BoolIndex, CollectionConfig, CollectionStatus, DatetimeIndex, DenseVectorConfig, Distance,
    FloatIndex, GeoIndex, IntegerIndex, KeywordIndex, KeywordPrefixParams, PayloadIndex,
    PrecisionTier, SnowballParams, SparseVectorConfig, StemmingAlgorithm, StopwordsSet, TextIndex,
    TextScoringParams, TextScoringType, Tokenizer, UpdatesInfo, UpdatesStatus, UuidIndex,
};

fn distance_to_grpc(distance: Distance) -> GrpcDistance {
    match distance {
        Distance::Cosine => GrpcDistance::Cosine,
        Distance::Euclid => GrpcDistance::Euclid,
        Distance::Dot => GrpcDistance::Dot,
        Distance::Manhattan => GrpcDistance::Manhattan,
    }
}

fn distance_from_grpc(distance: GrpcDistance) -> Result<Distance, QdrantError> {
    match distance {
        GrpcDistance::Cosine => Ok(Distance::Cosine),
        GrpcDistance::Euclid => Ok(Distance::Euclid),
        GrpcDistance::Dot => Ok(Distance::Dot),
        GrpcDistance::Manhattan => Ok(Distance::Manhattan),
        GrpcDistance::Unspecified => Err(QdrantError::ConversionError(
            "serverless Distance is unspecified".into(),
        )),
    }
}

fn precision_to_grpc(tier: PrecisionTier) -> GrpcPrecisionTier {
    match tier {
        PrecisionTier::Low => GrpcPrecisionTier::Low,
        PrecisionTier::Medium => GrpcPrecisionTier::Medium,
        PrecisionTier::High => GrpcPrecisionTier::High,
        PrecisionTier::Full => GrpcPrecisionTier::Full,
    }
}

fn precision_from_grpc(tier: GrpcPrecisionTier) -> Result<PrecisionTier, QdrantError> {
    match tier {
        GrpcPrecisionTier::Low => Ok(PrecisionTier::Low),
        GrpcPrecisionTier::Medium => Ok(PrecisionTier::Medium),
        GrpcPrecisionTier::High => Ok(PrecisionTier::High),
        GrpcPrecisionTier::Full => Ok(PrecisionTier::Full),
        GrpcPrecisionTier::Unspecified => Err(QdrantError::ConversionError(
            "serverless PrecisionTier is unspecified".into(),
        )),
    }
}

fn tokenizer_to_grpc(tokenizer: Tokenizer) -> GrpcTokenizer {
    match tokenizer {
        Tokenizer::Prefix => GrpcTokenizer::Prefix,
        Tokenizer::Whitespace => GrpcTokenizer::Whitespace,
        Tokenizer::Word => GrpcTokenizer::Word,
        Tokenizer::Multilingual => GrpcTokenizer::Multilingual,
        Tokenizer::Code => GrpcTokenizer::Code,
    }
}

fn tokenizer_from_grpc(tokenizer: GrpcTokenizer) -> Result<Tokenizer, QdrantError> {
    match tokenizer {
        GrpcTokenizer::Prefix => Ok(Tokenizer::Prefix),
        GrpcTokenizer::Whitespace => Ok(Tokenizer::Whitespace),
        GrpcTokenizer::Word => Ok(Tokenizer::Word),
        GrpcTokenizer::Multilingual => Ok(Tokenizer::Multilingual),
        GrpcTokenizer::Code => Ok(Tokenizer::Code),
        GrpcTokenizer::Unspecified => Err(QdrantError::ConversionError(
            "serverless Tokenizer is unspecified".into(),
        )),
    }
}

fn text_scoring_type_to_grpc(scoring_type: TextScoringType) -> GrpcTextScoringType {
    match scoring_type {
        TextScoringType::Bm25 => GrpcTextScoringType::Bm25,
    }
}

fn text_scoring_type_from_grpc(
    scoring_type: GrpcTextScoringType,
) -> Result<TextScoringType, QdrantError> {
    match scoring_type {
        GrpcTextScoringType::Bm25 => Ok(TextScoringType::Bm25),
        GrpcTextScoringType::Unspecified => Err(QdrantError::ConversionError(
            "serverless TextScoringType is unspecified".into(),
        )),
    }
}

fn text_scoring_to_grpc(TextScoringParams { r#type }: &TextScoringParams) -> GrpcTextScoringParams {
    GrpcTextScoringParams {
        r#type: r#type.map(|t| text_scoring_type_to_grpc(t) as i32),
    }
}

fn text_scoring_from_grpc(
    GrpcTextScoringParams { r#type }: &GrpcTextScoringParams,
) -> Result<TextScoringParams, QdrantError> {
    Ok(TextScoringParams {
        r#type: r#type
            .map(|t| {
                text_scoring_type_from_grpc(GrpcTextScoringType::try_from(t).map_err(|_| {
                    QdrantError::ConversionError(format!("unknown TextScoringType {t}"))
                })?)
            })
            .transpose()?,
    })
}

/// `Unspecified` means the status could not be determined. Unknown values (statuses added
/// later on the server) also read as `None`, so `get_collection` keeps working.
pub(crate) fn collection_status_from_grpc(status: i32) -> Option<CollectionStatus> {
    match GrpcCollectionStatus::try_from(status) {
        Ok(GrpcCollectionStatus::Green) => Some(CollectionStatus::Green),
        Ok(GrpcCollectionStatus::Yellow) => Some(CollectionStatus::Yellow),
        Ok(GrpcCollectionStatus::Unspecified) | Err(_) => None,
    }
}

pub(crate) fn updates_info_from_grpc(
    GrpcUpdatesInfo { status }: &GrpcUpdatesInfo,
) -> Result<UpdatesInfo, QdrantError> {
    let status = match GrpcUpdatesStatus::try_from(*status) {
        Ok(GrpcUpdatesStatus::Applied) => UpdatesStatus::Applied,
        Ok(GrpcUpdatesStatus::Pending) => UpdatesStatus::Pending,
        Ok(GrpcUpdatesStatus::Unspecified) | Err(_) => {
            return Err(QdrantError::ConversionError(format!(
                "unknown or unspecified serverless UpdatesStatus {status}"
            )))
        }
    };
    Ok(UpdatesInfo { status })
}

fn stopwords_to_grpc(StopwordsSet { languages, custom }: &StopwordsSet) -> GrpcStopwordsSet {
    GrpcStopwordsSet {
        languages: languages.clone(),
        custom: custom.clone(),
    }
}

fn stopwords_from_grpc(GrpcStopwordsSet { languages, custom }: &GrpcStopwordsSet) -> StopwordsSet {
    StopwordsSet {
        languages: languages.clone(),
        custom: custom.clone(),
    }
}

fn stemmer_to_grpc(stemmer: &StemmingAlgorithm) -> GrpcStemmingAlgorithm {
    match stemmer {
        StemmingAlgorithm::Snowball(SnowballParams { language }) => GrpcStemmingAlgorithm {
            stemming_params: Some(stemming_algorithm::StemmingParams::Snowball(
                GrpcSnowballParams {
                    language: language.clone(),
                },
            )),
        },
        StemmingAlgorithm::Disabled => GrpcStemmingAlgorithm {
            stemming_params: Some(stemming_algorithm::StemmingParams::Disabled(
                GrpcDisabledStemmer {},
            )),
        },
    }
}

fn stemmer_from_grpc(
    GrpcStemmingAlgorithm { stemming_params }: &GrpcStemmingAlgorithm,
) -> Result<StemmingAlgorithm, QdrantError> {
    match stemming_params {
        Some(stemming_algorithm::StemmingParams::Snowball(GrpcSnowballParams { language })) => {
            Ok(StemmingAlgorithm::Snowball(SnowballParams {
                language: language.clone(),
            }))
        }
        Some(stemming_algorithm::StemmingParams::Disabled(GrpcDisabledStemmer {})) => {
            Ok(StemmingAlgorithm::Disabled)
        }
        None => Err(QdrantError::ConversionError(
            "serverless StemmingAlgorithm has no stemming_params variant".into(),
        )),
    }
}

pub(crate) fn dense_vector_to_grpc(
    DenseVectorConfig {
        size,
        distance,
        multivector,
        precision_tier,
    }: &DenseVectorConfig,
) -> GrpcDenseVectorConfig {
    GrpcDenseVectorConfig {
        size: *size,
        distance: distance_to_grpc(*distance) as i32,
        multivector: *multivector,
        precision_tier: precision_tier.map(|t| precision_to_grpc(t) as i32),
    }
}

pub(crate) fn dense_vector_from_grpc(
    GrpcDenseVectorConfig {
        size,
        distance,
        multivector,
        precision_tier,
    }: &GrpcDenseVectorConfig,
) -> Result<DenseVectorConfig, QdrantError> {
    Ok(DenseVectorConfig {
        size: *size,
        distance: distance_from_grpc(
            GrpcDistance::try_from(*distance).map_err(|_| {
                QdrantError::ConversionError(format!("unknown Distance {distance}"))
            })?,
        )?,
        multivector: *multivector,
        precision_tier: precision_tier
            .map(|t| {
                precision_from_grpc(GrpcPrecisionTier::try_from(t).map_err(|_| {
                    QdrantError::ConversionError(format!("unknown PrecisionTier {t}"))
                })?)
            })
            .transpose()?,
    })
}

pub(crate) fn sparse_vector_to_grpc(
    SparseVectorConfig {
        use_idf,
        precision_tier,
    }: &SparseVectorConfig,
) -> GrpcSparseVectorConfig {
    GrpcSparseVectorConfig {
        use_idf: *use_idf,
        precision_tier: precision_tier.map(|t| precision_to_grpc(t) as i32),
    }
}

pub(crate) fn sparse_vector_from_grpc(
    GrpcSparseVectorConfig {
        use_idf,
        precision_tier,
    }: &GrpcSparseVectorConfig,
) -> Result<SparseVectorConfig, QdrantError> {
    Ok(SparseVectorConfig {
        use_idf: *use_idf,
        precision_tier: precision_tier
            .map(|t| {
                precision_from_grpc(GrpcPrecisionTier::try_from(t).map_err(|_| {
                    QdrantError::ConversionError(format!("unknown PrecisionTier {t}"))
                })?)
            })
            .transpose()?,
    })
}

pub(crate) fn payload_index_to_grpc(model: &PayloadIndex) -> PayloadIndexConfig {
    let index = match model {
        PayloadIndex::Keyword(KeywordIndex { prefix, nested }) => {
            payload_index_config::Index::Keyword(GrpcKeywordIndex {
                prefix: prefix
                    .as_ref()
                    .map(|KeywordPrefixParams| GrpcKeywordPrefixParams {}),
                nested: nested.clone(),
            })
        }
        PayloadIndex::Integer(IntegerIndex {
            lookup,
            range,
            nested,
        }) => payload_index_config::Index::Integer(GrpcIntegerIndex {
            lookup: *lookup,
            range: *range,
            nested: nested.clone(),
        }),
        PayloadIndex::Float(FloatIndex { nested }) => {
            payload_index_config::Index::Float(GrpcFloatIndex {
                nested: nested.clone(),
            })
        }
        PayloadIndex::Uuid(UuidIndex { nested }) => {
            payload_index_config::Index::Uuid(GrpcUuidIndex {
                nested: nested.clone(),
            })
        }
        PayloadIndex::Datetime(DatetimeIndex { nested }) => {
            payload_index_config::Index::Datetime(GrpcDatetimeIndex {
                nested: nested.clone(),
            })
        }
        PayloadIndex::Text(TextIndex {
            tokenizer,
            lowercase,
            phrase_matching,
            min_token_len,
            max_token_len,
            ascii_folding,
            stopwords,
            stemmer,
            scoring,
            nested,
        }) => payload_index_config::Index::Text(GrpcTextIndex {
            tokenizer: tokenizer.map(|t| tokenizer_to_grpc(t) as i32),
            lowercase: *lowercase,
            phrase_matching: *phrase_matching,
            min_token_len: *min_token_len,
            max_token_len: *max_token_len,
            ascii_folding: *ascii_folding,
            stopwords: stopwords.as_ref().map(stopwords_to_grpc),
            stemmer: stemmer.as_ref().map(stemmer_to_grpc),
            scoring: scoring.as_ref().map(text_scoring_to_grpc),
            nested: nested.clone(),
        }),
        PayloadIndex::Geo(GeoIndex { nested }) => payload_index_config::Index::Geo(GrpcGeoIndex {
            nested: nested.clone(),
        }),
        PayloadIndex::Bool(BoolIndex { nested }) => {
            payload_index_config::Index::Bool(GrpcBoolIndex {
                nested: nested.clone(),
            })
        }
    };
    PayloadIndexConfig { index: Some(index) }
}

pub(crate) fn payload_index_from_grpc(
    PayloadIndexConfig { index }: &PayloadIndexConfig,
) -> Result<PayloadIndex, QdrantError> {
    match index.as_ref() {
        Some(payload_index_config::Index::Keyword(GrpcKeywordIndex { prefix, nested })) => {
            Ok(PayloadIndex::Keyword(KeywordIndex {
                prefix: prefix
                    .as_ref()
                    .map(|GrpcKeywordPrefixParams {}| KeywordPrefixParams),
                nested: nested.clone(),
            }))
        }
        Some(payload_index_config::Index::Integer(GrpcIntegerIndex {
            lookup,
            range,
            nested,
        })) => Ok(PayloadIndex::Integer(IntegerIndex {
            lookup: *lookup,
            range: *range,
            nested: nested.clone(),
        })),
        Some(payload_index_config::Index::Float(GrpcFloatIndex { nested })) => {
            Ok(PayloadIndex::Float(FloatIndex {
                nested: nested.clone(),
            }))
        }
        Some(payload_index_config::Index::Uuid(GrpcUuidIndex { nested })) => {
            Ok(PayloadIndex::Uuid(UuidIndex {
                nested: nested.clone(),
            }))
        }
        Some(payload_index_config::Index::Datetime(GrpcDatetimeIndex { nested })) => {
            Ok(PayloadIndex::Datetime(DatetimeIndex {
                nested: nested.clone(),
            }))
        }
        Some(payload_index_config::Index::Text(GrpcTextIndex {
            tokenizer,
            lowercase,
            phrase_matching,
            min_token_len,
            max_token_len,
            ascii_folding,
            stopwords,
            stemmer,
            scoring,
            nested,
        })) => Ok(PayloadIndex::Text(TextIndex {
            tokenizer: tokenizer
                .map(|t| {
                    tokenizer_from_grpc(GrpcTokenizer::try_from(t).map_err(|_| {
                        QdrantError::ConversionError(format!("unknown Tokenizer {t}"))
                    })?)
                })
                .transpose()?,
            lowercase: *lowercase,
            phrase_matching: *phrase_matching,
            min_token_len: *min_token_len,
            max_token_len: *max_token_len,
            ascii_folding: *ascii_folding,
            stopwords: stopwords.as_ref().map(stopwords_from_grpc),
            stemmer: stemmer.as_ref().map(stemmer_from_grpc).transpose()?,
            scoring: scoring.as_ref().map(text_scoring_from_grpc).transpose()?,
            nested: nested.clone(),
        })),
        Some(payload_index_config::Index::Geo(GrpcGeoIndex { nested })) => {
            Ok(PayloadIndex::Geo(GeoIndex {
                nested: nested.clone(),
            }))
        }
        Some(payload_index_config::Index::Bool(GrpcBoolIndex { nested })) => {
            Ok(PayloadIndex::Bool(BoolIndex {
                nested: nested.clone(),
            }))
        }
        None => Err(QdrantError::ConversionError(
            "serverless PayloadIndexConfig has no index variant".into(),
        )),
    }
}

pub(crate) fn collection_config_to_grpc(
    CollectionConfig {
        dense_vectors,
        sparse_vectors,
        payload_indexes,
    }: &CollectionConfig,
) -> grpc::CollectionConfig {
    grpc::CollectionConfig {
        dense_vectors: dense_vectors
            .iter()
            .map(|(name, dense)| (name.clone(), dense_vector_to_grpc(dense)))
            .collect(),
        sparse_vectors: sparse_vectors
            .iter()
            .map(|(name, sparse)| (name.clone(), sparse_vector_to_grpc(sparse)))
            .collect(),
        payload_indexes: payload_indexes
            .iter()
            .map(|(field, index)| (field.clone(), payload_index_to_grpc(index)))
            .collect(),
    }
}

pub(crate) fn collection_config_from_grpc(
    grpc::CollectionConfig {
        dense_vectors,
        sparse_vectors,
        payload_indexes,
    }: &grpc::CollectionConfig,
) -> Result<CollectionConfig, QdrantError> {
    Ok(CollectionConfig {
        dense_vectors: dense_vectors
            .iter()
            .map(|(name, dense)| Ok((name.clone(), dense_vector_from_grpc(dense)?)))
            .collect::<Result<_, QdrantError>>()?,
        sparse_vectors: sparse_vectors
            .iter()
            .map(|(name, sparse)| Ok((name.clone(), sparse_vector_from_grpc(sparse)?)))
            .collect::<Result<_, QdrantError>>()?,
        payload_indexes: payload_indexes
            .iter()
            .map(|(field, index)| Ok((field.clone(), payload_index_from_grpc(index)?)))
            .collect::<Result<_, QdrantError>>()?,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::serverless::models::{IntegerIndex, KeywordIndex, TextIndex};

    #[test]
    fn collection_config_grpc_roundtrip() {
        let config = CollectionConfig::new()
            .dense_vector(DenseVectorConfig::new(1536, Distance::Cosine))
            .named_dense_vector(
                "colbert",
                DenseVectorConfig::new(128, Distance::Dot)
                    .multivector(true)
                    .precision_tier(PrecisionTier::Low),
            )
            .named_sparse_vector("bm25", SparseVectorConfig::new().use_idf(true))
            .payload_index("user_id", KeywordIndex::new().with_prefix())
            .payload_index("age", IntegerIndex::new().lookup(true).range(false))
            .payload_index(
                "description",
                TextIndex::new()
                    .tokenizer(Tokenizer::Word)
                    .lowercase(false)
                    .ascii_folding(true)
                    .stopwords(StopwordsSet::new().languages(["english"]))
                    .stemmer(StemmingAlgorithm::Snowball(SnowballParams::new("english"))),
            );

        let roundtrip = collection_config_from_grpc(&collection_config_to_grpc(&config)).unwrap();
        assert_eq!(roundtrip, config);
    }

    #[test]
    fn optional_fields_stay_unset() {
        let config = CollectionConfig::new()
            .dense_vector(DenseVectorConfig::new(4, Distance::Euclid))
            .payload_index("age", IntegerIndex::new())
            .payload_index("user_id", KeywordIndex::new())
            .payload_index("text", TextIndex::new());

        let grpc_config = collection_config_to_grpc(&config);
        let dense = grpc_config.dense_vectors.get("").unwrap();
        assert!(dense.precision_tier.is_none());
        let age = grpc_config.payload_indexes.get("age").unwrap();
        match age.index.as_ref().unwrap() {
            payload_index_config::Index::Integer(GrpcIntegerIndex {
                lookup,
                range,
                nested,
            }) => {
                assert!(lookup.is_none());
                assert!(range.is_none());
                assert!(nested.is_none());
            }
            other => panic!("expected integer index, got {other:?}"),
        }
        let keyword = grpc_config.payload_indexes.get("user_id").unwrap();
        match keyword.index.as_ref().unwrap() {
            payload_index_config::Index::Keyword(GrpcKeywordIndex { prefix, nested }) => {
                assert!(prefix.is_none());
                assert!(nested.is_none());
            }
            other => panic!("expected keyword index, got {other:?}"),
        }
        let text = grpc_config.payload_indexes.get("text").unwrap();
        match text.index.as_ref().unwrap() {
            payload_index_config::Index::Text(GrpcTextIndex {
                tokenizer,
                lowercase,
                phrase_matching,
                min_token_len,
                max_token_len,
                ascii_folding,
                stopwords,
                stemmer,
                scoring,
                nested,
            }) => {
                assert!(tokenizer.is_none());
                assert!(lowercase.is_none());
                assert!(phrase_matching.is_none());
                assert!(min_token_len.is_none());
                assert!(max_token_len.is_none());
                assert!(ascii_folding.is_none());
                assert!(stopwords.is_none());
                assert!(stemmer.is_none());
                assert!(scoring.is_none());
                assert!(nested.is_none());
            }
            other => panic!("expected text index, got {other:?}"),
        }

        let roundtrip = collection_config_from_grpc(&grpc_config).unwrap();
        assert_eq!(roundtrip, config);
    }

    #[test]
    fn text_scoring_code_tokenizer_and_nested_roundtrip() {
        let config = CollectionConfig::new()
            .payload_index(
                "default",
                TextIndex::new().scoring(TextScoringParams::new()),
            )
            .payload_index(
                "offers[].title",
                TextIndex::new()
                    .tokenizer(Tokenizer::Code)
                    .bm25_scoring()
                    .nested("offers"),
            )
            .payload_index("offers[].color", KeywordIndex::new().nested("offers"))
            .payload_index("offers[].qty", IntegerIndex::new().nested("offers"))
            .payload_index("offers[].price", FloatIndex::new().nested("offers"))
            .payload_index("offers[].id", UuidIndex::new().nested("offers"))
            .payload_index("offers[].at", DatetimeIndex::new().nested("offers"))
            .payload_index("offers[].location", GeoIndex::new().nested("offers"))
            .payload_index("offers[].in_stock", BoolIndex::new().nested("offers"));

        let grpc_config = collection_config_to_grpc(&config);
        match grpc_config.payload_indexes["default"]
            .index
            .as_ref()
            .unwrap()
        {
            payload_index_config::Index::Text(text) => {
                // presence enables scoring; an unset type means BM25
                assert_eq!(text.scoring, Some(GrpcTextScoringParams { r#type: None }));
            }
            other => panic!("expected text index, got {other:?}"),
        }
        match grpc_config.payload_indexes["offers[].title"]
            .index
            .as_ref()
            .unwrap()
        {
            payload_index_config::Index::Text(text) => {
                assert_eq!(text.tokenizer, Some(GrpcTokenizer::Code as i32));
                assert_eq!(
                    text.scoring,
                    Some(GrpcTextScoringParams {
                        r#type: Some(GrpcTextScoringType::Bm25 as i32)
                    })
                );
                assert_eq!(text.nested.as_deref(), Some("offers"));
            }
            other => panic!("expected text index, got {other:?}"),
        }

        let roundtrip = collection_config_from_grpc(&grpc_config).unwrap();
        assert_eq!(roundtrip, config);
    }

    #[test]
    fn collection_status_and_updates_from_grpc() {
        assert_eq!(
            collection_status_from_grpc(GrpcCollectionStatus::Green as i32),
            Some(CollectionStatus::Green)
        );
        assert_eq!(
            collection_status_from_grpc(GrpcCollectionStatus::Yellow as i32),
            Some(CollectionStatus::Yellow)
        );
        assert_eq!(
            collection_status_from_grpc(GrpcCollectionStatus::Unspecified as i32),
            None
        );
        // a status added later on the server must not break get_collection
        assert_eq!(collection_status_from_grpc(3), None);

        let pending = GrpcUpdatesInfo {
            status: GrpcUpdatesStatus::Pending as i32,
        };
        assert_eq!(
            updates_info_from_grpc(&pending).unwrap(),
            UpdatesInfo {
                status: UpdatesStatus::Pending
            }
        );
        let applied = GrpcUpdatesInfo {
            status: GrpcUpdatesStatus::Applied as i32,
        };
        assert_eq!(
            updates_info_from_grpc(&applied).unwrap().status,
            UpdatesStatus::Applied
        );
        let unspecified = GrpcUpdatesInfo {
            status: GrpcUpdatesStatus::Unspecified as i32,
        };
        assert!(updates_info_from_grpc(&unspecified).is_err());
    }
}
