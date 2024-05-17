//! For service types which are in a sub-module in qdrant.rs we can't use dervie_builder if we
//! want to preserve consistency with the other builders. This is because the generated builders types are
//! private but for custom type conversions, build() function and constructor with required values we
//! need access to those private fields. For this reason we introduce a few manually created builder here.

use derive_builder::Builder;
use std::path::PathBuf;

/// Builder for service types within qdrant::points_update_operation.
/// This sub-module is necessary since we do a `use manual_builder::*` in the qdrant.rs file to have
/// all builder coming from qdrant.rs file in the user API. However there are some builder types here
/// whichs name is already part of the qdrant.rs module. This submodule prevents ambiguity for those builder.
pub mod points_update_operation {
    use crate::grpc_macros::builder_type_conversions;
    use crate::qdrant::points_update_operation::{
        ClearPayload, DeletePayload, DeletePoints, DeleteVectors, OverwritePayload,
        PointStructList, SetPayload, UpdateVectors,
    };
    use crate::qdrant::{
        points_selector, PointStruct, PointVectors, PointsSelector, ShardKeySelector, Value,
        VectorsSelector,
    };
    use std::collections::HashMap;

    #[derive(Clone)]
    pub struct PointStructListBuilder {
        points: Vec<PointStruct>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl PointStructListBuilder {
        pub fn new(points: impl Into<Vec<PointStruct>>) -> Self {
            Self {
                points: points.into(),
                shard_key_selector: None,
            }
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<PointStructList, ()> {
            let builder = self.clone();
            Ok(PointStructList {
                points: builder.points,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(PointStructList, PointStructListBuilder);

    #[derive(Clone)]
    pub struct SetPayloadBuilder {
        payload: HashMap<String, Value>,
        points_selector: Option<PointsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
        key: Option<String>,
    }

    impl SetPayloadBuilder {
        pub fn new(payload: impl Into<HashMap<String, Value>>) -> Self {
            Self {
                payload: payload.into(),
                points_selector: None,
                shard_key_selector: None,
                key: None,
            }
        }

        /// Affected points
        pub fn points_selector(
            &mut self,
            points_selector: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points_selector = Some(PointsSelector {
                points_selector_one_of: Some(points_selector.into()),
            });
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        /// Option for indicate property of payload
        pub fn key(&mut self, key: impl Into<String>) -> &mut Self {
            self.key = Some(key.into());
            self
        }

        fn build_inner(&self) -> Result<SetPayload, ()> {
            let builder = self.clone();
            Ok(SetPayload {
                payload: builder.payload,
                points_selector: builder.points_selector,
                shard_key_selector: builder.shard_key_selector,
                key: builder.key,
            })
        }
    }

    builder_type_conversions!(SetPayload, SetPayloadBuilder);

    #[derive(Clone)]
    pub struct OverwritePayloadBuilder {
        payload: HashMap<String, Value>,
        points_selector: Option<PointsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
        key: Option<String>,
    }

    impl OverwritePayloadBuilder {
        pub fn new(payload: impl Into<HashMap<String, Value>>) -> Self {
            Self {
                payload: payload.into(),
                points_selector: None,
                shard_key_selector: None,
                key: None,
            }
        }

        /// Affected points
        pub fn points_selector(
            &mut self,
            points_selector: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points_selector = Some(PointsSelector {
                points_selector_one_of: Some(points_selector.into()),
            });
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        /// Option for indicate property of payload
        pub fn key(&mut self, key: impl Into<String>) -> &mut Self {
            self.key = Some(key.into());
            self
        }

        fn build_inner(&self) -> Result<OverwritePayload, ()> {
            let builder = self.clone();
            Ok(OverwritePayload {
                payload: builder.payload,
                points_selector: builder.points_selector,
                shard_key_selector: builder.shard_key_selector,
                key: builder.key,
            })
        }
    }

    builder_type_conversions!(OverwritePayload, OverwritePayloadBuilder);

    #[derive(Clone)]
    pub struct DeletePayloadBuilder {
        keys: Vec<String>,
        points_selector: Option<PointsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl DeletePayloadBuilder {
        pub fn new(keys: impl Into<Vec<String>>) -> Self {
            Self {
                keys: keys.into(),
                points_selector: None,
                shard_key_selector: None,
            }
        }

        /// Affected points
        pub fn points_selector(
            &mut self,
            points_selector: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points_selector = Some(PointsSelector {
                points_selector_one_of: Some(points_selector.into()),
            });
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<DeletePayload, ()> {
            let builder = self.clone();
            Ok(DeletePayload {
                keys: builder.keys,
                points_selector: builder.points_selector,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(DeletePayload, DeletePayloadBuilder);

    #[derive(Clone)]
    pub struct UpdateVectorsBuilder {
        points: Vec<PointVectors>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl UpdateVectorsBuilder {
        pub fn new(points: impl Into<Vec<PointVectors>>) -> Self {
            Self {
                points: points.into(),
                shard_key_selector: None,
            }
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<UpdateVectors, ()> {
            let builder = self.clone();
            Ok(UpdateVectors {
                points: builder.points,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(UpdateVectors, UpdateVectorsBuilder);

    #[derive(Clone, Default)]
    pub struct DeleteVectorsBuilder {
        points_selector: Option<PointsSelector>,
        vectors: Option<VectorsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl DeleteVectorsBuilder {
        pub fn new() -> Self {
            Self {
                points_selector: None,
                vectors: None,
                shard_key_selector: None,
            }
        }

        /// Affected points
        pub fn points_selector(
            &mut self,
            points_selector: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points_selector = Some(PointsSelector {
                points_selector_one_of: Some(points_selector.into()),
            });
            self
        }

        /// List of vector names to delete
        pub fn vectors(&mut self, vectors: impl Into<VectorsSelector>) -> &mut Self {
            self.vectors = Some(vectors.into());
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<DeleteVectors, ()> {
            let builder = self.clone();
            Ok(DeleteVectors {
                points_selector: builder.points_selector,
                vectors: builder.vectors,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(DeleteVectors, DeleteVectorsBuilder);

    #[derive(Clone, Default)]
    pub struct DeletePointsBuilder {
        points: Option<PointsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl DeletePointsBuilder {
        pub fn new() -> Self {
            Self {
                points: None,
                shard_key_selector: None,
            }
        }

        /// Affected points
        pub fn points(
            &mut self,
            points: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points = Some(PointsSelector {
                points_selector_one_of: Some(points.into()),
            });
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<DeletePoints, ()> {
            let builder = self.clone();
            Ok(DeletePoints {
                points: builder.points,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(DeletePoints, DeletePointsBuilder);

    #[derive(Clone, Default)]
    pub struct ClearPayloadBuilder {
        points: Option<PointsSelector>,
        shard_key_selector: Option<ShardKeySelector>,
    }

    impl ClearPayloadBuilder {
        pub fn new() -> Self {
            Self {
                points: None,
                shard_key_selector: None,
            }
        }

        /// Affected points
        pub fn points(
            &mut self,
            points: impl Into<points_selector::PointsSelectorOneOf>,
        ) -> &mut Self {
            self.points = Some(PointsSelector {
                points_selector_one_of: Some(points.into()),
            });
            self
        }

        /// Option for custom sharding to specify used shard keys
        pub fn shard_key_selector(
            &mut self,
            shard_key_selector: impl Into<ShardKeySelector>,
        ) -> &mut Self {
            self.shard_key_selector = Some(shard_key_selector.into());
            self
        }

        fn build_inner(&self) -> Result<ClearPayload, ()> {
            let builder = self.clone();
            Ok(ClearPayload {
                points: builder.points,
                shard_key_selector: builder.shard_key_selector,
            })
        }
    }

    builder_type_conversions!(ClearPayload, ClearPayloadBuilder);
}

#[derive(Builder)]
#[builder(
    build_fn(private, name = "build_inner"),
    pattern = "owned",
    custom_constructor
)]
pub struct SnapshotDownload {
    pub out_path: PathBuf,
    pub collection_name: String,
    #[builder(default, setter(strip_option, into))]
    pub snapshot_name: Option<String>,
    #[builder(default, setter(strip_option, into))]
    pub rest_api_uri: Option<String>,
}

impl SnapshotDownloadBuilder {
    pub fn new(out_path: impl Into<PathBuf>, collection_name: impl Into<String>) -> Self {
        let mut builder = Self::create_empty();
        builder.out_path = Some(out_path.into());
        builder.collection_name = Some(collection_name.into());
        builder
    }

    pub fn build(self) -> SnapshotDownload {
        self.build_inner().unwrap()
    }
}

impl From<SnapshotDownloadBuilder> for SnapshotDownload {
    fn from(value: SnapshotDownloadBuilder) -> Self {
        value.build()
    }
}
