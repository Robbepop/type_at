mod derive_struct;
mod impl_tuples;

use proc_macro::TokenStream;

#[proc_macro]
#[doc(hidden)]
pub fn impl_type_at_for_tuples(input: TokenStream) -> TokenStream {
    assert!(input.is_empty(), "macro does not accept any input");
    impl_tuples::impl_type_at_for_tuples().into()
}

/// Derives an implementation for the [`ConstDefault`] trait.
///
/// # Note
///
/// Only works with `struct` type inputs.
///
/// # Example
///
/// ## Struct
///
/// ```
/// # use type_at::TypeAt;
/// #[derive(TypeAt)]
/// # #[derive(Debug, PartialEq)]
/// pub struct Struct {
///     a: i8,
///     b: i16,
///     c: i32,
/// }
///
/// assert_eq!(
///     let _: i8  = <Struct as TypeAt<0>>::Type::default();
///     let _: i16 = <Struct as TypeAt<1>>::Type::default();
///     let _: i32 = <Struct as TypeAt<2>>::Type::default();
/// )
/// ```
///
/// ## Tuple Struct
///
/// ```
/// # use type_at::TypeAt;
/// #[derive(TypeAt)]
/// # #[derive(Debug, PartialEq)]
/// pub struct TupleStruct(i8, i16, i32);
///
/// assert_eq!(
///     let _: i8  = <TupleStruct as TypeAt<0>>::Type::default();
///     let _: i16 = <TupleStruct as TypeAt<1>>::Type::default();
///     let _: i32 = <TupleStruct as TypeAt<2>>::Type::default();
/// )
/// ```
#[proc_macro_derive(TypeAt)]
pub fn derive_type_at(input: TokenStream) -> TokenStream {
    match derive_struct::derive_type_at(input.into()) {
        Ok(output) => output.into(),
        Err(error) => error.to_compile_error().into(),
    }
}
