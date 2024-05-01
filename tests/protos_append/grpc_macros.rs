/// Default type conversions between builders and their built types. This allows us not writing .build() all the time, especially if we use builders within buliders.
///
/// This also adds a build() function for all builders. The build() function automatically generated by derive_builder returns
/// a Result which is ugly to have to unwrap() all the time, especially because they can't fail in our case.
macro_rules! builder_type_conversions {
    ($main_type:ident,$builder_type:ident,$build_fn:ident) => {
        impl From<$builder_type> for $main_type {
            fn from(value: $builder_type) -> Self {
                value.$build_fn().unwrap()
            }
        }

        impl From<&mut $builder_type> for $main_type {
            fn from(value: &mut $builder_type) -> Self {
                value.clone().$build_fn().unwrap()
            }
        }

        impl $builder_type {
            /// Builds the desired type. Can often be omitted.
            pub fn build(&self) -> $main_type {
                self.$build_fn().unwrap()
            }
        }
    };

    // Small shortcut so we don't have to write build_inner for most of the types.
    ($main_type:ident,$builder_type:ident) => {
        builder_type_conversions!($main_type, $builder_type, build_inner);
    };
}

/// Helper function to convert from &Option<T> to Option<U> if U: From<T> is satisfied.
fn convert_option<T, U>(input: &Option<T>) -> Option<U>
where
    U: From<T>,
    T: Clone,
{
    let input = input.as_ref()?;
    Some(input.clone().into())
}
