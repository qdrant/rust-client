//! Conversions between serverless public models and the internal gRPC types.
//!
//! Generated gRPC types are an implementation detail and must not leak into
//! the public interface.

use crate::qdrant_client::error::QdrantError;
use crate::serverless::grpc::{
    self, payload_index_config, BoolIndex as GrpcBoolIndex, DatetimeIndex as GrpcDatetimeIndex,
    DenseVectorConfig as GrpcDenseVectorConfig, Distance as GrpcDistance,
    FloatIndex as GrpcFloatIndex, GeoIndex as GrpcGeoIndex, IntegerIndex as GrpcIntegerIndex,
    KeywordIndex as GrpcKeywordIndex, PayloadIndexConfig, PrecisionTier as GrpcPrecisionTier,
    SparseVectorConfig as GrpcSparseVectorConfig, TextIndex as GrpcTextIndex,
    Tokenizer as GrpcTokenizer, UuidIndex as GrpcUuidIndex,
};
use crate::serverless::models::{
    BoolIndex, CollectionConfig, DatetimeIndex, DenseVectorConfig, Distance, FloatIndex, GeoIndex,
    IntegerIndex, KeywordIndex, PayloadIndex, PrecisionTier, SparseVectorConfig, TextIndex,
    Tokenizer, UuidIndex,
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
    }
}

fn precision_from_grpc(tier: GrpcPrecisionTier) -> Result<PrecisionTier, QdrantError> {
    match tier {
        GrpcPrecisionTier::Low => Ok(PrecisionTier::Low),
        GrpcPrecisionTier::Medium => Ok(PrecisionTier::Medium),
        GrpcPrecisionTier::High => Ok(PrecisionTier::High),
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
    }
}

fn tokenizer_from_grpc(tokenizer: GrpcTokenizer) -> Result<Tokenizer, QdrantError> {
    match tokenizer {
        GrpcTokenizer::Prefix => Ok(Tokenizer::Prefix),
        GrpcTokenizer::Whitespace => Ok(Tokenizer::Whitespace),
        GrpcTokenizer::Word => Ok(Tokenizer::Word),
        GrpcTokenizer::Multilingual => Ok(Tokenizer::Multilingual),
        GrpcTokenizer::Unspecified => Err(QdrantError::ConversionError(
            "serverless Tokenizer is unspecified".into(),
        )),
    }
}

pub(crate) fn dense_vector_to_grpc(model: &DenseVectorConfig) -> GrpcDenseVectorConfig {
    GrpcDenseVectorConfig {
        size: model.size,
        distance: distance_to_grpc(model.distance) as i32,
        multivector: model.multivector,
        precision_tier: model.precision_tier.map(|t| precision_to_grpc(t) as i32),
    }
}

pub(crate) fn dense_vector_from_grpc(
    grpc_model: &GrpcDenseVectorConfig,
) -> Result<DenseVectorConfig, QdrantError> {
    Ok(DenseVectorConfig {
        size: grpc_model.size,
        distance: distance_from_grpc(GrpcDistance::try_from(grpc_model.distance).map_err(
            |_| QdrantError::ConversionError(format!("unknown Distance {}", grpc_model.distance)),
        )?)?,
        multivector: grpc_model.multivector,
        precision_tier: grpc_model
            .precision_tier
            .map(|t| {
                precision_from_grpc(GrpcPrecisionTier::try_from(t).map_err(|_| {
                    QdrantError::ConversionError(format!("unknown PrecisionTier {t}"))
                })?)
            })
            .transpose()?,
    })
}

pub(crate) fn sparse_vector_to_grpc(model: &SparseVectorConfig) -> GrpcSparseVectorConfig {
    GrpcSparseVectorConfig {
        use_idf: model.use_idf,
        precision_tier: model.precision_tier.map(|t| precision_to_grpc(t) as i32),
    }
}

pub(crate) fn sparse_vector_from_grpc(
    grpc_model: &GrpcSparseVectorConfig,
) -> Result<SparseVectorConfig, QdrantError> {
    Ok(SparseVectorConfig {
        use_idf: grpc_model.use_idf,
        precision_tier: grpc_model
            .precision_tier
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
        PayloadIndex::Keyword(_) => payload_index_config::Index::Keyword(GrpcKeywordIndex {}),
        PayloadIndex::Integer(IntegerIndex { lookup, range }) => {
            payload_index_config::Index::Integer(GrpcIntegerIndex {
                lookup: *lookup,
                range: *range,
            })
        }
        PayloadIndex::Float(_) => payload_index_config::Index::Float(GrpcFloatIndex {}),
        PayloadIndex::Uuid(_) => payload_index_config::Index::Uuid(GrpcUuidIndex {}),
        PayloadIndex::Datetime(_) => payload_index_config::Index::Datetime(GrpcDatetimeIndex {}),
        PayloadIndex::Text(text) => payload_index_config::Index::Text(GrpcTextIndex {
            tokenizer: text.tokenizer.map(|t| tokenizer_to_grpc(t) as i32),
            lowercase: text.lowercase,
            phrase_matching: text.phrase_matching,
            min_token_len: text.min_token_len,
            max_token_len: text.max_token_len,
        }),
        PayloadIndex::Geo(_) => payload_index_config::Index::Geo(GrpcGeoIndex {}),
        PayloadIndex::Bool(_) => payload_index_config::Index::Bool(GrpcBoolIndex {}),
    };
    PayloadIndexConfig { index: Some(index) }
}

pub(crate) fn payload_index_from_grpc(
    grpc_model: &PayloadIndexConfig,
) -> Result<PayloadIndex, QdrantError> {
    match grpc_model.index.as_ref() {
        Some(payload_index_config::Index::Keyword(_)) => Ok(PayloadIndex::Keyword(KeywordIndex)),
        Some(payload_index_config::Index::Integer(integer)) => {
            Ok(PayloadIndex::Integer(IntegerIndex {
                lookup: integer.lookup,
                range: integer.range,
            }))
        }
        Some(payload_index_config::Index::Float(_)) => Ok(PayloadIndex::Float(FloatIndex)),
        Some(payload_index_config::Index::Uuid(_)) => Ok(PayloadIndex::Uuid(UuidIndex)),
        Some(payload_index_config::Index::Datetime(_)) => Ok(PayloadIndex::Datetime(DatetimeIndex)),
        Some(payload_index_config::Index::Text(text)) => Ok(PayloadIndex::Text(TextIndex {
            tokenizer: text
                .tokenizer
                .map(|t| {
                    tokenizer_from_grpc(GrpcTokenizer::try_from(t).map_err(|_| {
                        QdrantError::ConversionError(format!("unknown Tokenizer {t}"))
                    })?)
                })
                .transpose()?,
            lowercase: text.lowercase,
            phrase_matching: text.phrase_matching,
            min_token_len: text.min_token_len,
            max_token_len: text.max_token_len,
        })),
        Some(payload_index_config::Index::Geo(_)) => Ok(PayloadIndex::Geo(GeoIndex)),
        Some(payload_index_config::Index::Bool(_)) => Ok(PayloadIndex::Bool(BoolIndex)),
        None => Err(QdrantError::ConversionError(
            "serverless PayloadIndexConfig has no index variant".into(),
        )),
    }
}

pub(crate) fn collection_config_to_grpc(model: &CollectionConfig) -> grpc::CollectionConfig {
    grpc::CollectionConfig {
        dense_vectors: model
            .dense_vectors
            .iter()
            .map(|(name, dense)| (name.clone(), dense_vector_to_grpc(dense)))
            .collect(),
        sparse_vectors: model
            .sparse_vectors
            .iter()
            .map(|(name, sparse)| (name.clone(), sparse_vector_to_grpc(sparse)))
            .collect(),
        payload_indexes: model
            .payload_indexes
            .iter()
            .map(|(field, index)| (field.clone(), payload_index_to_grpc(index)))
            .collect(),
    }
}

pub(crate) fn collection_config_from_grpc(
    grpc_model: &grpc::CollectionConfig,
) -> Result<CollectionConfig, QdrantError> {
    Ok(CollectionConfig {
        dense_vectors: grpc_model
            .dense_vectors
            .iter()
            .map(|(name, dense)| Ok((name.clone(), dense_vector_from_grpc(dense)?)))
            .collect::<Result<_, QdrantError>>()?,
        sparse_vectors: grpc_model
            .sparse_vectors
            .iter()
            .map(|(name, sparse)| Ok((name.clone(), sparse_vector_from_grpc(sparse)?)))
            .collect::<Result<_, QdrantError>>()?,
        payload_indexes: grpc_model
            .payload_indexes
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
            .payload_index("user_id", KeywordIndex)
            .payload_index("age", IntegerIndex::new().lookup(true).range(false))
            .payload_index(
                "description",
                TextIndex::new().tokenizer(Tokenizer::Word).lowercase(false),
            );

        let roundtrip = collection_config_from_grpc(&collection_config_to_grpc(&config)).unwrap();
        assert_eq!(roundtrip, config);
    }

    #[test]
    fn optional_fields_stay_unset() {
        let config = CollectionConfig::new()
            .dense_vector(DenseVectorConfig::new(4, Distance::Euclid))
            .payload_index("age", IntegerIndex::new())
            .payload_index("text", TextIndex::new());

        let grpc_config = collection_config_to_grpc(&config);
        let dense = grpc_config.dense_vectors.get("").unwrap();
        assert!(dense.precision_tier.is_none());
        let age = grpc_config.payload_indexes.get("age").unwrap();
        match age.index.as_ref().unwrap() {
            payload_index_config::Index::Integer(integer) => {
                assert!(integer.lookup.is_none());
                assert!(integer.range.is_none());
            }
            other => panic!("expected integer index, got {other:?}"),
        }
        let text = grpc_config.payload_indexes.get("text").unwrap();
        match text.index.as_ref().unwrap() {
            payload_index_config::Index::Text(text) => {
                assert!(text.tokenizer.is_none());
            }
            other => panic!("expected text index, got {other:?}"),
        }

        let roundtrip = collection_config_from_grpc(&grpc_config).unwrap();
        assert_eq!(roundtrip, config);
    }
}
