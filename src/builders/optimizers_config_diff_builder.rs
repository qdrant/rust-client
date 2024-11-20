use crate::qdrant::*;

pub struct OptimizersConfigDiffBuilder {
    ///
    /// The minimal fraction of deleted vectors in a segment, required to perform segment optimization
    pub(crate) deleted_threshold: Option<Option<f64>>,
    ///
    /// The minimal number of vectors in a segment, required to perform segment optimization
    pub(crate) vacuum_min_vector_number: Option<Option<u64>>,
    ///
    /// Target amount of segments the optimizer will try to keep.
    /// Real amount of segments may vary depending on multiple parameters:
    ///
    /// - Amount of stored points.
    /// - Current write RPS.
    ///
    /// It is recommended to select the default number of segments as a factor of the number of search threads,
    /// so that each segment would be handled evenly by one of the threads.
    pub(crate) default_segment_number: Option<Option<u64>>,
    ///
    /// Do not create segments larger this size (in kilobytes).
    /// Large segments might require disproportionately long indexation times,
    /// therefore it makes sense to limit the size of segments.
    ///
    /// If indexing speed is more important - make this parameter lower.
    /// If search speed is more important - make this parameter higher.
    /// Note: 1Kb = 1 vector of size 256
    /// If not set, will be automatically selected considering the number of available CPUs.
    pub(crate) max_segment_size: Option<Option<u64>>,
    ///
    /// Maximum size (in kilobytes) of vectors to store in-memory per segment.
    /// Segments larger than this threshold will be stored as read-only memmaped file.
    ///
    /// Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.
    ///
    /// To disable memmap storage, set this to `0`.
    ///
    /// Note: 1Kb = 1 vector of size 256
    pub(crate) memmap_threshold: Option<Option<u64>>,
    ///
    /// Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing
    ///
    /// Default value is 20,000, based on <<https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md>.>
    ///
    /// To disable vector indexing, set to `0`.
    ///
    /// Note: 1kB = 1 vector of size 256.
    pub(crate) indexing_threshold: Option<Option<u64>>,
    ///
    /// Interval between forced flushes.
    pub(crate) flush_interval_sec: Option<Option<u64>>,
    ///
    /// Max number of threads (jobs) for running optimizations per shard.
    /// Note: each optimization job will also use `max_indexing_threads` threads by itself for index building.
    /// If null - have no limit and choose dynamically to saturate CPU.
    /// If 0 - no optimization threads, optimizations will be disabled.
    pub(crate) max_optimization_threads: Option<Option<u64>>,
}

impl OptimizersConfigDiffBuilder {
    ///
    /// The minimal fraction of deleted vectors in a segment, required to perform segment optimization
    #[allow(unused_mut)]
    pub fn deleted_threshold(self, value: f64) -> Self {
        let mut new = self;
        new.deleted_threshold = Option::Some(Option::Some(value));
        new
    }
    ///
    /// The minimal number of vectors in a segment, required to perform segment optimization
    #[allow(unused_mut)]
    pub fn vacuum_min_vector_number(self, value: u64) -> Self {
        let mut new = self;
        new.vacuum_min_vector_number = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Target amount of segments the optimizer will try to keep.
    /// Real amount of segments may vary depending on multiple parameters:
    ///
    /// - Amount of stored points.
    /// - Current write RPS.
    ///
    /// It is recommended to select the default number of segments as a factor of the number of search threads,
    /// so that each segment would be handled evenly by one of the threads.
    #[allow(unused_mut)]
    pub fn default_segment_number(self, value: u64) -> Self {
        let mut new = self;
        new.default_segment_number = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Do not create segments larger this size (in kilobytes).
    /// Large segments might require disproportionately long indexation times,
    /// therefore it makes sense to limit the size of segments.
    ///
    /// If indexing speed is more important - make this parameter lower.
    /// If search speed is more important - make this parameter higher.
    /// Note: 1Kb = 1 vector of size 256
    /// If not set, will be automatically selected considering the number of available CPUs.
    #[allow(unused_mut)]
    pub fn max_segment_size(self, value: u64) -> Self {
        let mut new = self;
        new.max_segment_size = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Maximum size (in kilobytes) of vectors to store in-memory per segment.
    /// Segments larger than this threshold will be stored as read-only memmaped file.
    ///
    /// Memmap storage is disabled by default, to enable it, set this threshold to a reasonable value.
    ///
    /// To disable memmap storage, set this to `0`.
    ///
    /// Note: 1Kb = 1 vector of size 256
    #[allow(unused_mut)]
    pub fn memmap_threshold(self, value: u64) -> Self {
        let mut new = self;
        new.memmap_threshold = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Maximum size (in kilobytes) of vectors allowed for plain index, exceeding this threshold will enable vector indexing
    ///
    /// Default value is 20,000, based on <<https://github.com/google-research/google-research/blob/master/scann/docs/algorithms.md>.>
    ///
    /// To disable vector indexing, set to `0`.
    ///
    /// Note: 1kB = 1 vector of size 256.
    #[allow(unused_mut)]
    pub fn indexing_threshold(self, value: u64) -> Self {
        let mut new = self;
        new.indexing_threshold = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Interval between forced flushes.
    #[allow(unused_mut)]
    pub fn flush_interval_sec(self, value: u64) -> Self {
        let mut new = self;
        new.flush_interval_sec = Option::Some(Option::Some(value));
        new
    }
    ///
    /// Max number of threads (jobs) for running optimizations per shard.
    /// Note: each optimization job will also use `max_indexing_threads` threads by itself for index building.
    /// If null - have no limit and choose dynamically to saturate CPU.
    /// If 0 - no optimization threads, optimizations will be disabled.
    #[allow(unused_mut)]
    pub fn max_optimization_threads(self, value: u64) -> Self {
        let mut new = self;
        new.max_optimization_threads = Option::Some(Option::Some(value));
        new
    }

    fn build_inner(self) -> Result<OptimizersConfigDiff, std::convert::Infallible> {
        Ok(OptimizersConfigDiff {
            deleted_threshold: match self.deleted_threshold {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            vacuum_min_vector_number: match self.vacuum_min_vector_number {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            default_segment_number: match self.default_segment_number {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_segment_size: match self.max_segment_size {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            memmap_threshold: match self.memmap_threshold {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            indexing_threshold: match self.indexing_threshold {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            flush_interval_sec: match self.flush_interval_sec {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            max_optimization_threads: match self.max_optimization_threads {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            deleted_threshold: core::default::Default::default(),
            vacuum_min_vector_number: core::default::Default::default(),
            default_segment_number: core::default::Default::default(),
            max_segment_size: core::default::Default::default(),
            memmap_threshold: core::default::Default::default(),
            indexing_threshold: core::default::Default::default(),
            flush_interval_sec: core::default::Default::default(),
            max_optimization_threads: core::default::Default::default(),
        }
    }
}

impl From<OptimizersConfigDiffBuilder> for OptimizersConfigDiff {
    fn from(value: OptimizersConfigDiffBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "OptimizersConfigDiffBuilder", "OptimizersConfigDiff",
        ))
    }
}

impl OptimizersConfigDiffBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> OptimizersConfigDiff {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "OptimizersConfigDiffBuilder", "OptimizersConfigDiff",
        ))
    }
}

impl Default for OptimizersConfigDiffBuilder {
    fn default() -> Self {
        Self::create_empty()
    }
}
