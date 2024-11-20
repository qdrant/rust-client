use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct WithLookupBuilder {
    /// Name of the collection to use for points lookup
    pub(crate) collection: Option<String>,
    /// Options for specifying which payload to include (or not)
    with_payload: Option<with_payload_selector::SelectorOptions>,
    /// Options for specifying which vectors to include (or not)
    with_vectors: Option<with_vectors_selector::SelectorOptions>,
}

impl WithLookupBuilder {
    /// Name of the collection to use for points lookup
    #[allow(unused_mut)]
    pub fn collection(self, value: String) -> Self {
        let mut new = self;
        new.collection = Option::Some(value);
        new
    }
    /// Options for specifying which payload to include (or not)
    #[allow(unused_mut)]
    pub fn with_payload<VALUE: core::convert::Into<with_payload_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_payload = Option::Some(value.into());
        new
    }
    /// Options for specifying which vectors to include (or not)
    #[allow(unused_mut)]
    pub fn with_vectors<VALUE: core::convert::Into<with_vectors_selector::SelectorOptions>>(
        self,
        value: VALUE,
    ) -> Self {
        let mut new = self;
        new.with_vectors = Option::Some(value.into());
        new
    }
    /**Builds a new `WithLookup`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<WithLookup, WithLookupBuilderError> {
        Ok(WithLookup {
            collection: match self.collection {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("collection"),
                    ));
                }
            },
            with_payload: { convert_option(&self.with_payload) },
            with_vectors: { convert_option(&self.with_vectors) },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            collection: core::default::Default::default(),
            with_payload: core::default::Default::default(),
            with_vectors: core::default::Default::default(),
        }
    }
}

impl From<WithLookupBuilder> for WithLookup {
    fn from(value: WithLookupBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "WithLookupBuilder", "WithLookup",
        ))
    }
}

impl WithLookupBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> WithLookup {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "WithLookupBuilder", "WithLookup",
        ))
    }
}

impl WithLookupBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
