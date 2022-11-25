#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorParams {
    /// Size of the vectors
    #[prost(uint64, tag="1")]
    pub size: u64,
    /// Distance function used for comparing vectors
    #[prost(enumeration="Distance", tag="2")]
    pub distance: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorParamsMap {
    #[prost(map="string, message", tag="1")]
    pub map: ::std::collections::HashMap<::prost::alloc::string::String, VectorParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorsConfig {
    #[prost(oneof="vectors_config::Config", tags="1, 2")]
    pub config: ::core::option::Option<vectors_config::Config>,
}
/// Nested message and enum types in `VectorsConfig`.
pub mod vectors_config {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Config {
        #[prost(message, tag="1")]
        Params(super::VectorParams),
        #[prost(message, tag="2")]
        ParamsMap(super::VectorParamsMap),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionInfoRequest {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionDescription {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetCollectionInfoResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<CollectionInfo>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListCollectionsResponse {
    #[prost(message, repeated, tag="1")]
    pub collections: ::prost::alloc::vec::Vec<CollectionDescription>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizerStatus {
    #[prost(bool, tag="1")]
    pub ok: bool,
    #[prost(string, tag="2")]
    pub error: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HnswConfigDiff {
    ///
    ///Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
    #[prost(uint64, optional, tag="1")]
    pub m: ::core::option::Option<u64>,
    ///
    ///Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build index.
    #[prost(uint64, optional, tag="2")]
    pub ef_construct: ::core::option::Option<u64>,
    ///
    ///Minimal size (in KiloBytes) of vectors for additional payload-based indexing.
    ///If payload chunk is smaller than `full_scan_threshold` additional indexing won't be used -
    ///in this case full-scan search should be preferred by query planner and additional indexing is not required.
    ///Note: 1Kb = 1 vector of size 256
    #[prost(uint64, optional, tag="3")]
    pub full_scan_threshold: ::core::option::Option<u64>,
    ///
    ///Number of parallel threads used for background index building. If 0 - auto selection.
    #[prost(uint64, optional, tag="4")]
    pub max_indexing_threads: ::core::option::Option<u64>,
    ///
    ///Store HNSW index on disk. If set to false, index will be stored in RAM.
    #[prost(bool, optional, tag="5")]
    pub on_disk: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WalConfigDiff {
    /// Size of a single WAL block file
    #[prost(uint64, optional, tag="1")]
    pub wal_capacity_mb: ::core::option::Option<u64>,
    /// Number of segments to create in advance
    #[prost(uint64, optional, tag="2")]
    pub wal_segments_ahead: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OptimizersConfigDiff {
    ///
    ///The minimal fraction of deleted vectors in a segment, required to perform segment optimization
    #[prost(double, optional, tag="1")]
    pub deleted_threshold: ::core::option::Option<f64>,
    ///
    ///The minimal number of vectors in a segment, required to perform segment optimization
    #[prost(uint64, optional, tag="2")]
    pub vacuum_min_vector_number: ::core::option::Option<u64>,
    ///
    ///Target amount of segments optimizer will try to keep.
    ///Real amount of segments may vary depending on multiple parameters:
    ///
    ///- Amount of stored points.
    ///- Current write RPS.
    ///
    ///It is recommended to select default number of segments as a factor of the number of search threads,
    ///so that each segment would be handled evenly by one of the threads.
    #[prost(uint64, optional, tag="3")]
    pub default_segment_number: ::core::option::Option<u64>,
    ///
    ///Do not create segments larger this size (in KiloBytes).
    ///Large segments might require disproportionately long indexation times,
    ///therefore it makes sense to limit the size of segments.
    ///
    ///If indexation speed have more priority for your - make this parameter lower.
    ///If search speed is more important - make this parameter higher.
    ///Note: 1Kb = 1 vector of size 256
    #[prost(uint64, optional, tag="4")]
    pub max_segment_size: ::core::option::Option<u64>,
    ///
    ///Maximum size (in KiloBytes) of vectors to store in-memory per segment.
    ///Segments larger than this threshold will be stored as read-only memmaped file.
    ///To enable memmap storage, lower the threshold
    ///Note: 1Kb = 1 vector of size 256
    #[prost(uint64, optional, tag="5")]
    pub memmap_threshold: ::core::option::Option<u64>,
    ///
    ///Maximum size (in KiloBytes) of vectors allowed for plain index.
    ///Default value based on <https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md>
    ///Note: 1Kb = 1 vector of size 256
    #[prost(uint64, optional, tag="6")]
    pub indexing_threshold: ::core::option::Option<u64>,
    ///
    ///Interval between forced flushes.
    #[prost(uint64, optional, tag="7")]
    pub flush_interval_sec: ::core::option::Option<u64>,
    ///
    ///Max number of threads, which can be used for optimization. If 0 - `NUM_CPU - 1` will be used
    #[prost(uint64, optional, tag="8")]
    pub max_optimization_threads: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateCollection {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Configuration of vector index
    #[prost(message, optional, tag="4")]
    pub hnsw_config: ::core::option::Option<HnswConfigDiff>,
    /// Configuration of the Write-Ahead-Log
    #[prost(message, optional, tag="5")]
    pub wal_config: ::core::option::Option<WalConfigDiff>,
    /// Configuration of the optimizers
    #[prost(message, optional, tag="6")]
    pub optimizers_config: ::core::option::Option<OptimizersConfigDiff>,
    /// Number of shards in the collection, default = 1
    #[prost(uint32, optional, tag="7")]
    pub shard_number: ::core::option::Option<u32>,
    /// If true - point's payload will not be stored in memory
    #[prost(bool, optional, tag="8")]
    pub on_disk_payload: ::core::option::Option<bool>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[prost(uint64, optional, tag="9")]
    pub timeout: ::core::option::Option<u64>,
    /// Configuration for vectors
    #[prost(message, optional, tag="10")]
    pub vectors_config: ::core::option::Option<VectorsConfig>,
    /// Number of replicas of each shard that network tries to maintain, default = 1
    #[prost(uint32, optional, tag="11")]
    pub replication_factor: ::core::option::Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful, default = 1
    #[prost(uint32, optional, tag="12")]
    pub write_consistency_factor: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateCollection {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// New configuration parameters for the collection
    #[prost(message, optional, tag="2")]
    pub optimizers_config: ::core::option::Option<OptimizersConfigDiff>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[prost(uint64, optional, tag="3")]
    pub timeout: ::core::option::Option<u64>,
    /// New configuration parameters for the collection
    #[prost(message, optional, tag="4")]
    pub params: ::core::option::Option<CollectionParamsDiff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteCollection {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[prost(uint64, optional, tag="2")]
    pub timeout: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionOperationResponse {
    /// if operation made changes
    #[prost(bool, tag="1")]
    pub result: bool,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionParams {
    /// Number of shards in collection
    #[prost(uint32, tag="3")]
    pub shard_number: u32,
    /// If true - point's payload will not be stored in memory
    #[prost(bool, tag="4")]
    pub on_disk_payload: bool,
    /// Configuration for vectors
    #[prost(message, optional, tag="5")]
    pub vectors_config: ::core::option::Option<VectorsConfig>,
    /// Number of replicas of each shard that network tries to maintain
    #[prost(uint32, optional, tag="6")]
    pub replication_factor: ::core::option::Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful
    #[prost(uint32, optional, tag="7")]
    pub write_consistency_factor: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionParamsDiff {
    /// Number of replicas of each shard that network tries to maintain
    #[prost(uint32, optional, tag="1")]
    pub replication_factor: ::core::option::Option<u32>,
    /// How many replicas should apply the operation for us to consider it successful
    #[prost(uint32, optional, tag="2")]
    pub write_consistency_factor: ::core::option::Option<u32>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionConfig {
    /// Collection parameters
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<CollectionParams>,
    /// Configuration of vector index
    #[prost(message, optional, tag="2")]
    pub hnsw_config: ::core::option::Option<HnswConfigDiff>,
    /// Configuration of the optimizers
    #[prost(message, optional, tag="3")]
    pub optimizer_config: ::core::option::Option<OptimizersConfigDiff>,
    /// Configuration of the Write-Ahead-Log
    #[prost(message, optional, tag="4")]
    pub wal_config: ::core::option::Option<WalConfigDiff>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TextIndexParams {
    /// Tokenizer type
    #[prost(enumeration="TokenizerType", tag="1")]
    pub tokenizer: i32,
    /// If true - all tokens will be lowercased
    #[prost(bool, optional, tag="2")]
    pub lowercase: ::core::option::Option<bool>,
    /// Minimal token length
    #[prost(uint64, optional, tag="3")]
    pub min_token_len: ::core::option::Option<u64>,
    /// Maximal token length
    #[prost(uint64, optional, tag="4")]
    pub max_token_len: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadIndexParams {
    #[prost(oneof="payload_index_params::IndexParams", tags="1")]
    pub index_params: ::core::option::Option<payload_index_params::IndexParams>,
}
/// Nested message and enum types in `PayloadIndexParams`.
pub mod payload_index_params {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum IndexParams {
        /// Parameters for text index
        #[prost(message, tag="1")]
        TextIndexParams(super::TextIndexParams),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadSchemaInfo {
    /// Field data type
    #[prost(enumeration="PayloadSchemaType", tag="1")]
    pub data_type: i32,
    /// Field index parameters
    #[prost(message, optional, tag="2")]
    pub params: ::core::option::Option<PayloadIndexParams>,
    /// Number of points indexed within this field indexed
    #[prost(uint64, optional, tag="3")]
    pub points: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CollectionInfo {
    /// operating condition of the collection
    #[prost(enumeration="CollectionStatus", tag="1")]
    pub status: i32,
    /// status of collection optimizers
    #[prost(message, optional, tag="2")]
    pub optimizer_status: ::core::option::Option<OptimizerStatus>,
    /// number of vectors in the collection
    #[prost(uint64, tag="3")]
    pub vectors_count: u64,
    /// Number of independent segments
    #[prost(uint64, tag="4")]
    pub segments_count: u64,
    /// Configuration
    #[prost(message, optional, tag="7")]
    pub config: ::core::option::Option<CollectionConfig>,
    /// Collection data types
    #[prost(map="string, message", tag="8")]
    pub payload_schema: ::std::collections::HashMap<::prost::alloc::string::String, PayloadSchemaInfo>,
    /// number of points in the collection
    #[prost(uint64, tag="9")]
    pub points_count: u64,
    /// number of indexed vectors in the collection.
    #[prost(uint64, optional, tag="10")]
    pub indexed_vectors_count: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ChangeAliases {
    /// List of actions
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<AliasOperations>,
    /// Wait timeout for operation commit in seconds, if not specified - default value will be supplied
    #[prost(uint64, optional, tag="2")]
    pub timeout: ::core::option::Option<u64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AliasOperations {
    #[prost(oneof="alias_operations::Action", tags="1, 2, 3")]
    pub action: ::core::option::Option<alias_operations::Action>,
}
/// Nested message and enum types in `AliasOperations`.
pub mod alias_operations {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Action {
        #[prost(message, tag="1")]
        CreateAlias(super::CreateAlias),
        #[prost(message, tag="2")]
        RenameAlias(super::RenameAlias),
        #[prost(message, tag="3")]
        DeleteAlias(super::DeleteAlias),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateAlias {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// New name of the alias
    #[prost(string, tag="2")]
    pub alias_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RenameAlias {
    /// Name of the alias to rename
    #[prost(string, tag="1")]
    pub old_alias_name: ::prost::alloc::string::String,
    /// Name of the alias
    #[prost(string, tag="2")]
    pub new_alias_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteAlias {
    /// Name of the alias
    #[prost(string, tag="1")]
    pub alias_name: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Distance {
    UnknownDistance = 0,
    Cosine = 1,
    Euclid = 2,
    Dot = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CollectionStatus {
    UnknownCollectionStatus = 0,
    /// All segments are ready
    Green = 1,
    /// Optimization in process
    Yellow = 2,
    /// Something went wrong
    Red = 3,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PayloadSchemaType {
    UnknownType = 0,
    Keyword = 1,
    Integer = 2,
    Float = 3,
    Geo = 4,
    Text = 5,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TokenizerType {
    Unknown = 0,
    Prefix = 1,
    Whitespace = 2,
    Word = 3,
}
/// Generated client implementations.
pub mod collections_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct CollectionsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl CollectionsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> CollectionsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> CollectionsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            CollectionsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///
        ///Get detailed information about specified existing collection
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetCollectionInfoRequest>,
        ) -> Result<tonic::Response<super::GetCollectionInfoResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Collections/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Get list name of all existing collections
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListCollectionsRequest>,
        ) -> Result<tonic::Response<super::ListCollectionsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Collections/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Create new collection with given parameters
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Collections/Create",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Update parameters of the existing collection
        pub async fn update(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Collections/Update",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Drop collection and all associated data
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Collections/Delete",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Update Aliases of the existing collection
        pub async fn update_aliases(
            &mut self,
            request: impl tonic::IntoRequest<super::ChangeAliases>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Collections/UpdateAliases",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod collections_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with CollectionsServer.
    #[async_trait]
    pub trait Collections: Send + Sync + 'static {
        ///
        ///Get detailed information about specified existing collection
        async fn get(
            &self,
            request: tonic::Request<super::GetCollectionInfoRequest>,
        ) -> Result<tonic::Response<super::GetCollectionInfoResponse>, tonic::Status>;
        ///
        ///Get list name of all existing collections
        async fn list(
            &self,
            request: tonic::Request<super::ListCollectionsRequest>,
        ) -> Result<tonic::Response<super::ListCollectionsResponse>, tonic::Status>;
        ///
        ///Create new collection with given parameters
        async fn create(
            &self,
            request: tonic::Request<super::CreateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
        ///
        ///Update parameters of the existing collection
        async fn update(
            &self,
            request: tonic::Request<super::UpdateCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
        ///
        ///Drop collection and all associated data
        async fn delete(
            &self,
            request: tonic::Request<super::DeleteCollection>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
        ///
        ///Update Aliases of the existing collection
        async fn update_aliases(
            &self,
            request: tonic::Request<super::ChangeAliases>,
        ) -> Result<tonic::Response<super::CollectionOperationResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct CollectionsServer<T: Collections> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Collections> CollectionsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for CollectionsServer<T>
    where
        T: Collections,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qdrant.Collections/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::GetCollectionInfoRequest>
                    for GetSvc<T> {
                        type Response = super::GetCollectionInfoResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetCollectionInfoRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Collections/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::ListCollectionsRequest>
                    for ListSvc<T> {
                        type Response = super::ListCollectionsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListCollectionsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Collections/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::CreateCollection>
                    for CreateSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Collections/Update" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::UpdateCollection>
                    for UpdateSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).update(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Collections/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::DeleteCollection>
                    for DeleteSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Collections/UpdateAliases" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateAliasesSvc<T: Collections>(pub Arc<T>);
                    impl<
                        T: Collections,
                    > tonic::server::UnaryService<super::ChangeAliases>
                    for UpdateAliasesSvc<T> {
                        type Response = super::CollectionOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ChangeAliases>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).update_aliases(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateAliasesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Collections> Clone for CollectionsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Collections> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Collections> tonic::transport::NamedService for CollectionsServer<T> {
        const NAME: &'static str = "qdrant.Collections";
    }
}
/// `Struct` represents a structured data value, consisting of fields
/// which map to dynamically typed values. In some languages, `Struct`
/// might be supported by a native representation. For example, in
/// scripting languages like JS a struct is represented as an
/// object. The details of that representation are described together
/// with the proto support for the language.
///
/// The JSON representation for `Struct` is JSON object.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Struct {
    /// Unordered map of dynamically typed values.
    #[prost(map="string, message", tag="1")]
    pub fields: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
}
/// `Value` represents a dynamically typed value which can be either
/// null, a number, a string, a boolean, a recursive struct value, or a
/// list of values. A producer of value is expected to set one of that
/// variants, absence of any variant indicates an error.
///
/// The JSON representation for `Value` is JSON value.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Value {
    /// The kind of value.
    #[prost(oneof="value::Kind", tags="1, 2, 3, 4, 5, 6, 7")]
    pub kind: ::core::option::Option<value::Kind>,
}
/// Nested message and enum types in `Value`.
pub mod value {
    /// The kind of value.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Kind {
        /// Represents a null value.
        #[prost(enumeration="super::NullValue", tag="1")]
        NullValue(i32),
        /// Represents a double value.
        #[prost(double, tag="2")]
        DoubleValue(f64),
        /// Represents an integer value
        #[prost(int64, tag="3")]
        IntegerValue(i64),
        /// Represents a string value.
        #[prost(string, tag="4")]
        StringValue(::prost::alloc::string::String),
        /// Represents a boolean value.
        #[prost(bool, tag="5")]
        BoolValue(bool),
        /// Represents a structured value.
        #[prost(message, tag="6")]
        StructValue(super::Struct),
        /// Represents a repeated `Value`.
        #[prost(message, tag="7")]
        ListValue(super::ListValue),
    }
}
/// `ListValue` is a wrapper around a repeated field of values.
///
/// The JSON representation for `ListValue` is JSON array.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListValue {
    /// Repeated field of dynamically typed values.
    #[prost(message, repeated, tag="1")]
    pub values: ::prost::alloc::vec::Vec<Value>,
}
/// `NullValue` is a singleton enumeration to represent the null value for the
/// `Value` type union.
///
///  The JSON representation for `NullValue` is JSON `null`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum NullValue {
    /// Null value.
    NullValue = 0,
}
// ---------------------------------------------
// ------------- Point Id Requests -------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointId {
    #[prost(oneof="point_id::PointIdOptions", tags="1, 2")]
    pub point_id_options: ::core::option::Option<point_id::PointIdOptions>,
}
/// Nested message and enum types in `PointId`.
pub mod point_id {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointIdOptions {
        /// Numerical ID of the point
        #[prost(uint64, tag="1")]
        Num(u64),
        /// UUID
        #[prost(string, tag="2")]
        Uuid(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vector {
    #[prost(float, repeated, tag="1")]
    pub data: ::prost::alloc::vec::Vec<f32>,
}
// ---------------------------------------------
// ---------------- RPC Requests ---------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpsertPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    #[prost(message, repeated, tag="3")]
    pub points: ::prost::alloc::vec::Vec<PointStruct>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// Affected points
    #[prost(message, optional, tag="3")]
    pub points: ::core::option::Option<PointsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// List of points to retrieve
    #[prost(message, repeated, tag="2")]
    pub ids: ::prost::alloc::vec::Vec<PointId>,
    /// Options for specifying which payload to include or not
    #[prost(message, optional, tag="4")]
    pub with_payload: ::core::option::Option<WithPayloadSelector>,
    /// Options for specifying which vectors to include into response
    #[prost(message, optional, tag="5")]
    pub with_vectors: ::core::option::Option<WithVectorsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPayloadPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// New payload values
    #[prost(map="string, message", tag="3")]
    pub payload: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
    /// List of point to modify
    #[prost(message, repeated, tag="4")]
    pub points: ::prost::alloc::vec::Vec<PointId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePayloadPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// List of keys to delete
    #[prost(string, repeated, tag="3")]
    pub keys: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Affected points
    #[prost(message, repeated, tag="4")]
    pub points: ::prost::alloc::vec::Vec<PointId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClearPayloadPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// Affected points
    #[prost(message, optional, tag="3")]
    pub points: ::core::option::Option<PointsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFieldIndexCollection {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// Field name to index
    #[prost(string, tag="3")]
    pub field_name: ::prost::alloc::string::String,
    /// Field type.
    #[prost(enumeration="FieldType", optional, tag="4")]
    pub field_type: ::core::option::Option<i32>,
    /// Payload index params.
    #[prost(message, optional, tag="5")]
    pub field_index_params: ::core::option::Option<PayloadIndexParams>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteFieldIndexCollection {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Wait until the changes have been applied?
    #[prost(bool, optional, tag="2")]
    pub wait: ::core::option::Option<bool>,
    /// Field name to delete
    #[prost(string, tag="3")]
    pub field_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadIncludeSelector {
    /// List of payload keys to include into result
    #[prost(string, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadExcludeSelector {
    /// List of payload keys to exclude from the result
    #[prost(string, repeated, tag="1")]
    pub fields: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithPayloadSelector {
    #[prost(oneof="with_payload_selector::SelectorOptions", tags="1, 2, 3")]
    pub selector_options: ::core::option::Option<with_payload_selector::SelectorOptions>,
}
/// Nested message and enum types in `WithPayloadSelector`.
pub mod with_payload_selector {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SelectorOptions {
        /// If `true` - return all payload, if `false` - none
        #[prost(bool, tag="1")]
        Enable(bool),
        #[prost(message, tag="2")]
        Include(super::PayloadIncludeSelector),
        #[prost(message, tag="3")]
        Exclude(super::PayloadExcludeSelector),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NamedVectors {
    #[prost(map="string, message", tag="1")]
    pub vectors: ::std::collections::HashMap<::prost::alloc::string::String, Vector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vectors {
    #[prost(oneof="vectors::VectorsOptions", tags="1, 2")]
    pub vectors_options: ::core::option::Option<vectors::VectorsOptions>,
}
/// Nested message and enum types in `Vectors`.
pub mod vectors {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum VectorsOptions {
        #[prost(message, tag="1")]
        Vector(super::Vector),
        #[prost(message, tag="2")]
        Vectors(super::NamedVectors),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VectorsSelector {
    /// List of vectors to include into result
    #[prost(string, repeated, tag="1")]
    pub names: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithVectorsSelector {
    #[prost(oneof="with_vectors_selector::SelectorOptions", tags="1, 2")]
    pub selector_options: ::core::option::Option<with_vectors_selector::SelectorOptions>,
}
/// Nested message and enum types in `WithVectorsSelector`.
pub mod with_vectors_selector {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum SelectorOptions {
        /// If `true` - return all vectors, if `false` - none
        #[prost(bool, tag="1")]
        Enable(bool),
        /// List of payload keys to include into result
        #[prost(message, tag="2")]
        Include(super::VectorsSelector),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchParams {
    ///
    ///Params relevant to HNSW index. Size of the beam in a beam-search.
    ///Larger the value - more accurate the result, more time required for search.
    #[prost(uint64, optional, tag="1")]
    pub hnsw_ef: ::core::option::Option<u64>,
    ///
    ///Search without approximation. If set to true, search may run long but with exact results.
    #[prost(bool, optional, tag="2")]
    pub exact: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// vector
    #[prost(float, repeated, tag="2")]
    pub vector: ::prost::alloc::vec::Vec<f32>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[prost(message, optional, tag="3")]
    pub filter: ::core::option::Option<Filter>,
    /// Max number of result
    #[prost(uint64, tag="4")]
    pub limit: u64,
    /// Options for specifying which payload to include or not
    #[prost(message, optional, tag="6")]
    pub with_payload: ::core::option::Option<WithPayloadSelector>,
    /// Search config
    #[prost(message, optional, tag="7")]
    pub params: ::core::option::Option<SearchParams>,
    /// If provided - cut off results with worse scores
    #[prost(float, optional, tag="8")]
    pub score_threshold: ::core::option::Option<f32>,
    /// Offset of the result
    #[prost(uint64, optional, tag="9")]
    pub offset: ::core::option::Option<u64>,
    /// Which vector to use for search, if not specified - use default vector
    #[prost(string, optional, tag="10")]
    pub vector_name: ::core::option::Option<::prost::alloc::string::String>,
    /// Options for specifying which vectors to include into response
    #[prost(message, optional, tag="11")]
    pub with_vectors: ::core::option::Option<WithVectorsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBatchPoints {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub search_points: ::prost::alloc::vec::Vec<SearchPoints>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrollPoints {
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[prost(message, optional, tag="2")]
    pub filter: ::core::option::Option<Filter>,
    /// Start with this ID
    #[prost(message, optional, tag="3")]
    pub offset: ::core::option::Option<PointId>,
    /// Max number of result
    #[prost(uint32, optional, tag="4")]
    pub limit: ::core::option::Option<u32>,
    /// Options for specifying which payload to include or not
    #[prost(message, optional, tag="6")]
    pub with_payload: ::core::option::Option<WithPayloadSelector>,
    /// Options for specifying which vectors to include into response
    #[prost(message, optional, tag="7")]
    pub with_vectors: ::core::option::Option<WithVectorsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Look for vectors closest to those
    #[prost(message, repeated, tag="2")]
    pub positive: ::prost::alloc::vec::Vec<PointId>,
    /// Try to avoid vectors like this
    #[prost(message, repeated, tag="3")]
    pub negative: ::prost::alloc::vec::Vec<PointId>,
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[prost(message, optional, tag="4")]
    pub filter: ::core::option::Option<Filter>,
    /// Max number of result
    #[prost(uint64, tag="5")]
    pub limit: u64,
    /// Options for specifying which payload to include or not
    #[prost(message, optional, tag="7")]
    pub with_payload: ::core::option::Option<WithPayloadSelector>,
    /// Search config
    #[prost(message, optional, tag="8")]
    pub params: ::core::option::Option<SearchParams>,
    /// If provided - cut off results with worse scores
    #[prost(float, optional, tag="9")]
    pub score_threshold: ::core::option::Option<f32>,
    /// Offset of the result
    #[prost(uint64, optional, tag="10")]
    pub offset: ::core::option::Option<u64>,
    /// Define which vector to use for recommendation, if not specified - default vector
    #[prost(string, optional, tag="11")]
    pub using: ::core::option::Option<::prost::alloc::string::String>,
    /// Options for specifying which vectors to include into response
    #[prost(message, optional, tag="12")]
    pub with_vectors: ::core::option::Option<WithVectorsSelector>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendBatchPoints {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(message, repeated, tag="2")]
    pub recommend_points: ::prost::alloc::vec::Vec<RecommendPoints>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountPoints {
    /// name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    /// Filter conditions - return only those points that satisfy the specified conditions
    #[prost(message, optional, tag="2")]
    pub filter: ::core::option::Option<Filter>,
    /// If `true` - return exact count, if `false` - return approximate count
    #[prost(bool, optional, tag="3")]
    pub exact: ::core::option::Option<bool>,
}
// ---------------------------------------------
// ---------------- RPC Response ---------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsOperationResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<UpdateResult>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateResult {
    /// Number of operation
    #[prost(uint64, tag="1")]
    pub operation_id: u64,
    /// Operation status
    #[prost(enumeration="UpdateStatus", tag="2")]
    pub status: i32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScoredPoint {
    /// Point id
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<PointId>,
    /// Payload
    #[prost(map="string, message", tag="2")]
    pub payload: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
    /// Similarity score
    #[prost(float, tag="3")]
    pub score: f32,
    /// Last update operation applied to this point
    #[prost(uint64, tag="5")]
    pub version: u64,
    /// Vectors to search
    #[prost(message, optional, tag="6")]
    pub vectors: ::core::option::Option<Vectors>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<ScoredPoint>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BatchResult {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<ScoredPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchBatchResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<BatchResult>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountResponse {
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<CountResult>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ScrollResponse {
    /// Use this offset for the next query
    #[prost(message, optional, tag="1")]
    pub next_page_offset: ::core::option::Option<PointId>,
    #[prost(message, repeated, tag="2")]
    pub result: ::prost::alloc::vec::Vec<RetrievedPoint>,
    /// Time spent to process
    #[prost(double, tag="3")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CountResult {
    #[prost(uint64, tag="1")]
    pub count: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RetrievedPoint {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<PointId>,
    #[prost(map="string, message", tag="2")]
    pub payload: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
    #[prost(message, optional, tag="4")]
    pub vectors: ::core::option::Option<Vectors>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<RetrievedPoint>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<ScoredPoint>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RecommendBatchResponse {
    #[prost(message, repeated, tag="1")]
    pub result: ::prost::alloc::vec::Vec<BatchResult>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
// ---------------------------------------------
// ------------- Filter Conditions -------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Filter {
    /// At least one of those conditions should match
    #[prost(message, repeated, tag="1")]
    pub should: ::prost::alloc::vec::Vec<Condition>,
    /// All conditions must match
    #[prost(message, repeated, tag="2")]
    pub must: ::prost::alloc::vec::Vec<Condition>,
    /// All conditions must NOT match
    #[prost(message, repeated, tag="3")]
    pub must_not: ::prost::alloc::vec::Vec<Condition>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Condition {
    #[prost(oneof="condition::ConditionOneOf", tags="1, 2, 3, 4")]
    pub condition_one_of: ::core::option::Option<condition::ConditionOneOf>,
}
/// Nested message and enum types in `Condition`.
pub mod condition {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum ConditionOneOf {
        #[prost(message, tag="1")]
        Field(super::FieldCondition),
        #[prost(message, tag="2")]
        IsEmpty(super::IsEmptyCondition),
        #[prost(message, tag="3")]
        HasId(super::HasIdCondition),
        #[prost(message, tag="4")]
        Filter(super::Filter),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IsEmptyCondition {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HasIdCondition {
    #[prost(message, repeated, tag="1")]
    pub has_id: ::prost::alloc::vec::Vec<PointId>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FieldCondition {
    #[prost(string, tag="1")]
    pub key: ::prost::alloc::string::String,
    /// Check if point has field with a given value
    #[prost(message, optional, tag="2")]
    pub r#match: ::core::option::Option<Match>,
    /// Check if points value lies in a given range
    #[prost(message, optional, tag="3")]
    pub range: ::core::option::Option<Range>,
    /// Check if points geo location lies in a given area
    #[prost(message, optional, tag="4")]
    pub geo_bounding_box: ::core::option::Option<GeoBoundingBox>,
    /// Check if geo point is within a given radius
    #[prost(message, optional, tag="5")]
    pub geo_radius: ::core::option::Option<GeoRadius>,
    /// Check number of values for a specific field
    #[prost(message, optional, tag="6")]
    pub values_count: ::core::option::Option<ValuesCount>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Match {
    #[prost(oneof="r#match::MatchValue", tags="1, 2, 3, 4")]
    pub match_value: ::core::option::Option<r#match::MatchValue>,
}
/// Nested message and enum types in `Match`.
pub mod r#match {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum MatchValue {
        /// Match string keyword
        #[prost(string, tag="1")]
        Keyword(::prost::alloc::string::String),
        /// Match integer
        #[prost(int64, tag="2")]
        Integer(i64),
        /// Match boolean
        #[prost(bool, tag="3")]
        Boolean(bool),
        /// Match text
        #[prost(string, tag="4")]
        Text(::prost::alloc::string::String),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Range {
    #[prost(double, optional, tag="1")]
    pub lt: ::core::option::Option<f64>,
    #[prost(double, optional, tag="2")]
    pub gt: ::core::option::Option<f64>,
    #[prost(double, optional, tag="3")]
    pub gte: ::core::option::Option<f64>,
    #[prost(double, optional, tag="4")]
    pub lte: ::core::option::Option<f64>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoBoundingBox {
    /// north-west corner
    #[prost(message, optional, tag="1")]
    pub top_left: ::core::option::Option<GeoPoint>,
    /// south-east corner
    #[prost(message, optional, tag="2")]
    pub bottom_right: ::core::option::Option<GeoPoint>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoRadius {
    /// Center of the circle
    #[prost(message, optional, tag="1")]
    pub center: ::core::option::Option<GeoPoint>,
    /// In meters
    #[prost(float, tag="2")]
    pub radius: f32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValuesCount {
    #[prost(uint64, optional, tag="1")]
    pub lt: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="2")]
    pub gt: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="3")]
    pub gte: ::core::option::Option<u64>,
    #[prost(uint64, optional, tag="4")]
    pub lte: ::core::option::Option<u64>,
}
// ---------------------------------------------
// -------------- Points Selector --------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsSelector {
    #[prost(oneof="points_selector::PointsSelectorOneOf", tags="1, 2")]
    pub points_selector_one_of: ::core::option::Option<points_selector::PointsSelectorOneOf>,
}
/// Nested message and enum types in `PointsSelector`.
pub mod points_selector {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum PointsSelectorOneOf {
        #[prost(message, tag="1")]
        Points(super::PointsIdsList),
        #[prost(message, tag="2")]
        Filter(super::Filter),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointsIdsList {
    #[prost(message, repeated, tag="1")]
    pub ids: ::prost::alloc::vec::Vec<PointId>,
}
// ---------------------------------------------
// ------------------- Point -------------------
// ---------------------------------------------

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PointStruct {
    #[prost(message, optional, tag="1")]
    pub id: ::core::option::Option<PointId>,
    #[prost(map="string, message", tag="3")]
    pub payload: ::std::collections::HashMap<::prost::alloc::string::String, Value>,
    #[prost(message, optional, tag="4")]
    pub vectors: ::core::option::Option<Vectors>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GeoPoint {
    #[prost(double, tag="1")]
    pub lon: f64,
    #[prost(double, tag="2")]
    pub lat: f64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FieldType {
    Keyword = 0,
    Integer = 1,
    Float = 2,
    Geo = 3,
    Text = 4,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UpdateStatus {
    UnknownUpdateStatus = 0,
    /// Update is received, but not processed yet
    Acknowledged = 1,
    /// Update is applied and ready for search
    Completed = 2,
}
/// Generated client implementations.
pub mod points_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct PointsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl PointsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> PointsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> PointsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            PointsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///
        ///Perform insert + updates on points. If point with given ID already exists - it will be overwritten.
        pub async fn upsert(
            &mut self,
            request: impl tonic::IntoRequest<super::UpsertPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Upsert");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Delete points
        pub async fn delete(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Delete");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Retrieve points
        pub async fn get(
            &mut self,
            request: impl tonic::IntoRequest<super::GetPoints>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Get");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Set payload for points
        pub async fn set_payload(
            &mut self,
            request: impl tonic::IntoRequest<super::SetPayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/SetPayload");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Delete specified key payload for points
        pub async fn delete_payload(
            &mut self,
            request: impl tonic::IntoRequest<super::DeletePayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/DeletePayload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Remove all payload for specified points
        pub async fn clear_payload(
            &mut self,
            request: impl tonic::IntoRequest<super::ClearPayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/ClearPayload",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Create index for field in collection
        pub async fn create_field_index(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFieldIndexCollection>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/CreateFieldIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Delete field index for collection
        pub async fn delete_field_index(
            &mut self,
            request: impl tonic::IntoRequest<super::DeleteFieldIndexCollection>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/DeleteFieldIndex",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Retrieve closest points based on vector similarity and given filtering conditions
        pub async fn search(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchPoints>,
        ) -> Result<tonic::Response<super::SearchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Search");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Retrieve closest points based on vector similarity and given filtering conditions
        pub async fn search_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::SearchBatchPoints>,
        ) -> Result<tonic::Response<super::SearchBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/SearchBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Iterate over all or filtered points points
        pub async fn scroll(
            &mut self,
            request: impl tonic::IntoRequest<super::ScrollPoints>,
        ) -> Result<tonic::Response<super::ScrollResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Scroll");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Look for the points which are closer to stored positive examples and at the same time further to negative examples.
        pub async fn recommend(
            &mut self,
            request: impl tonic::IntoRequest<super::RecommendPoints>,
        ) -> Result<tonic::Response<super::RecommendResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Recommend");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Look for the points which are closer to stored positive examples and at the same time further to negative examples.
        pub async fn recommend_batch(
            &mut self,
            request: impl tonic::IntoRequest<super::RecommendBatchPoints>,
        ) -> Result<tonic::Response<super::RecommendBatchResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Points/RecommendBatch",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Count points in collection with given filtering conditions
        pub async fn count(
            &mut self,
            request: impl tonic::IntoRequest<super::CountPoints>,
        ) -> Result<tonic::Response<super::CountResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Points/Count");
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod points_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with PointsServer.
    #[async_trait]
    pub trait Points: Send + Sync + 'static {
        ///
        ///Perform insert + updates on points. If point with given ID already exists - it will be overwritten.
        async fn upsert(
            &self,
            request: tonic::Request<super::UpsertPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Delete points
        async fn delete(
            &self,
            request: tonic::Request<super::DeletePoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Retrieve points
        async fn get(
            &self,
            request: tonic::Request<super::GetPoints>,
        ) -> Result<tonic::Response<super::GetResponse>, tonic::Status>;
        ///
        ///Set payload for points
        async fn set_payload(
            &self,
            request: tonic::Request<super::SetPayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Delete specified key payload for points
        async fn delete_payload(
            &self,
            request: tonic::Request<super::DeletePayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Remove all payload for specified points
        async fn clear_payload(
            &self,
            request: tonic::Request<super::ClearPayloadPoints>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Create index for field in collection
        async fn create_field_index(
            &self,
            request: tonic::Request<super::CreateFieldIndexCollection>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Delete field index for collection
        async fn delete_field_index(
            &self,
            request: tonic::Request<super::DeleteFieldIndexCollection>,
        ) -> Result<tonic::Response<super::PointsOperationResponse>, tonic::Status>;
        ///
        ///Retrieve closest points based on vector similarity and given filtering conditions
        async fn search(
            &self,
            request: tonic::Request<super::SearchPoints>,
        ) -> Result<tonic::Response<super::SearchResponse>, tonic::Status>;
        ///
        ///Retrieve closest points based on vector similarity and given filtering conditions
        async fn search_batch(
            &self,
            request: tonic::Request<super::SearchBatchPoints>,
        ) -> Result<tonic::Response<super::SearchBatchResponse>, tonic::Status>;
        ///
        ///Iterate over all or filtered points points
        async fn scroll(
            &self,
            request: tonic::Request<super::ScrollPoints>,
        ) -> Result<tonic::Response<super::ScrollResponse>, tonic::Status>;
        ///
        ///Look for the points which are closer to stored positive examples and at the same time further to negative examples.
        async fn recommend(
            &self,
            request: tonic::Request<super::RecommendPoints>,
        ) -> Result<tonic::Response<super::RecommendResponse>, tonic::Status>;
        ///
        ///Look for the points which are closer to stored positive examples and at the same time further to negative examples.
        async fn recommend_batch(
            &self,
            request: tonic::Request<super::RecommendBatchPoints>,
        ) -> Result<tonic::Response<super::RecommendBatchResponse>, tonic::Status>;
        ///
        ///Count points in collection with given filtering conditions
        async fn count(
            &self,
            request: tonic::Request<super::CountPoints>,
        ) -> Result<tonic::Response<super::CountResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct PointsServer<T: Points> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Points> PointsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for PointsServer<T>
    where
        T: Points,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qdrant.Points/Upsert" => {
                    #[allow(non_camel_case_types)]
                    struct UpsertSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::UpsertPoints>
                    for UpsertSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpsertPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).upsert(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpsertSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Delete" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::DeletePoints>
                    for DeleteSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).delete(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Get" => {
                    #[allow(non_camel_case_types)]
                    struct GetSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::GetPoints>
                    for GetSvc<T> {
                        type Response = super::GetResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).get(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/SetPayload" => {
                    #[allow(non_camel_case_types)]
                    struct SetPayloadSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::SetPayloadPoints>
                    for SetPayloadSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SetPayloadPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).set_payload(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SetPayloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/DeletePayload" => {
                    #[allow(non_camel_case_types)]
                    struct DeletePayloadSvc<T: Points>(pub Arc<T>);
                    impl<
                        T: Points,
                    > tonic::server::UnaryService<super::DeletePayloadPoints>
                    for DeletePayloadSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeletePayloadPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_payload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeletePayloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/ClearPayload" => {
                    #[allow(non_camel_case_types)]
                    struct ClearPayloadSvc<T: Points>(pub Arc<T>);
                    impl<
                        T: Points,
                    > tonic::server::UnaryService<super::ClearPayloadPoints>
                    for ClearPayloadSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ClearPayloadPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).clear_payload(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ClearPayloadSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/CreateFieldIndex" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFieldIndexSvc<T: Points>(pub Arc<T>);
                    impl<
                        T: Points,
                    > tonic::server::UnaryService<super::CreateFieldIndexCollection>
                    for CreateFieldIndexSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFieldIndexCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).create_field_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFieldIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/DeleteFieldIndex" => {
                    #[allow(non_camel_case_types)]
                    struct DeleteFieldIndexSvc<T: Points>(pub Arc<T>);
                    impl<
                        T: Points,
                    > tonic::server::UnaryService<super::DeleteFieldIndexCollection>
                    for DeleteFieldIndexSvc<T> {
                        type Response = super::PointsOperationResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::DeleteFieldIndexCollection>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).delete_field_index(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = DeleteFieldIndexSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Search" => {
                    #[allow(non_camel_case_types)]
                    struct SearchSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::SearchPoints>
                    for SearchSvc<T> {
                        type Response = super::SearchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).search(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/SearchBatch" => {
                    #[allow(non_camel_case_types)]
                    struct SearchBatchSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::SearchBatchPoints>
                    for SearchBatchSvc<T> {
                        type Response = super::SearchBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::SearchBatchPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).search_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = SearchBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Scroll" => {
                    #[allow(non_camel_case_types)]
                    struct ScrollSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::ScrollPoints>
                    for ScrollSvc<T> {
                        type Response = super::ScrollResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ScrollPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).scroll(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ScrollSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Recommend" => {
                    #[allow(non_camel_case_types)]
                    struct RecommendSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::RecommendPoints>
                    for RecommendSvc<T> {
                        type Response = super::RecommendResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecommendPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).recommend(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecommendSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/RecommendBatch" => {
                    #[allow(non_camel_case_types)]
                    struct RecommendBatchSvc<T: Points>(pub Arc<T>);
                    impl<
                        T: Points,
                    > tonic::server::UnaryService<super::RecommendBatchPoints>
                    for RecommendBatchSvc<T> {
                        type Response = super::RecommendBatchResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RecommendBatchPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).recommend_batch(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RecommendBatchSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Points/Count" => {
                    #[allow(non_camel_case_types)]
                    struct CountSvc<T: Points>(pub Arc<T>);
                    impl<T: Points> tonic::server::UnaryService<super::CountPoints>
                    for CountSvc<T> {
                        type Response = super::CountResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CountPoints>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).count(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CountSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Points> Clone for PointsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Points> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Points> tonic::transport::NamedService for PointsServer<T> {
        const NAME: &'static str = "qdrant.Points";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateFullSnapshotRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListFullSnapshotsRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotRequest {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsRequest {
    /// Name of the collection
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SnapshotDescription {
    /// Name of the snapshot
    #[prost(string, tag="1")]
    pub name: ::prost::alloc::string::String,
    /// Creation time of the snapshot
    #[prost(message, optional, tag="2")]
    pub creation_time: ::core::option::Option<::prost_types::Timestamp>,
    /// Size of the snapshot in bytes
    #[prost(int64, tag="3")]
    pub size: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateSnapshotResponse {
    #[prost(message, optional, tag="1")]
    pub snapshot_description: ::core::option::Option<SnapshotDescription>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListSnapshotsResponse {
    #[prost(message, repeated, tag="1")]
    pub snapshot_descriptions: ::prost::alloc::vec::Vec<SnapshotDescription>,
    /// Time spent to process
    #[prost(double, tag="2")]
    pub time: f64,
}
/// Generated client implementations.
pub mod snapshots_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct SnapshotsClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl SnapshotsClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> SnapshotsClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> SnapshotsClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            SnapshotsClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        ///
        ///Create collection snapshot
        pub async fn create(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateSnapshotRequest>,
        ) -> Result<tonic::Response<super::CreateSnapshotResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Snapshots/Create");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///List collection snapshots
        pub async fn list(
            &mut self,
            request: impl tonic::IntoRequest<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static("/qdrant.Snapshots/List");
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///Create full storage snapshot
        pub async fn create_full(
            &mut self,
            request: impl tonic::IntoRequest<super::CreateFullSnapshotRequest>,
        ) -> Result<tonic::Response<super::CreateSnapshotResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Snapshots/CreateFull",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
        ///
        ///List full storage snapshots
        pub async fn list_full(
            &mut self,
            request: impl tonic::IntoRequest<super::ListFullSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Snapshots/ListFull",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod snapshots_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with SnapshotsServer.
    #[async_trait]
    pub trait Snapshots: Send + Sync + 'static {
        ///
        ///Create collection snapshot
        async fn create(
            &self,
            request: tonic::Request<super::CreateSnapshotRequest>,
        ) -> Result<tonic::Response<super::CreateSnapshotResponse>, tonic::Status>;
        ///
        ///List collection snapshots
        async fn list(
            &self,
            request: tonic::Request<super::ListSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status>;
        ///
        ///Create full storage snapshot
        async fn create_full(
            &self,
            request: tonic::Request<super::CreateFullSnapshotRequest>,
        ) -> Result<tonic::Response<super::CreateSnapshotResponse>, tonic::Status>;
        ///
        ///List full storage snapshots
        async fn list_full(
            &self,
            request: tonic::Request<super::ListFullSnapshotsRequest>,
        ) -> Result<tonic::Response<super::ListSnapshotsResponse>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct SnapshotsServer<T: Snapshots> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Snapshots> SnapshotsServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for SnapshotsServer<T>
    where
        T: Snapshots,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qdrant.Snapshots/Create" => {
                    #[allow(non_camel_case_types)]
                    struct CreateSvc<T: Snapshots>(pub Arc<T>);
                    impl<
                        T: Snapshots,
                    > tonic::server::UnaryService<super::CreateSnapshotRequest>
                    for CreateSvc<T> {
                        type Response = super::CreateSnapshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Snapshots/List" => {
                    #[allow(non_camel_case_types)]
                    struct ListSvc<T: Snapshots>(pub Arc<T>);
                    impl<
                        T: Snapshots,
                    > tonic::server::UnaryService<super::ListSnapshotsRequest>
                    for ListSvc<T> {
                        type Response = super::ListSnapshotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Snapshots/CreateFull" => {
                    #[allow(non_camel_case_types)]
                    struct CreateFullSvc<T: Snapshots>(pub Arc<T>);
                    impl<
                        T: Snapshots,
                    > tonic::server::UnaryService<super::CreateFullSnapshotRequest>
                    for CreateFullSvc<T> {
                        type Response = super::CreateSnapshotResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CreateFullSnapshotRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).create_full(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CreateFullSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/qdrant.Snapshots/ListFull" => {
                    #[allow(non_camel_case_types)]
                    struct ListFullSvc<T: Snapshots>(pub Arc<T>);
                    impl<
                        T: Snapshots,
                    > tonic::server::UnaryService<super::ListFullSnapshotsRequest>
                    for ListFullSvc<T> {
                        type Response = super::ListSnapshotsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::ListFullSnapshotsRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move { (*inner).list_full(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = ListFullSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Snapshots> Clone for SnapshotsServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Snapshots> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Snapshots> tonic::transport::NamedService for SnapshotsServer<T> {
        const NAME: &'static str = "qdrant.Snapshots";
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckRequest {
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct HealthCheckReply {
    #[prost(string, tag="1")]
    pub title: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub version: ::prost::alloc::string::String,
}
/// Generated client implementations.
pub mod qdrant_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    #[derive(Debug, Clone)]
    pub struct QdrantClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl QdrantClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: std::convert::TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> QdrantClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> QdrantClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            QdrantClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with `gzip`.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_gzip(mut self) -> Self {
            self.inner = self.inner.send_gzip();
            self
        }
        /// Enable decompressing responses with `gzip`.
        #[must_use]
        pub fn accept_gzip(mut self) -> Self {
            self.inner = self.inner.accept_gzip();
            self
        }
        pub async fn health_check(
            &mut self,
            request: impl tonic::IntoRequest<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::HealthCheckReply>, tonic::Status> {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/qdrant.Qdrant/HealthCheck",
            );
            self.inner.unary(request.into_request(), path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod qdrant_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    ///Generated trait containing gRPC methods that should be implemented for use with QdrantServer.
    #[async_trait]
    pub trait Qdrant: Send + Sync + 'static {
        async fn health_check(
            &self,
            request: tonic::Request<super::HealthCheckRequest>,
        ) -> Result<tonic::Response<super::HealthCheckReply>, tonic::Status>;
    }
    #[derive(Debug)]
    pub struct QdrantServer<T: Qdrant> {
        inner: _Inner<T>,
        accept_compression_encodings: (),
        send_compression_encodings: (),
    }
    struct _Inner<T>(Arc<T>);
    impl<T: Qdrant> QdrantServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for QdrantServer<T>
    where
        T: Qdrant,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/qdrant.Qdrant/HealthCheck" => {
                    #[allow(non_camel_case_types)]
                    struct HealthCheckSvc<T: Qdrant>(pub Arc<T>);
                    impl<
                        T: Qdrant,
                    > tonic::server::UnaryService<super::HealthCheckRequest>
                    for HealthCheckSvc<T> {
                        type Response = super::HealthCheckReply;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::HealthCheckRequest>,
                        ) -> Self::Future {
                            let inner = self.0.clone();
                            let fut = async move {
                                (*inner).health_check(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = HealthCheckSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: Qdrant> Clone for QdrantServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
            }
        }
    }
    impl<T: Qdrant> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(self.0.clone())
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: Qdrant> tonic::transport::NamedService for QdrantServer<T> {
        const NAME: &'static str = "qdrant.Qdrant";
    }
}
