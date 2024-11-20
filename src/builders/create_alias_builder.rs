use crate::qdrant::*;

pub struct CreateAliasBuilder {
    /// Name of the collection
    pub(crate) collection_name: Option<String>,
    /// New name of the alias
    pub(crate) alias_name: Option<String>,
}

impl CreateAliasBuilder {
    /// Name of the collection
    #[allow(unused_mut)]
    pub fn collection_name(self, value: String) -> Self {
        let mut new = self;
        new.collection_name = Option::Some(value);
        new
    }
    /// New name of the alias
    #[allow(unused_mut)]
    pub fn alias_name(self, value: String) -> Self {
        let mut new = self;
        new.alias_name = Option::Some(value);
        new
    }
    /**Builds a new `CreateAlias`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<CreateAlias, CreateAliasBuilderError> {
        Ok(CreateAlias {
            collection_name: match self.collection_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection_name"),
                    ));
                }
            },
            alias_name: match self.alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("alias_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection_name: core::default::Default::default(),
            alias_name: core::default::Default::default(),
        }
    }
}

impl From<CreateAliasBuilder> for CreateAlias {
    fn from(value: CreateAliasBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "CreateAliasBuilder", "CreateAlias",
        ))
    }
}

impl CreateAliasBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> CreateAlias {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "CreateAliasBuilder", "CreateAlias",
        ))
    }
}

impl CreateAliasBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
