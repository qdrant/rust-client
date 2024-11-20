use crate::qdrant::*;

pub struct ScalarQuantizationBuilder {
    /// Type of quantization
    pub(crate) r#type: Option<i32>,
    /// Number of bits to use for quantization
    pub(crate) quantile: Option<Option<f32>>,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl ScalarQuantizationBuilder {
    /// Type of quantization
    #[allow(unused_mut)]
    pub fn r#type(self, value: i32) -> Self {
        let mut new = self;
        new.r#type = Option::Some(value);
        new
    }
    /// Number of bits to use for quantization
    #[allow(unused_mut)]
    pub fn quantile(self, value: f32) -> Self {
        let mut new = self;
        new.quantile = Option::Some(Option::Some(value));
        new
    }
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `ScalarQuantization`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<ScalarQuantization, ScalarQuantizationBuilderError> {
        Ok(ScalarQuantization {
            r#type: match self.r#type {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("r#type"),
                    ));
                }
            },
            quantile: match self.quantile {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            always_ram: match self.always_ram {
                Some(value) => value,
                None => core::default::Default::default(),
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            r#type: core::default::Default::default(),
            quantile: core::default::Default::default(),
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<ScalarQuantizationBuilder> for ScalarQuantization {
    fn from(value: ScalarQuantizationBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ScalarQuantizationBuilder", "ScalarQuantization",
        ))
    }
}

impl ScalarQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ScalarQuantization {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ScalarQuantizationBuilder", "ScalarQuantization",
        ))
    }
}

impl ScalarQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
