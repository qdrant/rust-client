use crate::qdrant::*;

/// Max number of threads (jobs) for running optimizations per shard.
/// Each optimization job will also use `max_indexing_threads` threads by itself for index building.
///
/// - If `auto` - have no limit and choose dynamically to saturate CPU.
/// - If `disabled` or `0` - no optimization threads, optimizations will be disabled.
#[derive(Clone)]
pub struct MaxOptimizationThreadsBuilder {
    pub(crate) inner: MaxOptimizationThreads,
}

impl MaxOptimizationThreadsBuilder {
    /// Use specific number of optimization threads.
    ///
    /// - If `0` - no optimization threads, optimizations will be disabled.
    #[allow(unused_mut)]
    #[inline]
    pub fn threads(threads: u64) -> Self {
        Self {
            inner: MaxOptimizationThreads::from(threads),
        }
    }

    /// No optimization threads, optimizations will be disabled.
    #[allow(unused_mut)]
    #[inline]
    pub fn disabled() -> Self {
        Self::threads(0)
    }

    /// Have no limit and choose dynamically to saturate CPU.
    #[allow(unused_mut)]
    #[inline]
    pub fn auto() -> Self {
        Self {
            inner: MaxOptimizationThreads::from(max_optimization_threads::Variant::Setting(
                max_optimization_threads::Setting::Auto as i32,
            )),
        }
    }
}

impl From<MaxOptimizationThreadsBuilder> for MaxOptimizationThreads {
    fn from(value: MaxOptimizationThreadsBuilder) -> Self {
        value.build()
    }
}

impl MaxOptimizationThreadsBuilder {
    pub fn build(self) -> MaxOptimizationThreads {
        self.inner
    }
}

impl Default for MaxOptimizationThreadsBuilder {
    fn default() -> Self {
        Self::auto()
    }
}

impl From<u64> for MaxOptimizationThreads {
    fn from(threads: u64) -> Self {
        MaxOptimizationThreads {
            variant: Some(max_optimization_threads::Variant::from(threads)),
        }
    }
}

impl From<max_optimization_threads::Variant> for MaxOptimizationThreads {
    fn from(setting: max_optimization_threads::Variant) -> Self {
        MaxOptimizationThreads {
            variant: Some(setting),
        }
    }
}

impl From<u64> for max_optimization_threads::Variant {
    fn from(threads: u64) -> Self {
        Self::Value(threads)
    }
}

impl From<max_optimization_threads::Setting> for max_optimization_threads::Variant {
    fn from(setting: max_optimization_threads::Setting) -> Self {
        Self::Setting(setting as i32)
    }
}
