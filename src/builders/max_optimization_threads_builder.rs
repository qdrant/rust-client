use crate::qdrant::*;

impl From<u64> for MaxOptimizationThreads {
    fn from(threads: u64) -> Self {
        MaxOptimizationThreads {
            variant: Some(max_optimization_threads::Variant::from(threads)),
        }
    }
}

impl From<u64> for max_optimization_threads::Variant {
    fn from(threads: u64) -> Self {
        Self::Value(threads)
    }
}
