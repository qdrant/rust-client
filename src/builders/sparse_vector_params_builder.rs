use crate::qdrant::*;

pub struct SparseVectorParamsBuilder {
    /// Configuration of sparse index
    pub(crate) index:
        Option<Option<SparseIndexConfig>>,
    /// If set - apply modifier to the vector values
    pub(crate) modifier:
        Option<Option<i32>>,
}
