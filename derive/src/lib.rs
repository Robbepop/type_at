mod impl_tuples;

use proc_macro::TokenStream;

#[proc_macro]
pub fn impl_type_at_for_tuples(input: TokenStream) -> TokenStream {
    assert!(input.is_empty(), "macro does not accept any input");
    impl_tuples::impl_type_at_for_tuples().into()
}
