use crate::grpc_macros::convert_option;
use crate::qdrant::*;

pub struct OrderByBuilder {
    /// Payload key to order by
    pub(crate) key: Option<String>,
    /// Ascending or descending order
    pub(crate) direction: Option<Option<i32>>,
    /// Start from this value
    start_from: Option<start_from::Value>,
}

impl OrderByBuilder {
    /// Payload key to order by
    #[allow(unused_mut)]
    pub fn key(self, value: String) -> Self {
        let mut new = self;
        new.key = Option::Some(value);
        new
    }
    /// Ascending or descending order
    #[allow(unused_mut)]
    pub fn direction(self, value: i32) -> Self {
        let mut new = self;
        new.direction = Option::Some(Option::Some(value));
        new
    }
    /// Start from this value
    #[allow(unused_mut)]
    pub fn start_from<VALUE: core::convert::Into<start_from::Value>>(self, value: VALUE) -> Self {
        let mut new = self;
        new.start_from = Option::Some(value.into());
        new
    }
    /**Builds a new `OrderBy`.

    # Errors

    If a required field has not been initialized.
    */
    fn build_inner(self) -> Result<OrderBy, OrderByBuilderError> {
        Ok(OrderBy {
            key: match self.key {
                Some(value) => value,
                None => {
                    return Result::Err(core::convert::Into::into(
                        ::derive_builder::UninitializedFieldError::from("key"),
                    ));
                }
            },
            direction: match self.direction {
                Some(value) => value,
                None => core::default::Default::default(),
            },
            start_from: { convert_option(&self.start_from) },
        })
    }
    /// Create an empty builder, with all fields set to `None` or `PhantomData`.
    fn create_empty() -> Self {
        Self {
            key: core::default::Default::default(),
            direction: core::default::Default::default(),
            start_from: core::default::Default::default(),
        }
    }
}

impl From<OrderByBuilder> for OrderBy {
    fn from(value: OrderByBuilder) -> Self {
        value.build_inner().expect(&format!(
            "Failed to convert {0} to {1}",
            "OrderByBuilder", "OrderBy",
        ))
    }
}

impl OrderByBuilder {
    /// Builds the desired type. Can often be omitted.
    pub fn build(self) -> OrderBy {
        self.build_inner().expect(&format!(
            "Failed to build {0} into {1}",
            "OrderByBuilder", "OrderBy",
        ))
    }
}

impl OrderByBuilder {
    pub(crate) fn empty() -> Self {
        Self::create_empty()
    }
}
