use crate::qdrant::*;

pub struct ProductQuantizationBuilder {
    /// Compression ratio
    pub(crate) compression: Option<i32>,
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    pub(crate) always_ram: Option<Option<bool>>,
}

impl ProductQuantizationBuilder {
    /// Compression ratio
    #[allow(unused_mut)]
    pub fn compression(self, value: i32) -> Self {
        let mut new = self;
        new.compression = Option::Some(value);
        new
    }
    /// If true - quantized vectors always will be stored in RAM, ignoring the config of main storage
    #[allow(unused_mut)]
    pub fn always_ram(self, value: bool) -> Self {
        let mut new = self;
        new.always_ram = Option::Some(Option::Some(value));
        new
    }
    /**Builds a new `ProductQuantization`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<ProductQuantization, ProductQuantizationBuilderError> {
        Ok(ProductQuantization {
            compression: match self.compression {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("compression"),
                    ));
                }
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
            compression: core::default::Default::default(),
            always_ram: core::default::Default::default(),
        }
    }
}

impl From<ProductQuantizationBuilder> for ProductQuantization {
    fn from(value: ProductQuantizationBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "ProductQuantizationBuilder", "ProductQuantization",
        ))
    }
}

impl ProductQuantizationBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> ProductQuantization {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "ProductQuantizationBuilder", "ProductQuantization",
        ))
    }
}

impl ProductQuantizationBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
