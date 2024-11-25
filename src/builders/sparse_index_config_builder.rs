use crate::qdrant::*;

pub struct SparseIndexConfigBuilder {
    ///
    /// Prefer a full scan search upto (excluding) this number of vectors.
    /// Note: this is number of vectors, not KiloBytes.
    pub(crate) full_scan_threshold: Option<Option<u64>>,
    ///
    /// Store inverted index on disk. If set to false, the index will be stored in RAM.
    pub(crate) on_disk: Option<Option<bool>>,
    ///
    /// Datatype used to store weights in the index.
    pub(crate) datatype: Option<Option<i32>>,
}

impl SparseIndexConfigBuilder {
    ///
    /// Prefer a full scan search upto (excluding) this number of vectors.
    /// Note: this is number of vectors, not KiloBytes.
    #[allow(unused_mut)]
    pub fn full_scan_threshold<VALUE: core::convert::Into<u64>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.full_scan_threshold = Option::Some(Option::Some(value.into()));
        new
    }
    ///
    /// Store inverted index on disk. If set to false, the index will be stored in RAM.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Datatype used to store weights in the index.
    #[allow(unused_mut)]
    pub fn datatype<VALUE: core::convert::Into<i32>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.datatype = Option::Some(Option::Some(value.into()));
        new
    }

    fn build_inner(self) -> Result<SparseIndexConfig, std::convert::Infallible> {
        Ok(SparseIndexConfig {
            full_scan_threshold: self.full_scan_threshold.unwrap_or_default(),
            on_disk: self.on_disk.unwrap_or_default(),
            datatype: self.datatype.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            full_scan_threshold: core::default::Default::default(),
            on_disk: core::default::Default::default(),
            datatype: core::default::Default::default(),
        }
    }
}

impl From<SparseIndexConfigBuilder> for SparseIndexConfig {
    fn from(value: SparseIndexConfigBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "SparseIndexConfigBuilder", "SparseIndexConfig"
            )
        })
    }
}

impl SparseIndexConfigBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> SparseIndexConfig {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "SparseIndexConfigBuilder", "SparseIndexConfig"
            )
        })
    }
}

impl Default for SparseIndexConfigBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
