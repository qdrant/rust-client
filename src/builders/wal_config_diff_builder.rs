use crate::qdrant::*;

pub struct WalConfigDiffBuilder {
    /// Size of a single WAL block file
    pub(crate) wal_capacity_mb: Option<Option<u64>>,
    /// Number of segments to create in advance
    pub(crate) wal_segments_ahead: Option<Option<u64>>,
}

impl WalConfigDiffBuilder {
    /// Size of a single WAL block file
    #[allow(unused_mut)]
    pub fn wal_capacity_mb(self, value: u64) -> Self {
        let mut new = self;
        new.wal_capacity_mb = Option::Some(Option::Some(value));
        new
    }
    /// Number of segments to create in advance
    #[allow(unused_mut)]
    pub fn wal_segments_ahead(self, value: u64) -> Self {
        let mut new = self;
        new.wal_segments_ahead = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<WalConfigDiff, std::convert::Infallible> {
        Ok(WalConfigDiff {
            wal_capacity_mb: self.wal_capacity_mb.unwrap_or_default(),
            wal_segments_ahead: self.wal_segments_ahead.unwrap_or_default(),
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            wal_capacity_mb: core::default::Default::default(),
            wal_segments_ahead: core::default::Default::default(),
        }
    }
}

impl Default for WalConfigDiffBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}

impl From<WalConfigDiffBuilder> for WalConfigDiff {
    fn from(value: WalConfigDiffBuilder) -> Self {
        value.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to convert {0} to {1}",
                "WalConfigDiffBuilder", "WalConfigDiff"
            )
        })
    }
}

impl WalConfigDiffBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> WalConfigDiff {
        self.build_inner().unwrap_or_else(|_| {
            panic!(
                "Failed to build {0} into {1}",
                "WalConfigDiffBuilder", "WalConfigDiff"
            )
        })
    }
}
