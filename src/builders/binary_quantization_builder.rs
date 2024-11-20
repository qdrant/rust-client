use crate::qdrant::*;

pub struct BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl BinaryQuantizationBuilder {
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `BinaryQuantization`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<BinaryQuantization, BinaryQuantizationBuilderError> {
        Ok(BinaryQuantization {
            always_ram: match self.always_ram {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<BinaryQuantizationBuilder> for BinaryQuantization {
    fn from(value: BinaryQuantizationBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "BinaryQuantizationBuilder", "BinaryQuantization",
        ))
    }
}

impl BinaryQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> BinaryQuantization {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "BinaryQuantizationBuilder", "BinaryQuantization",
        ))
    }
}

impl BinaryQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
