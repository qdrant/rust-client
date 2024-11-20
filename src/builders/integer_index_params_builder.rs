use crate::qdrant::*;

pub struct IntegerIndexParamsBuilder {
    /// If true - support direct lookups.
    pub(crate) lookup: Option<Option<bool>>,
    /// If true - support ranges filters.
    pub(crate) range: Option<Option<bool>>,
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    pub(crate) is_principal: Option<Option<bool>>,
    /// If true - store index on disk.
    pub(crate) on_disk: Option<Option<bool>>,
}

impl IntegerIndexParamsBuilder {
    /// If true - support direct lookups.
    #[allow(unused_mut)]
    pub fn lookup<VALUE: core::convert::Into<bool>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.lookup = Option::Some(Option::Some(value.into()));
        new
    }
    /// If true - support ranges filters.
    #[allow(unused_mut)]
    pub fn range<VALUE: core::convert::Into<bool>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.range = Option::Some(Option::Some(value.into()));
        new
    }
    /// If true - use this key to organize storage of the collection data. This option assumes that this key will be used in majority of filtered requests.
    #[allow(unused_mut)]
    pub fn is_principal(self, value: bool) -> Self {
        let mut new = self;
        new.is_principal = Option::Some(Option::Some(value));
        new
    }
    /// If true - store index on disk.
    #[allow(unused_mut)]
    pub fn on_disk(self, value: bool) -> Self {
        let mut new = self;
        new.on_disk = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `IntegerIndexParams`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<IntegerIndexParams, IntegerIndexParamsBuilderError> {
        Ok(IntegerIndexParams {
            lookup: match self.lookup {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            range: match self.range {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            is_principal: match self.is_principal {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            on_disk: match self.on_disk {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            lookup: core::default::Default::default(),
            range: core::default::Default::default(),
            is_principal: core::default::Default::default(),
            on_disk: core::default::Default::default(),
        }
    }
}

impl From<IntegerIndexParamsBuilder> for IntegerIndexParams {
    fn from(value: IntegerIndexParamsBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "IntegerIndexParamsBuilder", "IntegerIndexParams",
        ))
    }
}

impl IntegerIndexParamsBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> IntegerIndexParams {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "IntegerIndexParamsBuilder", "IntegerIndexParams",
        ))
    }
}

impl IntegerIndexParamsBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
