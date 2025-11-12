use crate::qdrant::*;

#[derive(Clone)]
pub struct HnswConfigDiffBuilder {
    ///
    /// Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
    pub(crate) m: Option<Option<u64>>,
    ///
    /// Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index.
    pub(crate) ef_construct: Option<Option<u64>>,
    ///
    /// Minimal size (in KiloBytes) of vectors for additional payload-based indexing.
    /// If the payload chunk is smaller than `full_scan_threshold` additional indexing won't be used -
    /// in this case full-scan search should be preferred by query planner and additional indexing is not required.
    /// Note: 1 Kb = 1 vector of size 256
    pub(crate) full_scan_threshold: Option<Option<u64>>,
    ///
    /// Number of parallel threads used for background index building.
    /// If 0 - automatically select from 8 to 16.
    /// Best to keep between 8 and 16 to prevent likelihood of building broken/inefficient HNSW graphs.
    /// On small CPUs, less threads are used.
    pub(crate) max_indexing_threads: Option<Option<u64>>,
    ///
    /// Store HNSW index on disk. If set to false, the index will be stored in RAM.
    pub(crate) on_disk: Option<Option<bool>>,
    ///
    /// Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used.
    pub(crate) payload_m: Option<Option<u64>>,
    ///
    /// Store copies of original and quantized vectors within the HNSW index file. Default: false.
    /// Enabling this option will trade the search speed for disk usage by reducing amount of
    /// random seeks during the search.
    /// Requires quantized vectors to be enabled. Multi-vectors are not supported.
    pub(crate) inline_storage: Option<Option<bool>>,
}
#[allow(clippy::all)]
#[allow(clippy::derive_partial_eq_without_eq)]
impl HnswConfigDiffBuilder {
    ///
    /// Number of edges per node in the index graph. Larger the value - more accurate the search, more space required.
    pub fn m(self, value: u64) -> Self {
        let mut new = self;
        new.m = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Number of neighbours to consider during the index building. Larger the value - more accurate the search, more time required to build the index.
    pub fn ef_construct(self, value: u64) -> Self {
        let mut new = self;
        new.ef_construct = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Minimal size (in KiloBytes) of vectors for additional payload-based indexing.
    /// If the payload chunk is smaller than `full_scan_threshold` additional indexing won't be used -
    /// in this case full-scan search should be preferred by query planner and additional indexing is not required.
    /// Note: 1 Kb = 1 vector of size 256
    pub fn full_scan_threshold(self, value: u64) -> Self {
        let mut new = self;
        new.full_scan_threshold = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Number of parallel threads used for background index building.
    /// If 0 - automatically select from 8 to 16.
    /// Best to keep between 8 and 16 to prevent likelihood of building broken/inefficient HNSW graphs.
    /// On small CPUs, less threads are used.
    pub fn max_indexing_threads(self, value: u64) -> Self {
        let mut new = self;
        new.max_indexing_threads = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Store HNSW index on disk. If set to false, the index will be stored in RAM.
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Number of additional payload-aware links per node in the index graph. If not set - regular M parameter will be used.
    pub fn payload_m(self, value: u64) -> Self {
        let mut new = self;
        new.payload_m = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Store copies of original and quantized vectors within the HNSW index file. Default: false.
    /// Enabling this option will trade the search speed for disk usage by reducing amount of
    /// random seeks during the search.
    /// Requires quantized vectors to be enabled. Multi-vectors are not supported.
    pub fn inline_storage(self, value: bool) -> Self {
        let mut new = self;
        new.inline_storage = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<HnswConfigDiff, std::convert::Infallible> {
        Ok(HnswConfigDiff {
            m: match self.m {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            ef_construct: match self.ef_construct {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            full_scan_threshold: match self.full_scan_threshold {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_indexing_threads: match self.max_indexing_threads {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            payload_m: match self.payload_m {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            inline_storage: match self.inline_storage {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            m: core::default::Default::default(),
            ef_construct: core::default::Default::default(),
            full_scan_threshold: core::default::Default::default(),
            max_indexing_threads: core::default::Default::default(),
            on_disk: core::default::Default::default(),
            payload_m: core::default::Default::default(),
            inline_storage: core::default::Default::default(),
        }
    }
}

impl From<HnswConfigDiffBuilder> for HnswConfigDiff {
    fn from(value: HnswConfigDiffBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "HnswConfigDiffBuilder", "HnswConfigDiff"
            )
        })
    }
}

impl HnswConfigDiffBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> HnswConfigDiff {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "HnswConfigDiffBuilder", "HnswConfigDiff"
            )
        })
    }
}

impl Default for HnswConfigDiffBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
