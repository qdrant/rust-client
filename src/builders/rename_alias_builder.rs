use crate::qdrant::*;

pub struct RenameAliasBuilder {
    /// Name of the alias to rename
    pub(crate) old_alias_name: Option<String>,
    /// Name of the alias
    pub(crate) new_alias_name: Option<String>,
}

impl RenameAliasBuilder {
    /// Name of the alias to rename
    #[allow(unused_mut)]
    pub fn old_alias_name(self, value: String) -> Self {
        let mut new = self;
        new.old_alias_name = Option::Some(value);
        new
    }
    /// Name of the alias
    #[allow(unused_mut)]
    pub fn new_alias_name(self, value: String) -> Self {
        let mut new = self;
        new.new_alias_name = Option::Some(value);
        new
    }
    /**Builds a new `RenameAlias`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<RenameAlias, RenameAliasBuilderError> {
        Ok(RenameAlias {
            old_alias_name: match self.old_alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("old_alias_name"),
                    ));
                }
            },
            new_alias_name: match self.new_alias_name {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("new_alias_name"),
                    ));
                }
            },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            old_alias_name: core::default::Default::default(),
            new_alias_name: core::default::Default::default(),
        }
    }
}

impl From<RenameAliasBuilder> for RenameAlias {
    fn from(value: RenameAliasBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "RenameAliasBuilder", "RenameAlias",
        ))
    }
}

impl RenameAliasBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> RenameAlias {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "RenameAliasBuilder", "RenameAlias",
        ))
    }
}

impl RenameAliasBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
